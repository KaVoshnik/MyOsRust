; Ассемблерные обёртки для обработчиков прерываний

section .text
bits 64

; Макрос для создания обработчика исключения без кода ошибки
%macro exception_handler_no_error 1
global exception_handler_%1
exception_handler_%1:
    push 0          ; Пустой код ошибки
    push %1         ; Номер вектора
    jmp exception_common
%endmacro

; Макрос для создания обработчика исключения с кодом ошибки
%macro exception_handler_error 1
global exception_handler_%1
exception_handler_%1:
    push %1         ; Номер вектора (код ошибки уже в стеке)
    jmp exception_common
%endmacro

; Макрос для создания обработчика прерывания
%macro interrupt_handler 1
global interrupt_handler_%1
interrupt_handler_%1:
    push 0          ; Пустой код ошибки
    push %1         ; Номер вектора
    jmp interrupt_common
%endmacro

; Общий код для исключений
exception_common:
    ; Сохранение всех регистров
    push rax
    push rbx
    push rcx
    push rdx
    push rsi
    push rdi
    push rbp
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15

    ; Вызов Rust обработчика
    ; После push всех регистров (15 * 8 = 120 байт) в стеке:
    ; [rsp + 0] = r15 (последний push)
    ; ...
    ; [rsp + 112] = rax (первый push)
    ; [rsp + 120] = код ошибки (8 байт)
    ; [rsp + 128] = номер вектора (8 байт)
    mov rdi, [rsp + 128]  ; Номер вектора
    mov rsi, [rsp + 120]  ; Код ошибки
    extern exception_handler_wrapper
    call exception_handler_wrapper

    ; Восстановление регистров (недостижимо, но для полноты)
    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop rbp
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop rbx
    pop rax

    add rsp, 16     ; Удаление номера вектора и кода ошибки
    iretq

; Общий код для прерываний
interrupt_common:
    ; Сохранение всех регистров
    push rax
    push rbx
    push rcx
    push rdx
    push rsi
    push rdi
    push rbp
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15

    ; Вызов Rust обработчика
    ; После push всех регистров (15 * 8 = 120 байт) в стеке:
    ; [rsp + 0] = r15 (последний push)
    ; ...
    ; [rsp + 112] = rax (первый push)
    ; [rsp + 120] = код ошибки (8 байт, всегда 0 для прерываний)
    ; [rsp + 128] = номер вектора (8 байт)
    mov rdi, [rsp + 128]  ; Номер вектора
    extern interrupt_handler_wrapper
    call interrupt_handler_wrapper

    ; EOI уже отправлен в Rust коде, восстанавливаем регистры
    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop rbp
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop rbx
    pop rax

    add rsp, 16     ; Удаление номера вектора и кода ошибки
    iretq

; Обработчики исключений
exception_handler_no_error 0   ; Divide Error
exception_handler_no_error 1   ; Debug
exception_handler_no_error 2   ; NMI
exception_handler_no_error 3   ; Breakpoint
exception_handler_no_error 4   ; Overflow
exception_handler_no_error 5   ; Bound Range
exception_handler_no_error 6   ; Invalid Opcode
exception_handler_no_error 7   ; Device Not Available
exception_handler_error 8      ; Double Fault
exception_handler_error 10     ; Invalid TSS
exception_handler_error 11     ; Segment Not Present
exception_handler_error 12     ; Stack Fault
exception_handler_error 13     ; General Protection Fault
exception_handler_error 14     ; Page Fault
exception_handler_no_error 16  ; x87 FPU Error
exception_handler_error 17     ; Alignment Check
exception_handler_no_error 18  ; Machine Check
exception_handler_no_error 19  ; SIMD FPU Error
exception_handler_no_error 20  ; Virtualization

; Обработчики прерываний
interrupt_handler 32   ; Timer (IRQ 0)
interrupt_handler 33   ; Keyboard (IRQ 1)

; Экспорт символов для Rust
global divide_error_handler
global debug_handler
global nmi_handler
global breakpoint_handler
global overflow_handler
global bound_range_handler
global invalid_opcode_handler
global device_not_available_handler
global double_fault_handler
global invalid_tss_handler
global segment_not_present_handler
global stack_fault_handler
global general_protection_fault_handler
global page_fault_handler
global x87_fpu_error_handler
global alignment_check_handler
global machine_check_handler
global simd_fpu_error_handler
global virtualization_handler
global timer_handler
global keyboard_handler

divide_error_handler: jmp exception_handler_0
debug_handler: jmp exception_handler_1
nmi_handler: jmp exception_handler_2
breakpoint_handler: jmp exception_handler_3
overflow_handler: jmp exception_handler_4
bound_range_handler: jmp exception_handler_5
invalid_opcode_handler: jmp exception_handler_6
device_not_available_handler: jmp exception_handler_7
double_fault_handler: jmp exception_handler_8
invalid_tss_handler: jmp exception_handler_10
segment_not_present_handler: jmp exception_handler_11
stack_fault_handler: jmp exception_handler_12
general_protection_fault_handler: jmp exception_handler_13
page_fault_handler: jmp exception_handler_14
x87_fpu_error_handler: jmp exception_handler_16
alignment_check_handler: jmp exception_handler_17
machine_check_handler: jmp exception_handler_18
simd_fpu_error_handler: jmp exception_handler_19
virtualization_handler: jmp exception_handler_20
timer_handler: jmp interrupt_handler_32
keyboard_handler: jmp interrupt_handler_33

