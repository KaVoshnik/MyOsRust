// Модуль для работы с IDT (Interrupt Descriptor Table)

use core::arch::asm;

/// Типы шлюзов прерываний
#[repr(u8)]
#[allow(dead_code)]
enum GateType {
    Interrupt = 0x8E,
    Trap = 0x8F,
}

/// Структура дескриптора прерывания
#[repr(C, packed)]
#[derive(Copy, Clone)]
struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    flags: u8,
    offset_middle: u16,
    offset_high: u32,
    reserved: u32,
}

impl IdtEntry {
    fn set_handler(&mut self, handler: unsafe extern "C" fn(), gate_type: GateType) {
        let addr = handler as u64;
        self.offset_low = addr as u16;
        self.offset_middle = (addr >> 16) as u16;
        self.offset_high = (addr >> 32) as u32;
        self.selector = 0x08; // Сегмент кода
        self.flags = gate_type as u8;
        self.ist = 0;
        self.reserved = 0;
    }
}

const IDT_SIZE: usize = 256;
static mut IDT: [IdtEntry; IDT_SIZE] = [IdtEntry {
    offset_low: 0,
    selector: 0,
    ist: 0,
    flags: 0,
    offset_middle: 0,
    offset_high: 0,
    reserved: 0,
}; IDT_SIZE];

#[repr(C, packed)]
struct IdtPtr {
    limit: u16,
    base: u64,
}

/// Инициализация IDT
pub fn init() {
    unsafe {
        // Установка обработчиков исключений
        set_handler(0, divide_error_handler, GateType::Interrupt);
        set_handler(1, debug_handler, GateType::Interrupt);
        set_handler(2, nmi_handler, GateType::Interrupt);
        set_handler(3, breakpoint_handler, GateType::Interrupt);
        set_handler(4, overflow_handler, GateType::Interrupt);
        set_handler(5, bound_range_handler, GateType::Interrupt);
        set_handler(6, invalid_opcode_handler, GateType::Interrupt);
        set_handler(7, device_not_available_handler, GateType::Interrupt);
        set_handler(8, double_fault_handler, GateType::Interrupt);
        set_handler(10, invalid_tss_handler, GateType::Interrupt);
        set_handler(11, segment_not_present_handler, GateType::Interrupt);
        set_handler(12, stack_fault_handler, GateType::Interrupt);
        set_handler(13, general_protection_fault_handler, GateType::Interrupt);
        set_handler(14, page_fault_handler, GateType::Interrupt);
        set_handler(16, x87_fpu_error_handler, GateType::Interrupt);
        set_handler(17, alignment_check_handler, GateType::Interrupt);
        set_handler(18, machine_check_handler, GateType::Interrupt);
        set_handler(19, simd_fpu_error_handler, GateType::Interrupt);
        set_handler(20, virtualization_handler, GateType::Interrupt);

        // Установка обработчика таймера (IRQ 0 -> прерывание 32)
        set_handler(32, timer_handler, GateType::Interrupt);

        // Установка обработчика клавиатуры (IRQ 1 -> прерывание 33)
        set_handler(33, keyboard_handler, GateType::Interrupt);

        // Загрузка IDT
        let idt_ptr = IdtPtr {
            limit: (core::mem::size_of::<IdtEntry>() * IDT_SIZE - 1) as u16,
            base: &raw const IDT as *const _ as u64,
        };

        asm!("lidt [{0}]", in(reg) &idt_ptr, options(nostack));
    }
}

unsafe fn set_handler(
    vector: u8,
    handler: unsafe extern "C" fn(),
    gate_type: GateType,
) {
    IDT[vector as usize].set_handler(handler, gate_type);
}

// Обработчики исключений
extern "C" {
    fn divide_error_handler();
    fn debug_handler();
    fn nmi_handler();
    fn breakpoint_handler();
    fn overflow_handler();
    fn bound_range_handler();
    fn invalid_opcode_handler();
    fn device_not_available_handler();
    fn double_fault_handler();
    fn invalid_tss_handler();
    fn segment_not_present_handler();
    fn stack_fault_handler();
    fn general_protection_fault_handler();
    fn page_fault_handler();
    fn x87_fpu_error_handler();
    fn alignment_check_handler();
    fn machine_check_handler();
    fn simd_fpu_error_handler();
    fn virtualization_handler();
    fn timer_handler();
    fn keyboard_handler();
}

// Обёртки для обработчиков
#[no_mangle]
pub extern "C" fn exception_handler_wrapper(vector: u8, error_code: u64) {
    match vector {
        0 => crate::println!("Исключение: Деление на ноль"),
        1 => crate::println!("Исключение: Отладка"),
        2 => crate::println!("Исключение: NMI"),
        3 => crate::println!("Исключение: Точка останова"),
        4 => crate::println!("Исключение: Переполнение"),
        5 => crate::println!("Исключение: Выход за границы"),
        6 => crate::println!("Исключение: Неверный код операции"),
        7 => crate::println!("Исключение: Устройство недоступно"),
        8 => crate::println!("Исключение: Двойная ошибка (код ошибки: {})", error_code),
        10 => crate::println!("Исключение: Неверный TSS (код ошибки: {})", error_code),
        11 => crate::println!("Исключение: Сегмент отсутствует (код ошибки: {})", error_code),
        12 => crate::println!("Исключение: Ошибка стека (код ошибки: {})", error_code),
        13 => crate::println!("Исключение: Общая ошибка защиты (код ошибки: {})", error_code),
        14 => {
            let cr2: u64;
            unsafe {
                asm!("mov {}, cr2", out(reg) cr2, options(nomem, nostack));
            }
            crate::println!("Исключение: Ошибка страницы (CR2: 0x{:X}, код ошибки: {})", cr2, error_code);
        }
        16 => crate::println!("Исключение: Ошибка FPU x87"),
        17 => crate::println!("Исключение: Проверка выравнивания (код ошибки: {})", error_code),
        18 => crate::println!("Исключение: Проверка машины"),
        19 => crate::println!("Исключение: Ошибка SIMD FPU"),
        20 => crate::println!("Исключение: Виртуализация"),
        _ => crate::println!("Неизвестное исключение: {} (код ошибки: {})", vector, error_code),
    }
    loop {}
}

/// Обработчик прерываний
#[no_mangle]
pub extern "C" fn interrupt_handler_wrapper(vector: u8) {
    match vector {
        32 => {
            // Таймер
            crate::timer::handle_interrupt();
            crate::pic::send_eoi(0);
        }
        33 => {
            // Клавиатура
            crate::keyboard::handle_interrupt();
            crate::pic::send_eoi(1);
        }
        _ => {
            crate::println!("Необработанное прерывание: {}", vector);
            crate::pic::send_eoi((vector - 32) as u8);
        }
    }
}

