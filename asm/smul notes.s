; smult notes
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

    // Return 0
    mov     x0, #0
    ldp     x29, x30, [sp], #16
    ret

 smult:
  ; all the register numbers can be reduced by one
  ; x4 round
  mul x9, x0, x4
  umulh x10, x0, x4
  ; Writing this way the umulh goes together with the mul of the next
  ; can't use multiply accumulate as there is no madds instruction
  mul x11, x1, x4
  umulh x12, x1, x4 
  adds x10, x10, x11
  ; can adds be squeezed in between or does umulh affect the flags? umulh doens't affect flags but shouldn't make a difference
  ; x11 is free
  cinc x12, hs
    
  mul x13, x2, x4
  umulh x14, x2, x4
  
  adds x12, x12, x13
  ; x13 is free
  cinc x14, hs

  mul x15, x3, x4
  umulh x16, x3, x4
  
  adds x14, x14, x15
  ; x15 is free
  cinc x16, hs

; adcs is an option

smult_adcs:
  ; all the register numbers can be reduced by one
  mul x9, x0, x4
  umulh x10, x0, x4
  
  mul x11, x1, x4
  umulh x12, x1, x4 
  adds x10, x10, x11
  ; can adds be squeezed in between or does umulh affect the flags? umulh doens't affect flags but shouldn't make a difference
  ; x11 is free
    
  mul x13, x2, x4
  umulh x14, x2, x4 
  adcs x12, x12, x13
  ; x13 is free

  mul x15, x3, x4
  umulh x16, x3, x4 
  adcs x14, x14, x15
  ; x15 is free
  cinc x16, hs

 smult_adcs_less_reg:
  ; all the register numbers can be reduced by one
  mul x5, x0, x4
  umulh x6, x0, x4
  ; x0 free
  
  mul x10, x1, x4
  umulh x7, x1, x4 
  adds x6, x6, x10
  ; can adds be squeezed in between or does umulh affect the flags? umulh doens't affect flags but shouldn't make a difference
  ; x1 + x10 is free
    
  mul x10, x2, x4
  umulh x8, x2, x4 
  adcs x7, x7, x10
  ; x2 + x10 is free

  mul x10, x3, x4
  umulh x9, x3, x4 
  adcs x8, x8, x10
  cinc x9, hs
  ; x3 + x10 is free

; adcs is an option

; movs that you will run into, but those should be relatively cheap
; call convention to hang on to