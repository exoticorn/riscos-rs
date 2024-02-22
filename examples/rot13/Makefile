default: target/rot13,ff8

target/rot13,ff8: target/arm-none-eabi/release/rot13
	arm-none-eabi-objcopy -O binary target/arm-none-eabi/release/rot13 target/rot13,ff8

target/arm-none-eabi/release/rot13: PHONY
	cargo build --release

PHONY:
