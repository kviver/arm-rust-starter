# STM32+Rust FFI starter kit

## Using docker
Run `docker-compose up -d`, then use `docker-compose exec dev bash` to get a shell with all the tools installed.

## How to build this
0. Download STM32CubeMX from st.com: [https://www.st.com/en/development-tools/stm32cubemx.html]
1. Create new STM32CubeMX makefile project with freertos into project dir. Main task needs at least 256 words of stack. For example:
	1. new project -> select your board
	2. pinout configuration, in left panel:
		1. enable freertos
		2. usart2 -> mode: asyncronous
	3. tab configuration:
	4. usart2 -> usart2 global interrupt enabled
	5. freertos -> task and queues -> set default task stack size to 256
	6. menu -> project -> settings -> set repo dir, toolchain: Makefile
	7. menu -> project -> generate code

2. Add to makefile after `all` target:

```
# libraries
RUST_LIB_NAME = rust_lib
LIBS += -l$(RUST_LIB_NAME) 
RUST_LIB_SRC = rust_lib
RUST_LIB_BUILD = $(RUST_LIB_SRC)/target/thumbv7em-none-eabihf/debug
LIBDIR += -L$(RUST_LIB_BUILD)

.PHONY: $(RUST_LIB_BUILD)/lib$(RUST_LIB_NAME).a

$(RUST_LIB_BUILD)/lib$(RUST_LIB_NAME).a:
	cd $(RUST_LIB_SRC) && cargo build --target=thumbv7em-none-eabihf
```
add `$(RUST_LIB_BUILD)/lib$(RUST_LIB_NAME).a` dependency to `$(BUILD_DIR)/$(TARGET).elf:`

add `cd $(RUST_LIB_SRC) && cargo clean` to `make clean` target

rm `-lc` from linker option

3. also add `rust_main_task()` call to `main.c/StartDefaultTask()`, GPIO port/pin macro static exports
```
void rust_main_task();
```
```
GPIO_TypeDef* HAL_LD2_GPIO_Port = LD2_GPIO_Port;
uint16_t HAL_LD2_Pin = LD2_Pin;
```

4. don't forget binpath in makefile (`BINPATH = /usr/bin/`, for example)

5. Compile
`make`

6. Flash uС
`./deploy.sh`

To debug Rust code use 
`RUST_GDB=arm-none-eabi-gdb rust-gdb <path to .elf file>`
