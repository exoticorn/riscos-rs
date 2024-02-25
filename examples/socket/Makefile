default: target/socket,ff8

target/socket,ff8: target/arm-none-eabi/release/socket
	arm-none-eabi-objcopy -O binary target/arm-none-eabi/release/socket target/socket,ff8

target/arm-none-eabi/release/socket: PHONY
	cargo build --release

PHONY:
