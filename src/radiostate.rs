pub struct MainWin {
    pub files_flag:     bool,
    pub folders_flag:   bool,
    pub all_flag:       bool,
}
pub struct CreateWin {
    pub file:   bool,
    pub folder: bool,
}

pub struct RadioState {
    pub main_win:       MainWin,
    pub create_win:     CreateWin,
}

impl MainWin {
    pub fn new() -> Self {
        Self {
            files_flag:     false,
            folders_flag:   false,
            all_flag:       true,
        }
    }
}
impl CreateWin {
    pub fn new() -> Self {
        Self {
            file:   true,
            folder: false,
        }
    }
}
impl RadioState {
    pub fn new() -> Self {
        Self {
            main_win:   MainWin::new(),
            create_win: CreateWin::new(),
        }
    }
}