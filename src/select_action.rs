use std::mem;
use std::path::PathBuf;
use std::collections::HashSet;

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

pub struct SelectAction {
    pub files:  HashSet<PathBuf>,
    pub mode:   SelectionMode,
}

impl SelectAction {
    pub fn new() -> Self {
        Self {
            files:  HashSet::new(),
            mode:   SelectionMode::SINGLE,
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

                if !self.files.is_empty() {
                    self.files.clear();
                }

                self.files.insert(file.clone());
            }
            SelectionMode::MULTIPLE => {
                if self.files.contains(file) {
                    self.files.remove(file);
                    return;
                }

                self.files.insert(file.clone());
            }
            SelectionMode::RANGED => {
                if self.files.is_empty() {
                    if let Some(idx) = content.iter().position(|f| *f == *file) {
                        for i in 0..=idx {
                            self.files.insert(content[i].clone());
                        }
                    }

                    return;
                }

                let mut min_idx = content.len() - 1;
                for selected_file in &self.files {
                    if let Some(idx) = content.iter().position(|f| *f == *selected_file) {
                        if idx < min_idx {
                            min_idx = idx;
                        }
                    }
                }

                if let Some(mut idx) = content.iter().position(|f| *f == *file) {
                    if min_idx > idx { mem::swap(&mut min_idx, &mut idx) }
                    for i in min_idx..=idx {
                        self.files.insert(content[i].clone());
                    }
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
            return SelectionResult::Err("No file is selected".to_owned()) // TODO
        }

        return match self.mode {
            SelectionMode::SINGLE => {
                if let Some(f) = self.files.clone().into_iter().next() {
                    SelectionResult::Single(f)
                } else {
                    SelectionResult::Err("TODO".to_owned())
                }
            }
            SelectionMode::MULTIPLE => {
                SelectionResult::Multiple(self.files.clone().into_iter().collect())
            }
            SelectionMode::RANGED => {
                SelectionResult::Multiple(self.files.clone().into_iter().collect())
            }
        }
    }

    pub fn check(&mut self) {
        for file in self.files.clone() {
            if let Ok(value) = file.try_exists() {
                if !value {
                    self.files.remove(&file);
                }
            }
        }
    }
}