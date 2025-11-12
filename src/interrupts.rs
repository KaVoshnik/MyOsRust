// Модуль для обработки прерываний

use core::arch::asm;

/// Инициализация обработки прерываний
pub fn init() {
    unsafe {
        // Отключение прерываний
        asm!("cli", options(nomem, nostack));
        
        // Инициализация GDT
        crate::gdt::init();
        
        // Инициализация PIC
        crate::pic::init();
        
        // Инициализация IDT
        crate::idt::init();
        
        // Инициализация таймера
        crate::timer::init();
        
        // Размаскирование таймера и клавиатуры
        crate::pic::unmask_irq(0); // Таймер
        crate::pic::unmask_irq(1); // Клавиатура
        
        // Включение прерываний
        asm!("sti", options(nomem, nostack));
    }
}

/// Включение прерываний
pub unsafe fn enable() {
    asm!("sti", options(nomem, nostack));
}

/// Отключение прерываний
pub unsafe fn disable() {
    asm!("cli", options(nomem, nostack));
}

/// Ожидание прерывания
pub fn hlt() {
    unsafe {
        asm!("hlt", options(nomem, nostack));
    }
}

// Обработчик прерывания по умолчанию удалён - используется IDT

