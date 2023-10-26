use std::path::{PathBuf};
use std::{fs, thread};
use std::ffi::{c_char, c_ulong, CString};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};

#[derive(Clone, Copy)]
pub enum SearchFlag {
    FILES,
    DIRECTORIES,
    ALL,
}

pub fn get_all_occurrences(file_name: String, root_path: PathBuf, search_flag: SearchFlag, threads: u8, tx_data: Sender<PathBuf>, tx_finished: Sender<bool>, rx_should_stop: Arc<Mutex<Receiver<bool>>>) {
    if file_name.is_empty() {
        return;
    }
    if let Ok(path) = root_path.metadata() {
        if !path.is_dir() {
            return;
        }
    }

    let stop = Arc::new(Mutex::new(false));
    let stack: Arc<Mutex<Vec<PathBuf>>> = Arc::new(Mutex::new(Vec::with_capacity(128)));
    let mut handles = Vec::with_capacity(threads as usize);
    stack.lock().unwrap().push(root_path.clone());

    for _i in 0..threads {
        let stack = Arc::clone(&stack);
        let stop = Arc::clone(&stop);

        let search_flag = search_flag;
        let file_name = file_name.to_lowercase();

        let tx = tx_data.clone();
        let rx = Arc::clone(&rx_should_stop);

        let handle = thread::spawn(move || {
            let file_name = file_name.as_str();
            while let Some(current_path) = {
                let scope = stack.lock().unwrap().pop();
                scope
            } {
                if *stop.lock().unwrap() {
                    break;
                }
                if let Ok(_) = rx.lock().unwrap().try_recv() {
                    *stop.lock().unwrap() = true;
                    break;
                }
                if let Ok(content) = fs::read_dir(&current_path) {
                    for entry in content {
                        if let Ok(entry) = entry {
                            if let Ok(metadata) = entry.metadata() {
                                match search_flag {
                                    SearchFlag::ALL => {
                                        if entry.file_name().to_string_lossy().to_lowercase().contains(file_name) {
                                            if let Err(_e) = tx.send(entry.path()) {
                                                // todo
                                            }
                                        }
                                        if metadata.is_dir() {
                                            stack.lock().unwrap().push(entry.path());
                                        }
                                    }

                                    SearchFlag::DIRECTORIES => {
                                        if !metadata.is_dir() {
                                            continue;
                                        }

                                        let entry_path = entry.path();
                                        if entry.file_name().to_string_lossy().to_lowercase().contains(file_name) {
                                            if let Err(_e) = tx.send(entry.path()) {
                                                // todo
                                            }
                                        }

                                        stack.lock().unwrap().push(entry_path);
                                    }

                                    SearchFlag::FILES => {
                                        if metadata.is_dir() {
                                            stack.lock().unwrap().push(entry.path());
                                            continue;
                                        }

                                        if entry.file_name().to_string_lossy().to_lowercase().contains(file_name) {
                                            if let Err(_e) = tx.send(entry.path()) {
                                                // todo
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    tx_finished.send(true).unwrap();
}

pub fn get_all_occurrences_all_devices(file_name: String, devices: Vec<PathBuf>, search_flag: SearchFlag, threads: u8, tx_data: Sender<PathBuf>, tx_finished: Sender<bool>, rx_should_stop: Arc<Mutex<Receiver<bool>>>) {
    if file_name.is_empty() {
        return;
    }
    let stop = Arc::new(Mutex::new(false));
    let stack: Arc<Mutex<Vec<PathBuf>>> = Arc::new(Mutex::new(Vec::with_capacity(128)));

    for device in devices {
        if *stop.lock().unwrap() {
            break;
        }
        let mut handles = Vec::with_capacity(threads as usize);
        stack.lock().unwrap().clear();
        stack.lock().unwrap().push(device.clone());

        for _i in 0..threads {
            let stack = Arc::clone(&stack);
            let stop = Arc::clone(&stop);

            let search_flag = search_flag;
            let file_name = file_name.to_lowercase();

            let tx = tx_data.clone();
            let rx = Arc::clone(&rx_should_stop);

            let handle = thread::spawn(move || {
                let file_name = file_name.as_str();
                while let Some(current_path) = {
                    let scope = stack.lock().unwrap().pop();
                    scope
                } {
                    if *stop.lock().unwrap() {
                        break;
                    }
                    if let Ok(_) = rx.lock().unwrap().try_recv() {
                        *stop.lock().unwrap() = true;
                        break;
                    }
                    if let Ok(content) = fs::read_dir(&current_path) {
                        for entry in content {
                            if let Ok(entry) = entry {
                                if let Ok(metadata) = entry.metadata() {
                                    match search_flag {
                                        SearchFlag::ALL => {
                                            if entry.file_name().to_string_lossy().to_lowercase().contains(file_name) {
                                                if let Err(_e) = tx.send(entry.path()) {
                                                    // todo
                                                }
                                            }
                                            if metadata.is_dir() {
                                                stack.lock().unwrap().push(entry.path());
                                            }
                                        }

                                        SearchFlag::DIRECTORIES => {
                                            if !metadata.is_dir() {
                                                continue;
                                            }

                                            let entry_path = entry.path();
                                            if entry.file_name().to_string_lossy().to_lowercase().contains(file_name) {
                                                if let Err(_e) = tx.send(entry.path()) {
                                                    // todo
                                                }
                                            }

                                            stack.lock().unwrap().push(entry_path);
                                        }

                                        SearchFlag::FILES => {
                                            if metadata.is_dir() {
                                                stack.lock().unwrap().push(entry.path());
                                                continue;
                                            }

                                            if entry.file_name().to_string_lossy().to_lowercase().contains(file_name) {
                                                if let Err(_e) = tx.send(entry.path()) {
                                                    // todo
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    tx_finished.send(true).unwrap();
}
pub fn get_display_size(bytes: u64) -> String {
    let mut owner = String::new();
    if bytes >> 10 == 0 {
        owner.push_str(bytes.to_string().as_str());
        owner.push_str(" B");
    } else if bytes >> 20 == 0 {
        owner.push_str((bytes >> 10).to_string().as_str());
        owner.push_str(" KB");
    } else if bytes >> 30 == 0 {
        owner.push_str((bytes >> 20).to_string().as_str());
        owner.push_str(" MB");
    } else if bytes >> 40 == 0 {
        owner.push_str((bytes >> 30).to_string().as_str());
        owner.push_str(" GB");
    } else {
        owner.push_str((bytes >> 40).to_string().as_str());
        owner.push_str(" TB");
    }

    owner
}

#[derive(Default)]
#[repr(C)]
pub struct C_DiskSpace {
    pub sectors_per_cluster: c_ulong,
    pub bytes_per_sector: c_ulong,
    pub number_of_free_clusters: c_ulong,
    pub total_number_of_clusters: c_ulong,
}

pub struct DiskSpace {
    pub c_disk_space: Option<C_DiskSpace>,
    pub free: u128,
    pub total: u128,
}

impl DiskSpace {
    pub fn new() -> Self {
        Self {
            c_disk_space: None,
            free: 0,
            total: 0,
        }
    }
    pub fn from(c_ds: Option<C_DiskSpace>) -> Self {
        Self {
            c_disk_space: c_ds,
            free: 0,
            total: 0,
        }
    }

    pub fn recalculate(&mut self, device: &str) {
        self.c_disk_space = get_disk_space(device);
        if let Some(c_disk_space) = &self.c_disk_space {
            self.free = c_disk_space.bytes_per_sector as u128 * c_disk_space.sectors_per_cluster as u128 * c_disk_space.number_of_free_clusters as u128;
            self.total = c_disk_space.bytes_per_sector as u128 * c_disk_space.sectors_per_cluster as u128 * c_disk_space.total_number_of_clusters as u128;
        } else {
            self.free = 0;
            self.total = 0;
        }
    }
}

#[repr(C)]
pub struct DiskInfo {
    buf: *mut u8,
    len: usize,
}

impl Default for DiskInfo {
    fn default() -> Self {
        DiskInfo {
            buf: std::ptr::null_mut(),
            len: 0,
        }
    }
}

#[link(name = "harddisk")]
extern "C" {
    fn C_WRAPPER_get_disk_space(root: *const c_char, ds: *mut C_DiskSpace) -> bool;
    // fn C_WRAPPER_get_disks() -> *const c_char;
}

pub fn get_disk_space(root: &str) -> Option<C_DiskSpace> {
    let mut c_ds = C_DiskSpace::default();
    let result = unsafe {
        let root = CString::new(root).unwrap();
        let ptr = root.as_ptr();
        C_WRAPPER_get_disk_space(ptr, &mut c_ds)
    };

    if result {
        Some(c_ds)
    } else {
        None
    }
}

// pub fn get_disks() -> Option<String> {
//     let result = unsafe {
//         C_WRAPPER_get_disks()
//     };
//
//     unsafe {
//         match CStr::from_ptr(result).to_str() {
//             Ok(str) => Some(str.to_string()),
//             Err(_) => None
//         }
//     }
// }