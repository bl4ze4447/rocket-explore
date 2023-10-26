use std::mem;
use std::path::PathBuf;
use std::collections::HashSet;

// ##########################################
// SOME HELPER ENUMS && STRUCTS
// ##########################################
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SelectionMode {
    SINGLE,
    MULTIPLE,
    RANGED,
}
pub enum SelectionResult {
    Single(PathBuf),
    Multiple(Vec<PathBuf>),
    Err(String)
}
impl SelectionResult {
    pub fn clone(&self) -> SelectionResult {
        match self {
            SelectionResult::Single(file) => {
                SelectionResult::Single(file.clone())
            }
            SelectionResult::Multiple(files) => {
                SelectionResult::Multiple(files.clone())
            }
            SelectionResult::Err(err) => {
                SelectionResult::Err(err.clone())
            }
        }
    }
}
// ##########################################
// STRUCT THAT HOLDS ALL ACTIONS
pub struct FileAction {
    pub rename_action: RenameAction,
    pub create_action: CreateAction,
    pub copy_action: CopyAction,
    pub move_action: MoveAction,
    pub delete_action: DeleteAction,
    pub open_action: OpenAction,
    pub select_action: SelectAction,
}
// ##########################################
// DECLARATION OF ERROR MODAL
// ##########################################
pub struct ErrorModal {
    pub title: String,
    pub caption: String,
    pub show: bool,
}
// ##########################################
// ACTIONS THAT CAN BE APPLIED TO A FILE
// ##########################################
pub struct RenameAction {
    pub file: PathBuf,
    pub show_window: bool,
    pub name_after_rename: String,
    pub error_modal: ErrorModal,
}
pub struct CreateAction {
    pub file: PathBuf,
    pub show_window: bool,
    pub new_file_name: String,
    pub extension: String,
    pub error_modal: ErrorModal,
}
pub struct CopyAction {
    pub show_window: bool,
    pub paste: bool,
    pub from: SelectionResult,
    pub to: PathBuf,
    pub percentage: u8,
    pub error_modal: ErrorModal,
}
pub struct MoveAction {
    pub show_window: bool,
    pub paste: bool,
    pub from: SelectionResult,
    pub to: PathBuf,
    pub percentage: u8,
    pub error_modal: ErrorModal,
}
pub struct DeleteAction {
    pub file_list: SelectionResult,
    pub error_modal: ErrorModal,
    pub show_window: bool,
}
pub struct OpenAction {
    pub file_list: SelectionResult,
    pub error_modal: ErrorModal,
    pub open: bool,
}
pub struct SelectAction {
    pub files: Vec<PathBuf>,
    pub mode: SelectionMode,
}
// ##########################################
// ERROR MODAL
// ##########################################
impl ErrorModal {
    pub fn new(title: String) -> Self {
        Self {
            title,
            caption: String::new(),
            show: false,
        }
    }
    pub fn set(&mut self, caption: String, show: bool) {
        self.caption = caption;
        self.show = show;
    }
    pub fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            caption: self.caption.clone(),
            show: self.show,
        }
    }
}
// ##########################################
// RENAME ACTION
// ##########################################
impl RenameAction {
    pub fn new() -> Self {
        Self {
            file: PathBuf::new(),
            show_window: false,
            name_after_rename: String::new(),
            error_modal: ErrorModal::new(String::from("Rename Error")),
        }
    }
    pub fn clear(&mut self) {
        let old_modal = self.error_modal.clone();
        *self = RenameAction::new();
        self.error_modal = old_modal;
    }
}
// ##########################################
// CREATE ACTION
// ##########################################
impl CreateAction {
    pub fn new() -> Self {
        Self {
            file: PathBuf::new(),
            show_window: false,
            new_file_name: String::new(),
            extension: String::new(),
            error_modal: ErrorModal::new(String::from("Create Error")),
        }
    }
    pub fn clear(&mut self) {
        let old_modal = self.error_modal.clone();
        *self = CreateAction::new();
        self.error_modal = old_modal;
    }
}
// ##########################################
// COPY ACTION
// ##########################################
impl CopyAction {
    pub fn new() -> Self {
        Self {
            show_window: false,
            paste: false,
            from: SelectionResult::Single(PathBuf::new()),
            to: PathBuf::new(),
            percentage: 0,
            error_modal: ErrorModal::new(String::from("Copy Error")),
        }
    }
    pub fn increment_percentage(&mut self) {
        if self.percentage < 100 {
            self.percentage = self.percentage + 1;
        }
    }
    pub fn increment_percentage_by_value(&mut self, value: u8) {
        self.percentage = self.percentage + value;
        if self.percentage > 100 {
            self.percentage = 100;
        }
    }
    pub fn completed(&self) -> bool {
        self.percentage == 100
    }
    pub fn clear(&mut self) {
        let old_modal = self.error_modal.clone();
        *self = CopyAction::new();
        self.error_modal = old_modal;
    }
}
// ##########################################
// MOVE ACTION
// ##########################################
impl MoveAction {
    pub fn new() -> Self {
        Self {
            show_window: false,
            paste: false,
            from: SelectionResult::Single(PathBuf::new()),
            to: PathBuf::new(),
            percentage: 0,
            error_modal: ErrorModal::new(String::from("Move Error")),
        }
    }
    pub fn increment_percentage(&mut self) {
        if self.percentage < 100 {
            self.percentage = self.percentage + 1;
        }
    }
    pub fn increment_percentage_by_value(&mut self, value: u8) {
        self.percentage = self.percentage + value;
        if self.percentage > 100 {
            self.percentage = 100;
        }
    }
    pub fn action_completed(&self) -> bool {
        self.percentage == 100
    }
    pub fn clear(&mut self) {
        let old_modal = self.error_modal.clone();
        *self = MoveAction::new();
        self.error_modal = old_modal;
    }
}
// ##########################################
// DELETE ACTION
// ##########################################
impl DeleteAction {
    pub fn new() -> Self {
        Self {
            show_window: false,
            file_list: SelectionResult::Single(PathBuf::new()),
            error_modal: ErrorModal::new("Delete Error".to_owned()),
        }
    }
    pub fn clear(&mut self) {
        let old_modal = self.error_modal.clone();
        *self = DeleteAction::new();
        self.error_modal = old_modal;
    }
}
// ##########################################
// OPEN ACTION
// ##########################################
impl OpenAction {
    pub fn new() -> Self {
        Self {
            file_list: SelectionResult::Single(PathBuf::new()),
            error_modal: ErrorModal::new("Open Error".to_owned()),
            open: false,
        }
    }
    pub fn clear(&mut self) {
        let old_modal = self.error_modal.clone();
        *self = OpenAction::new();
        self.error_modal = old_modal;
    }
}
// ##########################################
// SELECT ACTION
// ##########################################
impl SelectAction {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            mode: SelectionMode::SINGLE,
        }
    }
    pub fn is_selected(&self, file: &PathBuf) -> bool {
        self.files.contains(file)
    }
    pub fn manage_selection(&mut self, file: &PathBuf, content: &Vec<PathBuf>) {
        match self.mode {
            SelectionMode::SINGLE => {
                if self.files.contains(file) {
                    self.files.clear();
                    return;
                }
                if self.files.len() >= 1 {
                    self.files.clear();
                }

                self.files.push(file.clone());
            }
            SelectionMode::MULTIPLE => {
                if self.files.contains(file) {
                    if let Some(idx) = self.files.iter().position(|f| *f == *file) {
                        self.files.remove(idx);
                    }
                } else {
                    self.files.push(file.clone());
                }
            }
            SelectionMode::RANGED => {
                if self.files.is_empty() {
                    if let Some(idx) = content.iter().position(|f| *f == *file) {
                        for i in 0..=idx {
                            self.files.push(content[i].clone());
                        }
                    }
                } else {
                    let mut min_idx = content.len() - 1;
                    for selected_file in &self.files {
                        if let Some(idx) = content.iter().position(|f| *f == *selected_file) {
                            println!("{}", idx);
                            if idx < min_idx {
                                min_idx = idx;
                            }
                        }
                    }

                    if let Some(mut idx) = content.iter().position(|f| *f == *file) {
                        if min_idx > idx { mem::swap(&mut min_idx, &mut idx) }
                        for i in min_idx..=idx {
                            self.files.push(content[i].clone());
                        }
                    }

                    let mut unique: HashSet<PathBuf> = HashSet::from_iter(self.files.iter().cloned());
                    self.files = unique.into_iter().collect();
                }
            }
        }
    }
    pub fn switch_mode(&mut self) {
        self.mode = match self.mode {
            SelectionMode::SINGLE => SelectionMode::MULTIPLE,
            SelectionMode::MULTIPLE => SelectionMode::SINGLE,
            SelectionMode::RANGED => SelectionMode::RANGED,
        }
    }
    pub fn clear(&mut self) {
        self.files.clear();
        self.mode = SelectionMode::SINGLE;
    }
    pub fn get_selection(&self) -> SelectionResult {
        if self.files.is_empty() {
            return SelectionResult::Err("No file is selected".to_owned())
        }

        return match self.mode {
            SelectionMode::SINGLE => {
                SelectionResult::Single(self.files[0].clone())
            }
            SelectionMode::MULTIPLE => {
                SelectionResult::Multiple(self.files.clone())
            }
            SelectionMode::RANGED => {
                SelectionResult::Multiple(self.files.clone())
            }
        }
    }
}

// ##########################################
impl FileAction {
    pub fn new() -> Self {
        Self {
            rename_action: RenameAction::new(),
            create_action: CreateAction::new(),
            copy_action: CopyAction::new(),
            move_action: MoveAction::new(),
            delete_action: DeleteAction::new(),
            open_action: OpenAction::new(),
            select_action: SelectAction::new(),
        }
    }
}