//in("x0") in0[0], in("x1") in1[0], in("x2") in2[0], in("x3") in3[0],
//lateout("x4") out0[0], lateout("x5") out0[1], lateout("x0") out0[2], lateout("x1") out0[3], lateout("x6") out2[0], lateout("x7") out2[1], lateout("x2") out2[2], lateout("x3") out2[3],
//lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _, lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _, lateout("x16") _, lateout("x17") _, lateout("x20") _,
//lateout("lr") _
.global _single_step_interleaved_triple_scalar
.align 4
.text
_single_step_interleaved_triple_scalar:
  ldp x4, x5, [x0, #0]
  ldp x0, x6, [x0, #16]
  ldp x7, x8, [x1, #0]
  ldp x1, x9, [x1, #16]
  mul x10, x4, x7
  umulh x11, x4, x7
  mul x12, x5, x7
  umulh x13, x5, x7
  adds x11, x12, x11
  cinc x12, x13, hs
  mul x13, x0, x7
  umulh x14, x0, x7
  adds x12, x13, x12
  cinc x13, x14, hs
  mul x14, x6, x7
  umulh x7, x6, x7
  adds x13, x14, x13
  cinc x7, x7, hs
  mul x14, x4, x8
  umulh x15, x4, x8
  adds x11, x14, x11
  cinc x14, x15, hs
  mul x15, x5, x8
  umulh x16, x5, x8
  adds x14, x15, x14
  cinc x15, x16, hs
  adds x12, x14, x12
  cinc x14, x15, hs
  mul x15, x0, x8
  umulh x16, x0, x8
  adds x14, x15, x14
  cinc x15, x16, hs
  adds x13, x14, x13
  cinc x14, x15, hs
  mul x15, x6, x8
  umulh x8, x6, x8
  adds x14, x15, x14
  cinc x8, x8, hs
  adds x7, x14, x7
  cinc x8, x8, hs
  mul x14, x4, x1
  umulh x15, x4, x1
  adds x12, x14, x12
  cinc x14, x15, hs
  mul x15, x5, x1
  umulh x16, x5, x1
  adds x14, x15, x14
  cinc x15, x16, hs
  adds x13, x14, x13
  cinc x14, x15, hs
  mul x15, x0, x1
  umulh x16, x0, x1
  adds x14, x15, x14
  cinc x15, x16, hs
  adds x7, x14, x7
  cinc x14, x15, hs
  mul x15, x6, x1
  umulh x1, x6, x1
  adds x14, x15, x14
  cinc x1, x1, hs
  adds x8, x14, x8
  cinc x1, x1, hs
  mul x14, x4, x9
  umulh x4, x4, x9
  adds x13, x14, x13
  cinc x4, x4, hs
  mul x14, x5, x9
  umulh x5, x5, x9
  adds x4, x14, x4
  cinc x5, x5, hs
  adds x4, x4, x7
  cinc x5, x5, hs
  mul x7, x0, x9
  umulh x0, x0, x9
  adds x5, x7, x5
  cinc x0, x0, hs
  adds x5, x5, x8
  cinc x0, x0, hs
  mul x7, x6, x9
  umulh x6, x6, x9
  adds x0, x7, x0
  cinc x6, x6, hs
  adds x0, x0, x1
  cinc x1, x6, hs
  mov x6, #48718
  movk x6, #4732, lsl 16
  movk x6, #45078, lsl 32
  movk x6, #39852, lsl 48
  mov x7, #16676
  movk x7, #12692, lsl 16
  movk x7, #20986, lsl 32
  movk x7, #2848, lsl 48
  mov x8, #51052
  movk x8, #24721, lsl 16
  movk x8, #61092, lsl 32
  movk x8, #45156, lsl 48
  mov x9, #3197
  movk x9, #18936, lsl 16
  movk x9, #10922, lsl 32
  movk x9, #11014, lsl 48
  mul x14, x6, x10
  umulh x6, x6, x10
  adds x13, x14, x13
  cinc x6, x6, hs
  mul x14, x7, x10
  umulh x7, x7, x10
  adds x6, x14, x6
  cinc x7, x7, hs
  adds x4, x6, x4
  cinc x6, x7, hs
  mul x7, x8, x10
  umulh x8, x8, x10
  adds x6, x7, x6
  cinc x7, x8, hs
  adds x5, x6, x5
  cinc x6, x7, hs
  mul x7, x9, x10
  umulh x8, x9, x10
  adds x6, x7, x6
  cinc x7, x8, hs
  adds x0, x6, x0
  cinc x6, x7, hs
  add x1, x1, x6
  mov x6, #56431
  movk x6, #30457, lsl 16
  movk x6, #30012, lsl 32
  movk x6, #6382, lsl 48
  mov x7, #59151
  movk x7, #41769, lsl 16
  movk x7, #32276, lsl 32
  movk x7, #21677, lsl 48
  mov x8, #34015
  movk x8, #20342, lsl 16
  movk x8, #13935, lsl 32
  movk x8, #11030, lsl 48
  mov x9, #13689
  movk x9, #8159, lsl 16
  movk x9, #215, lsl 32
  movk x9, #4913, lsl 48
  mul x10, x6, x11
  umulh x6, x6, x11
  adds x10, x10, x13
  cinc x6, x6, hs
  mul x13, x7, x11
  umulh x7, x7, x11
  adds x6, x13, x6
  cinc x7, x7, hs
  adds x4, x6, x4
  cinc x6, x7, hs
  mul x7, x8, x11
  umulh x8, x8, x11
  adds x6, x7, x6
  cinc x7, x8, hs
  adds x5, x6, x5
  cinc x6, x7, hs
  mul x7, x9, x11
  umulh x8, x9, x11
  adds x6, x7, x6
  cinc x7, x8, hs
  adds x0, x6, x0
  cinc x6, x7, hs
  add x1, x1, x6
  mov x6, #61005
  movk x6, #58262, lsl 16
  movk x6, #32851, lsl 32
  movk x6, #11582, lsl 48
  mov x7, #37581
  movk x7, #43836, lsl 16
  movk x7, #36286, lsl 32
  movk x7, #51783, lsl 48
  mov x8, #10899
  movk x8, #30709, lsl 16
  movk x8, #61551, lsl 32
  movk x8, #45784, lsl 48
  mov x9, #36612
  movk x9, #63402, lsl 16
  movk x9, #47623, lsl 32
  movk x9, #9430, lsl 48
  mul x11, x6, x12
  umulh x6, x6, x12
  adds x10, x11, x10
  cinc x6, x6, hs
  mul x11, x7, x12
  umulh x7, x7, x12
  adds x6, x11, x6
  cinc x7, x7, hs
  adds x4, x6, x4
  cinc x6, x7, hs
  mul x7, x8, x12
  umulh x8, x8, x12
  adds x6, x7, x6
  cinc x7, x8, hs
  adds x5, x6, x5
  cinc x6, x7, hs
  mul x7, x9, x12
  umulh x8, x9, x12
  adds x6, x7, x6
  cinc x7, x8, hs
  adds x0, x6, x0
  cinc x6, x7, hs
  add x1, x1, x6
  mov x6, #65535
  movk x6, #61439, lsl 16
  movk x6, #62867, lsl 32
  movk x6, #49889, lsl 48
  mul x6, x6, x10
  mov x7, #1
  movk x7, #61440, lsl 16
  movk x7, #62867, lsl 32
  movk x7, #17377, lsl 48
  mov x8, #28817
  movk x8, #31161, lsl 16
  movk x8, #59464, lsl 32
  movk x8, #10291, lsl 48
  mov x9, #22621
  movk x9, #33153, lsl 16
  movk x9, #17846, lsl 32
  movk x9, #47184, lsl 48
  mov x11, #41001
  movk x11, #57649, lsl 16
  movk x11, #20082, lsl 32
  movk x11, #12388, lsl 48
  mul x12, x7, x6
  umulh x7, x7, x6
  cmn x12, x10
  cinc x7, x7, hs
  mul x10, x8, x6
  umulh x8, x8, x6
  adds x7, x10, x7
  cinc x8, x8, hs
  adds x4, x7, x4
  cinc x7, x8, hs
  mul x8, x9, x6
  umulh x9, x9, x6
  adds x7, x8, x7
  cinc x8, x9, hs
  adds x5, x7, x5
  cinc x7, x8, hs
  mul x8, x11, x6
  umulh x6, x11, x6
  adds x7, x8, x7
  cinc x6, x6, hs
  adds x0, x7, x0
  cinc x6, x6, hs
  add x1, x1, x6
  mov x6, #2
  movk x6, #57344, lsl 16
  movk x6, #60199, lsl 32
  movk x6, #34755, lsl 48
  mov x7, #57634
  movk x7, #62322, lsl 16
  movk x7, #53392, lsl 32
  movk x7, #20583, lsl 48
  mov x8, #45242
  movk x8, #770, lsl 16
  movk x8, #35693, lsl 32
  movk x8, #28832, lsl 48
  mov x9, #16467
  movk x9, #49763, lsl 16
  movk x9, #40165, lsl 32
  movk x9, #24776, lsl 48
  subs x6, x4, x6
  sbcs x7, x5, x7
  sbcs x8, x0, x8
  sbcs x9, x1, x9
  tst x1, #9223372036854775808
  csel x4, x6, x4, mi
  csel x5, x7, x5, mi
  csel x0, x8, x0, mi
  csel x1, x9, x1, mi
  ldp x6, x7, [x2, #0]
  ldp x2, x8, [x2, #16]
  ldp x9, x10, [x3, #0]
  ldp x3, x11, [x3, #16]
  mul x12, x6, x9
  umulh x13, x6, x9
  mul x14, x7, x9
  umulh x15, x7, x9
  adds x13, x14, x13
  cinc x14, x15, hs
  mul x15, x2, x9
  umulh x16, x2, x9
  adds x14, x15, x14
  cinc x15, x16, hs
  mul x16, x8, x9
  umulh x9, x8, x9
  adds x15, x16, x15
  cinc x9, x9, hs
  mul x16, x6, x10
  umulh x17, x6, x10
  adds x13, x16, x13
  cinc x16, x17, hs
  mul x17, x7, x10
  umulh x20, x7, x10
  adds x16, x17, x16
  cinc x17, x20, hs
  adds x14, x16, x14
  cinc x16, x17, hs
  mul x17, x2, x10
  umulh x20, x2, x10
  adds x16, x17, x16
  cinc x17, x20, hs
  adds x15, x16, x15
  cinc x16, x17, hs
  mul x17, x8, x10
  umulh x10, x8, x10
  adds x16, x17, x16
  cinc x10, x10, hs
  adds x9, x16, x9
  cinc x10, x10, hs
  mul x16, x6, x3
  umulh x17, x6, x3
  adds x14, x16, x14
  cinc x16, x17, hs
  mul x17, x7, x3
  umulh x20, x7, x3
  adds x16, x17, x16
  cinc x17, x20, hs
  adds x15, x16, x15
  cinc x16, x17, hs
  mul x17, x2, x3
  umulh x20, x2, x3
  adds x16, x17, x16
  cinc x17, x20, hs
  adds x9, x16, x9
  cinc x16, x17, hs
  mul x17, x8, x3
  umulh x3, x8, x3
  adds x16, x17, x16
  cinc x3, x3, hs
  adds x10, x16, x10
  cinc x3, x3, hs
  mul x16, x6, x11
  umulh x6, x6, x11
  adds x15, x16, x15
  cinc x6, x6, hs
  mul x16, x7, x11
  umulh x7, x7, x11
  adds x6, x16, x6
  cinc x7, x7, hs
  adds x6, x6, x9
  cinc x7, x7, hs
  mul x9, x2, x11
  umulh x2, x2, x11
  adds x7, x9, x7
  cinc x2, x2, hs
  adds x7, x7, x10
  cinc x2, x2, hs
  mul x9, x8, x11
  umulh x8, x8, x11
  adds x2, x9, x2
  cinc x8, x8, hs
  adds x2, x2, x3
  cinc x3, x8, hs
  mov x8, #48718
  movk x8, #4732, lsl 16
  movk x8, #45078, lsl 32
  movk x8, #39852, lsl 48
  mov x9, #16676
  movk x9, #12692, lsl 16
  movk x9, #20986, lsl 32
  movk x9, #2848, lsl 48
  mov x10, #51052
  movk x10, #24721, lsl 16
  movk x10, #61092, lsl 32
  movk x10, #45156, lsl 48
  mov x11, #3197
  movk x11, #18936, lsl 16
  movk x11, #10922, lsl 32
  movk x11, #11014, lsl 48
  mul x16, x8, x12
  umulh x8, x8, x12
  adds x15, x16, x15
  cinc x8, x8, hs
  mul x16, x9, x12
  umulh x9, x9, x12
  adds x8, x16, x8
  cinc x9, x9, hs
  adds x6, x8, x6
  cinc x8, x9, hs
  mul x9, x10, x12
  umulh x10, x10, x12
  adds x8, x9, x8
  cinc x9, x10, hs
  adds x7, x8, x7
  cinc x8, x9, hs
  mul x9, x11, x12
  umulh x10, x11, x12
  adds x8, x9, x8
  cinc x9, x10, hs
  adds x2, x8, x2
  cinc x8, x9, hs
  add x3, x3, x8
  mov x8, #56431
  movk x8, #30457, lsl 16
  movk x8, #30012, lsl 32
  movk x8, #6382, lsl 48
  mov x9, #59151
  movk x9, #41769, lsl 16
  movk x9, #32276, lsl 32
  movk x9, #21677, lsl 48
  mov x10, #34015
  movk x10, #20342, lsl 16
  movk x10, #13935, lsl 32
  movk x10, #11030, lsl 48
  mov x11, #13689
  movk x11, #8159, lsl 16
  movk x11, #215, lsl 32
  movk x11, #4913, lsl 48
  mul x12, x8, x13
  umulh x8, x8, x13
  adds x12, x12, x15
  cinc x8, x8, hs
  mul x15, x9, x13
  umulh x9, x9, x13
  adds x8, x15, x8
  cinc x9, x9, hs
  adds x6, x8, x6
  cinc x8, x9, hs
  mul x9, x10, x13
  umulh x10, x10, x13
  adds x8, x9, x8
  cinc x9, x10, hs
  adds x7, x8, x7
  cinc x8, x9, hs
  mul x9, x11, x13
  umulh x10, x11, x13
  adds x8, x9, x8
  cinc x9, x10, hs
  adds x2, x8, x2
  cinc x8, x9, hs
  add x3, x3, x8
  mov x8, #61005
  movk x8, #58262, lsl 16
  movk x8, #32851, lsl 32
  movk x8, #11582, lsl 48
  mov x9, #37581
  movk x9, #43836, lsl 16
  movk x9, #36286, lsl 32
  movk x9, #51783, lsl 48
  mov x10, #10899
  movk x10, #30709, lsl 16
  movk x10, #61551, lsl 32
  movk x10, #45784, lsl 48
  mov x11, #36612
  movk x11, #63402, lsl 16
  movk x11, #47623, lsl 32
  movk x11, #9430, lsl 48
  mul x13, x8, x14
  umulh x8, x8, x14
  adds x12, x13, x12
  cinc x8, x8, hs
  mul x13, x9, x14
  umulh x9, x9, x14
  adds x8, x13, x8
  cinc x9, x9, hs
  adds x6, x8, x6
  cinc x8, x9, hs
  mul x9, x10, x14
  umulh x10, x10, x14
  adds x8, x9, x8
  cinc x9, x10, hs
  adds x7, x8, x7
  cinc x8, x9, hs
  mul x9, x11, x14
  umulh x10, x11, x14
  adds x8, x9, x8
  cinc x9, x10, hs
  adds x2, x8, x2
  cinc x8, x9, hs
  add x3, x3, x8
  mov x8, #65535
  movk x8, #61439, lsl 16
  movk x8, #62867, lsl 32
  movk x8, #49889, lsl 48
  mul x8, x8, x12
  mov x9, #1
  movk x9, #61440, lsl 16
  movk x9, #62867, lsl 32
  movk x9, #17377, lsl 48
  mov x10, #28817
  movk x10, #31161, lsl 16
  movk x10, #59464, lsl 32
  movk x10, #10291, lsl 48
  mov x11, #22621
  movk x11, #33153, lsl 16
  movk x11, #17846, lsl 32
  movk x11, #47184, lsl 48
  mov x13, #41001
  movk x13, #57649, lsl 16
  movk x13, #20082, lsl 32
  movk x13, #12388, lsl 48
  mul x14, x9, x8
  umulh x9, x9, x8
  cmn x14, x12
  cinc x9, x9, hs
  mul x12, x10, x8
  umulh x10, x10, x8
  adds x9, x12, x9
  cinc x10, x10, hs
  adds x6, x9, x6
  cinc x9, x10, hs
  mul x10, x11, x8
  umulh x11, x11, x8
  adds x9, x10, x9
  cinc x10, x11, hs
  adds x7, x9, x7
  cinc x9, x10, hs
  mul x10, x13, x8
  umulh x8, x13, x8
  adds x9, x10, x9
  cinc x8, x8, hs
  adds x2, x9, x2
  cinc x8, x8, hs
  add x3, x3, x8
  mov x8, #2
  movk x8, #57344, lsl 16
  movk x8, #60199, lsl 32
  movk x8, #34755, lsl 48
  mov x9, #57634
  movk x9, #62322, lsl 16
  movk x9, #53392, lsl 32
  movk x9, #20583, lsl 48
  mov x10, #45242
  movk x10, #770, lsl 16
  movk x10, #35693, lsl 32
  movk x10, #28832, lsl 48
  mov x11, #16467
  movk x11, #49763, lsl 16
  movk x11, #40165, lsl 32
  movk x11, #24776, lsl 48
  subs x8, x6, x8
  sbcs x9, x7, x9
  sbcs x10, x2, x10
  sbcs x11, x3, x11
  tst x3, #9223372036854775808
  csel x6, x8, x6, mi
  csel x7, x9, x7, mi
  csel x2, x10, x2, mi
  csel x3, x11, x3, mi
ret
