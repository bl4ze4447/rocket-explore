use std::path::PathBuf;
use crate::error_modal::ErrorModal;
use crate::select_action::SelectionResult;

pub struct DeleteAction {
    pub file_list:      SelectionResult,
    pub error_modal:    ErrorModal,
    pub show_window:    bool,
}

impl DeleteAction {
    pub fn new() -> Self {
        Self {
            show_window:    false,
            file_list:      SelectionResult::Single(PathBuf::new()),
            error_modal:    ErrorModal::new("Delete Error".to_owned()),
        }
    }
    pub fn clear(&mut self) {
        let old_modal       =   self.error_modal.clone();
        *self               =   DeleteAction::new();
        self.error_modal    =   old_modal;
    }
}