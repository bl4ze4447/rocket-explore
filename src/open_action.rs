use std::path::PathBuf;
use crate::error_modal::ErrorModal;
use crate::select_action::SelectionResult;

pub struct OpenAction {
    pub file_list:      SelectionResult,
    pub error_modal:    ErrorModal,
    pub open:           bool,
}
impl OpenAction {
    pub fn new() -> Self {
        Self {
            file_list:      SelectionResult::Single(PathBuf::new()),
            error_modal:    ErrorModal::new("Open Error".to_owned()),
            open:           false,
        }
    }
    pub fn clear(&mut self) {
        let old_modal       =   self.error_modal.clone();
        *self               =   OpenAction::new();
        self.error_modal    =   old_modal;
    }
}