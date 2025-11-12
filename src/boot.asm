; Загрузчик для MyOS
; Использует Multiboot 2 спецификацию

; Multiboot 2 заголовок должен быть в первых 32KB файла
; Используем отдельную секцию, которая будет размещена первой
section .multiboot_header, alloc
align 8
multiboot_header:
    dd 0xe85250d6                ; Magic number (Multiboot 2)
    dd 0                          ; Architecture 0 (i386/x86-64)
    dd multiboot_header_end - multiboot_header  ; Header length
    dd 0x100000000 - (0xe85250d6 + 0 + (multiboot_header_end - multiboot_header)) ; Checksum

    ; End tag
    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size
multiboot_header_end:

section .text
bits 64

global _start
extern rust_main

_start:
    ; Настройка стека (64-битный режим)
    ; Используем RIP-relative адресацию для избежания проблем с релокацией
    lea rsp, [rel stack_top]
    
    ; Вызов Rust кода
    call rust_main
    
    ; Бесконечный цикл если что-то пойдет не так
    cli
.hang:
    hlt
    jmp .hang

section .bss
align 16
stack_bottom:
    resb 16384 ; 16 KB стек
stack_top:

; Экспортируем символ стека для использования в Rust
global stack_top

