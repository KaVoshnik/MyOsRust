// Модуль для работы с последовательным портом (COM1)
// Используется для отладки

use crate::ports;

const SERIAL_PORT: u16 = 0x3F8; // COM1

/// Инициализация последовательного порта
pub fn init() {
    unsafe {
        // Отключение всех прерываний
        ports::outb(SERIAL_PORT + 1, 0x00);
        // Установка скорости (115200 бод)
        ports::outb(SERIAL_PORT + 3, 0x80); // Включить DLAB
        ports::outb(SERIAL_PORT + 0, 0x01); // Младший байт делителя
        ports::outb(SERIAL_PORT + 1, 0x00); // Старший байт делителя
        ports::outb(SERIAL_PORT + 3, 0x03); // 8 бит, без паритета, 1 стоп-бит
        ports::outb(SERIAL_PORT + 2, 0xC7); // Включить FIFO
        ports::outb(SERIAL_PORT + 4, 0x0B); // Включить RTS, DSR
    }
}

/// Проверка готовности к отправке
fn is_transmit_empty() -> bool {
    unsafe {
        (ports::inb(SERIAL_PORT + 5) & 0x20) != 0
    }
}

/// Отправка байта через последовательный порт
pub fn write_byte(byte: u8) {
    unsafe {
        while !is_transmit_empty() {}
        ports::outb(SERIAL_PORT, byte);
    }
}

/// Отправка строки через последовательный порт
pub fn write_string(s: &str) {
    for byte in s.bytes() {
        write_byte(byte);
    }
}

