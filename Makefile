# Makefile для сборки MyOS

# Переменные
RUST_TARGET = x86_64-unknown-none
CARGO = cargo
NASM = nasm
QEMU = qemu-system-x86_64

# Флаги
NASMFLAGS = -f elf64
QEMUFLAGS = -drive format=raw,file=target/$(RUST_TARGET)/debug/bootimage-myos.bin

.PHONY: all build run clean

all: build

build:
	$(CARGO) build --target $(RUST_TARGET)

run: build
	$(QEMU) $(QEMUFLAGS)

clean:
	$(CARGO) clean

