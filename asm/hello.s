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



; alloc and free, and put the freed register in the back of the list.
; how to track numbers that aren't free
; keep in mind that not all registers are free to use

; tricky part is too keep enough registers free
; write a test function to see if the NEON (q,v) registers and x register overlap. They shouldn't
; there are registers internally in use, but those can't be used by anything else. So just a simple macro approach is going to get stuck quickly 

; adds and cinc need to form a block those can't be interleaved by other adds instructions without having to spillover. 
; where spill over can also be spilling over to a register. I.e. flag -> register -> memory

; write test that this actually works

; writing it this way probably lower the pressure on the flag

; allocate the array in assembly contiguously to make it easier to understand which are temps and which are not?

; register renaming is something to take account of
; an optimizer can move things around as long as it doesn't change the order of instructions that mention a certain register.

; multiprecision multiplication:
; (x0 x1 x2 x3) x x4 -> x9 x10 x12 x14 x16
; x11 x13 x15 are temps
; but it could have done with a single temp register, after freeing x11 it could be used instead of x13 etc
; -> using less temp registers does make it harder for a later optimiser. 
; optimisation has to be done on the register like structure
; don't want to go full ssa to still have some control over how things are collected
; look into write after read for a register

