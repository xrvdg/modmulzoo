//in("[x0, #0]") in0[0], in("[x1, #0]") in1[0],
//lateout("x0") out0[0], lateout("x1") out0[1], lateout("x2") out0[2], lateout("x3") out0[3],
//lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _, lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _, lateout("x12") _, lateout("x13") _, lateout("x14") _,
//lateout("lr") _
.global _single_step_load
.align 4
.text
_single_step_load:
  ldr x2, [x0, #0]
  ldr x3, [x1, #0]
  mul x4, x2, x3
  umulh x5, x2, x3
  ldr x6, [x0, #8]
  mul x7, x6, x3
  umulh x8, x6, x3
  adds x5, x7, x5
  cinc x7, x8, hs
  ldr x8, [x0, #16]
  mul x9, x8, x3
  umulh x10, x8, x3
  adds x7, x9, x7
  cinc x9, x10, hs
  ldr x0, [x0, #24]
  mul x10, x0, x3
  umulh x3, x0, x3
  adds x9, x10, x9
  cinc x3, x3, hs
  ldr x10, [x1, #8]
  mul x11, x2, x10
  umulh x12, x2, x10
  adds x5, x11, x5
  cinc x11, x12, hs
  mul x12, x6, x10
  umulh x13, x6, x10
  adds x11, x12, x11
  cinc x12, x13, hs
  adds x7, x11, x7
  cinc x11, x12, hs
  mul x12, x8, x10
  umulh x13, x8, x10
  adds x11, x12, x11
  cinc x12, x13, hs
  adds x9, x11, x9
  cinc x11, x12, hs
  mul x12, x0, x10
  umulh x10, x0, x10
  adds x11, x12, x11
  cinc x10, x10, hs
  adds x3, x11, x3
  cinc x10, x10, hs
  ldr x11, [x1, #16]
  mul x12, x2, x11
  umulh x13, x2, x11
  adds x7, x12, x7
  cinc x12, x13, hs
  mul x13, x6, x11
  umulh x14, x6, x11
  adds x12, x13, x12
  cinc x13, x14, hs
  adds x9, x12, x9
  cinc x12, x13, hs
  mul x13, x8, x11
  umulh x14, x8, x11
  adds x12, x13, x12
  cinc x13, x14, hs
  adds x3, x12, x3
  cinc x12, x13, hs
  mul x13, x0, x11
  umulh x11, x0, x11
  adds x12, x13, x12
  cinc x11, x11, hs
  adds x10, x12, x10
  cinc x11, x11, hs
  ldr x1, [x1, #24]
  mul x12, x2, x1
  umulh x2, x2, x1
  adds x9, x12, x9
  cinc x2, x2, hs
  mul x12, x6, x1
  umulh x6, x6, x1
  adds x2, x12, x2
  cinc x6, x6, hs
  adds x2, x2, x3
  cinc x3, x6, hs
  mul x6, x8, x1
  umulh x8, x8, x1
  adds x3, x6, x3
  cinc x6, x8, hs
  adds x3, x3, x10
  cinc x6, x6, hs
  mul x8, x0, x1
  umulh x0, x0, x1
  adds x1, x8, x6
  cinc x0, x0, hs
  adds x1, x1, x11
  cinc x0, x0, hs
  mov x6, #48718
  movk x6, #4732, lsl 16
  movk x6, #45078, lsl 32
  movk x6, #39852, lsl 48
  mov x8, #16676
  movk x8, #12692, lsl 16
  movk x8, #20986, lsl 32
  movk x8, #2848, lsl 48
  mov x10, #51052
  movk x10, #24721, lsl 16
  movk x10, #61092, lsl 32
  movk x10, #45156, lsl 48
  mov x11, #3197
  movk x11, #18936, lsl 16
  movk x11, #10922, lsl 32
  movk x11, #11014, lsl 48
  mul x12, x6, x4
  umulh x6, x6, x4
  adds x9, x12, x9
  cinc x6, x6, hs
  mul x12, x8, x4
  umulh x8, x8, x4
  adds x6, x12, x6
  cinc x8, x8, hs
  adds x2, x6, x2
  cinc x6, x8, hs
  mul x8, x10, x4
  umulh x10, x10, x4
  adds x6, x8, x6
  cinc x8, x10, hs
  adds x3, x6, x3
  cinc x6, x8, hs
  mul x8, x11, x4
  umulh x4, x11, x4
  adds x6, x8, x6
  cinc x4, x4, hs
  adds x1, x6, x1
  cinc x4, x4, hs
  add x0, x0, x4
  mov x4, #56431
  movk x4, #30457, lsl 16
  movk x4, #30012, lsl 32
  movk x4, #6382, lsl 48
  mov x6, #59151
  movk x6, #41769, lsl 16
  movk x6, #32276, lsl 32
  movk x6, #21677, lsl 48
  mov x8, #34015
  movk x8, #20342, lsl 16
  movk x8, #13935, lsl 32
  movk x8, #11030, lsl 48
  mov x10, #13689
  movk x10, #8159, lsl 16
  movk x10, #215, lsl 32
  movk x10, #4913, lsl 48
  mul x11, x4, x5
  umulh x4, x4, x5
  adds x9, x11, x9
  cinc x4, x4, hs
  mul x11, x6, x5
  umulh x6, x6, x5
  adds x4, x11, x4
  cinc x6, x6, hs
  adds x2, x4, x2
  cinc x4, x6, hs
  mul x6, x8, x5
  umulh x8, x8, x5
  adds x4, x6, x4
  cinc x6, x8, hs
  adds x3, x4, x3
  cinc x4, x6, hs
  mul x6, x10, x5
  umulh x5, x10, x5
  adds x4, x6, x4
  cinc x5, x5, hs
  adds x1, x4, x1
  cinc x4, x5, hs
  add x0, x0, x4
  mov x4, #61005
  movk x4, #58262, lsl 16
  movk x4, #32851, lsl 32
  movk x4, #11582, lsl 48
  mov x5, #37581
  movk x5, #43836, lsl 16
  movk x5, #36286, lsl 32
  movk x5, #51783, lsl 48
  mov x6, #10899
  movk x6, #30709, lsl 16
  movk x6, #61551, lsl 32
  movk x6, #45784, lsl 48
  mov x8, #36612
  movk x8, #63402, lsl 16
  movk x8, #47623, lsl 32
  movk x8, #9430, lsl 48
  mul x10, x4, x7
  umulh x4, x4, x7
  adds x9, x10, x9
  cinc x4, x4, hs
  mul x10, x5, x7
  umulh x5, x5, x7
  adds x4, x10, x4
  cinc x5, x5, hs
  adds x2, x4, x2
  cinc x4, x5, hs
  mul x5, x6, x7
  umulh x6, x6, x7
  adds x4, x5, x4
  cinc x5, x6, hs
  adds x3, x4, x3
  cinc x4, x5, hs
  mul x5, x8, x7
  umulh x6, x8, x7
  adds x4, x5, x4
  cinc x5, x6, hs
  adds x1, x4, x1
  cinc x4, x5, hs
  add x0, x0, x4
  mov x4, #65535
  movk x4, #61439, lsl 16
  movk x4, #62867, lsl 32
  movk x4, #49889, lsl 48
  mul x4, x4, x9
  mov x5, #1
  movk x5, #61440, lsl 16
  movk x5, #62867, lsl 32
  movk x5, #17377, lsl 48
  mov x6, #28817
  movk x6, #31161, lsl 16
  movk x6, #59464, lsl 32
  movk x6, #10291, lsl 48
  mov x7, #22621
  movk x7, #33153, lsl 16
  movk x7, #17846, lsl 32
  movk x7, #47184, lsl 48
  mov x8, #41001
  movk x8, #57649, lsl 16
  movk x8, #20082, lsl 32
  movk x8, #12388, lsl 48
  mul x10, x5, x4
  umulh x5, x5, x4
  cmn x10, x9
  cinc x5, x5, hs
  mul x9, x6, x4
  umulh x6, x6, x4
  adds x5, x9, x5
  cinc x6, x6, hs
  adds x2, x5, x2
  cinc x5, x6, hs
  mul x6, x7, x4
  umulh x7, x7, x4
  adds x5, x6, x5
  cinc x6, x7, hs
  adds x3, x5, x3
  cinc x5, x6, hs
  mul x6, x8, x4
  umulh x4, x8, x4
  adds x5, x6, x5
  cinc x4, x4, hs
  adds x5, x5, x1
  cinc x1, x4, hs
  add x4, x0, x1
  mov x0, #2
  movk x0, #57344, lsl 16
  movk x0, #60199, lsl 32
  movk x0, #34755, lsl 48
  mov x1, #57634
  movk x1, #62322, lsl 16
  movk x1, #53392, lsl 32
  movk x1, #20583, lsl 48
  mov x6, #45242
  movk x6, #770, lsl 16
  movk x6, #35693, lsl 32
  movk x6, #28832, lsl 48
  mov x7, #16467
  movk x7, #49763, lsl 16
  movk x7, #40165, lsl 32
  movk x7, #24776, lsl 48
  subs x0, x2, x0
  sbcs x1, x3, x1
  sbcs x6, x5, x6
  sbcs x7, x4, x7
  tst x4, #9223372036854775808
  csel x0, x0, x2, mi
  csel x1, x1, x3, mi
  csel x2, x6, x5, mi
  csel x3, x7, x4, mi
ret
