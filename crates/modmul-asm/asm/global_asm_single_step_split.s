//in("x0") a[0], in("x1") a[1], in("x2") a[2], in("x3") a[3],
//in("x4") b[0], in("x5") b[1], in("x6") b[2], in("x7") b[3],
//lateout("x0") out[0], lateout("x1") out[1], lateout("x2") out[2], lateout("x3") out[3],
//lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _, lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _, lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _, lateout("x16") _, lateout("x17") _, lateout("x20") _, lateout("x21") _, lateout("x22") _, lateout("x23") _,
//lateout("lr") _
        
.global _single_step_split
.align 4
.text
_single_step_split:
  mul x8, x0, x4
  umulh x9, x0, x4
  mul x10, x1, x4
  umulh x11, x1, x4
  adds x9, x10, x9
  cinc x10, x11, hs
  mul x11, x2, x4
  umulh x12, x2, x4
  adds x10, x11, x10
  cinc x11, x12, hs
  mul x12, x3, x4
  umulh x4, x3, x4
  adds x11, x12, x11
  cinc x4, x4, hs
  mul x12, x0, x5
  umulh x13, x0, x5
  adds x9, x12, x9
  cinc x12, x13, hs
  mul x13, x1, x5
  umulh x14, x1, x5
  adds x12, x13, x12
  cinc x13, x14, hs
  adds x10, x12, x10
  cinc x12, x13, hs
  mul x13, x2, x5
  umulh x14, x2, x5
  adds x12, x13, x12
  cinc x13, x14, hs
  adds x11, x12, x11
  cinc x12, x13, hs
  mul x13, x3, x5
  umulh x5, x3, x5
  adds x12, x13, x12
  cinc x5, x5, hs
  adds x4, x12, x4
  cinc x5, x5, hs
  mul x12, x0, x6
  umulh x13, x0, x6
  adds x10, x12, x10
  cinc x12, x13, hs
  mul x13, x1, x6
  umulh x14, x1, x6
  adds x12, x13, x12
  cinc x13, x14, hs
  adds x11, x12, x11
  cinc x12, x13, hs
  mul x13, x2, x6
  umulh x14, x2, x6
  adds x12, x13, x12
  cinc x13, x14, hs
  adds x4, x12, x4
  cinc x12, x13, hs
  mul x13, x3, x6
  umulh x6, x3, x6
  adds x12, x13, x12
  cinc x6, x6, hs
  adds x5, x12, x5
  cinc x6, x6, hs
  mul x12, x0, x7
  umulh x0, x0, x7
  adds x11, x12, x11
  cinc x0, x0, hs
  mul x12, x1, x7
  umulh x1, x1, x7
  adds x0, x12, x0
  cinc x1, x1, hs
  adds x0, x0, x4
  cinc x1, x1, hs
  mul x4, x2, x7
  umulh x2, x2, x7
  adds x1, x4, x1
  cinc x2, x2, hs
  adds x1, x1, x5
  cinc x2, x2, hs
  mul x4, x3, x7
  umulh x3, x3, x7
  adds x2, x4, x2
  cinc x3, x3, hs
  adds x2, x2, x6
  cinc x3, x3, hs
  mov x4, #48718
  movk x4, #4732, lsl 16
  movk x4, #45078, lsl 32
  movk x4, #39852, lsl 48
  mov x5, #16676
  movk x5, #12692, lsl 16
  movk x5, #20986, lsl 32
  movk x5, #2848, lsl 48
  mov x6, #51052
  movk x6, #24721, lsl 16
  movk x6, #61092, lsl 32
  movk x6, #45156, lsl 48
  mov x7, #3197
  movk x7, #18936, lsl 16
  movk x7, #10922, lsl 32
  movk x7, #11014, lsl 48
  mul x12, x4, x8
  umulh x4, x4, x8
  mul x13, x5, x8
  umulh x5, x5, x8
  adds x4, x13, x4
  cinc x5, x5, hs
  mul x13, x6, x8
  umulh x6, x6, x8
  adds x5, x13, x5
  cinc x6, x6, hs
  mul x13, x7, x8
  umulh x7, x7, x8
  adds x6, x13, x6
  cinc x7, x7, hs
  mov x8, #56431
  movk x8, #30457, lsl 16
  movk x8, #30012, lsl 32
  movk x8, #6382, lsl 48
  mov x13, #59151
  movk x13, #41769, lsl 16
  movk x13, #32276, lsl 32
  movk x13, #21677, lsl 48
  mov x14, #34015
  movk x14, #20342, lsl 16
  movk x14, #13935, lsl 32
  movk x14, #11030, lsl 48
  mov x15, #13689
  movk x15, #8159, lsl 16
  movk x15, #215, lsl 32
  movk x15, #4913, lsl 48
  mul x16, x8, x9
  umulh x8, x8, x9
  mul x17, x13, x9
  umulh x13, x13, x9
  adds x8, x17, x8
  cinc x13, x13, hs
  mul x17, x14, x9
  umulh x14, x14, x9
  adds x13, x17, x13
  cinc x14, x14, hs
  mul x17, x15, x9
  umulh x9, x15, x9
  adds x14, x17, x14
  cinc x9, x9, hs
  mov x15, #61005
  movk x15, #58262, lsl 16
  movk x15, #32851, lsl 32
  movk x15, #11582, lsl 48
  mov x17, #37581
  movk x17, #43836, lsl 16
  movk x17, #36286, lsl 32
  movk x17, #51783, lsl 48
  mov x20, #10899
  movk x20, #30709, lsl 16
  movk x20, #61551, lsl 32
  movk x20, #45784, lsl 48
  mov x21, #36612
  movk x21, #63402, lsl 16
  movk x21, #47623, lsl 32
  movk x21, #9430, lsl 48
  mul x22, x15, x10
  umulh x15, x15, x10
  mul x23, x17, x10
  umulh x17, x17, x10
  adds x15, x23, x15
  cinc x17, x17, hs
  mul x23, x20, x10
  umulh x20, x20, x10
  adds x17, x23, x17
  cinc x20, x20, hs
  mul x23, x21, x10
  umulh x10, x21, x10
  adds x20, x23, x20
  cinc x10, x10, hs
  adds x12, x12, x16
  adcs x4, x4, x8
  adcs x5, x5, x13
  adcs x6, x6, x14
  adc x7, x7, x9
  adds x8, x12, x22
  adcs x4, x4, x15
  adcs x5, x5, x17
  adcs x6, x6, x20
  adc x7, x7, x10
  adds x8, x8, x11
  adcs x0, x4, x0
  adcs x1, x5, x1
  adcs x2, x6, x2
  adc x3, x7, x3
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
  mul x11, x6, x4
  umulh x6, x6, x4
  adds x5, x11, x5
  cinc x6, x6, hs
  mul x11, x7, x4
  umulh x7, x7, x4
  adds x6, x11, x6
  cinc x7, x7, hs
  mul x11, x9, x4
  umulh x4, x9, x4
  adds x7, x11, x7
  cinc x4, x4, hs
  cmn x10, x8
  adcs x0, x5, x0
  adcs x1, x6, x1
  adcs x2, x7, x2
  adcs x5, x4, x3
  adc x5, x4, x3
  mov x3, #2
  movk x3, #57344, lsl 16
  movk x3, #60199, lsl 32
  movk x3, #34755, lsl 48
  mov x4, #57634
  movk x4, #62322, lsl 16
  movk x4, #53392, lsl 32
  movk x4, #20583, lsl 48
  mov x6, #45242
  movk x6, #770, lsl 16
  movk x6, #35693, lsl 32
  movk x6, #28832, lsl 48
  mov x7, #16467
  movk x7, #49763, lsl 16
  movk x7, #40165, lsl 32
  movk x7, #24776, lsl 48
  subs x3, x0, x3
  sbcs x4, x1, x4
  sbcs x6, x2, x6
  sbcs x7, x5, x7
  tst x5, #9223372036854775808
  csel x0, x3, x0, mi
  csel x1, x4, x1, mi
  csel x2, x6, x2, mi
  csel x3, x7, x5, mi
  ret