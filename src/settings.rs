use std::thread::available_parallelism;
pub struct Search {
    pub available_threads:      Vec<u8>,
    pub selected_thread_count:  u8,
}

pub struct Settings {
    pub search: Search,
}

impl Settings {
    pub fn new() -> Self {
        let max_thread_count = match available_parallelism() {
            Ok(count) => usize::from(count) as u8,
            Err(_) => 1_u8,
        };

        let mut available_threads = Vec::with_capacity(((max_thread_count & !(max_thread_count - 1)).leading_zeros() + 1_u32) as usize);
        available_threads.push(1);

        let mut thread_option: u8 = 2;
        while thread_option < max_thread_count {
            available_threads.push(thread_option);
            thread_option = thread_option * 2;
        }
        available_threads.push(max_thread_count);

        Self {
            search: Search {
                available_threads,
                selected_thread_count: max_thread_count,
            }
        }
    }
}