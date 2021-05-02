use std::sync::{Mutex, MutexGuard};
use virt::connect::Connect;

pub struct ThreadSafeVirtConn {
    conn: Mutex<Connect>,
}

impl ThreadSafeVirtConn {
    pub fn new(uri: &str) -> ThreadSafeVirtConn {
        ThreadSafeVirtConn {
            conn: Mutex::new(Connect::open(uri).unwrap()),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, Connect> {
        self.conn.lock().unwrap()
    }
}

unsafe impl Send for ThreadSafeVirtConn {}
unsafe impl Sync for ThreadSafeVirtConn {}
