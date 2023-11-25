# mode: makefile

build:
	cargo build --release --target thumbv6m-none-eabi
	rust-objcopy -O binary target/thumbv6m-none-eabi/release/{{project-name}} target/{{project-name}}.bin

upload serial_port:
    arduino-cli upload -i target/{{project-name}}.bin -b arduino:samd:nano_33_iot -p {{serial_port}}