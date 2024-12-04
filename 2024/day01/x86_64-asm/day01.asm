global _start

extern trim
extern sort
extern count_occurance

extern fopen
extern getline
extern printf
extern puts

extern calloc
extern reallocarray
extern free

extern strtok
extern strtoll

section .rodata
    input_file: db "input.txt", 0x00
    input_file_mode: db "r", 0x00

    delim: db " ", 0x00

    pf1: db "%s", 0x0A, 0x00
    pf2: db "%ld", 0x0A, 0x00

    pf_part1: db "part 1: %ld", 0x0A, 0x00
    pf_part2: db "part 2: %ld", 0x0A, 0x00

    no_file: db "could not find file 'input.txt'", 0x00

section .bss
    getline_lineptr: dq ?
    getline_n: dq ?

    left_vals: dq ?     ; i64[]
    right_vals: dq ?    ; i64[]


section .text

_start:
    ; TODO: currently, it just statically allocated 0x1000 for the value array
    ; meaning an array longer than that will fuck up everything
    ; we should really dynamically resize it for a good solution, but this gives us the correct answer at least

    ; allocate left_vals and right_vals
    ; starting with size 0x1000
    mov rbp, 0x100 ; rbp will always contain the size of the allocated area

    mov rdi, 0x8
    mov rsi, rbp
    call calloc wrt ..plt
    mov [rel left_vals], rax

    mov rdi, 0x8
    mov rsi, rbp
    call calloc wrt ..plt
    mov [rel right_vals], rax

    mov r12, [rel left_vals]    ; left_vals*
    mov r13, [rel right_vals]   ; right_vals*

    ; open input file
    lea rdi, [rel input_file]
    lea rsi, [rel input_file_mode]
    call fopen wrt ..plt

     ; check if the file existed
    test rax, rax
    jne .Lfile_exists

    lea rdi, [rel no_file]
    call puts wrt ..plt
    jmp .Lexit

    .Lfile_exists:

    mov r14, rax ; FILE*

    xor rbx, rbx ; will contain our current index in left_vals/right_vals

    .Lgetline_loop:
        ; get line from input file
        lea rdi, [rel getline_lineptr]
        lea rsi, [rel getline_n]
        mov rdx, r14
        call getline wrt ..plt

        cmp rax, -1
        je .Lgetline_loop_end

        mov rdi, [rel getline_lineptr]
        call trim
        mov r15, rax

        ; check for empty line
        mov dil, byte [r15]
        test dil, dil
        je .Lgetline_loop

        ; TODO: neither strtok nor strtoll errors are checked here

        ; == get left number
        mov rdi, r15
        lea rsi, [rel delim]
        call strtok wrt ..plt

        ; rax now contains a pointer to the line's left value (char*)

        ; convert to number
        mov rdi, rax
        mov rsi, 0
        mov rdx, 10
        call strtoll wrt ..plt
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
            mov r12, rax

            mov rdi, r13
            mov rsi, 8
            mov rdx, rbp
            call reallocarray wrt ..plt
            mov r13, rax

            pop rax
        .Lend_resize:

        mov [r12 + rbx*8], rax


        ; == get right number
        mov rdi, 0
        lea rsi, [rel delim]
        call strtok wrt ..plt

        ; rax now contains a pointer to the line's right value (char*)

        ; convert to number
        mov rdi, rax
        mov rsi, 0
        mov rdx, 10
        call strtoll wrt ..plt
        ; rax now contains line's right value (i64)

        ; update right_vals
        mov [r13 + rbx*8], rax


        ; no need to free getline_lineptr
        ; because it gets reused in the next run of the loop

        inc rbx
        jmp .Lgetline_loop

    .Lgetline_loop_end:
    mov rdi, [rel getline_lineptr]
    call free wrt ..plt

    ; left_vals and right_vals now contain the appropriate values
    ; and rbx has their size

    ; first sort them
    mov rdi, r12
    mov rsi, rbx
    call sort

    mov rdi, r13
    mov rsi, rbx
    call sort

    ; print out left values
    ; replace r12 with r13 to print out right values
    ; xor eax, eax
    ; jmp .Lprintloop_begin
    ; .Lprintloop:
    ;     lea rdi, [rel pf2]
    ;     mov rsi, [r12 + rax*8]
    ;
    ;     push rax
    ;     sub rsp, 8
    ;     call printf wrt ..plt
    ;     add rsp, 0x8
    ;     pop rax
    ;
    ;     inc rax
    ; .Lprintloop_begin:
    ;     cmp rbx, rax
    ;     jg .Lprintloop

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
    mov rdi, r12
    call free wrt ..plt
    mov rdi, r13
    call free wrt ..plt

    .Lexit:
    mov rax, 60
    mov rdi, 0
    syscall
