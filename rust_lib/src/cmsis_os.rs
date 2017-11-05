extern crate core;

use core::cell::UnsafeCell;
use core::ops::Deref;
use core::ops::DerefMut;

#[repr(C)]
#[derive(PartialEq, Eq)]
pub enum osStatus {
    osOK = 0,
    osErrorOS = 0xFF,
}

#[repr(C)]
pub struct osMutexDef_t {
  dummy: u32,
// #if( configSUPPORT_STATIC_ALLOCATION == 1 )
//   osStaticMutexDef_t         *controlblock;      ///< control block for static allocation; NULL for dynamic allocation
// #endif
}

impl osMutexDef_t {
    pub const fn new() -> osMutexDef_t {
        return osMutexDef_t { dummy: 0 };
    }
}

#[repr(C)]
pub struct osSemaphoreDef_t {
  dummy: u32,
// #if( configSUPPORT_STATIC_ALLOCATION == 1 )
//   osStaticSemaphoreDef_t     *controlblock;      ///< control block for static allocation; NULL for dynamic allocation
// #endif
}

impl osSemaphoreDef_t {
    pub const fn new() -> osSemaphoreDef_t {
        return osSemaphoreDef_t { dummy: 0 };
    }
} 

// opaque struct
#[repr(C)]
pub struct QueueHandle;

pub type QueueHandle_t = *const QueueHandle;
pub type SemaphoreHandle_t = QueueHandle_t;
pub type osMutexId = SemaphoreHandle_t;
pub type osSemaphoreId = SemaphoreHandle_t;


extern {
    pub fn osDelay(delay: u32) -> osStatus;

    pub fn osMutexCreate(mutex_def: *const osMutexDef_t) -> osMutexId;
    pub fn osMutexWait (mutex_id: osMutexId, millisec: u32) -> osStatus;
    pub fn osMutexRelease (mutex_id: osMutexId ) -> osStatus;

    pub fn osSemaphoreCreate (semaphore_def : *const osSemaphoreDef_t, count:i32) -> osSemaphoreId;
    pub fn osSemaphoreWait (semaphore_id: osSemaphoreId, millisec: u32) -> i32;
    pub fn osSemaphoreRelease (semaphore_id: osSemaphoreId) -> osStatus;
}

// #define osWaitForever     0xFFFFFFFF     ///< wait forever timeout value
pub const osWaitForever : u32 = 0xFFFFFFFF;

struct InnerMutex {
    id: osMutexId,
}

impl InnerMutex {
    pub fn new() -> InnerMutex {
        let mutex_def = osMutexDef_t::new();
        let mutex_id = unsafe { osMutexCreate(&mutex_def) };
        if mutex_id == core::ptr::null() {
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

impl<'mutex, T> MutexGuard<'mutex, T> {
    fn new(lock: &'mutex Mutex<T>) -> MutexGuard<'mutex, T> {
        MutexGuard {
            __lock: lock,
        }
    }
}

impl<'mutex, T> Deref for MutexGuard<'mutex, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.__lock.data.get() }
    }
}

impl<'mutex, T> DerefMut for MutexGuard<'mutex, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.__lock.data.get() }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    #[inline]
    fn drop(&mut self) {
        self.__lock.inner.unlock();
    }
}

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
}

impl<T> Mutex<*mut T> {
    pub fn matches(&self, t:*mut T) -> bool {
        let local_ptr = unsafe { *(self.data.get()) };
        return local_ptr == t;
    }
}

unsafe impl<T> Sync for Mutex<T> {}

pub struct Semaphore {
    id: osSemaphoreId,
}

impl Semaphore {
    pub fn new(count: i32) -> Semaphore {
        let semaphore_def = osSemaphoreDef_t::new();
        let semaphore_id = unsafe { osSemaphoreCreate(&semaphore_def, count) };
        if semaphore_id == core::ptr::null() {
            panic!("Failed to create semaphore");
        }
        Semaphore {
            id: semaphore_id
        }
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