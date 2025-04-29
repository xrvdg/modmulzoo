//in("x0") a,
//in("x1") b,
//in("x2") a,
//in("x3") b,
//in("x4") a,
//in("x5") b,
//in("v0") av[0], in("v1") av[1], in("v2") av[2], in("v3") av[3],
//in("v4") bv[0], in("v5") bv[1], in("v6") bv[2], in("v7") bv[3],
//lateout("x0") a,
//lateout("x2") a,
//lateout("x4") a,
//lateout("v0") outv[0], lateout("v1") outv[1], lateout("v2") outv[2], lateout("v3") outv[3],
//lateout("x1") _, lateout("x3") _, lateout("x5") _, lateout("x6") _, lateout("x7") _, lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _, lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _, lateout("x16") _, lateout("x17") _, lateout("x20") _, lateout("x21") _, lateout("x22") _, lateout("x23") _, lateout("v4") _, lateout("v5") _, lateout("v6") _, lateout("v7") _, lateout("v8") _, lateout("v9") _, lateout("v10") _, lateout("v11") _, lateout("v12") _, lateout("v13") _, lateout("v14") _, lateout("v15") _, lateout("v16") _, lateout("v17") _, lateout("v18") _, lateout("v19") _, lateout("v20") _, lateout("v21") _, lateout("v22") _, lateout("v23") _, lateout("v24") _,
//lateout("lr") _
        
.global _single_step_interleaved_triple_scalar
.align 4
.text
_single_step_interleaved_triple_scalar:
  ldp x6, x7, [x0, #0]
  mov x8, #4503599627370495
  ldp x9, x10, [x0, #16]
  dup.2d v8, x8
  ldp x11, x12, [x1, #0]
  shl.2d v9, v1, #14
  ldp x1, x13, [x1, #16]
  shl.2d v10, v2, #26
  mul x14, x6, x11
  shl.2d v11, v3, #38
  umulh x15, x6, x11
  ushr.2d v3, v3, #14
  mul x16, x7, x11
  umulh x17, x7, x11
  shl.2d v12, v0, #2
  adds x15, x16, x15
  cinc x16, x17, hs
  usra.2d v9, v0, #50
  mul x17, x9, x11
  usra.2d v10, v1, #38
  umulh x20, x9, x11
  usra.2d v11, v2, #26
  adds x16, x17, x16
  cinc x17, x20, hs
  and.16b v0, v12, v8
  mul x20, x10, x11
  and.16b v1, v9, v8
  umulh x11, x10, x11
  and.16b v2, v10, v8
  adds x17, x20, x17
  cinc x11, x11, hs
  mul x20, x6, x12
  and.16b v9, v11, v8
  umulh x21, x6, x12
  shl.2d v10, v5, #14
  adds x15, x20, x15
  cinc x20, x21, hs
  shl.2d v11, v6, #26
  mul x21, x7, x12
  shl.2d v12, v7, #38
  umulh x22, x7, x12
  ushr.2d v7, v7, #14
  adds x20, x21, x20
  cinc x21, x22, hs
  shl.2d v13, v4, #2
  adds x16, x20, x16
  cinc x20, x21, hs
  usra.2d v10, v4, #50
  mul x21, x9, x12
  umulh x22, x9, x12
  usra.2d v11, v5, #38
  adds x20, x21, x20
  cinc x21, x22, hs
  usra.2d v12, v6, #26
  adds x17, x20, x17
  cinc x20, x21, hs
  and.16b v4, v13, v8
  mul x21, x10, x12
  and.16b v5, v10, v8
  umulh x12, x10, x12
  and.16b v6, v11, v8
  adds x20, x21, x20
  cinc x12, x12, hs
  and.16b v10, v12, v8
  adds x11, x20, x11
  cinc x12, x12, hs
  mov x20, #13605374474286268416
  mul x21, x6, x1
  umulh x22, x6, x1
  dup.2d v11, x20
  adds x16, x21, x16
  cinc x20, x22, hs
  mov x21, #6440147467139809280
  mul x22, x7, x1
  dup.2d v12, x21
  umulh x21, x7, x1
  mov x23, #3688448094816436224
  adds x20, x22, x20
  cinc x21, x21, hs
  dup.2d v13, x23
  adds x17, x20, x17
  cinc x20, x21, hs
  mov x21, #9209861237972664320
  mul x22, x9, x1
  dup.2d v14, x21
  umulh x21, x9, x1
  adds x20, x22, x20
  cinc x21, x21, hs
  mov x22, #12218265789056155648
  adds x11, x20, x11
  cinc x20, x21, hs
  dup.2d v15, x22
  mul x21, x10, x1
  mov x22, #17739678932212383744
  umulh x1, x10, x1
  dup.2d v16, x22
  adds x20, x21, x20
  cinc x1, x1, hs
  mov x21, #2301339409586323456
  adds x12, x20, x12
  cinc x1, x1, hs
  dup.2d v17, x21
  mul x20, x6, x13
  mov x21, #7822752552742551552
  umulh x6, x6, x13
  adds x17, x20, x17
  cinc x6, x6, hs
  dup.2d v18, x21
  mul x20, x7, x13
  mov x21, #5071053180419178496
  umulh x7, x7, x13
  dup.2d v19, x21
  adds x6, x20, x6
  cinc x7, x7, hs
  mov x20, #16352570246982270976
  adds x6, x6, x11
  cinc x7, x7, hs
  dup.2d v20, x20
  mul x11, x9, x13
  mov x20, #5075556780046548992
  umulh x9, x9, x13
  dup.2d v21, x20
  adds x7, x11, x7
  cinc x9, x9, hs
  adds x7, x7, x12
  cinc x9, x9, hs
  mov x11, #1
  mul x12, x10, x13
  movk x11, #18032, lsl 48
  umulh x10, x10, x13
  dup.2d v22, x11
  adds x9, x12, x9
  cinc x10, x10, hs
  ucvtf.2d v0, v0
  adds x1, x9, x1
  cinc x9, x10, hs
  ucvtf.2d v1, v1
  mov x10, #48718
  ucvtf.2d v2, v2
  movk x10, #4732, lsl 16
  ucvtf.2d v9, v9
  movk x10, #45078, lsl 32
  movk x10, #39852, lsl 48
  ucvtf.2d v3, v3
  mov x11, #16676
  ucvtf.2d v4, v4
  movk x11, #12692, lsl 16
  ucvtf.2d v5, v5
  movk x11, #20986, lsl 32
  ucvtf.2d v6, v6
  movk x11, #2848, lsl 48
  ucvtf.2d v10, v10
  mov x12, #51052
  ucvtf.2d v7, v7
  movk x12, #24721, lsl 16
  mov.16b v23, v21
  movk x12, #61092, lsl 32
  movk x12, #45156, lsl 48
  fmla.2d v23, v0, v4
  mov x13, #3197
  fsub.2d v24, v22, v23
  movk x13, #18936, lsl 16
  fmla.2d v24, v0, v4
  movk x13, #10922, lsl 32
  add.2d v13, v13, v23
  movk x13, #11014, lsl 48
  add.2d v11, v11, v24
  mul x20, x10, x14
  mov.16b v23, v21
  umulh x10, x10, x14
  fmla.2d v23, v0, v5
  adds x17, x20, x17
  cinc x10, x10, hs
  mul x20, x11, x14
  fsub.2d v24, v22, v23
  umulh x11, x11, x14
  fmla.2d v24, v0, v5
  adds x10, x20, x10
  cinc x11, x11, hs
  add.2d v15, v15, v23
  adds x6, x10, x6
  cinc x10, x11, hs
  add.2d v13, v13, v24
  mul x11, x12, x14
  mov.16b v23, v21
  umulh x12, x12, x14
  fmla.2d v23, v0, v6
  adds x10, x11, x10
  cinc x11, x12, hs
  fsub.2d v24, v22, v23
  adds x7, x10, x7
  cinc x10, x11, hs
  mul x11, x13, x14
  fmla.2d v24, v0, v6
  umulh x12, x13, x14
  add.2d v17, v17, v23
  adds x10, x11, x10
  cinc x11, x12, hs
  add.2d v15, v15, v24
  adds x1, x10, x1
  cinc x10, x11, hs
  mov.16b v23, v21
  add x9, x9, x10
  fmla.2d v23, v0, v10
  mov x10, #56431
  fsub.2d v24, v22, v23
  movk x10, #30457, lsl 16
  fmla.2d v24, v0, v10
  movk x10, #30012, lsl 32
  movk x10, #6382, lsl 48
  add.2d v19, v19, v23
  mov x11, #59151
  add.2d v17, v17, v24
  movk x11, #41769, lsl 16
  mov.16b v23, v21
  movk x11, #32276, lsl 32
  fmla.2d v23, v0, v7
  movk x11, #21677, lsl 48
  fsub.2d v24, v22, v23
  mov x12, #34015
  fmla.2d v24, v0, v7
  movk x12, #20342, lsl 16
  add.2d v0, v20, v23
  movk x12, #13935, lsl 32
  movk x12, #11030, lsl 48
  add.2d v19, v19, v24
  mov x13, #13689
  mov.16b v20, v21
  movk x13, #8159, lsl 16
  fmla.2d v20, v1, v4
  movk x13, #215, lsl 32
  fsub.2d v23, v22, v20
  movk x13, #4913, lsl 48
  fmla.2d v23, v1, v4
  mul x14, x10, x15
  add.2d v15, v15, v20
  umulh x10, x10, x15
  add.2d v13, v13, v23
  adds x14, x14, x17
  cinc x10, x10, hs
  mul x17, x11, x15
  mov.16b v20, v21
  umulh x11, x11, x15
  fmla.2d v20, v1, v5
  adds x10, x17, x10
  cinc x11, x11, hs
  fsub.2d v23, v22, v20
  adds x6, x10, x6
  cinc x10, x11, hs
  fmla.2d v23, v1, v5
  mul x11, x12, x15
  add.2d v17, v17, v20
  umulh x12, x12, x15
  add.2d v15, v15, v23
  adds x10, x11, x10
  cinc x11, x12, hs
  mov.16b v20, v21
  adds x7, x10, x7
  cinc x10, x11, hs
  mul x11, x13, x15
  fmla.2d v20, v1, v6
  umulh x12, x13, x15
  fsub.2d v23, v22, v20
  adds x10, x11, x10
  cinc x11, x12, hs
  fmla.2d v23, v1, v6
  adds x1, x10, x1
  cinc x10, x11, hs
  add.2d v19, v19, v20
  add x9, x9, x10
  add.2d v17, v17, v23
  mov x10, #61005
  mov.16b v20, v21
  movk x10, #58262, lsl 16
  fmla.2d v20, v1, v10
  movk x10, #32851, lsl 32
  movk x10, #11582, lsl 48
  fsub.2d v23, v22, v20
  mov x11, #37581
  fmla.2d v23, v1, v10
  movk x11, #43836, lsl 16
  add.2d v0, v0, v20
  movk x11, #36286, lsl 32
  add.2d v19, v19, v23
  movk x11, #51783, lsl 48
  mov.16b v20, v21
  mov x12, #10899
  fmla.2d v20, v1, v7
  movk x12, #30709, lsl 16
  fsub.2d v23, v22, v20
  movk x12, #61551, lsl 32
  movk x12, #45784, lsl 48
  fmla.2d v23, v1, v7
  mov x13, #36612
  add.2d v1, v18, v20
  movk x13, #63402, lsl 16
  add.2d v0, v0, v23
  movk x13, #47623, lsl 32
  mov.16b v18, v21
  movk x13, #9430, lsl 48
  fmla.2d v18, v2, v4
  mul x15, x10, x16
  fsub.2d v20, v22, v18
  umulh x10, x10, x16
  fmla.2d v20, v2, v4
  adds x14, x15, x14
  cinc x10, x10, hs
  mul x15, x11, x16
  add.2d v17, v17, v18
  umulh x11, x11, x16
  add.2d v15, v15, v20
  adds x10, x15, x10
  cinc x11, x11, hs
  mov.16b v18, v21
  adds x6, x10, x6
  cinc x10, x11, hs
  fmla.2d v18, v2, v5
  mul x11, x12, x16
  fsub.2d v20, v22, v18
  umulh x12, x12, x16
  fmla.2d v20, v2, v5
  adds x10, x11, x10
  cinc x11, x12, hs
  add.2d v18, v19, v18
  adds x7, x10, x7
  cinc x10, x11, hs
  mul x11, x13, x16
  add.2d v17, v17, v20
  umulh x12, x13, x16
  mov.16b v19, v21
  adds x10, x11, x10
  cinc x11, x12, hs
  fmla.2d v19, v2, v6
  adds x1, x10, x1
  cinc x10, x11, hs
  fsub.2d v20, v22, v19
  add x9, x9, x10
  fmla.2d v20, v2, v6
  mov x10, #65535
  add.2d v0, v0, v19
  movk x10, #61439, lsl 16
  add.2d v18, v18, v20
  movk x10, #62867, lsl 32
  movk x10, #49889, lsl 48
  mov.16b v19, v21
  mul x10, x10, x14
  fmla.2d v19, v2, v10
  mov x11, #1
  fsub.2d v20, v22, v19
  movk x11, #61440, lsl 16
  fmla.2d v20, v2, v10
  movk x11, #62867, lsl 32
  add.2d v1, v1, v19
  movk x11, #17377, lsl 48
  add.2d v0, v0, v20
  mov x12, #28817
  mov.16b v19, v21
  movk x12, #31161, lsl 16
  movk x12, #59464, lsl 32
  fmla.2d v19, v2, v7
  movk x12, #10291, lsl 48
  fsub.2d v20, v22, v19
  mov x13, #22621
  fmla.2d v20, v2, v7
  movk x13, #33153, lsl 16
  add.2d v2, v16, v19
  movk x13, #17846, lsl 32
  add.2d v1, v1, v20
  movk x13, #47184, lsl 48
  mov.16b v16, v21
  mov x15, #41001
  fmla.2d v16, v9, v4
  movk x15, #57649, lsl 16
  movk x15, #20082, lsl 32
  fsub.2d v19, v22, v16
  movk x15, #12388, lsl 48
  fmla.2d v19, v9, v4
  mul x16, x11, x10
  add.2d v16, v18, v16
  umulh x11, x11, x10
  add.2d v17, v17, v19
  cmn x16, x14
  cinc x11, x11, hs
  mov.16b v18, v21
  mul x14, x12, x10
  fmla.2d v18, v9, v5
  umulh x12, x12, x10
  fsub.2d v19, v22, v18
  adds x11, x14, x11
  cinc x12, x12, hs
  adds x6, x11, x6
  cinc x11, x12, hs
  fmla.2d v19, v9, v5
  mul x12, x13, x10
  add.2d v0, v0, v18
  umulh x13, x13, x10
  add.2d v16, v16, v19
  adds x11, x12, x11
  cinc x12, x13, hs
  mov.16b v18, v21
  adds x7, x11, x7
  cinc x11, x12, hs
  fmla.2d v18, v9, v6
  mul x12, x15, x10
  fsub.2d v19, v22, v18
  umulh x10, x15, x10
  fmla.2d v19, v9, v6
  adds x11, x12, x11
  cinc x10, x10, hs
  adds x1, x11, x1
  cinc x10, x10, hs
  add.2d v1, v1, v18
  add x9, x9, x10
  add.2d v0, v0, v19
  mov x10, #2
  mov.16b v18, v21
  movk x10, #57344, lsl 16
  fmla.2d v18, v9, v10
  movk x10, #60199, lsl 32
  fsub.2d v19, v22, v18
  movk x10, #34755, lsl 48
  fmla.2d v19, v9, v10
  mov x11, #57634
  add.2d v2, v2, v18
  movk x11, #62322, lsl 16
  movk x11, #53392, lsl 32
  add.2d v1, v1, v19
  movk x11, #20583, lsl 48
  mov.16b v18, v21
  mov x12, #45242
  fmla.2d v18, v9, v7
  movk x12, #770, lsl 16
  fsub.2d v19, v22, v18
  movk x12, #35693, lsl 32
  fmla.2d v19, v9, v7
  movk x12, #28832, lsl 48
  add.2d v9, v14, v18
  mov x13, #16467
  add.2d v2, v2, v19
  movk x13, #49763, lsl 16
  movk x13, #40165, lsl 32
  mov.16b v14, v21
  movk x13, #24776, lsl 48
  fmla.2d v14, v3, v4
  subs x10, x6, x10
  sbcs x11, x7, x11
  sbcs x12, x1, x12
  sbcs x13, x9, x13
  fsub.2d v18, v22, v14
  tst x9, #9223372036854775808
  csel x6, x10, x6, mi
  csel x7, x11, x7, mi
  csel x1, x12, x1, mi
  csel x9, x13, x9, mi
  fmla.2d v18, v3, v4
  stp x6, x7, [x0, #0]
  add.2d v0, v0, v14
  stp x1, x9, [x0, #16]
  add.2d v4, v16, v18
  ldp x1, x6, [x2, #0]
  mov.16b v14, v21
  ldp x7, x9, [x2, #16]
  ldp x10, x11, [x3, #0]
  fmla.2d v14, v3, v5
  ldp x3, x12, [x3, #16]
  fsub.2d v16, v22, v14
  mul x13, x1, x10
  fmla.2d v16, v3, v5
  umulh x14, x1, x10
  add.2d v1, v1, v14
  mul x15, x6, x10
  add.2d v0, v0, v16
  umulh x16, x6, x10
  mov.16b v5, v21
  adds x14, x15, x14
  cinc x15, x16, hs
  fmla.2d v5, v3, v6
  mul x16, x7, x10
  umulh x17, x7, x10
  fsub.2d v14, v22, v5
  adds x15, x16, x15
  cinc x16, x17, hs
  fmla.2d v14, v3, v6
  mul x17, x9, x10
  add.2d v2, v2, v5
  umulh x10, x9, x10
  add.2d v1, v1, v14
  adds x16, x17, x16
  cinc x10, x10, hs
  mov.16b v5, v21
  mul x17, x1, x11
  fmla.2d v5, v3, v10
  umulh x20, x1, x11
  fsub.2d v6, v22, v5
  adds x14, x17, x14
  cinc x17, x20, hs
  mul x20, x6, x11
  fmla.2d v6, v3, v10
  umulh x21, x6, x11
  add.2d v5, v9, v5
  adds x17, x20, x17
  cinc x20, x21, hs
  add.2d v2, v2, v6
  adds x15, x17, x15
  cinc x17, x20, hs
  mov.16b v6, v21
  mul x20, x7, x11
  fmla.2d v6, v3, v7
  umulh x21, x7, x11
  fsub.2d v9, v22, v6
  adds x17, x20, x17
  cinc x20, x21, hs
  fmla.2d v9, v3, v7
  adds x16, x17, x16
  cinc x17, x20, hs
  mul x20, x9, x11
  add.2d v3, v12, v6
  umulh x11, x9, x11
  add.2d v5, v5, v9
  adds x17, x20, x17
  cinc x11, x11, hs
  usra.2d v13, v11, #52
  adds x10, x17, x10
  cinc x11, x11, hs
  usra.2d v15, v13, #52
  mul x17, x1, x3
  usra.2d v17, v15, #52
  umulh x20, x1, x3
  usra.2d v4, v17, #52
  adds x15, x17, x15
  cinc x17, x20, hs
  and.16b v6, v11, v8
  mul x20, x6, x3
  umulh x21, x6, x3
  and.16b v7, v13, v8
  adds x17, x20, x17
  cinc x20, x21, hs
  and.16b v9, v15, v8
  adds x16, x17, x16
  cinc x17, x20, hs
  and.16b v8, v17, v8
  mul x20, x7, x3
  ucvtf.2d v6, v6
  umulh x21, x7, x3
  mov x22, #37864
  adds x17, x20, x17
  cinc x20, x21, hs
  movk x22, #1815, lsl 16
  adds x10, x17, x10
  cinc x17, x20, hs
  movk x22, #28960, lsl 32
  mul x20, x9, x3
  umulh x3, x9, x3
  movk x22, #17153, lsl 48
  adds x17, x20, x17
  cinc x3, x3, hs
  dup.2d v10, x22
  adds x11, x17, x11
  cinc x3, x3, hs
  mov.16b v11, v21
  mul x17, x1, x12
  fmla.2d v11, v6, v10
  umulh x1, x1, x12
  fsub.2d v12, v22, v11
  adds x16, x17, x16
  cinc x1, x1, hs
  fmla.2d v12, v6, v10
  mul x17, x6, x12
  add.2d v0, v0, v11
  umulh x6, x6, x12
  adds x1, x17, x1
  cinc x6, x6, hs
  add.2d v4, v4, v12
  adds x1, x1, x10
  cinc x6, x6, hs
  mov x10, #46128
  mul x17, x7, x12
  movk x10, #29964, lsl 16
  umulh x7, x7, x12
  movk x10, #7587, lsl 32
  adds x6, x17, x6
  cinc x7, x7, hs
  movk x10, #17161, lsl 48
  adds x6, x6, x11
  cinc x7, x7, hs
  dup.2d v10, x10
  mul x10, x9, x12
  mov.16b v11, v21
  umulh x9, x9, x12
  adds x7, x10, x7
  cinc x9, x9, hs
  fmla.2d v11, v6, v10
  adds x3, x7, x3
  cinc x7, x9, hs
  fsub.2d v12, v22, v11
  mov x9, #48718
  fmla.2d v12, v6, v10
  movk x9, #4732, lsl 16
  add.2d v1, v1, v11
  movk x9, #45078, lsl 32
  add.2d v0, v0, v12
  movk x9, #39852, lsl 48
  mov x10, #52826
  mov x11, #16676
  movk x10, #57790, lsl 16
  movk x11, #12692, lsl 16
  movk x11, #20986, lsl 32
  movk x10, #55431, lsl 32
  movk x11, #2848, lsl 48
  movk x10, #17196, lsl 48
  mov x12, #51052
  dup.2d v10, x10
  movk x12, #24721, lsl 16
  mov.16b v11, v21
  movk x12, #61092, lsl 32
  fmla.2d v11, v6, v10
  movk x12, #45156, lsl 48
  fsub.2d v12, v22, v11
  mov x10, #3197
  fmla.2d v12, v6, v10
  movk x10, #18936, lsl 16
  movk x10, #10922, lsl 32
  add.2d v2, v2, v11
  movk x10, #11014, lsl 48
  add.2d v1, v1, v12
  mul x17, x9, x13
  mov x20, #31276
  umulh x9, x9, x13
  movk x20, #21262, lsl 16
  adds x16, x17, x16
  cinc x9, x9, hs
  movk x20, #2304, lsl 32
  mul x17, x11, x13
  movk x20, #17182, lsl 48
  umulh x11, x11, x13
  dup.2d v10, x20
  adds x9, x17, x9
  cinc x11, x11, hs
  adds x1, x9, x1
  cinc x9, x11, hs
  mov.16b v11, v21
  mul x11, x12, x13
  fmla.2d v11, v6, v10
  umulh x12, x12, x13
  fsub.2d v12, v22, v11
  adds x9, x11, x9
  cinc x11, x12, hs
  fmla.2d v12, v6, v10
  adds x6, x9, x6
  cinc x9, x11, hs
  add.2d v5, v5, v11
  mul x11, x10, x13
  add.2d v2, v2, v12
  umulh x10, x10, x13
  mov x12, #28672
  adds x9, x11, x9
  cinc x10, x10, hs
  adds x3, x9, x3
  cinc x9, x10, hs
  movk x12, #24515, lsl 16
  add x7, x7, x9
  movk x12, #54929, lsl 32
  mov x9, #56431
  movk x12, #17064, lsl 48
  movk x9, #30457, lsl 16
  dup.2d v10, x12
  movk x9, #30012, lsl 32
  mov.16b v11, v21
  movk x9, #6382, lsl 48
  fmla.2d v11, v6, v10
  mov x10, #59151
  fsub.2d v12, v22, v11
  movk x10, #41769, lsl 16
  movk x10, #32276, lsl 32
  fmla.2d v12, v6, v10
  movk x10, #21677, lsl 48
  add.2d v3, v3, v11
  mov x11, #34015
  add.2d v5, v5, v12
  movk x11, #20342, lsl 16
  ucvtf.2d v6, v7
  movk x11, #13935, lsl 32
  mov x12, #44768
  movk x11, #11030, lsl 48
  movk x12, #51919, lsl 16
  mov x13, #13689
  movk x12, #6346, lsl 32
  movk x13, #8159, lsl 16
  movk x13, #215, lsl 32
  movk x12, #17133, lsl 48
  movk x13, #4913, lsl 48
  dup.2d v7, x12
  mul x12, x9, x14
  mov.16b v10, v21
  umulh x9, x9, x14
  fmla.2d v10, v6, v7
  adds x12, x12, x16
  cinc x9, x9, hs
  fsub.2d v11, v22, v10
  mul x16, x10, x14
  fmla.2d v11, v6, v7
  umulh x10, x10, x14
  add.2d v0, v0, v10
  adds x9, x16, x9
  cinc x10, x10, hs
  adds x1, x9, x1
  cinc x9, x10, hs
  add.2d v4, v4, v11
  mul x10, x11, x14
  mov x16, #47492
  umulh x11, x11, x14
  movk x16, #23630, lsl 16
  adds x9, x10, x9
  cinc x10, x11, hs
  movk x16, #49985, lsl 32
  adds x6, x9, x6
  cinc x9, x10, hs
  movk x16, #17168, lsl 48
  mul x10, x13, x14
  dup.2d v7, x16
  umulh x11, x13, x14
  mov.16b v10, v21
  adds x9, x10, x9
  cinc x10, x11, hs
  adds x3, x9, x3
  cinc x9, x10, hs
  fmla.2d v10, v6, v7
  add x7, x7, x9
  fsub.2d v11, v22, v10
  mov x9, #61005
  fmla.2d v11, v6, v7
  movk x9, #58262, lsl 16
  add.2d v1, v1, v10
  movk x9, #32851, lsl 32
  add.2d v0, v0, v11
  movk x9, #11582, lsl 48
  mov x10, #57936
  mov x11, #37581
  movk x10, #54828, lsl 16
  movk x11, #43836, lsl 16
  movk x11, #36286, lsl 32
  movk x10, #18292, lsl 32
  movk x11, #51783, lsl 48
  movk x10, #17197, lsl 48
  mov x13, #10899
  dup.2d v7, x10
  movk x13, #30709, lsl 16
  mov.16b v10, v21
  movk x13, #61551, lsl 32
  fmla.2d v10, v6, v7
  movk x13, #45784, lsl 48
  fsub.2d v11, v22, v10
  mov x10, #36612
  fmla.2d v11, v6, v7
  movk x10, #63402, lsl 16
  movk x10, #47623, lsl 32
  add.2d v2, v2, v10
  movk x10, #9430, lsl 48
  add.2d v1, v1, v11
  mul x14, x9, x15
  mov x16, #17708
  umulh x9, x9, x15
  movk x16, #43915, lsl 16
  adds x12, x14, x12
  cinc x9, x9, hs
  movk x16, #64348, lsl 32
  mul x14, x11, x15
  movk x16, #17188, lsl 48
  umulh x11, x11, x15
  dup.2d v7, x16
  adds x9, x14, x9
  cinc x11, x11, hs
  adds x1, x9, x1
  cinc x9, x11, hs
  mov.16b v10, v21
  mul x11, x13, x15
  fmla.2d v10, v6, v7
  umulh x13, x13, x15
  fsub.2d v11, v22, v10
  adds x9, x11, x9
  cinc x11, x13, hs
  fmla.2d v11, v6, v7
  adds x6, x9, x6
  cinc x9, x11, hs
  add.2d v5, v5, v10
  mul x11, x10, x15
  add.2d v2, v2, v11
  umulh x10, x10, x15
  mov x13, #29184
  adds x9, x11, x9
  cinc x10, x10, hs
  adds x3, x9, x3
  cinc x9, x10, hs
  movk x13, #20789, lsl 16
  add x7, x7, x9
  movk x13, #19197, lsl 32
  mov x9, #65535
  movk x13, #17083, lsl 48
  movk x9, #61439, lsl 16
  dup.2d v7, x13
  movk x9, #62867, lsl 32
  mov.16b v10, v21
  movk x9, #49889, lsl 48
  fmla.2d v10, v6, v7
  mul x9, x9, x12
  fsub.2d v11, v22, v10
  mov x10, #1
  movk x10, #61440, lsl 16
  fmla.2d v11, v6, v7
  movk x10, #62867, lsl 32
  add.2d v3, v3, v10
  movk x10, #17377, lsl 48
  add.2d v5, v5, v11
  mov x11, #28817
  ucvtf.2d v6, v9
  movk x11, #31161, lsl 16
  mov x13, #58856
  movk x11, #59464, lsl 32
  movk x13, #14953, lsl 16
  movk x11, #10291, lsl 48
  movk x13, #15155, lsl 32
  mov x14, #22621
  movk x14, #33153, lsl 16
  movk x13, #17181, lsl 48
  movk x14, #17846, lsl 32
  dup.2d v7, x13
  movk x14, #47184, lsl 48
  mov.16b v9, v21
  mov x13, #41001
  fmla.2d v9, v6, v7
  movk x13, #57649, lsl 16
  fsub.2d v10, v22, v9
  movk x13, #20082, lsl 32
  fmla.2d v10, v6, v7
  movk x13, #12388, lsl 48
  add.2d v0, v0, v9
  mul x15, x10, x9
  umulh x10, x10, x9
  add.2d v4, v4, v10
  cmn x15, x12
  cinc x10, x10, hs
  mov x12, #35392
  mul x15, x11, x9
  movk x12, #12477, lsl 16
  umulh x11, x11, x9
  movk x12, #56780, lsl 32
  adds x10, x15, x10
  cinc x11, x11, hs
  movk x12, #17142, lsl 48
  adds x1, x10, x1
  cinc x10, x11, hs
  dup.2d v7, x12
  mul x11, x14, x9
  mov.16b v9, v21
  umulh x12, x14, x9
  adds x10, x11, x10
  cinc x11, x12, hs
  fmla.2d v9, v6, v7
  adds x6, x10, x6
  cinc x10, x11, hs
  fsub.2d v10, v22, v9
  mul x11, x13, x9
  fmla.2d v10, v6, v7
  umulh x9, x13, x9
  add.2d v1, v1, v9
  adds x10, x11, x10
  cinc x9, x9, hs
  add.2d v0, v0, v10
  adds x3, x10, x3
  cinc x9, x9, hs
  mov x10, #9848
  add x7, x7, x9
  movk x10, #54501, lsl 16
  mov x9, #2
  movk x9, #57344, lsl 16
  movk x10, #31540, lsl 32
  movk x9, #60199, lsl 32
  movk x10, #17170, lsl 48
  movk x9, #34755, lsl 48
  dup.2d v7, x10
  mov x10, #57634
  mov.16b v9, v21
  movk x10, #62322, lsl 16
  fmla.2d v9, v6, v7
  movk x10, #53392, lsl 32
  fsub.2d v10, v22, v9
  movk x10, #20583, lsl 48
  fmla.2d v10, v6, v7
  mov x11, #45242
  movk x11, #770, lsl 16
  add.2d v2, v2, v9
  movk x11, #35693, lsl 32
  add.2d v1, v1, v10
  movk x11, #28832, lsl 48
  mov x12, #9584
  mov x13, #16467
  movk x12, #63883, lsl 16
  movk x13, #49763, lsl 16
  movk x12, #18253, lsl 32
  movk x13, #40165, lsl 32
  movk x12, #17190, lsl 48
  movk x13, #24776, lsl 48
  dup.2d v7, x12
  subs x9, x1, x9
  sbcs x10, x6, x10
  sbcs x11, x3, x11
  sbcs x12, x7, x13
  tst x7, #9223372036854775808
  csel x1, x9, x1, mi
  csel x6, x10, x6, mi
  csel x3, x11, x3, mi
  csel x7, x12, x7, mi
  mov.16b v9, v21
  stp x1, x6, [x2, #0]
  fmla.2d v9, v6, v7
  stp x3, x7, [x2, #16]
  fsub.2d v10, v22, v9
  ldp x1, x3, [x4, #0]
  fmla.2d v10, v6, v7
  ldp x6, x7, [x4, #16]
  add.2d v5, v5, v9
  ldp x9, x10, [x5, #0]
  add.2d v2, v2, v10
  ldp x5, x11, [x5, #16]
  mov x12, #51712
  mul x13, x1, x9
  umulh x14, x1, x9
  movk x12, #16093, lsl 16
  mul x15, x3, x9
  movk x12, #30633, lsl 32
  umulh x16, x3, x9
  movk x12, #17068, lsl 48
  adds x14, x15, x14
  cinc x15, x16, hs
  dup.2d v7, x12
  mul x12, x6, x9
  mov.16b v9, v21
  umulh x16, x6, x9
  fmla.2d v9, v6, v7
  adds x12, x12, x15
  cinc x15, x16, hs
  fsub.2d v10, v22, v9
  mul x16, x7, x9
  umulh x9, x7, x9
  fmla.2d v10, v6, v7
  adds x15, x16, x15
  cinc x9, x9, hs
  add.2d v3, v3, v9
  mul x16, x1, x10
  add.2d v5, v5, v10
  umulh x17, x1, x10
  ucvtf.2d v6, v8
  adds x14, x16, x14
  cinc x16, x17, hs
  mov x17, #34724
  mul x20, x3, x10
  movk x17, #40393, lsl 16
  umulh x21, x3, x10
  movk x17, #23752, lsl 32
  adds x16, x20, x16
  cinc x20, x21, hs
  adds x12, x16, x12
  cinc x16, x20, hs
  movk x17, #17184, lsl 48
  mul x20, x6, x10
  dup.2d v7, x17
  umulh x17, x6, x10
  mov.16b v8, v21
  adds x16, x20, x16
  cinc x17, x17, hs
  fmla.2d v8, v6, v7
  adds x15, x16, x15
  cinc x16, x17, hs
  fsub.2d v9, v22, v8
  mul x17, x7, x10
  fmla.2d v9, v6, v7
  umulh x10, x7, x10
  add.2d v0, v0, v8
  adds x16, x17, x16
  cinc x10, x10, hs
  adds x9, x16, x9
  cinc x10, x10, hs
  add.2d v4, v4, v9
  mul x16, x1, x5
  mov x17, #25532
  umulh x20, x1, x5
  movk x17, #31025, lsl 16
  adds x12, x16, x12
  cinc x16, x20, hs
  movk x17, #10002, lsl 32
  mul x20, x3, x5
  movk x17, #17199, lsl 48
  umulh x21, x3, x5
  dup.2d v7, x17
  adds x16, x20, x16
  cinc x17, x21, hs
  mov.16b v8, v21
  adds x15, x16, x15
  cinc x16, x17, hs
  mul x17, x6, x5
  fmla.2d v8, v6, v7
  umulh x20, x6, x5
  fsub.2d v9, v22, v8
  adds x16, x17, x16
  cinc x17, x20, hs
  fmla.2d v9, v6, v7
  adds x9, x16, x9
  cinc x16, x17, hs
  add.2d v1, v1, v8
  mul x17, x7, x5
  add.2d v0, v0, v9
  umulh x5, x7, x5
  mov x20, #18830
  adds x16, x17, x16
  cinc x5, x5, hs
  movk x20, #2465, lsl 16
  adds x10, x16, x10
  cinc x5, x5, hs
  mul x16, x1, x11
  movk x20, #36348, lsl 32
  umulh x1, x1, x11
  movk x20, #17194, lsl 48
  adds x15, x16, x15
  cinc x1, x1, hs
  dup.2d v7, x20
  mul x16, x3, x11
  mov.16b v8, v21
  umulh x3, x3, x11
  fmla.2d v8, v6, v7
  adds x1, x16, x1
  cinc x3, x3, hs
  fsub.2d v9, v22, v8
  adds x1, x1, x9
  cinc x3, x3, hs
  fmla.2d v9, v6, v7
  mul x9, x6, x11
  umulh x6, x6, x11
  add.2d v2, v2, v8
  adds x3, x9, x3
  cinc x6, x6, hs
  add.2d v1, v1, v9
  adds x3, x3, x10
  cinc x6, x6, hs
  mov x9, #21566
  mul x10, x7, x11
  movk x9, #43708, lsl 16
  umulh x7, x7, x11
  movk x9, #57685, lsl 32
  adds x6, x10, x6
  cinc x7, x7, hs
  movk x9, #17185, lsl 48
  adds x5, x6, x5
  cinc x6, x7, hs
  dup.2d v7, x9
  mov x7, #48718
  movk x7, #4732, lsl 16
  mov.16b v8, v21
  movk x7, #45078, lsl 32
  fmla.2d v8, v6, v7
  movk x7, #39852, lsl 48
  fsub.2d v9, v22, v8
  mov x9, #16676
  fmla.2d v9, v6, v7
  movk x9, #12692, lsl 16
  add.2d v5, v5, v8
  movk x9, #20986, lsl 32
  add.2d v2, v2, v9
  movk x9, #2848, lsl 48
  mov x10, #3072
  mov x11, #51052
  movk x11, #24721, lsl 16
  movk x10, #8058, lsl 16
  movk x11, #61092, lsl 32
  movk x10, #46097, lsl 32
  movk x11, #45156, lsl 48
  movk x10, #17047, lsl 48
  mov x16, #3197
  dup.2d v7, x10
  movk x16, #18936, lsl 16
  mov.16b v8, v21
  movk x16, #10922, lsl 32
  fmla.2d v8, v6, v7
  movk x16, #11014, lsl 48
  fsub.2d v9, v22, v8
  mul x10, x7, x13
  umulh x7, x7, x13
  fmla.2d v9, v6, v7
  adds x10, x10, x15
  cinc x7, x7, hs
  add.2d v3, v3, v8
  mul x15, x9, x13
  add.2d v5, v5, v9
  umulh x9, x9, x13
  mov x17, #65535
  adds x7, x15, x7
  cinc x9, x9, hs
  movk x17, #61439, lsl 16
  adds x1, x7, x1
  cinc x7, x9, hs
  movk x17, #62867, lsl 32
  mul x9, x11, x13
  movk x17, #1, lsl 48
  umulh x11, x11, x13
  adds x7, x9, x7
  cinc x9, x11, hs
  umov x11, v4.d[0]
  adds x3, x7, x3
  cinc x7, x9, hs
  umov x9, v4.d[1]
  mul x15, x16, x13
  mul x11, x11, x17
  umulh x13, x16, x13
  mul x9, x9, x17
  adds x7, x15, x7
  cinc x13, x13, hs
  and x11, x11, x8
  adds x5, x7, x5
  cinc x7, x13, hs
  and x8, x9, x8
  add x6, x6, x7
  ins v6.d[0], x11
  ins v6.d[1], x8
  mov x7, #56431
  movk x7, #30457, lsl 16
  ucvtf.2d v6, v6
  movk x7, #30012, lsl 32
  mov x8, #16
  movk x7, #6382, lsl 48
  movk x8, #22847, lsl 32
  mov x9, #59151
  movk x8, #17151, lsl 48
  movk x9, #41769, lsl 16
  dup.2d v7, x8
  movk x9, #32276, lsl 32
  mov.16b v8, v21
  movk x9, #21677, lsl 48
  fmla.2d v8, v6, v7
  mov x8, #34015
  movk x8, #20342, lsl 16
  fsub.2d v9, v22, v8
  movk x8, #13935, lsl 32
  fmla.2d v9, v6, v7
  movk x8, #11030, lsl 48
  add.2d v0, v0, v8
  mov x11, #13689
  add.2d v4, v4, v9
  movk x11, #8159, lsl 16
  mov x13, #20728
  movk x11, #215, lsl 32
  movk x13, #23588, lsl 16
  movk x11, #4913, lsl 48
  movk x13, #7790, lsl 32
  mul x15, x7, x14
  umulh x7, x7, x14
  movk x13, #17170, lsl 48
  adds x10, x15, x10
  cinc x7, x7, hs
  dup.2d v7, x13
  mul x13, x9, x14
  mov.16b v8, v21
  umulh x9, x9, x14
  fmla.2d v8, v6, v7
  adds x7, x13, x7
  cinc x9, x9, hs
  fsub.2d v9, v22, v8
  adds x1, x7, x1
  cinc x7, x9, hs
  fmla.2d v9, v6, v7
  mul x9, x8, x14
  add.2d v1, v1, v8
  umulh x8, x8, x14
  adds x7, x9, x7
  cinc x8, x8, hs
  add.2d v0, v0, v9
  adds x3, x7, x3
  cinc x7, x8, hs
  mov x8, #16000
  mul x9, x11, x14
  movk x8, #53891, lsl 16
  umulh x11, x11, x14
  movk x8, #5509, lsl 32
  adds x7, x9, x7
  cinc x9, x11, hs
  movk x8, #17144, lsl 48
  adds x5, x7, x5
  cinc x7, x9, hs
  dup.2d v7, x8
  add x6, x6, x7
  mov.16b v8, v21
  mov x7, #61005
  movk x7, #58262, lsl 16
  fmla.2d v8, v6, v7
  movk x7, #32851, lsl 32
  fsub.2d v9, v22, v8
  movk x7, #11582, lsl 48
  fmla.2d v9, v6, v7
  mov x8, #37581
  add.2d v2, v2, v8
  movk x8, #43836, lsl 16
  add.2d v1, v1, v9
  movk x8, #36286, lsl 32
  mov x9, #46800
  movk x8, #51783, lsl 48
  movk x9, #2568, lsl 16
  mov x11, #10899
  movk x11, #30709, lsl 16
  movk x9, #1335, lsl 32
  movk x11, #61551, lsl 32
  movk x9, #17188, lsl 48
  movk x11, #45784, lsl 48
  dup.2d v7, x9
  mov x9, #36612
  mov.16b v8, v21
  movk x9, #63402, lsl 16
  fmla.2d v8, v6, v7
  movk x9, #47623, lsl 32
  fsub.2d v9, v22, v8
  movk x9, #9430, lsl 48
  fmla.2d v9, v6, v7
  mul x13, x7, x12
  umulh x7, x7, x12
  add.2d v5, v5, v8
  adds x10, x13, x10
  cinc x7, x7, hs
  add.2d v2, v2, v9
  mul x13, x8, x12
  mov x14, #39040
  umulh x8, x8, x12
  movk x14, #14704, lsl 16
  adds x7, x13, x7
  cinc x8, x8, hs
  movk x14, #12839, lsl 32
  adds x1, x7, x1
  cinc x7, x8, hs
  movk x14, #17096, lsl 48
  mul x8, x11, x12
  dup.2d v7, x14
  umulh x11, x11, x12
  adds x7, x8, x7
  cinc x8, x11, hs
  mov.16b v8, v21
  adds x3, x7, x3
  cinc x7, x8, hs
  fmla.2d v8, v6, v7
  mul x8, x9, x12
  fsub.2d v9, v22, v8
  umulh x9, x9, x12
  fmla.2d v9, v6, v7
  adds x7, x8, x7
  cinc x8, x9, hs
  add.2d v3, v3, v8
  adds x5, x7, x5
  cinc x7, x8, hs
  add.2d v5, v5, v9
  add x6, x6, x7
  mov x7, #140737488355328
  mov x8, #65535
  movk x8, #61439, lsl 16
  dup.2d v6, x7
  movk x8, #62867, lsl 32
  and.16b v6, v3, v6
  movk x8, #49889, lsl 48
  cmeq.2d v6, v6, #0
  mul x7, x8, x10
  mov x8, #2
  mov x9, #1
  movk x8, #57344, lsl 16
  movk x9, #61440, lsl 16
  movk x8, #60199, lsl 32
  movk x9, #62867, lsl 32
  movk x8, #3, lsl 48
  movk x9, #17377, lsl 48
  mov x11, #28817
  dup.2d v7, x8
  movk x11, #31161, lsl 16
  bic.16b v7, v7, v6
  movk x11, #59464, lsl 32
  mov x8, #10364
  movk x11, #10291, lsl 48
  movk x8, #11794, lsl 16
  mov x12, #22621
  movk x8, #3895, lsl 32
  movk x12, #33153, lsl 16
  movk x8, #9, lsl 48
  movk x12, #17846, lsl 32
  dup.2d v8, x8
  movk x12, #47184, lsl 48
  mov x8, #41001
  bic.16b v8, v8, v6
  movk x8, #57649, lsl 16
  mov x13, #26576
  movk x8, #20082, lsl 32
  movk x13, #47696, lsl 16
  movk x8, #12388, lsl 48
  movk x13, #688, lsl 32
  mul x14, x9, x7
  movk x13, #3, lsl 48
  umulh x9, x9, x7
  dup.2d v9, x13
  cmn x14, x10
  cinc x9, x9, hs
  bic.16b v9, v9, v6
  mul x10, x11, x7
  umulh x11, x11, x7
  mov x13, #46800
  adds x9, x10, x9
  cinc x10, x11, hs
  movk x13, #2568, lsl 16
  adds x1, x9, x1
  cinc x9, x10, hs
  movk x13, #1335, lsl 32
  mul x10, x12, x7
  movk x13, #4, lsl 48
  umulh x11, x12, x7
  dup.2d v10, x13
  adds x9, x10, x9
  cinc x10, x11, hs
  bic.16b v10, v10, v6
  adds x3, x9, x3
  cinc x9, x10, hs
  mov x10, #49763
  mul x11, x8, x7
  umulh x7, x8, x7
  movk x10, #40165, lsl 16
  adds x8, x11, x9
  cinc x7, x7, hs
  movk x10, #24776, lsl 32
  adds x5, x8, x5
  cinc x7, x7, hs
  dup.2d v11, x10
  add x6, x6, x7
  bic.16b v6, v11, v6
  mov x7, #2
  sub.2d v0, v0, v7
  movk x7, #57344, lsl 16
  ssra.2d v0, v4, #52
  movk x7, #60199, lsl 32
  sub.2d v4, v1, v8
  movk x7, #34755, lsl 48
  mov x8, #57634
  ssra.2d v4, v0, #52
  movk x8, #62322, lsl 16
  sub.2d v7, v2, v9
  movk x8, #53392, lsl 32
  ssra.2d v7, v4, #52
  movk x8, #20583, lsl 48
  sub.2d v5, v5, v10
  mov x9, #45242
  ssra.2d v5, v7, #52
  movk x9, #770, lsl 16
  sub.2d v6, v3, v6
  movk x9, #35693, lsl 32
  ssra.2d v6, v5, #52
  movk x9, #28832, lsl 48
  mov x10, #16467
  ushr.2d v1, v4, #12
  movk x10, #49763, lsl 16
  ushr.2d v2, v7, #24
  movk x10, #40165, lsl 32
  ushr.2d v3, v5, #36
  movk x10, #24776, lsl 48
  sli.2d v0, v4, #52
  subs x7, x1, x7
  sbcs x8, x3, x8
  sbcs x9, x5, x9
  sbcs x10, x6, x10
  sli.2d v1, v7, #40
  tst x6, #9223372036854775808
  csel x1, x7, x1, mi
  csel x3, x8, x3, mi
  csel x5, x9, x5, mi
  csel x6, x10, x6, mi
  sli.2d v2, v5, #28
  stp x1, x3, [x4, #0]
  stp x5, x6, [x4, #16]
  sli.2d v3, v6, #16
  ret