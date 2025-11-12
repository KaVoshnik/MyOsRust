// Модуль для работы с памятью

/// Копирование памяти
pub unsafe fn memcpy(dest: *mut u8, src: *const u8, n: usize) {
    for i in 0..n {
        *dest.add(i) = *src.add(i);
    }
}

/// Заполнение памяти значением
pub unsafe fn memset(s: *mut u8, c: u8, n: usize) {
    for i in 0..n {
        *s.add(i) = c;
    }
}

/// Сравнение памяти
pub unsafe fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let a = *s1.add(i);
        let b = *s2.add(i);
        if a != b {
            return (a as i32) - (b as i32);
        }
    }
    0
}

/// Длина строки
pub unsafe fn strlen(s: *const u8) -> usize {
    let mut len = 0;
    while *s.add(len) != 0 {
        len += 1;
    }
    len
}

