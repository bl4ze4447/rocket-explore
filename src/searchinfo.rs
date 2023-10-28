use std::path::PathBuf;
use std::sync::{ Arc, Mutex, mpsc, mpsc::{ Sender, Receiver } };
use std::thread;
use crate::filemanager::{get_all_occurrences, get_all_occurrences_all_devices, SearchFlag};

pub struct SearchInfo {
    pub files:                  Vec<PathBuf>,
    pub tx_path:                Sender<PathBuf>,
    pub rx_path:                Receiver<PathBuf>,
    pub tx_stop:                Sender<bool>,
    pub rx_stop:                Arc<Mutex<Receiver<bool>>>,
    pub tx_finished:            Sender<bool>,
    pub rx_finished:            Receiver<bool>,
    pub filename_to_search:     String,
    pub searching:              bool,
    pub display_search_content: bool,
    pub flag:                   SearchFlag,
}


impl SearchInfo {
    pub fn new() -> Self {
        let path       =    mpsc::channel();
        let stop            =   mpsc::channel();
        let finished        =   mpsc::channel();

        Self {
            files:                  Vec::with_capacity(32),
            tx_path:                path.0,
            rx_path:                path.1,
            tx_stop:                stop.0,
            rx_stop:                Arc::new(Mutex::new(stop.1)),
            tx_finished:            finished.0,
            rx_finished:            finished.1,
            filename_to_search:     String::new(),
            searching:              false,
            display_search_content: false,
            flag:                   SearchFlag::ALL,
        }
    }

    pub fn search(&mut self, device_mode: bool, path: PathBuf, devices: Vec<PathBuf>, threads: u8) {
        if !device_mode {
            let file_to_search = self.filename_to_search.clone();
            let current_path = path;
            let search_flag = self.flag;
            let tx_path = self.tx_path.clone();
            let tx_finished = self.tx_finished.clone();
            let rx_stop_signal = Arc::clone(&self.rx_stop);
            let threads_num = threads;

            self.files = Vec::new();

            thread::spawn(move || {
                get_all_occurrences(file_to_search, current_path, search_flag, threads_num, tx_path, tx_finished, rx_stop_signal);
            });
        } else {
            let file_to_search = self.filename_to_search.clone();
            let devices = devices.clone();
            let search_flag = self.flag;
            let tx_path = self.tx_path.clone();
            let tx_finished = self.tx_finished.clone();
            let rx_stop_signal = Arc::clone(&self.rx_stop);
            let threads_num = threads;

            self.files = Vec::new();

            thread::spawn(move || {
                get_all_occurrences_all_devices(file_to_search, devices, search_flag, threads_num, tx_path, tx_finished, rx_stop_signal);
            });
        }

        self.searching = true;
        self.display_search_content = true;
    }

    pub fn stop_search(&mut self) -> Result<(), ()> {
        if let Ok(_) = self.tx_stop.send(true) {
            if let Ok(_) = self.rx_finished.recv() {
                let new_path_channel = mpsc::channel();
                self.tx_path = new_path_channel.0;
                self.rx_path = new_path_channel.1;

                return Ok(());
            }
        }

        return Err(());
    }
}