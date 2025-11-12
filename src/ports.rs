// Модуль для работы с портами ввода/вывода

use core::arch::asm;

/// Чтение байта из порта
#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    asm!("in al, dx", out("al") result, in("dx") port, options(nomem, nostack));
    result
}

/// Запись байта в порт
#[inline(always)]
pub unsafe fn outb(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack));
}

/// Чтение слова из порта
#[inline(always)]
pub unsafe fn inw(port: u16) -> u16 {
    let result: u16;
    asm!("in ax, dx", out("ax") result, in("dx") port, options(nomem, nostack));
    result
}

/// Запись слова в порт
#[inline(always)]
pub unsafe fn outw(port: u16, value: u16) {
    asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack));
}

/// Чтение двойного слова из порта
#[inline(always)]
pub unsafe fn inl(port: u16) -> u32 {
    let result: u32;
    asm!("in eax, dx", out("eax") result, in("dx") port, options(nomem, nostack));
    result
}

/// Запись двойного слова в порт
#[inline(always)]
pub unsafe fn outl(port: u16, value: u32) {
    asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack));
}

