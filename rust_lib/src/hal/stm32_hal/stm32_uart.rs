extern crate core;

use fmt;

use os::{Mutex, Semaphore};

use hal::stm32_hal::bindings::{
    UART_HandleTypeDef,
    HAL_UART_Transmit_IT,
    HAL_StatusTypeDef,
    huart2
};

use super::release::{checked_release, Release};

use peripheral::{Static};

use hal::uart_printf::{UART};

pub struct STM32_UART {
    uart: *mut UART_HandleTypeDef,
    semaphore: Semaphore,
}

impl STM32_UART {
    fn new(uart: *mut UART_HandleTypeDef) -> STM32_UART {
        STM32_UART {
            uart,
            semaphore: Semaphore::empty(1),
        }
    }

    fn write_slice_unsafe(&self, bytes:&[u8]) {
        let len = bytes.len();
        if len > core::u16::MAX as usize {
            panic!("Can't send more than 65kB at once");
        }
        let status = unsafe { HAL_UART_Transmit_IT(
            self.uart,
            // for some reason, HAL_UART_Transmit_IT accept buffer as mutable
            bytes.as_ptr() as *mut u8,
            len as u16)
        };
        match status {
            HAL_StatusTypeDef::HAL_OK => {}
            _ => panic!("Unexpected status from HAL_UART_Transmit_IT")
        }
    }

    fn write_slice_blocking(&self, bytes: &[u8]) {
        self.write_slice_unsafe(bytes);
        self.semaphore.acquire();
    }
}

impl UART for STM32_UART {
    fn write(&self, s: &str){
        let bytes = s.as_bytes();
        for chunk in bytes.chunks(core::u16::MAX as usize) {
            self.write_slice_blocking(chunk);
        }
    }
}

impl Release<UART_HandleTypeDef> for STM32_UART {
    fn checked_release(&self, ptr:*mut UART_HandleTypeDef) {
        if self.uart == ptr {
            self.semaphore.release();
        }
    }
}

impl_uart_write!(STM32_UART);

pub static DEBUG_UART: Static<Mutex<STM32_UART>> = Static::new();
pub static ALL_UARTS: [&Static<Mutex<STM32_UART>>;1] = [&DEBUG_UART];

pub fn debug_uart_init_static() {
    DEBUG_UART.init(
        Mutex::new(
            STM32_UART::new(unsafe { &mut huart2 })
        )
    );
}

pub fn debug_uart_get() -> &'static Mutex<STM32_UART> {
    DEBUG_UART.get()
}

#[no_mangle]
pub extern "C" fn HAL_UART_TxCpltCallback(_huart: *mut UART_HandleTypeDef) {
    checked_release(&ALL_UARTS, _huart);
}
