pub struct CheckedNumbers {
    previous_number: Option<i32>,
    pub current_number: Option<i32>,
}

impl CheckedNumbers {
    pub fn new(previous_number: Option<i32>, current_number: Option<i32>) -> Self {
        CheckedNumbers {
            previous_number,
            current_number,
        }
    }

    pub fn are_both_absent(&self) -> bool {
        self.previous_number.is_none() && self.current_number.is_none()
    }

    pub fn is_current_invalid_starting_number(&self) -> bool {
        self.previous_number.is_none() && self.current_number.unwrap_or(0) != 1
    }

    pub fn is_current_number_absent(&self) -> bool {
        self.current_number.is_none()
    }

    pub fn is_current_number_incorrect(&self) -> bool {
        self.previous_number.is_some()
            && self.current_number.unwrap() - 1 != self.previous_number.unwrap()
    }
}
