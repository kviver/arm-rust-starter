extern crate core;

use cmsis_os::{
    Mutex,
    MutexGuard,
    Semaphore
};

//for async operations, like completing via interrupt
pub struct MutexSemaphore<T> {
    mutex : Mutex<T>,
    semaphore : Semaphore,
}

unsafe impl<T> Sync for MutexSemaphore<T> {}

impl<T> MutexSemaphore<T> {
    pub fn new(t: T) -> MutexSemaphore<T> {
        return MutexSemaphore {
            mutex: Mutex::new(t),
            semaphore: Semaphore::new(1),
        };
    }

    pub fn init(&mut self) {
        // semaphore created full
        self.semaphore.acquire();
    }

    pub fn lock(&self) -> MutexGuard<T> {
        self.mutex.lock()
    }

    pub fn get_semaphore(&self) -> &Semaphore {
        &self.semaphore
    }

    pub fn blocking<F:FnOnce(&T)>(&self, op:F) {
        let lock = self.mutex.lock();

        op(&lock);

        self.semaphore.acquire();
    }

    pub fn complete(&self) {
        self.semaphore.release();
    }
}

impl<T> MutexSemaphore<*mut T> {
    pub fn matches(&self, t: *mut T) -> bool {
        return self.mutex.matches(t);
    }
}