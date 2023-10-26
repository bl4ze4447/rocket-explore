use std::path::PathBuf;
use std::sync::{ Arc, Mutex, mpsc, mpsc::{ Sender, Receiver } };
use crate::filemanager::SearchFlag;

pub struct SearchInfo {
    pub files: Vec<PathBuf>,
    pub tx_path: Sender<PathBuf>,
    pub rx_path: Receiver<PathBuf>,
    pub tx_stop: Sender<bool>,
    pub rx_stop: Arc<Mutex<Receiver<bool>>>,
    pub tx_finished: Sender<bool>,
    pub rx_finished: Receiver<bool>,
    pub filename_to_search: String,
    pub searching: bool,
    pub display_search_content: bool,
    pub flag: SearchFlag,
}


impl SearchInfo {
    pub fn new() -> Self {
        let path = mpsc::channel();
        let stop = mpsc::channel();
        let finished = mpsc::channel();

        Self {
            files: Vec::with_capacity(32),
            tx_path: path.0,
            rx_path: path.1,
            tx_stop: stop.0,
            rx_stop: Arc::new(Mutex::new(stop.1)),
            tx_finished: finished.0,
            rx_finished: finished.1,
            filename_to_search: String::new(),
            searching: false,
            display_search_content: false,
            flag: SearchFlag::ALL,
        }
    }
}