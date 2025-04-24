//in("x0") in0[0],
//in("x1") in1[0],
//lateout("x0") out0[0],
//lateout("x1") _, lateout("x2") _, lateout("x3") _, lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _, lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _, lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
//lateout("lr") _
        .global _single_step_load
.align 4
.text
_single_step_load:
  ldp x2, x3, [x0, #0]
  ldp x4, x5, [x0, #16]
  ldp x6, x7, [x1, #0]
  ldp x1, x8, [x1, #16]
  mul x9, x2, x6
  umulh x10, x2, x6
  mul x11, x3, x6
  umulh x12, x3, x6
  adds x10, x11, x10
  cinc x11, x12, hs
  mul x12, x4, x6
  umulh x13, x4, x6
  adds x11, x12, x11
  cinc x12, x13, hs
  mul x13, x5, x6
  umulh x6, x5, x6
  adds x12, x13, x12
  cinc x6, x6, hs
  mul x13, x2, x7
  umulh x14, x2, x7
  adds x10, x13, x10
  cinc x13, x14, hs
  mul x14, x3, x7
  umulh x15, x3, x7
  adds x13, x14, x13
  cinc x14, x15, hs
  adds x11, x13, x11
  cinc x13, x14, hs
  mul x14, x4, x7
  umulh x15, x4, x7
  adds x13, x14, x13
  cinc x14, x15, hs
  adds x12, x13, x12
  cinc x13, x14, hs
  mul x14, x5, x7
  umulh x7, x5, x7
  adds x13, x14, x13
  cinc x7, x7, hs
  adds x6, x13, x6
  cinc x7, x7, hs
  mul x13, x2, x1
  umulh x14, x2, x1
  adds x11, x13, x11
  cinc x13, x14, hs
  mul x14, x3, x1
  umulh x15, x3, x1
  adds x13, x14, x13
  cinc x14, x15, hs
  adds x12, x13, x12
  cinc x13, x14, hs
  mul x14, x4, x1
  umulh x15, x4, x1
  adds x13, x14, x13
  cinc x14, x15, hs
  adds x6, x13, x6
  cinc x13, x14, hs
  mul x14, x5, x1
  umulh x1, x5, x1
  adds x13, x14, x13
  cinc x1, x1, hs
  adds x7, x13, x7
  cinc x1, x1, hs
  mul x13, x2, x8
  umulh x2, x2, x8
  adds x12, x13, x12
  cinc x2, x2, hs
  mul x13, x3, x8
  umulh x3, x3, x8
  adds x2, x13, x2
  cinc x3, x3, hs
  adds x2, x2, x6
  cinc x3, x3, hs
  mul x6, x4, x8
  umulh x4, x4, x8
  adds x3, x6, x3
  cinc x4, x4, hs
  adds x3, x3, x7
  cinc x4, x4, hs
  mul x6, x5, x8
  umulh x5, x5, x8
  adds x4, x6, x4
  cinc x5, x5, hs
  adds x1, x4, x1
  cinc x4, x5, hs
  mov x5, #48718
  movk x5, #4732, lsl 16
  movk x5, #45078, lsl 32
  movk x5, #39852, lsl 48
  mov x6, #16676
  movk x6, #12692, lsl 16
  movk x6, #20986, lsl 32
  movk x6, #2848, lsl 48
  mov x7, #51052
  movk x7, #24721, lsl 16
  movk x7, #61092, lsl 32
  movk x7, #45156, lsl 48
  mov x8, #3197
  movk x8, #18936, lsl 16
  movk x8, #10922, lsl 32
  movk x8, #11014, lsl 48
  mul x13, x5, x9
  umulh x5, x5, x9
  adds x12, x13, x12
  cinc x5, x5, hs
  mul x13, x6, x9
  umulh x6, x6, x9
  adds x5, x13, x5
  cinc x6, x6, hs
  adds x2, x5, x2
  cinc x5, x6, hs
  mul x6, x7, x9
  umulh x7, x7, x9
  adds x5, x6, x5
  cinc x6, x7, hs
  adds x3, x5, x3
  cinc x5, x6, hs
  mul x6, x8, x9
  umulh x7, x8, x9
  adds x5, x6, x5
  cinc x6, x7, hs
  adds x1, x5, x1
  cinc x5, x6, hs
  add x4, x4, x5
  mov x5, #56431
  movk x5, #30457, lsl 16
  movk x5, #30012, lsl 32
  movk x5, #6382, lsl 48
  mov x6, #59151
  movk x6, #41769, lsl 16
  movk x6, #32276, lsl 32
  movk x6, #21677, lsl 48
  mov x7, #34015
  movk x7, #20342, lsl 16
  movk x7, #13935, lsl 32
  movk x7, #11030, lsl 48
  mov x8, #13689
  movk x8, #8159, lsl 16
  movk x8, #215, lsl 32
  movk x8, #4913, lsl 48
  mul x9, x5, x10
  umulh x5, x5, x10
  adds x9, x9, x12
  cinc x5, x5, hs
  mul x12, x6, x10
  umulh x6, x6, x10
  adds x5, x12, x5
  cinc x6, x6, hs
  adds x2, x5, x2
  cinc x5, x6, hs
  mul x6, x7, x10
  umulh x7, x7, x10
  adds x5, x6, x5
  cinc x6, x7, hs
  adds x3, x5, x3
  cinc x5, x6, hs
  mul x6, x8, x10
  umulh x7, x8, x10
  adds x5, x6, x5
  cinc x6, x7, hs
  adds x1, x5, x1
  cinc x5, x6, hs
  add x4, x4, x5
  mov x5, #61005
  movk x5, #58262, lsl 16
  movk x5, #32851, lsl 32
  movk x5, #11582, lsl 48
  mov x6, #37581
  movk x6, #43836, lsl 16
  movk x6, #36286, lsl 32
  movk x6, #51783, lsl 48
  mov x7, #10899
  movk x7, #30709, lsl 16
  movk x7, #61551, lsl 32
  movk x7, #45784, lsl 48
  mov x8, #36612
  movk x8, #63402, lsl 16
  movk x8, #47623, lsl 32
  movk x8, #9430, lsl 48
  mul x10, x5, x11
  umulh x5, x5, x11
  adds x9, x10, x9
  cinc x5, x5, hs
  mul x10, x6, x11
  umulh x6, x6, x11
  adds x5, x10, x5
  cinc x6, x6, hs
  adds x2, x5, x2
  cinc x5, x6, hs
  mul x6, x7, x11
  umulh x7, x7, x11
  adds x5, x6, x5
  cinc x6, x7, hs
  adds x3, x5, x3
  cinc x5, x6, hs
  mul x6, x8, x11
  umulh x7, x8, x11
  adds x5, x6, x5
  cinc x6, x7, hs
  adds x1, x5, x1
  cinc x5, x6, hs
  add x4, x4, x5
  mov x5, #65535
  movk x5, #61439, lsl 16
  movk x5, #62867, lsl 32
  movk x5, #49889, lsl 48
  mul x5, x5, x9
  mov x6, #1
  movk x6, #61440, lsl 16
  movk x6, #62867, lsl 32
  movk x6, #17377, lsl 48
  mov x7, #28817
  movk x7, #31161, lsl 16
  movk x7, #59464, lsl 32
  movk x7, #10291, lsl 48
  mov x8, #22621
  movk x8, #33153, lsl 16
  movk x8, #17846, lsl 32
  movk x8, #47184, lsl 48
  mov x10, #41001
  movk x10, #57649, lsl 16
  movk x10, #20082, lsl 32
  movk x10, #12388, lsl 48
  mul x11, x6, x5
  umulh x6, x6, x5
  cmn x11, x9
  cinc x6, x6, hs
  mul x9, x7, x5
  umulh x7, x7, x5
  adds x6, x9, x6
  cinc x7, x7, hs
  adds x2, x6, x2
  cinc x6, x7, hs
  mul x7, x8, x5
  umulh x8, x8, x5
  adds x6, x7, x6
  cinc x7, x8, hs
  adds x3, x6, x3
  cinc x6, x7, hs
  mul x7, x10, x5
  umulh x5, x10, x5
  adds x6, x7, x6
  cinc x5, x5, hs
  adds x1, x6, x1
  cinc x5, x5, hs
  add x4, x4, x5
  mov x5, #2
  movk x5, #57344, lsl 16
  movk x5, #60199, lsl 32
  movk x5, #34755, lsl 48
  mov x6, #57634
  movk x6, #62322, lsl 16
  movk x6, #53392, lsl 32
  movk x6, #20583, lsl 48
  mov x7, #45242
  movk x7, #770, lsl 16
  movk x7, #35693, lsl 32
  movk x7, #28832, lsl 48
  mov x8, #16467
  movk x8, #49763, lsl 16
  movk x8, #40165, lsl 32
  movk x8, #24776, lsl 48
  subs x5, x2, x5
  sbcs x6, x3, x6
  sbcs x7, x1, x7
  sbcs x8, x4, x8
  tst x4, #9223372036854775808
  csel x2, x5, x2, mi
  csel x3, x6, x3, mi
  csel x1, x7, x1, mi
  csel x4, x8, x4, mi
  stp x2, x3, [x0, #0]
  stp x1, x4, [x0, #16]
  ret