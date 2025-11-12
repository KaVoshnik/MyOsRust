// Модуль для работы с PIC (Programmable Interrupt Controller)

use crate::ports;

const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const ICW1_ICW4: u8 = 0x01;
const ICW1_SINGLE: u8 = 0x02;
const ICW1_INTERVAL4: u8 = 0x04;
const ICW1_LEVEL: u8 = 0x08;
const ICW1_INIT: u8 = 0x10;

const ICW4_8086: u8 = 0x01;
const ICW4_AUTO: u8 = 0x02;
const ICW4_BUF_SLAVE: u8 = 0x08;
const ICW4_BUF_MASTER: u8 = 0x0C;
const ICW4_SFNM: u8 = 0x10;

/// Инициализация PIC
/// Перенастраивает прерывания на векторы 32-47
pub fn init() {
    unsafe {
        let a1 = ports::inb(PIC1_DATA);
        let a2 = ports::inb(PIC2_DATA);

        // Инициализация
        ports::outb(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4);
        ports::outb(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4);
        
        // Смещение векторов
        ports::outb(PIC1_DATA, 32);  // IRQ 0-7 -> прерывания 32-39
        ports::outb(PIC2_DATA, 40);  // IRQ 8-15 -> прерывания 40-47
        
        // Подключение slave к master
        ports::outb(PIC1_DATA, 4);
        ports::outb(PIC2_DATA, 2);
        
        // Режим 8086
        ports::outb(PIC1_DATA, ICW4_8086);
        ports::outb(PIC2_DATA, ICW4_8086);
        
        // Восстановление масок
        ports::outb(PIC1_DATA, a1);
        ports::outb(PIC2_DATA, a2);
    }
}

/// Маскирование прерывания
pub unsafe fn mask_irq(irq: u8) {
    let port = if irq < 8 { PIC1_DATA } else { PIC2_DATA };
    let irq = if irq < 8 { irq } else { irq - 8 };
    let value = ports::inb(port) | (1 << irq);
    ports::outb(port, value);
}

/// Размаскирование прерывания
pub unsafe fn unmask_irq(irq: u8) {
    let port = if irq < 8 { PIC1_DATA } else { PIC2_DATA };
    let irq = if irq < 8 { irq } else { irq - 8 };
    let value = ports::inb(port) & !(1 << irq);
    ports::outb(port, value);
}

/// Отправка EOI (End of Interrupt)
pub fn send_eoi(irq: u8) {
    unsafe {
        if irq >= 8 {
            ports::outb(PIC2_COMMAND, 0x20);
        }
        ports::outb(PIC1_COMMAND, 0x20);
    }
}

