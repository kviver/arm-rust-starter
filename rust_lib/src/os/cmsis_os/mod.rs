extern crate core;

use core::cell::UnsafeCell;
use core::ops::Deref;
use core::ops::DerefMut;

mod bindings;
use self::bindings::*;

struct InnerMutex {
    id: osMutexId,
}

#[allow(dead_code)]
impl InnerMutex {
    pub fn new() -> InnerMutex {
        let mutex_def = osMutexDef_t { dummy: 0 };
        let mutex_id = unsafe { osMutexCreate(&mutex_def) };
        // mutex_id is actually a pointer
        if mutex_id == 0 {
            panic!("Failed to create mutex");
        }
        InnerMutex {
            id: mutex_id,
        }
    }

    pub fn lock(&self) {
        let status = unsafe { osMutexWait(self.id, osWaitForever) };
        if status != osStatus::osOK {
            panic!("blocking -> osMutexWait");
        }
    }

    pub fn unlock(&self) {
        let status = unsafe { osMutexRelease(self.id) };
        if status != osStatus::osOK {
            panic!("blocking -> osMutexRelease");
        }
    }
}

pub struct Mutex<T> {
    inner: InnerMutex,
    data: UnsafeCell<T>,
}

#[must_use]
pub struct MutexGuard<'a, T: 'a> {
    // funny underscores due to how Deref/DerefMut currently work (they
    // disregard field privacy).
    __lock: &'a Mutex<T>,
}

#[allow(dead_code)]
impl<'mutex, T> MutexGuard<'mutex, T> {
    fn new(lock: &'mutex Mutex<T>) -> MutexGuard<'mutex, T> {
        MutexGuard {
            __lock: lock,
        }
    }
}

#[allow(dead_code)]
impl<'mutex, T> Deref for MutexGuard<'mutex, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.__lock.data.get() }
    }
}

#[allow(dead_code)]
impl<'mutex, T> DerefMut for MutexGuard<'mutex, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.__lock.data.get() }
    }
}

#[allow(dead_code)]
impl<'a, T> Drop for MutexGuard<'a, T> {
    #[inline]
    fn drop(&mut self) {
        self.__lock.inner.unlock();
    }
}

#[allow(dead_code)]
impl<T> Mutex<T> {
    pub fn new(t: T) -> Mutex<T> {
        Mutex {
            inner: InnerMutex::new(),
            data: UnsafeCell::new(t),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        self.inner.lock();
        MutexGuard::new(self)
    }

    pub unsafe fn unsafe_get(&self) -> &T {
        &*self.data.get()
    }
}


unsafe impl<T> Sync for Mutex<T> {}

pub struct Semaphore {
    id: osSemaphoreId,
}

#[allow(dead_code)]
impl Semaphore {
    pub fn new(count: u32) -> Semaphore {
        assert!(count <= core::i32::MAX as u32);
        let semaphore_def = osSemaphoreDef_t { dummy: 0 };
        let semaphore_id = unsafe { osSemaphoreCreate(&semaphore_def, count as i32) };
        // semaphore_id is actually a pointer
        if semaphore_id == 0 {
            panic!("Failed to create semaphore");
        }
        Semaphore {
            id: semaphore_id
        }
    }

    // TODO it's a copy-paste, move to common code
    pub fn empty(count: u32) -> Semaphore {
        assert!(count <= core::i32::MAX as u32);
        let res = Semaphore::new(count);
        for _ in 0 .. count {
            // semaphore created full
            res.acquire();
        }
        return res;
    }

    pub fn acquire(&self) {
        let status = unsafe { osSemaphoreWait(self.id, osWaitForever) };
        if status == -1 {
            panic!("blocking -> osSemaphoreWait");
        }
    }

    pub fn release(&self) {
        let status = unsafe { osSemaphoreRelease(self.id) };
        if status != osStatus::osOK {
            panic!("complete -> osSemaphoreRelease");
        }
    }
}

unsafe impl Sync for Semaphore {}

pub fn delay(delay_ms: u32) {
    unsafe { osDelay(delay_ms); }
}
