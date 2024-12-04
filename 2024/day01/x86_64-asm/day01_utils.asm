extern isspace

section .text

global trim
global sort
global count_occurance

; removes whitespace at the beginning and end of a given string
; returns the new beginning, while setting a nullbyte for the new end
; i.e. this function is destructive for the underlying string
; but not the original pointer (so we can correctly free it later)
;
; rdi: char*
; ret: new char* (pointing to the same string)
trim:
    push r12
    push r13

    mov r12, rdi

    .Ltrim_begin_loop:
        movzx edi, byte [r12]
        call isspace wrt ..plt

        test eax, eax
        je .Ltrim_begin_loop_end

        inc r12
        jmp .Ltrim_begin_loop

    .Ltrim_begin_loop_end:
    ; set return value
    mov r13, r12

    ; go to end of string
    .Lgoto_null_loop:
        mov dil, byte [r12]
        test dil, dil
        je .Lgoto_null_loop_end
        inc r12
        jmp .Lgoto_null_loop

    .Lgoto_null_loop_end:
    ; r12 now points to the nullbyte of the string

    cmp r13, r12    ; checks if the string is empty
    je .Lend        ; => nothing to trim

    dec r12

    .Ltrim_end_loop:
        movzx edi, byte [r12]
        call isspace wrt ..plt

        test eax, eax
        je .Ltrim_end_loop_end

        dec r12
        jmp .Ltrim_end_loop

    .Ltrim_end_loop_end:
    inc r12
    mov byte [r12], 0

.Lend:

    mov rax, r13
    pop r13
    pop r12
    ret


; simple insertion sort, because I didn't want to go any crazier for this
; rdi: i64[] array
; rsi: i64 len
sort:
    mov rcx, 1
    jmp .Lloop_begin

    ; rcx: current index in array we are inserting
    ; rax: temporary index for moving the element to the correct position
    .Lloop:
        mov rax, rcx
        .Lmove_loop:
            mov r8, [rdi + rax*8 - 8]
            mov r9, [rdi + rax*8]
            cmp r8, r9
            jl .Lmove_loop_end

            mov [rdi + rax*8], r8
            mov [rdi + rax*8 - 8], r9

            dec rax
            cmp rax, 0
            jg .Lmove_loop
        .Lmove_loop_end:

        inc rcx
    .Lloop_begin:
        cmp rsi, rcx
        jg .Lloop
    ret


; counts how many times the value in rdi appears in the array in rsi
; and quits once it finds a value larger than rdi (i.e. we assume rsi is sorted)
; rdi: i64 value
; rsi: i64[] array
; rdx: array length
; return rax: count
count_occurance:
    xor eax, eax
    lea rcx, [rsi + rdx*8] ; last element's addr, used for bounds check
    jmp .Lloop_begin
    .Lloop:
        lea r8, [rax + 1]
        cmp rdi, [rsi]
        cmove rax, r8
        jl .Lend

        add rsi, 8
    .Lloop_begin:
        cmp rcx, rsi
        jge .Lloop
    .Lend:
    ret
