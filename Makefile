.PHONY: build run clean

TARGET := riscv64gc-unknown-none-elf
KERNEL := target/$(TARGET)/debug/nexa-kernel

build:
	cargo build

run: build
	qemu-system-riscv64 -machine virt -m 128M -smp 1 -nographic -bios default -kernel $(KERNEL)

clean:
	cargo clean
