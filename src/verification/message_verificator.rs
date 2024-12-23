use std::sync::Arc;

use serenity::{
    async_trait,
    futures::lock::Mutex,
    gateway::ActivityData,
    model::prelude::{Message, Ready},
    prelude::{Context, EventHandler},
};
use tracing::{
    log::{error, info},
    warn,
};

use crate::{
    discord::{
        message_fetcher::MessageFetcher,
        message_sender::MessageSender,
        message_utils::{self, contains_number, extract_number_from_message},
    },
    globals::Globals,
};

use super::{
    checked_numbers::CheckedNumbers, error_handler::ErrorHandler,
    gameover_manager::GameoverManager, message_error::MessageError, prize_manager::PrizeManager,
};

pub struct MessageVerificator {
    message_fetcher: MessageFetcher,
    prize_manager: PrizeManager,
    globals: Arc<Globals>,
    error_handler: ErrorHandler,
    gameover_manager: GameoverManager,
    mutex: Arc<Mutex<bool>>,
}

impl MessageVerificator {
    pub fn new(globals: Arc<Globals>) -> Self {
        let message_sender = Arc::new(MessageSender::new(Arc::clone(&globals)));
        MessageVerificator {
            message_fetcher: MessageFetcher::new(globals.message_fetch_limit),
            prize_manager: PrizeManager::new(Arc::clone(&globals), Arc::clone(&message_sender)),
            globals: Arc::clone(&globals),
            error_handler: ErrorHandler::new(Arc::clone(&message_sender)),
            gameover_manager: GameoverManager::new(
                Arc::clone(&globals),
                Arc::clone(&message_sender),
            ),
            mutex: Arc::new(Mutex::new(false)),
        }
    }

    async fn verify_message(&self, msg: &Message, context: &Context) -> Result<(), MessageError> {
        let messages = self
            .message_fetcher
            .get_last_messages(msg, &context.http)
            .await
            .unwrap();

        let checked_numbers = CheckedNumbers::new(
            messages.first().and_then(extract_number_from_message),
            extract_number_from_message(msg),
        );

        let author_name = msg
            .author_nick(context)
            .await
            .unwrap_or(msg.author.name.to_owned());

        if checked_numbers.are_both_absent() {
            if MessageVerificator::all_message_does_not_contain_numbers(messages) {
                warn!("Skipping further validation as counting doesn't start yet");
                return Ok(());
            } else {
                error!("Something really bad happen: two messages without numbers when there are other numbers in channel!");
                return Err(MessageError::WrongFormat);
            }
        }
        if checked_numbers.is_current_invalid_starting_number() {
            warn!(
                "{} tried to start game with value higher than 1!",
                author_name
            );
            return Err(MessageError::WrongNumber);
        }
        if checked_numbers.is_current_number_absent() {
            warn!("{} sent message not starting with number.", author_name);
            return Err(MessageError::WrongFormat);
        }
        if checked_numbers.is_current_number_incorrect() {
            warn!("{} posted wrong number!", author_name);
            return Err(MessageError::WrongNumber);
        }

        self.prize_manager
            .add_role_for_prized_number(msg, context, checked_numbers.current_number.unwrap())
            .await;

        self.gameover_manager
            .check_for_game_over(msg, context, checked_numbers.current_number.unwrap())
            .await;

        Ok(())
    }

    fn all_message_does_not_contain_numbers(messages: Vec<Message>) -> bool {
        messages.iter().all(|m| !contains_number(m))
    }
}

#[async_trait]
impl EventHandler for MessageVerificator {
    async fn ready(&self, context: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
        context.set_activity(Some(ActivityData::watching("grę w cyferki")))
    }

    async fn message(&self, context: Context, msg: Message) {
        let _guard = self.mutex.lock().await;
        let channel_name = msg.channel_id.name(&context).await.unwrap();
        if channel_name == self.globals.watched_channel && message_utils::is_from_user(&msg) {
            info!(
                "Veryfing message={} sent to channel={} by={}",
                msg.content, channel_name, msg.author.name
            );

            match self.verify_message(&msg, &context).await {
                Ok(_) => info!(
                    "Finished verification of message={} from={}",
                    msg.content, msg.author.name
                ),
                Err(error) => {
                    self.error_handler.handle_error(error, &msg, &context).await;
                }
            }
        }
    }
}
