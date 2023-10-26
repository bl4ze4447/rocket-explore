use std::path::PathBuf;
use crate::error_modal::ErrorModal;
use crate::select_action::SelectionResult;

pub struct MoveAction {
    pub show_window:    bool,
    pub paste:          bool,
    pub from:           SelectionResult,
    pub to:             PathBuf,
    pub percentage:     u8,
    pub error_modal:    ErrorModal,
}
impl MoveAction {
    pub fn new() -> Self {
        Self {
            show_window:    false,
            paste:          false,
            from:           SelectionResult::Single(PathBuf::new()),
            to:             PathBuf::new(),
            percentage:     0,
            error_modal:    ErrorModal::new(String::from("Move Error")),
        }
    }
    pub fn increment_percentage(&mut self) {
        self.percentage = (self.percentage + 1).clamp(0, 10);
    }
    pub fn increment_percentage_by_value(&mut self, value: u8) {
        self.percentage = (self.percentage + value).clamp(0, 100);
    }
    pub fn completed(&self) -> bool {
        self.percentage == 100
    }

    // Does not clear error modal
    pub fn clear(&mut self) {
        let old_modal      =   self.error_modal.clone();
        *self               =   MoveAction::new();
        self.error_modal    =   old_modal;
    }
}