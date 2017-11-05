#[repr(C)]
pub struct GPIO_TypeDef;

#[repr(C)]
pub struct UART_HandleTypeDef;

#[repr(C)]
pub enum GPIO_PinState {
    GPIO_PIN_RESET = 0,
    GPIO_PIN_SET   = 1,
}

#[repr(C)]
pub enum HAL_StatusTypeDef {
    HAL_OK       = 0x00,
    HAL_ERROR    = 0x01,
    HAL_BUSY     = 0x02,
    HAL_TIMEOUT  = 0x03
}

extern {
    pub fn HAL_GPIO_WritePin(GPIOx: *mut GPIO_TypeDef, GPIO_Pin: u16, PinState: GPIO_PinState);
    pub fn HAL_GPIO_TogglePin(GPIOx: *mut GPIO_TypeDef, GPIO_Pin: u16);
    pub fn HAL_UART_Transmit_IT(huart: *mut UART_HandleTypeDef, pData: *const u8, Size: u16) -> HAL_StatusTypeDef;
    
    pub static HAL_LD2_GPIO_Port: *mut GPIO_TypeDef;
    pub static HAL_LD2_Pin: u16;
    
    pub fn get_huart2() -> *mut UART_HandleTypeDef;
}
