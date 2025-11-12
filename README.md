# MyOS - Операционная система на Rust и ассемблере

Проект операционной системы, написанной на Rust и ассемблере.

## Требования

- Rust (nightly версия с компонентом rust-src)
- NASM (ассемблер)
- QEMU (для эмуляции)
- LLVM tools (для линковки)

## Установка зависимостей

### Rust компоненты:

```bash
# Установка nightly версии Rust
rustup toolchain install nightly
rustup default nightly

# Добавление необходимых компонентов
rustup target add x86_64-unknown-none
rustup component add rust-src --toolchain nightly
```

### NASM:

- **Windows**: `choco install nasm` или скачать с https://www.nasm.us/
- **Linux**: `sudo apt install nasm` или `sudo yum install nasm`
- **macOS**: `brew install nasm`

### QEMU:

- **Windows**: `choco install qemu`
- **Linux**: `sudo apt install qemu-system-x86` или `sudo yum install qemu-system-x86`
- **macOS**: `brew install qemu`

### GRUB (для создания ISO образа):

- **Windows**: `choco install grub` или используйте WSL
- **Linux**: `sudo apt install grub-pc-bin` или `sudo yum install grub2-tools`
- **macOS**: `brew install grub` (может потребоваться дополнительная настройка)

### LLVM tools (для линковки):

- **Windows**: Обычно включены в Rust установку
- **Linux**: `sudo apt install llvm` или `sudo yum install llvm`
- **macOS**: `brew install llvm`

## Сборка

### Вариант 1: Используя Makefile

```bash
make build
```

### Вариант 2: Используя Cargo напрямую

```bash
cargo build --target x86_64-unknown-none
```

После сборки нужно создать загрузочный ISO образ. Используйте Makefile:

```bash
make iso
```

Это создаст `myos.iso` файл, который можно загрузить в QEMU или записать на диск.

**Примечание:** Для создания ISO образа требуется `grub-mkrescue` (часть пакета GRUB).

## Запуск

### Вариант 1: Используя Makefile

```bash
make run
```

### Вариант 2: Используя QEMU напрямую

```bash
# Сначала создайте ISO образ
make iso

# Затем запустите
qemu-system-x86_64 -cdrom myos.iso
```

### Запуск с отладкой (вывод в консоль)

```bash
make debug
```

## Структура проекта

```
MyOs/
├── src/
│   ├── main.rs      # Точка входа в Rust код
│   ├── boot.asm     # Загрузчик на ассемблере (Multiboot 2)
│   ├── lib.rs       # Библиотека (экспорт модулей)
│   ├── vga.rs       # Драйвер VGA для вывода текста
│   ├── ports.rs     # Работа с портами ввода/вывода
│   ├── interrupts.rs # Обработка прерываний
│   ├── memory.rs    # Утилиты для работы с памятью
│   └── serial.rs    # Последовательный порт для отладки
├── .cargo/
│   └── config.toml  # Конфигурация Cargo
├── Cargo.toml       # Конфигурация проекта Rust
├── linker.ld        # Скрипт линковки
├── build.rs         # Скрипт сборки
├── Makefile         # Makefile для удобной сборки
└── README.md        # Этот файл
```

## Текущий функционал

- ✅ Базовый загрузчик на ассемблере (Multiboot 2)
- ✅ Инициализация стека
- ✅ Полноценный VGA драйвер с прокруткой экрана
- ✅ Макросы `print!` и `println!` для удобного вывода
- ✅ Поддержка цветов в VGA режиме
- ✅ Модуль для работы с портами ввода/вывода
- ✅ GDT (Global Descriptor Table) для управления сегментами
- ✅ IDT (Interrupt Descriptor Table) с обработчиками всех исключений CPU
- ✅ PIC (Programmable Interrupt Controller) для управления прерываниями
- ✅ Таймер (PIT) с частотой 100 Hz
- ✅ Обработка клавиатуры (PS/2) с выводом символов
- ✅ Утилиты для работы с памятью (memcpy, memset, memcmp, strlen)
- ✅ Последовательный порт (COM1) для отладки
- ✅ Улучшенный обработчик паники с выводом информации

## Следующие шаги

- [ ] Управление памятью (paging, heap allocator)
- [ ] Планировщик задач и многозадачность
- [ ] Улучшенная обработка клавиатуры (специальные клавиши, буфер ввода)
- [ ] Файловая система
- [ ] Системные вызовы
- [ ] Драйверы для других устройств (мышь, диски)
- [ ] Поддержка многопроцессорности

## Полезные ресурсы

- [Writing an OS in Rust](https://os.phil-opp.com/) - Отличный учебник по написанию ОС на Rust
- [Multiboot Specification](https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html) - Спецификация Multiboot 2
- [OSDev Wiki](https://wiki.osdev.org/) - Вики по разработке ОС
