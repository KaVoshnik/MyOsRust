; Загрузчик для MyOS
; Использует Multiboot 2 спецификацию

section .multiboot_header
align 8
header_start:
    dd 0xe85250d6                ; Magic number (Multiboot 2)
    dd 0                          ; Architecture 0 (i386)
    dd header_end - header_start  ; Header length
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start)) ; Checksum

    ; End tag
    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size
header_end:

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

