# x86 Assembly

.global _start
.intel_syntax
.section .text

_start:
    #wirte syscall
    mov %eax, 4
    mov %ebx, 1 #STDOUT, where we are writing
    lea %ecx, [message]
    mov %edx, 13
    int 0x80

    # exit syscall
    mov %eax, 1
    mov %ebx, 65
    int 0x80

# write a string to stdout

.section .data
message:
    .ascii "Hello, World\n"
