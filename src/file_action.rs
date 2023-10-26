use crate::copy_action::CopyAction;
use crate::create_action::CreateAction;
use crate::delete_action::DeleteAction;
use crate::move_action::MoveAction;
use crate::open_action::OpenAction;
use crate::rename_action::RenameAction;
use crate::select_action::SelectAction;

pub struct FileAction {
    pub rename_action:      RenameAction,
    pub create_action:      CreateAction,
    pub copy_action:        CopyAction,
    pub move_action:        MoveAction,
    pub delete_action:      DeleteAction,
    pub open_action:        OpenAction,
    pub select_action:      SelectAction,
}
impl FileAction {
    pub fn new() -> Self {
        Self {
            rename_action:      RenameAction::new(),
            create_action:      CreateAction::new(),
            copy_action:        CopyAction::new(),
            move_action:        MoveAction::new(),
            delete_action:      DeleteAction::new(),
            open_action:        OpenAction::new(),
            select_action:      SelectAction::new(),
        }
    }
}