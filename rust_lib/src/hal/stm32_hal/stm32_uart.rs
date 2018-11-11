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

pub struct Stm32Uart {
    uart: *mut UART_HandleTypeDef,
    semaphore: Semaphore,
}

impl Stm32Uart {
    fn new(uart: *mut UART_HandleTypeDef) -> Stm32Uart {
        Stm32Uart {
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

impl UART for Stm32Uart {
    fn write(&self, s: &str){
        let bytes = s.as_bytes();
        for chunk in bytes.chunks(core::u16::MAX as usize) {
            self.write_slice_blocking(chunk);
        }
    }
}

impl Release<UART_HandleTypeDef> for Stm32Uart {
    fn checked_release(&self, ptr:*mut UART_HandleTypeDef) {
        if self.uart == ptr {
            self.semaphore.release();
        }
    }
}

impl_uart_write!(Stm32Uart);

pub static DEBUG_UART: Static<Mutex<Stm32Uart>> = Static::new();
pub static ALL_UARTS: [&Static<Mutex<Stm32Uart>>;1] = [&DEBUG_UART];

pub fn debug_uart_init_static() {
    DEBUG_UART.init(
        Mutex::new(
            Stm32Uart::new(unsafe { &mut huart2 })
        )
    );
}

pub fn debug_uart_get() -> &'static Mutex<Stm32Uart> {
    DEBUG_UART.get()
}

#[no_mangle]
pub extern "C" fn HAL_UART_TxCpltCallback(_huart: *mut UART_HandleTypeDef) {
    checked_release(&ALL_UARTS, _huart);
}
