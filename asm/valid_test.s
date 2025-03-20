.global _main
.align 2

// Data section
.data
hello_str:
    .ascii "Hello, World!\n"

hello_len = . - hello_str

#define henk mov x29, sp

.text
_main:
    // Setup stack frame
    stp     x29, x30, [sp, #-16]!
    ; mov     x29, sp
    henk

    // write(1, hello_str, hello_len)
    mov     x0, #1          // file descriptor 1 is stdout
    adrp    x1, hello_str@PAGE
    add     x1, x1, hello_str@PAGEOFF
    mov     x2, #hello_len  // length of the string
    mov     x16, #4         // macOS write syscall number
    svc     #0x80           // invoke syscall

   /// Test codes
    ucvtf d1, x0

    // Return 0
    mov     x0, #0
    ldp     x29, x30, [sp], #16
    ret
 

