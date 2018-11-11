use os::Mutex;
use peripheral::Static;

pub trait Release<P> {
    fn checked_release(&self, ptr: *mut P);
}

#[allow(dead_code)]
pub fn checked_release<P, T:Release<P>>(statics:&[&Static<Mutex<T>>], ptr:*mut P) {
    for statik in statics {
        let local = statik.get();
        unsafe {
            local.unsafe_get().checked_release(ptr);
        }
    }
}
