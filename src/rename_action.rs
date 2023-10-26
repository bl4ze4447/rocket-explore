use std::path::PathBuf;
use crate::error_modal::ErrorModal;

pub struct RenameAction {
    pub file:                   PathBuf,
    pub show_window:            bool,
    pub name_after_rename:      String,
    pub error_modal:            ErrorModal,
}
impl RenameAction {
    pub fn new() -> Self {
        Self {
            file:                   PathBuf::new(),
            show_window:            false,
            name_after_rename:      String::new(),
            error_modal:            ErrorModal::new(String::from("Rename Error")),
        }
    }
    pub fn clear(&mut self) {
        let old_modal       =   self.error_modal.clone();
        *self               =   RenameAction::new();
        self.error_modal    =   old_modal;
    }
}