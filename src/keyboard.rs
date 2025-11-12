// Модуль для работы с клавиатурой

use crate::ports;

const KEYBOARD_DATA_PORT: u16 = 0x60;
const KEYBOARD_STATUS_PORT: u16 = 0x64;

// Скан-коды клавиш (без Shift)
const SCANCODE_TABLE: [char; 128] = [
    '\0', '\x1B', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\x08',
    '\t', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n',
    '\0', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '`',
    '\0', '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/',
    '\0', '*', '\0', ' ', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
    '\0', '7', '8', '9', '-', '4', '5', '6', '+', '1', '2', '3', '0', '.',
    '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
    '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
    '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
];

static mut LAST_SCANCODE: u8 = 0;

/// Обработчик прерывания клавиатуры
pub fn handle_interrupt() {
    unsafe {
        let scancode = ports::inb(KEYBOARD_DATA_PORT);
        
        // Игнорируем отпускание клавиши (старший бит установлен)
        if scancode & 0x80 != 0 {
            return;
        }
        
        LAST_SCANCODE = scancode;
        
        if scancode < 128 {
            let ch = SCANCODE_TABLE[scancode as usize];
            if ch != '\0' {
                crate::print!("{}", ch);
            }
        }
    }
}

/// Получить последний скан-код
pub fn get_last_scancode() -> u8 {
    unsafe { LAST_SCANCODE }
}

/// Проверка готовности клавиатуры
pub fn is_key_pressed() -> bool {
    unsafe {
        (ports::inb(KEYBOARD_STATUS_PORT) & 1) != 0
    }
}

