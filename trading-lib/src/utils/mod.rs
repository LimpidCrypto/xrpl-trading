use std::sync::{Arc, Mutex, MutexGuard};

use anyhow::Result;

pub(crate) fn anyhow_mutex<'a, T>(mutex: &'a Mutex<T>) -> Result<MutexGuard<'a, T>> {
    match mutex.lock() {
        Ok(guard) => Ok(guard),
        Err(poisoned) => Err(anyhow::anyhow!("Mutex poisoned: {:?}", poisoned)),
    }
}
