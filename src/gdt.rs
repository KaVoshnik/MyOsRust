// Модуль для работы с GDT (Global Descriptor Table)

use core::arch::asm;

/// Структура дескриптора сегмента
#[repr(C, packed)]
struct SegmentDescriptor {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

/// Структура GDT
#[repr(C, packed)]
struct Gdt {
    null: SegmentDescriptor,
    code: SegmentDescriptor,
    data: SegmentDescriptor,
}

impl SegmentDescriptor {
    fn new(base: u32, limit: u32, access: u8, granularity: u8) -> Self {
        SegmentDescriptor {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: ((base >> 16) & 0xFF) as u8,
            access,
            granularity: ((limit >> 16) & 0x0F) as u8 | (granularity & 0xF0),
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }
}

static mut GDT: Gdt = Gdt {
    null: SegmentDescriptor {
        limit_low: 0,
        base_low: 0,
        base_middle: 0,
        access: 0,
        granularity: 0,
        base_high: 0,
    },
    code: SegmentDescriptor {
        limit_low: 0,
        base_low: 0,
        base_middle: 0,
        access: 0,
        granularity: 0,
        base_high: 0,
    },
    data: SegmentDescriptor {
        limit_low: 0,
        base_low: 0,
        base_middle: 0,
        access: 0,
        granularity: 0,
        base_high: 0,
    },
};

#[repr(C, packed)]
struct GdtPtr {
    limit: u16,
    base: u64,
}

/// Инициализация GDT
/// В 64-битном режиме сегментация упрощена, но GDT всё ещё нужна
pub fn init() {
    unsafe {
        GDT.null = SegmentDescriptor::new(0, 0, 0, 0);
        GDT.code = SegmentDescriptor::new(0, 0, 0x9A, 0x20); // Код сегмент (64-bit)
        GDT.data = SegmentDescriptor::new(0, 0, 0x92, 0);   // Данные сегмент

        let gdt_ptr = GdtPtr {
            limit: (core::mem::size_of::<Gdt>() - 1) as u16,
            base: &raw const GDT as *const _ as u64,
        };

        // Загрузка GDT
        asm!("lgdt [{0}]", in(reg) &gdt_ptr, options(nostack));
    }
}

