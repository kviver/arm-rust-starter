use super::bindings::{
    HAL_GPIO_TogglePin,
    HAL_GPIO_WritePin,
    GPIO_TypeDef,
    GPIO_PinState,
    HAL_LD2_GPIO_Port,
    HAL_LD2_Pin,
};

use os::Mutex;
use peripheral::Static;

use hal::traits::{Pin, PinState};

pub struct GPIOPin {
    gpio_port: *mut GPIO_TypeDef,
    pin: u16,
}

impl Pin for GPIOPin {
    fn write(&mut self, state: PinState) {
        let stm_state = match state {
            PinState::Reset => GPIO_PinState::GPIO_PIN_RESET,
            PinState::Set => GPIO_PinState::GPIO_PIN_SET,
        };
        unsafe { HAL_GPIO_WritePin(self.gpio_port, self.pin, stm_state) };
    }

    fn toggle(&mut self) {
        unsafe { HAL_GPIO_TogglePin(self.gpio_port, self.pin) };
    }
}

pub static LD2_PIN: Static<Mutex<GPIOPin>> = Static::new();

pub fn init_pins() {
    LD2_PIN.init(Mutex::new(GPIOPin {
        gpio_port: unsafe { HAL_LD2_GPIO_Port },
        pin: HAL_LD2_Pin,
    }));
}
