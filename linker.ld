ENTRY(_start)

/* Multiboot header должен быть в первых 32KB исполняемого файла */
SECTIONS
{
    . = 1M;

    /* Заголовок Multiboot 2 - должен быть первым */
    .multiboot_header : {
        KEEP(*(.multiboot_header))
    }

    /* Код */
    .text : {
        *(.text)
    }

    /* Только для чтения данные */
    .rodata : {
        *(.rodata)
    }

    /* Данные */
    .data : {
        *(.data)
    }

    /* BSS */
    .bss : {
        *(COMMON)
        *(.bss)
    }
}

