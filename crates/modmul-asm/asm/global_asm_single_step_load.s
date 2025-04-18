//in("[x0, #0]") in0[0], in("x1") in1[0], in("x2") in1[1], in("x3") in1[2], in("x4") in1[3],
//lateout("x0") out0[0], lateout("x1") out0[1], lateout("x2") out0[2], lateout("x3") out0[3],
//lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _, lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _, lateout("x12") _, lateout("x13") _, lateout("x14") _,
//lateout("lr") _
.global _single_step_load
.align 4
.text
_single_step_load:
  ldr x5, [x0, #0]
  mul x6, x5, x1
  umulh x7, x5, x1
  ldr x8, [x0, #8]
  mul x9, x8, x1
  umulh x10, x8, x1
  adds x7, x9, x7
  cinc x9, x10, hs
  ldr x10, [x0, #16]
  mul x11, x10, x1
  umulh x12, x10, x1
  adds x9, x11, x9
  cinc x11, x12, hs
  ldr x0, [x0, #24]
  mul x12, x0, x1
  umulh x1, x0, x1
  adds x11, x12, x11
  cinc x1, x1, hs
  mul x12, x5, x2
  umulh x13, x5, x2
  adds x7, x12, x7
  cinc x12, x13, hs
  mul x13, x8, x2
  umulh x14, x8, x2
  adds x12, x13, x12
  cinc x13, x14, hs
  adds x9, x12, x9
  cinc x12, x13, hs
  mul x13, x10, x2
  umulh x14, x10, x2
  adds x12, x13, x12
  cinc x13, x14, hs
  adds x11, x12, x11
  cinc x12, x13, hs
  mul x13, x0, x2
  umulh x2, x0, x2
  adds x12, x13, x12
  cinc x2, x2, hs
  adds x1, x12, x1
  cinc x2, x2, hs
  mul x12, x5, x3
  umulh x13, x5, x3
  adds x9, x12, x9
  cinc x12, x13, hs
  mul x13, x8, x3
  umulh x14, x8, x3
  adds x12, x13, x12
  cinc x13, x14, hs
  adds x11, x12, x11
  cinc x12, x13, hs
  mul x13, x10, x3
  umulh x14, x10, x3
  adds x12, x13, x12
  cinc x13, x14, hs
  adds x1, x12, x1
  cinc x12, x13, hs
  mul x13, x0, x3
  umulh x3, x0, x3
  adds x12, x13, x12
  cinc x3, x3, hs
  adds x2, x12, x2
  cinc x3, x3, hs
  mul x12, x5, x4
  umulh x5, x5, x4
  adds x11, x12, x11
  cinc x5, x5, hs
  mul x12, x8, x4
  umulh x8, x8, x4
  adds x5, x12, x5
  cinc x8, x8, hs
  adds x1, x5, x1
  cinc x5, x8, hs
  mul x8, x10, x4
  umulh x10, x10, x4
  adds x5, x8, x5
  cinc x8, x10, hs
  adds x2, x5, x2
  cinc x5, x8, hs
  mul x8, x0, x4
  umulh x0, x0, x4
  adds x4, x8, x5
  cinc x0, x0, hs
  adds x3, x4, x3
  cinc x0, x0, hs
  mov x4, #48718
  movk x4, #4732, lsl 16
  movk x4, #45078, lsl 32
  movk x4, #39852, lsl 48
  mov x5, #16676
  movk x5, #12692, lsl 16
  movk x5, #20986, lsl 32
  movk x5, #2848, lsl 48
  mov x8, #51052
  movk x8, #24721, lsl 16
  movk x8, #61092, lsl 32
  movk x8, #45156, lsl 48
  mov x10, #3197
  movk x10, #18936, lsl 16
  movk x10, #10922, lsl 32
  movk x10, #11014, lsl 48
  mul x12, x4, x6
  umulh x4, x4, x6
  adds x11, x12, x11
  cinc x4, x4, hs
  mul x12, x5, x6
  umulh x5, x5, x6
  adds x4, x12, x4
  cinc x5, x5, hs
  adds x1, x4, x1
  cinc x4, x5, hs
  mul x5, x8, x6
  umulh x8, x8, x6
  adds x4, x5, x4
  cinc x5, x8, hs
  adds x2, x4, x2
  cinc x4, x5, hs
  mul x5, x10, x6
  umulh x6, x10, x6
  adds x4, x5, x4
  cinc x5, x6, hs
  adds x3, x4, x3
  cinc x4, x5, hs
  add x0, x0, x4
  mov x4, #56431
  movk x4, #30457, lsl 16
  movk x4, #30012, lsl 32
  movk x4, #6382, lsl 48
  mov x5, #59151
  movk x5, #41769, lsl 16
  movk x5, #32276, lsl 32
  movk x5, #21677, lsl 48
  mov x6, #34015
  movk x6, #20342, lsl 16
  movk x6, #13935, lsl 32
  movk x6, #11030, lsl 48
  mov x8, #13689
  movk x8, #8159, lsl 16
  movk x8, #215, lsl 32
  movk x8, #4913, lsl 48
  mul x10, x4, x7
  umulh x4, x4, x7
  adds x10, x10, x11
  cinc x4, x4, hs
  mul x11, x5, x7
  umulh x5, x5, x7
  adds x4, x11, x4
  cinc x5, x5, hs
  adds x1, x4, x1
  cinc x4, x5, hs
  mul x5, x6, x7
  umulh x6, x6, x7
  adds x4, x5, x4
  cinc x5, x6, hs
  adds x2, x4, x2
  cinc x4, x5, hs
  mul x5, x8, x7
  umulh x6, x8, x7
  adds x4, x5, x4
  cinc x5, x6, hs
  adds x3, x4, x3
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
  mov x7, #36612
  movk x7, #63402, lsl 16
  movk x7, #47623, lsl 32
  movk x7, #9430, lsl 48
  mul x8, x4, x9
  umulh x4, x4, x9
  adds x8, x8, x10
  cinc x4, x4, hs
  mul x10, x5, x9
  umulh x5, x5, x9
  adds x4, x10, x4
  cinc x5, x5, hs
  adds x1, x4, x1
  cinc x4, x5, hs
  mul x5, x6, x9
  umulh x6, x6, x9
  adds x4, x5, x4
  cinc x5, x6, hs
  adds x2, x4, x2
  cinc x4, x5, hs
  mul x5, x7, x9
  umulh x6, x7, x9
  adds x4, x5, x4
  cinc x5, x6, hs
  adds x3, x4, x3
  cinc x4, x5, hs
  add x0, x0, x4
  mov x4, #65535
  movk x4, #61439, lsl 16
  movk x4, #62867, lsl 32
  movk x4, #49889, lsl 48
  mul x4, x4, x8
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
  mov x9, #41001
  movk x9, #57649, lsl 16
  movk x9, #20082, lsl 32
  movk x9, #12388, lsl 48
  mul x10, x5, x4
  umulh x5, x5, x4
  cmn x10, x8
  cinc x5, x5, hs
  mul x8, x6, x4
  umulh x6, x6, x4
  adds x5, x8, x5
  cinc x6, x6, hs
  adds x1, x5, x1
  cinc x5, x6, hs
  mul x6, x7, x4
  umulh x7, x7, x4
  adds x5, x6, x5
  cinc x6, x7, hs
  adds x2, x5, x2
  cinc x5, x6, hs
  mul x6, x9, x4
  umulh x4, x9, x4
  adds x5, x6, x5
  cinc x4, x4, hs
  adds x3, x5, x3
  cinc x4, x4, hs
  add x4, x0, x4
  mov x0, #2
  movk x0, #57344, lsl 16
  movk x0, #60199, lsl 32
  movk x0, #34755, lsl 48
  mov x5, #57634
  movk x5, #62322, lsl 16
  movk x5, #53392, lsl 32
  movk x5, #20583, lsl 48
  mov x6, #45242
  movk x6, #770, lsl 16
  movk x6, #35693, lsl 32
  movk x6, #28832, lsl 48
  mov x7, #16467
  movk x7, #49763, lsl 16
  movk x7, #40165, lsl 32
  movk x7, #24776, lsl 48
  subs x0, x1, x0
  sbcs x5, x2, x5
  sbcs x6, x3, x6
  sbcs x7, x4, x7
  tst x4, #9223372036854775808
  csel x0, x0, x1, mi
  csel x1, x5, x2, mi
  csel x2, x6, x3, mi
  csel x3, x7, x4, mi
ret
