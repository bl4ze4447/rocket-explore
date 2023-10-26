use std::path::PathBuf;
use crate::error_modal::ErrorModal;

pub struct CreateAction {
    pub file:               PathBuf,
    pub show_window:        bool,
    pub new_file_name:      String,
    pub extension:          String,
    pub error_modal:        ErrorModal,
}

impl CreateAction {
    pub fn new() -> Self {
        Self {
            file:               PathBuf::new(),
            show_window:        false,
            new_file_name:      String::new(),
            extension:          String::new(),
            error_modal:        ErrorModal::new(String::from("Create Error")),
        }
    }

    // Does not clear error modal
    pub fn clear(&mut self) {
        let old_modal       =   self.error_modal.clone();
        *self               =   CreateAction::new();
        self.error_modal    =   old_modal;
    }
}