#![no_std]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(const_unsafe_cell_new)]
#![feature(core_float)]
#![feature(proc_macro)]

mod cmsis_os;
mod mutex_semaphore;
mod gpio_pin;
mod hal;

#[macro_use]
mod uart_printf;

use core::cell::UnsafeCell;

use cmsis_os::{delay};
use gpio_pin::{GPIOPin};
use hal::{
    HAL_LD2_GPIO_Port,
    HAL_LD2_Pin,
    UART_HandleTypeDef,
    get_huart2
};

use mutex_semaphore::MutexSemaphore;

use uart_printf::ExclusiveUART;
use core::fmt::Write;

pub struct StaticMutexSemaphore<T> {
    cell: UnsafeCell<Option<MutexSemaphore<T>>>,
}

impl<T> StaticMutexSemaphore<T> {
    pub const fn new() -> StaticMutexSemaphore<T> {
        StaticMutexSemaphore {
            cell: UnsafeCell::new(None)
        }
    }

    pub fn init(&self, t:T){
        let opt_ptr = self.cell.get();
        {
            let opt = unsafe { opt_ptr.as_ref() };
            let opt = match opt {
                Some(res) => res,
                None => panic!("StaticMutexSemaphore cell is null")
            };
            if opt.is_some() {
                panic!("StaticMutexSemaphore already initialized");
            }
        }
        let mut ms = MutexSemaphore::new(t);
        ms.init();
        unsafe { *opt_ptr = Some(ms) };
    }

    pub fn get(&self) -> &MutexSemaphore<T> {
        // TODO check for thread-safety
        let opt_ptr = self.cell.get();
        let opt = unsafe { opt_ptr.as_ref() };
        let opt = match opt {
            Some(res) => res,
            None => panic!("StaticMutexSemaphore cell is null")
        };

        return match opt {
            &Some(ref res) => &res,
            &None => panic!("StaticMutexSemaphore is not initialized"),
        }
    }
}

unsafe impl<T> Sync for StaticMutexSemaphore<T> {}

#[no_mangle]
pub extern "C" fn app_init_statics() {
    UART2.init(unsafe{get_huart2()});
}

pub static UART2: StaticMutexSemaphore<*mut UART_HandleTypeDef> = StaticMutexSemaphore::new();

#[no_mangle]
pub extern "C" fn app_task() {
    debug_println!("\n\n==== arm/rust started =====");

    let mut ld2_pin = GPIOPin {
        gpio_port: unsafe { HAL_LD2_GPIO_Port },
        pin: unsafe { HAL_LD2_Pin },
    };

    loop {
        ld2_pin.toggle();
        
        debug_print!("+");

        delay(1000);
    }
}

#[no_mangle]
pub extern "C" fn HAL_UART_TxCpltCallback(_huart: *mut UART_HandleTypeDef) {
    // TODO get UART mutex from _huart pointer comparison for multiple uart
    let uart2_local = UART2.get();
    uart2_local.get_semaphore().release();
}

fn checked_release<T>(ms:&StaticMutexSemaphore<*mut T>, ptr:*mut T) {
    let local_ms = ms.get();
    if local_ms.matches(ptr) {
        local_ms.get_semaphore().release();
    }
}

// These functions are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {}

// This function may be needed based on the compilation target.
#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    loop {}
}
