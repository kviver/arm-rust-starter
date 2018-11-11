#![cfg_attr(not(unix), no_std)]
#![feature(panic_implementation)]
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
        use core::panic::PanicInfo;

        // FIXME no_mangle should not be required
        // https://github.com/rust-lang/rust/issues/51342
        #[panic_implementation]
        #[no_mangle]
        fn panic(_info: &PanicInfo) -> ! {
            loop {}
        }

        pub use hal::stm32_hal::stm32_uart::{HAL_UART_TxCpltCallback};
    }
}
