extern trim
extern sort
extern count_occurance

extern fopen
extern fclose
extern getline
extern printf
extern fprintf
extern stderr

extern calloc
extern reallocarray
extern free

extern strtok
extern strtoll

extern exit

section .rodata
    input_file: db "input.txt", 0x00
    input_file_mode: db "r", 0x00

    delim: db " ", 0x00

    pf1: db "%s", 0x0A, 0x00
    pf2: db "%ld", 0x0A, 0x00

    pf_part1: db "part 1: %ld", 0x0A, 0x00
    pf_part2: db "part 2: %ld", 0x0A, 0x00

    no_file: db "could not find file '%s'", 0x0A, 0x00
    incorrect_file: db "incorrect file format", 0x0A, 0x00
    alloc_failed: db "failed to allocate memory", 0x0A, 0x00

    usage: db "usage: day01 [INPUT_FILE]", 0x0A, 0x00

section .bss
    getline_lineptr: dq ?
    getline_n: dq ?


section .text

global _start

%macro free_left_right_vals 0
    mov rdi, r12
    call free wrt ..plt
    mov rdi, r13
    call free wrt ..plt
%endmacro

%macro close_file 0
    mov rdi, r14
    call fclose wrt ..plt
%endmacro

%macro print_simple_err 1
    mov rdi, [rel stderr]
    lea rsi, [rel %1]
    call fprintf wrt ..plt
%endmacro

%macro exit_err 0
    mov rdi, 1
    call exit wrt ..plt
%endmacro

_start:
    ; technically need to sub rsp, 0x8?
    ; but doesn't seem to matter so whatever

    ; [rsp]: argc
    ; [rsp+16]: argv[1]
    mov rdi, [rsp]
    cmp rdi, 2
    je .Lusage_error_skip
        print_simple_err usage
        exit_err
    .Lusage_error_skip:

    ; open input file
    mov rdi, [rsp + 16]
    mov r15, rdi ; used for the error case when the file doesn't exist
    lea rsi, [rel input_file_mode]
    call fopen wrt ..plt

    ; check if the file existed
    test eax, eax
    jnz .Lno_file_error_skip
        mov rdx, r15
        print_simple_err no_file
        exit_err
    .Lno_file_error_skip:

    mov r14, rax ; FILE*

    ; allocate left_vals and right_vals
    ; starting with size 0x100
    mov rbp, 0x100 ; rbp will always contain the size of the allocated area
    ; will get resized if needed

    mov rdi, 0x8
    mov rsi, rbp
    call calloc wrt ..plt
    test eax, eax
    jnz .Lalloc_error_skip1
        print_simple_err alloc_failed
        close_file
        exit_err
    .Lalloc_error_skip1:
    mov r12, rax ; left_vals*

    mov rdi, 0x8
    mov rsi, rbp
    call calloc wrt ..plt
    test eax, eax
    jnz .Lalloc_error_skip2
        ; if only the second calloc fails, we need to free the first
        mov rdi, r12
        call free wrt ..plt

        print_simple_err alloc_failed
        close_file
        exit_err
    .Lalloc_error_skip2:
    mov r13, rax ; right_vals*

    xor ebx, ebx ; will contain our current index in left_vals/right_vals

    .Lgetline_loop:
        ; get line from input file
        lea rdi, [rel getline_lineptr]
        lea rsi, [rel getline_n]
        mov rdx, r14
        call getline wrt ..plt

        cmp rax, -1
        je .Lgetline_loop_end

        ; trim the line (needed primarily for the emtpy check)
        mov rdi, [rel getline_lineptr]
        call trim
        mov r15, rax

        ; check for empty line
        mov dil, byte [r15]
        test dil, dil
        jz .Lgetline_loop

        ; == get left number
        mov rdi, r15
        lea rsi, [rel delim]
        call strtok wrt ..plt
        test eax, eax
        jnz .Lincorrect_file_format_skip1
            free_left_right_vals
            print_simple_err incorrect_file
            close_file
            exit_err
        .Lincorrect_file_format_skip1:

        ; rax now contains a pointer to the line's left value (char*)

        ; convert to number
        mov rdi, rax
        mov rsi, 0
        mov rdx, 10
        call strtoll wrt ..plt
        ; strtoll ignores all non-digits for us
        ; so we don't exactly need error handling
        ; (idc about overflow cases)

        ; rax now contains line's left value (i64)

        ; update left_vals

        ; check first if left/right_vals needs to be resized
        ; if so, we should resize both
        ; rbp: size of the array
        ; rbx: current number of elements in the array
        cmp rbp, rbx
        jg .Lend_resize
            push rax

            shl rbp, 1

            mov rdi, r12
            mov rsi, 8
            mov rdx, rbp
            call reallocarray wrt ..plt
            test eax, eax
            jnz .Lrealloc_error_skip1
                free_left_right_vals
                print_simple_err alloc_failed
                close_file
                exit_err
            .Lrealloc_error_skip1:

            mov r12, rax

            mov rdi, r13
            mov rsi, 8
            mov rdx, rbp
            call reallocarray wrt ..plt
            test eax, eax
            jnz .Lrealloc_error_skip2
                free_left_right_vals
                print_simple_err alloc_failed
                close_file
                exit_err
            .Lrealloc_error_skip2:

            mov r13, rax

            pop rax
        .Lend_resize:

        mov [r12 + rbx*8], rax


        ; == get right number
        mov rdi, 0
        lea rsi, [rel delim]
        call strtok wrt ..plt
        test eax, eax
        jnz .Lincorrect_file_format_skip2
            free_left_right_vals
            print_simple_err incorrect_file
            close_file
            exit_err
        .Lincorrect_file_format_skip2:

        ; rax now contains a pointer to the line's right value (char*)

        ; convert to number
        mov rdi, rax
        mov rsi, 0
        mov rdx, 10
        call strtoll wrt ..plt
        ; rax now contains line's right value (i64)

        ; update right_vals
        ; no need to resize as left_vals and right_vals will always be the same size
        mov [r13 + rbx*8], rax


        ; no need to free getline_lineptr
        ; because it gets reused in the next run of the loop

        inc rbx
        jmp .Lgetline_loop

    .Lgetline_loop_end:
    mov rdi, [rel getline_lineptr]
    call free wrt ..plt

    close_file

    ; left_vals and right_vals now contain the appropriate values
    ; and rbx has their size

    ; first sort them
    mov rdi, r12
    mov rsi, rbx
    call sort

    mov rdi, r13
    mov rsi, rbx
    call sort

    ; solve part 1
    xor eax, eax ; index
    xor esi, esi ; result
    jmp .Ldistance_loop_begin
    .Ldistance_loop:
        mov rcx, [r12 + rax*8]
        mov rdx, [r13 + rax*8]

        sub rdx, rcx
        mov rcx, rdx
        neg rcx
        cmp rdx, 0
        cmovl rdx, rcx

        add rsi, rdx

        inc rax
    .Ldistance_loop_begin:
        cmp rbx, rax
        jg .Ldistance_loop


    ; print out the result of part 1
    lea rdi, [rel pf_part1]
    call printf wrt ..plt


    ; solve part 2
    xor r14d, r14d ; index
    xor r15d, r15d ; result
    jmp .Loccurance_loop_begin
    .Loccurance_loop:
        ; we don't need the size of the allocated area anymore,
        ; so we can reuse rbp
        mov rbp, [r12 + r14*8]

        mov rdi, rbp
        mov rsi, r13
        mov rdx, rbx
        call count_occurance

        imul rbp, rax
        add r15, rbp

        inc r14
    .Loccurance_loop_begin:
        cmp rbx, r14
        jg .Loccurance_loop

    ; print out the result of part 2
    lea rdi, [rel pf_part2]
    mov rsi, r15
    call printf wrt ..plt


    ; free left_vals and right_vals
    free_left_right_vals

    ; we call glibc exit instead of using syscalls
    ; because otherwise the prints seem to not get flushed properly
    ; e.g. when using `./day01 > out.txt`
    mov rdi, 0
    call exit wrt ..plt
