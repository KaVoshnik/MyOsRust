// Модуль для работы с таймером (PIT - Programmable Interval Timer)

use crate::ports;

const PIT_CHANNEL0: u16 = 0x40;
const PIT_COMMAND: u16 = 0x43;

const PIT_FREQUENCY: u32 = 1193182; // Базовая частота PIT в Hz
const TARGET_FREQUENCY: u32 = 100;  // 100 Hz (10 мс на тик)

static mut TICKS: u64 = 0;

/// Инициализация таймера
pub fn init() {
    unsafe {
        let divisor = (PIT_FREQUENCY / TARGET_FREQUENCY) as u16;
        
        // Команда: канал 0, режим 3 (square wave), формат binary
        ports::outb(PIT_COMMAND, 0x36);
        
        // Установка делителя (младший и старший байты)
        ports::outb(PIT_CHANNEL0, (divisor & 0xFF) as u8);
        ports::outb(PIT_CHANNEL0, ((divisor >> 8) & 0xFF) as u8);
    }
}

/// Обработчик прерывания таймера
pub fn handle_interrupt() {
    unsafe {
        TICKS += 1;
        
        // Каждую секунду выводим сообщение
        if TICKS % 100 == 0 {
            crate::println!("Тик: {} секунд", TICKS / 100);
        }
    }
}

/// Получить количество тиков
pub fn get_ticks() -> u64 {
    unsafe { TICKS }
}

/// Задержка в миллисекундах
pub fn sleep_ms(ms: u64) {
    let target_ticks = get_ticks() + (ms / 10);
    while get_ticks() < target_ticks {
        crate::interrupts::hlt();
    }
}

