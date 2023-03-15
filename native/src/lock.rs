use std::sync::{RwLock, RwLockWriteGuard};

pub fn wait_for_write<'a, T>(lock: &'a RwLock<T>) -> RwLockWriteGuard<'a, T> {
    let result;
    loop {
        match lock.try_write() {
            Ok(lock) => {
                result = lock;
                break;
            }
            Err(_) => continue,
        };
    }
    result
}

pub fn wait_for_read<'a, T>(lock: &'a RwLock<T>) -> std::sync::RwLockReadGuard<'a, T> {
    let result;
    loop {
        match lock.try_read() {
            Ok(lock) => {
                result = lock;
                break;
            }
            Err(_) => continue,
        };
    }
    result
}
