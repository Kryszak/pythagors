use super::message_error::MessageError;

pub struct ErrorHandler;

impl ErrorHandler {
    pub fn new() -> Self {
        return ErrorHandler {};
    }

    pub async fn handle_error(&self, error: MessageError) {
        match error {
            MessageError::WRONG_FORMAT => {
                // TODO handle wrong message format
            }
            MessageError::WRONG_NUMBER => {
                // TODO handle wrong number in message
            }
        }
    }
}
