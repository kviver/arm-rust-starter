extern crate core;

use cell::UnsafeCell;

pub struct Static<T> {
    cell: UnsafeCell<Option<T>>,
}

#[allow(dead_code)]
impl<T> Static<T> {
    pub const fn new() -> Static<T> {
        Static {
            cell: UnsafeCell::new(None)
        }
    }

    pub fn init(&self, t:T) {
        // TODO maybe should run in interrupt-free context
        let opt_ptr = self.cell.get();
        {
            let opt = unsafe { opt_ptr.as_ref() };
            let opt = match opt {
                Some(res) => res,
                None => panic!("Static cell is null")
            };
            if opt.is_some() {
                panic!("Static already initialized");
            }
        }
        unsafe { *opt_ptr = Some(t) };
    }

    pub fn get(&self) -> &T {
        // TODO check for thread-safety
        let opt_ptr = self.cell.get();
        let opt = unsafe { opt_ptr.as_ref() };
        let opt = match opt {
            Some(res) => res,
            None => panic!("Static cell is null")
        };

        return match opt {
            &Some(ref res) => &res,
            &None => panic!("Static is not initialized"),
        }
    }
}

unsafe impl<T: Send> Send for Static<T> {}
unsafe impl<T: Sync> Sync for Static<T> {}
