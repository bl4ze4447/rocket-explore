#[cfg(windows)]
use std::os::windows::fs::MetadataExt;

use std::borrow::Cow;
use std::fs;
use std::path::PathBuf;

use crate::language_strings::{LangKeys, LangString};
use crate::filemanager::DiskSpace;

pub enum SortByType {
    FolderFirst,
    FileFirst,
}
pub enum SortByOrder {
    Ascending,
    Descending,
}
pub struct Filters {
    pub show_symlink: bool,
    pub show_hidden: bool,
    pub show_system: bool,
    pub show_dir: bool,
    pub show_files: bool,
}
impl Filters {
    pub fn new() -> Self {
        Self {
            show_symlink: false,
            show_hidden: false,
            show_system: false,
            show_dir: true,
            show_files: true,
        }
    }
}

pub struct PathInfo {
    pub disk_space:                 DiskSpace,
    pub lang_string:                LangString,
    pub filters:                    Filters,
    pub connected_devices:          Vec<PathBuf>,
    pub current_directory_content:  Vec<PathBuf>,
    pub previous_paths:             Vec<PathBuf>,
    pub next_paths:                 Vec<PathBuf>,
    pub display_path:               String,
    pub current_absolute_path:      PathBuf,
    pub sort_type:                  SortByType,
    pub sort_order:                 SortByOrder,
    pub show_dir_content:           bool,
    pub empty_dir:                  bool,
    pub deleted_dir:                bool,
}

impl PathInfo {
    pub fn new() -> Self {
        let lang_string = LangString::new();
        let root_path       = lang_string.get(&LangKeys::RootPath);
        Self {
            disk_space: DiskSpace::new(),
            lang_string,
            filters: Filters::new(),
            connected_devices: Vec::with_capacity(4),
            current_directory_content: Vec::with_capacity(16),
            previous_paths: Vec::new(),
            next_paths: Vec::new(),
            display_path: root_path,
            current_absolute_path: PathBuf::new(),
            sort_type: SortByType::FolderFirst,
            sort_order: SortByOrder::Ascending,
            show_dir_content: false,
            empty_dir: false,
            deleted_dir: false,
        }
    }

    pub fn root_display_path(&self) -> String { self.lang_string.get(&LangKeys::RootPath) }
    pub fn search_for_devices(self: &mut Self) {
        self.connected_devices.clear();

        let prefix = String::from("\\\\?\\");
        for letter in 'A'..='Z' {
            let mut device = prefix.clone();

            device.push(letter);
            device.push_str(":\\");

            let device = PathBuf::from(device);
            if let Ok(metadata) = device.metadata() {
                if metadata.is_dir() {
                    self.connected_devices.push(device);
                }
            }
        }
    }
    #[cfg(windows)]
    pub fn strip_win_prefix(path: &Cow<'_, str>) -> String {
        let owner = String::from(&path[4..]);
        owner
    }
    #[cfg(unix)]
    // Just converts to string on UNIX
    fn strip_win_prefix(path: &Cow<'_, str>) -> String {
        path.to_string();
    }
    #[cfg(windows)]
    pub fn add_win_prefix_to_display(&self) -> PathBuf {
        let mut prefix = String::from("\\\\?\\");
        prefix.push_str(self.display_path.as_str());

        PathBuf::from(prefix)
    }
    #[cfg(unix)]
    // Just converts display path to PathBuf
    pub fn add_win_prefix_to_display(&self) -> PathBuf {
        PathBuf::from(&self.display_path)
    }

    fn sort_directory_content(&mut self) {
        match self.sort_type {
            SortByType::FileFirst => {
                self.current_directory_content.sort_unstable_by_key(|key| {
                    key.metadata().map(|metadata| !metadata.is_file()).unwrap_or(true)
                });
            }
            SortByType::FolderFirst => {
                self.current_directory_content.sort_unstable_by_key(|key| {
                    key.metadata().map(|metadata| metadata.is_file()).unwrap_or(true)
                });
            }
        }

        // match self.sort_order {
        //     SortByOrder::Ascending => {
        //         self.current_directory_content.sort_unstable_by(|el1, el2| el1.to_string_lossy().to_string().cmp(&el2.to_string_lossy().to_string()));
        //     }
        //     SortByOrder::Descending => {
        //         self.current_directory_content.sort_unstable_by(|el1, el2| el1.to_string_lossy().to_string().cmp(&el2.to_string_lossy().to_string()).reverse());
        //     }
        // }
    }

    pub fn fill_directory_content(&mut self) {
        self.current_directory_content.clear();
        if let Ok(content) = fs::read_dir(&self.current_absolute_path) {
            self.empty_dir = false;
            self.deleted_dir = false;

            for entry in content {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        let path = entry.path().clone();
                        if let Ok(is_hidden) = PathInfo::is_hidden(&path) {
                            if let Ok(is_system) = PathInfo::is_system(&path) {
                                let show = (is_hidden             == self.filters.show_hidden)  &&
                                                 (is_system             == self.filters.show_system)  &&
                                                 (metadata.is_symlink() == self.filters.show_symlink) &&
                                                 (metadata.is_dir()     == self.filters.show_dir)     ||
                                                 (metadata.is_file()    == self.filters.show_files);

                                if show { self.current_directory_content.push(path); }
                            }
                        }
                    }
                }
            }
        } else if !self.current_absolute_path.exists() {
            self.deleted_dir = true;
        } else {
            self.deleted_dir = true;
        }

        self.sort_directory_content();
    }

    #[cfg(windows)]
    fn is_hidden(file: &PathBuf) -> Result<bool, u8> {
        return if let Ok(metadata) = file.metadata() {
            const HIDDEN_ATTRIBUTE: u32 = 0x00000002;
            if metadata.file_attributes() & HIDDEN_ATTRIBUTE != 0 {
                return Ok(true)
            }

            Ok(false)
        } else {
            Err(1)
        }
    }
    #[cfg(windows)]
    fn is_system(file: &PathBuf) -> Result<bool, u8> {
        return if let Ok(metadata) = file.metadata() {
            const SYSTEM_ATTRIBUTE: u32 = 0x00000004;
            if metadata.file_attributes() & SYSTEM_ATTRIBUTE != 0 {
                return Ok(true)
            }

            Ok(false)
        } else {
            Err(1)
        }
    }
    #[cfg(unix)]
    fn is_hidden(file: &PathBuf) -> Result<bool, u8> {
        Ok(file.starts_with("."))
    }
}