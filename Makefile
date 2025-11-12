# Makefile для сборки MyOS

# Переменные
RUST_TARGET = x86_64-unknown-none
CARGO = cargo
NASM = nasm
QEMU = qemu-system-x86_64
LD = ld
GRUB_MKRESCUE = grub-mkrescue

# Флаги
NASMFLAGS = -f elf64
QEMUFLAGS = -drive format=raw,file=iso/boot/myos.iso

# Директории
ISO_DIR = iso
BOOT_DIR = $(ISO_DIR)/boot
GRUB_DIR = $(ISO_DIR)/boot/grub

.PHONY: all build kernel iso run clean

all: iso

build kernel:
	$(CARGO) build --target $(RUST_TARGET)

# Создание загрузочного ISO образа
iso: kernel
	@mkdir -p $(GRUB_DIR)
	@echo "Проверка наличия Multiboot header в первых 32KB..."
	@if hexdump -C target/$(RUST_TARGET)/debug/myos | head -100 | grep -q "d6 50 52 e8"; then \
		echo "✓ Multiboot header найден в первых 32KB"; \
	else \
		echo "⚠ Multiboot header не найден в первых 32KB"; \
		echo "Поиск заголовка в файле..."; \
		if hexdump -C target/$(RUST_TARGET)/debug/myos | grep -q "d6 50 52 e8"; then \
			echo "Заголовок найден, но не в первых 32KB"; \
		else \
			echo "Заголовок не найден в файле"; \
		fi; \
	fi
	@cp target/$(RUST_TARGET)/debug/myos $(BOOT_DIR)/kernel.bin
	@echo 'set timeout=0' > $(GRUB_DIR)/grub.cfg
	@echo 'set default=0' >> $(GRUB_DIR)/grub.cfg
	@echo '' >> $(GRUB_DIR)/grub.cfg
	@echo 'menuentry "MyOS" {' >> $(GRUB_DIR)/grub.cfg
	@echo '  multiboot2 /boot/kernel.bin' >> $(GRUB_DIR)/grub.cfg
	@echo '  boot' >> $(GRUB_DIR)/grub.cfg
	@echo '}' >> $(GRUB_DIR)/grub.cfg
	@$(GRUB_MKRESCUE) -o myos.iso $(ISO_DIR) 2>/dev/null || echo "Ошибка: grub-mkrescue не найден. Установите GRUB tools."

# Запуск в QEMU
run: iso
	$(QEMU) -cdrom myos.iso

# Запуск с отладкой
debug: iso
	$(QEMU) -cdrom myos.iso -serial stdio

clean:
	$(CARGO) clean
	rm -rf $(ISO_DIR) myos.iso

