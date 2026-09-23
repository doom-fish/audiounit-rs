use std::sync::{Mutex, MutexGuard, PoisonError};

static LIFECYCLE: Mutex<()> = Mutex::new(());

pub fn lock() -> MutexGuard<'static, ()> {
    LIFECYCLE.lock().unwrap_or_else(PoisonError::into_inner)
}
