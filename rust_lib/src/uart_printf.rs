extern crate core;

use core::fmt::{Result, Write};

use cmsis_os::{Semaphore};
use hal::{HAL_UART_Transmit_IT, UART_HandleTypeDef, HAL_StatusTypeDef};

pub struct ExclusiveUART<'a> {
    pub uart: *mut UART_HandleTypeDef,
    pub semaphore : &'a Semaphore,
}

impl<'a> ExclusiveUART<'a> {
    fn write_slice_unsafe(&self, bytes:&[u8]) {
        let len = bytes.len();
        if len > core::u16::MAX as usize {
            panic!("Can't send more than 65kB at once");
        }
        let status = unsafe { HAL_UART_Transmit_IT(self.uart, bytes.as_ptr(), len as u16) };
        match status {
            HAL_StatusTypeDef::HAL_OK => {}
            _ => panic!("Unexpected status from HAL_UART_Transmit_IT")
        }
    }

    fn write_slice_blocking(&self, bytes: &[u8]) {
        self.write_slice_unsafe(bytes);
        self.semaphore.acquire();
    }

    fn write_blocking(&self, s: &str){
        let bytes = s.as_bytes();
        for chunk in bytes.chunks(core::u16::MAX as usize) {
            self.write_slice_blocking(chunk);
        }
    }
}

impl<'a> Write for ExclusiveUART<'a> {
    fn write_str(&mut self, s: &str) -> Result {
        self.write_blocking(s);
        return Ok(());
    }
}

#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => {
        {
            let uart2_local = UART2.get();
            let uart2_lock = uart2_local.lock();
            let mut w = ExclusiveUART {
                uart: *uart2_lock,
                semaphore: uart2_local.get_semaphore(),
            };
            write!(w, $($arg)*).unwrap()
        }
    };
}

#[macro_export]
macro_rules! debug_println {
    () => (debug_print!("\n"));
    ($fmt:expr) => (debug_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (debug_print!(concat!($fmt, "\n"), $($arg)*));
}
