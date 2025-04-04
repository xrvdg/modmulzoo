;in("x0") _, in("x1") _, in("x2") _, in("x3") _, in("x4") _, in("x5") _, in("x6") _, in("x7") _,
;lateout("x2") out[0], lateout("x3") out[1], lateout("x1") out[2], lateout("x0") out[3],
;lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _, lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _, lateout("x12") _, lateout("x13") _, lateout("x14") _,
;lateout("lr") _
.global single_step
.align 4
.text
_single_step:
  mul x8, x0, x4
  umulh x9, x0, x4
  mul x10, x1, x4
  umulh x11, x1, x4
  adds x10, x10, x9
  cinc x11, x11, hs
  mul x9, x2, x4
  umulh x12, x2, x4
  adds x9, x9, x11
  cinc x12, x12, hs
  mul x11, x3, x4
  umulh x4, x3, x4
  adds x11, x11, x12
  cinc x4, x4, hs
  mul x12, x0, x5
  umulh x13, x0, x5
  adds x12, x12, x10
  cinc x13, x13, hs
  mul x10, x1, x5
  umulh x14, x1, x5
  adds x10, x10, x13
  cinc x14, x14, hs
  adds x10, x10, x9
  cinc x14, x14, hs
  mul x9, x2, x5
  umulh x13, x2, x5
  adds x9, x9, x14
  cinc x13, x13, hs
  adds x9, x9, x11
  cinc x13, x13, hs
  mul x11, x3, x5
  umulh x5, x3, x5
  adds x11, x11, x13
  cinc x5, x5, hs
  adds x11, x11, x4
  cinc x5, x5, hs
  mul x4, x0, x6
  umulh x13, x0, x6
  adds x4, x4, x10
  cinc x13, x13, hs
  mul x10, x1, x6
  umulh x14, x1, x6
  adds x10, x10, x13
  cinc x14, x14, hs
  adds x10, x10, x9
  cinc x14, x14, hs
  mul x9, x2, x6
  umulh x13, x2, x6
  adds x9, x9, x14
  cinc x13, x13, hs
  adds x9, x9, x11
  cinc x13, x13, hs
  mul x11, x3, x6
  umulh x6, x3, x6
  adds x11, x11, x13
  cinc x6, x6, hs
  adds x11, x11, x5
  cinc x6, x6, hs
  mul x5, x0, x7
  umulh x0, x0, x7
  adds x5, x5, x10
  cinc x0, x0, hs
  mul x10, x1, x7
  umulh x1, x1, x7
  adds x10, x10, x0
  cinc x1, x1, hs
  adds x10, x10, x9
  cinc x1, x1, hs
  mul x0, x2, x7
  umulh x2, x2, x7
  adds x0, x0, x1
  cinc x2, x2, hs
  adds x0, x0, x11
  cinc x2, x2, hs
  mul x1, x3, x7
  umulh x3, x3, x7
  adds x1, x1, x2
  cinc x3, x3, hs
  adds x1, x1, x6
  cinc x3, x3, hs
  movk x2, #48718, lsl 0
  movk x2, #4732, lsl 16
  movk x2, #45078, lsl 32
  movk x2, #39852, lsl 48
  movk x6, #16676, lsl 0
  movk x6, #12692, lsl 16
  movk x6, #20986, lsl 32
  movk x6, #2848, lsl 48
  movk x7, #51052, lsl 0
  movk x7, #24721, lsl 16
  movk x7, #61092, lsl 32
  movk x7, #45156, lsl 48
  movk x9, #3197, lsl 0
  movk x9, #18936, lsl 16
  movk x9, #10922, lsl 32
  movk x9, #11014, lsl 48
  mul x11, x2, x8
  umulh x2, x2, x8
  adds x11, x11, x5
  cinc x2, x2, hs
  mul x5, x6, x8
  umulh x6, x6, x8
  adds x5, x5, x2
  cinc x6, x6, hs
  adds x5, x5, x10
  cinc x6, x6, hs
  mul x2, x7, x8
  umulh x7, x7, x8
  adds x2, x2, x6
  cinc x7, x7, hs
  adds x2, x2, x0
  cinc x7, x7, hs
  mul x0, x9, x8
  umulh x6, x9, x8
  adds x0, x0, x7
  cinc x6, x6, hs
  adds x0, x0, x1
  cinc x6, x6, hs
  add x1, x3, x6
  movk x3, #56431, lsl 0
  movk x3, #30457, lsl 16
  movk x3, #30012, lsl 32
  movk x3, #6382, lsl 48
  movk x6, #59151, lsl 0
  movk x6, #41769, lsl 16
  movk x6, #32276, lsl 32
  movk x6, #21677, lsl 48
  movk x7, #34015, lsl 0
  movk x7, #20342, lsl 16
  movk x7, #13935, lsl 32
  movk x7, #11030, lsl 48
  movk x8, #13689, lsl 0
  movk x8, #8159, lsl 16
  movk x8, #215, lsl 32
  movk x8, #4913, lsl 48
  mul x9, x3, x12
  umulh x3, x3, x12
  adds x9, x9, x11
  cinc x3, x3, hs
  mul x10, x6, x12
  umulh x6, x6, x12
  adds x10, x10, x3
  cinc x6, x6, hs
  adds x10, x10, x5
  cinc x6, x6, hs
  mul x3, x7, x12
  umulh x5, x7, x12
  adds x3, x3, x6
  cinc x5, x5, hs
  adds x3, x3, x2
  cinc x5, x5, hs
  mul x2, x8, x12
  umulh x6, x8, x12
  adds x2, x2, x5
  cinc x6, x6, hs
  adds x2, x2, x0
  cinc x6, x6, hs
  add x0, x1, x6
  movk x1, #61005, lsl 0
  movk x1, #58262, lsl 16
  movk x1, #32851, lsl 32
  movk x1, #11582, lsl 48
  movk x5, #37581, lsl 0
  movk x5, #43836, lsl 16
  movk x5, #36286, lsl 32
  movk x5, #51783, lsl 48
  movk x6, #10899, lsl 0
  movk x6, #30709, lsl 16
  movk x6, #61551, lsl 32
  movk x6, #45784, lsl 48
  movk x7, #36612, lsl 0
  movk x7, #63402, lsl 16
  movk x7, #47623, lsl 32
  movk x7, #9430, lsl 48
  mul x8, x1, x4
  umulh x1, x1, x4
  adds x8, x8, x9
  cinc x1, x1, hs
  mul x9, x5, x4
  umulh x5, x5, x4
  adds x9, x9, x1
  cinc x5, x5, hs
  adds x9, x9, x10
  cinc x5, x5, hs
  mul x1, x6, x4
  umulh x6, x6, x4
  adds x1, x1, x5
  cinc x6, x6, hs
  adds x1, x1, x3
  cinc x6, x6, hs
  mul x3, x7, x4
  umulh x4, x7, x4
  adds x3, x3, x6
  cinc x4, x4, hs
  adds x3, x3, x2
  cinc x4, x4, hs
  add x0, x0, x4
  movk x2, #65535, lsl 0
  movk x2, #61439, lsl 16
  movk x2, #62867, lsl 32
  movk x2, #49889, lsl 48
  mul x2, x2, x8
  movk x4, #1, lsl 0
  movk x4, #61440, lsl 16
  movk x4, #62867, lsl 32
  movk x4, #17377, lsl 48
  movk x5, #28817, lsl 0
  movk x5, #31161, lsl 16
  movk x5, #59464, lsl 32
  movk x5, #10291, lsl 48
  movk x6, #22621, lsl 0
  movk x6, #33153, lsl 16
  movk x6, #17846, lsl 32
  movk x6, #47184, lsl 48
  movk x7, #41001, lsl 0
  movk x7, #57649, lsl 16
  movk x7, #20082, lsl 32
  movk x7, #12388, lsl 48
  mul x10, x4, x2
  umulh x4, x4, x2
  cmn x10, x8
  cinc x4, x4, hs
  mul x8, x5, x2
  umulh x5, x5, x2
  adds x8, x8, x4
  cinc x5, x5, hs
  adds x8, x8, x9
  cinc x5, x5, hs
  mul x4, x6, x2
  umulh x6, x6, x2
  adds x4, x4, x5
  cinc x6, x6, hs
  adds x4, x4, x1
  cinc x6, x6, hs
  mul x1, x7, x2
  umulh x2, x7, x2
  adds x1, x1, x6
  cinc x2, x2, hs
  adds x1, x1, x3
  cinc x2, x2, hs
  add x0, x0, x2
  movk x2, #2, lsl 0
  movk x2, #57344, lsl 16
  movk x2, #60199, lsl 32
  movk x2, #34755, lsl 48
  movk x3, #57634, lsl 0
  movk x3, #62322, lsl 16
  movk x3, #53392, lsl 32
  movk x3, #20583, lsl 48
  movk x5, #45242, lsl 0
  movk x5, #770, lsl 16
  movk x5, #35693, lsl 32
  movk x5, #28832, lsl 48
  movk x6, #16467, lsl 0
  movk x6, #49763, lsl 16
  movk x6, #40165, lsl 32
  movk x6, #24776, lsl 48
  subs x2, x8, x2
  sbcs x3, x4, x3
  sbcs x5, x1, x5
  sbcs x6, x0, x6
  tst x0, #9223372036854775808
  csel x2, x2, x8, mi
  csel x3, x3, x4, mi
  csel x1, x5, x1, mi
  csel x0, x6, x0, mi
ret
