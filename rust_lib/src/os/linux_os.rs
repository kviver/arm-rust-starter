use std::sync::Condvar;
use std::thread::sleep_ms;

use std::sync::Mutex as StdMutex;
pub use std::sync::MutexGuard;

pub struct Mutex<T> {
    inner: StdMutex<T>
}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Mutex<T> {
        Mutex {
            inner: StdMutex::new(t),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        self.inner.lock().unwrap()
    }
}

pub struct Semaphore {
    count: u32,
    free: StdMutex<u32>,
    condvar: Condvar,
}

impl Semaphore {
    pub fn new(count: u32) -> Semaphore {
        Semaphore {
            count,
            free: StdMutex::new(count),
            condvar: Condvar::new(),
        }
    }

    // TODO it's a copy-paste, move to common code
    pub fn empty(count: u32) -> Semaphore {
        let res = Semaphore::new(count);
        for _ in 0 .. count {
            // semaphore created full
            res.acquire();
        }
        return res;
    }

    pub fn acquire(&self) {
        let mut lock = self.free.lock().unwrap();
        while *lock == 0 {
            lock = self.condvar.wait(lock).unwrap();
        }
        *lock -= 1;
    }

    pub fn release(&self) {
        let mut lock = self.free.lock().unwrap();
        if *lock == self.count {
            panic!("All tokens already released");
        }
        *lock += 1;
        self.condvar.notify_one();
    }
}

pub fn delay(delay_ms: u32) {
    sleep_ms(delay_ms);
}
