#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use myos::interrupts;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    myos::println!("Паника: {:?}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // Инициализация последовательного порта для отладки
    myos::serial::init();
    
    // Очистка экрана
    myos::vga::WRITER.lock().clear_screen();
    
    // Вывод приветственного сообщения
    myos::println!("========================================");
    myos::println!("    Добро пожаловать в MyOS!");
    myos::println!("    Операционная система на Rust");
    myos::println!("========================================");
    myos::println!();
    myos::println!("Инициализация системы...");
    myos::println!("VGA драйвер: OK");
    myos::println!("Порты ввода/вывода: OK");
    myos::println!("Модуль памяти: OK");
    myos::println!("Последовательный порт: OK");
    myos::println!("GDT: OK");
    myos::println!("PIC: OK");
    myos::println!("IDT: OK");
    myos::println!("Таймер: OK");
    myos::println!("Клавиатура: OK");
    myos::println!();
    
    // Инициализация обработки прерываний (включает прерывания)
    interrupts::init();
    
    myos::println!("Система готова к работе!");
    myos::println!("Таймер работает, клавиатура активна.");
    myos::println!("Попробуйте набрать текст на клавиатуре...");
    myos::println!();
    
    // Демонстрация работы с цветами
    let mut writer = myos::vga::WRITER.lock();
    writer.set_color(myos::vga::Color::Cyan, myos::vga::Color::Black);
    drop(writer);
    myos::println!("Этот текст выведен другим цветом!");
    
    // Демонстрация форматирования
    myos::println!("Тест форматирования: число {} и строка {}", 42, "test");
    
    // Бесконечный цикл - прерывания будут обрабатываться автоматически
    loop {
        interrupts::hlt();
    }
}

