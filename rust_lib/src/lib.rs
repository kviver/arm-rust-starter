#![cfg_attr(not(unix), no_std)]
#![feature(lang_items)]
#![feature(const_fn)]

#[macro_use]
extern crate cfg_if;

mod fmt;
mod cell;

mod peripheral;

#[macro_use]
mod hal;
use hal::traits::{Pin};
use hal::gpio_pin::{
    LD2_PIN
};
use hal::{init_statics};

mod os;
use os::{delay};

use hal::{debug_uart_get};
use fmt::{Write};

#[no_mangle]
pub extern "C" fn rust_main_task() -> ! {
    init_statics();

    debug_println!("hello!");

    let mut ld2_pin = LD2_PIN.get().lock();

    loop {
        ld2_pin.toggle();
        delay(1000);
    }
}

cfg_if! {
    if #[cfg(all(target_arch="arm", target_os="none"))] {
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

        pub use hal::stm32_hal::stm32_uart::{HAL_UART_TxCpltCallback};
    }
}
