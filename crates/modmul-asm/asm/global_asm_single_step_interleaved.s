//in("x0") in0[0], in("x1") in0[1], in("x2") in0[2], in("x3") in0[3], in("x4") in1[0], in("x5") in1[1], in("x6") in1[2], in("x7") in1[3], in("v0") in2[0], in("v1") in2[1], in("v2") in2[2], in("v3") in2[3], in("v4") in3[0], in("v5") in3[1], in("v6") in3[2], in("v7") in3[3],
//lateout("x1") out0[0], lateout("x4") out0[1], lateout("x2") out0[2], lateout("x0") out0[3], lateout("v0") out1[0], lateout("v5") out1[1], lateout("v6") out1[2], lateout("v7") out1[3],
//lateout("v1") _, lateout("v2") _, lateout("x3") _, lateout("v3") _, lateout("v4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _, lateout("x8") _, lateout("v8") _, lateout("x9") _, lateout("v9") _, lateout("x10") _, lateout("v10") _, lateout("x11") _, lateout("v11") _, lateout("x12") _, lateout("v12") _, lateout("x13") _, lateout("v13") _, lateout("x14") _, lateout("v14") _, lateout("x15") _, lateout("v15") _, lateout("x16") _, lateout("v16") _, lateout("v17") _, lateout("v18") _, lateout("v19") _, lateout("v20") _, lateout("v21") _, lateout("v22") _, lateout("v23") _, lateout("v24") _,
//lateout("lr") _
.global _single_step_interleaved
.align 4
.text
_single_step_interleaved:
  mov x8, #4503599627370495
  dup.2d v8, x8
  mul x9, x0, x4
  shl.2d v9, v1, #14
  shl.2d v10, v2, #26
  shl.2d v11, v3, #38
  umulh x10, x0, x4
  shl.2d v12, v0, #2
  usra.2d v9, v0, #50
  usra.2d v10, v1, #38
  mul x11, x1, x4
  usra.2d v11, v2, #26
  and.16b v0, v12, v8
  umulh x12, x1, x4
  and.16b v1, v9, v8
  and.16b v2, v10, v8
  and.16b v9, v11, v8
  adds x11, x11, x10
  cinc x12, x12, hs
  ushr.2d v3, v3, #14
  shl.2d v10, v5, #14
  shl.2d v11, v6, #26
  mul x10, x2, x4
  shl.2d v12, v7, #38
  shl.2d v13, v4, #2
  umulh x13, x2, x4
  usra.2d v10, v4, #50
  usra.2d v11, v5, #38
  usra.2d v12, v6, #26
  adds x10, x10, x12
  cinc x13, x13, hs
  and.16b v4, v13, v8
  and.16b v5, v10, v8
  and.16b v6, v11, v8
  mul x12, x3, x4
  and.16b v10, v12, v8
  ushr.2d v7, v7, #14
  mov x14, #13605374474286268416
  umulh x4, x3, x4
  dup.2d v11, x14
  mov x14, #6440147467139809280
  adds x12, x12, x13
  cinc x4, x4, hs
  dup.2d v12, x14
  mov x13, #3688448094816436224
  dup.2d v13, x13
  mul x13, x0, x5
  mov x14, #9209861237972664320
  dup.2d v14, x14
  mov x14, #12218265789056155648
  umulh x15, x0, x5
  dup.2d v15, x14
  mov x14, #17739678932212383744
  adds x13, x13, x11
  cinc x15, x15, hs
  dup.2d v16, x14
  mov x11, #2301339409586323456
  dup.2d v17, x11
  mul x11, x1, x5
  mov x14, #7822752552742551552
  dup.2d v18, x14
  mov x14, #5071053180419178496
  umulh x16, x1, x5
  dup.2d v19, x14
  mov x14, #16352570246982270976
  adds x11, x11, x15
  cinc x16, x16, hs
  dup.2d v20, x14
  mov x14, #5075556780046548992
  dup.2d v21, x14
  adds x11, x11, x10
  cinc x16, x16, hs
  mov x10, #1
  movk x10, #18032, lsl 48
  dup.2d v22, x10
  mul x10, x2, x5
  ucvtf.2d v0, v0
  ucvtf.2d v1, v1
  ucvtf.2d v2, v2
  umulh x14, x2, x5
  ucvtf.2d v9, v9
  ucvtf.2d v3, v3
  adds x10, x10, x16
  cinc x14, x14, hs
  ucvtf.2d v4, v4
  ucvtf.2d v5, v5
  ucvtf.2d v6, v6
  adds x10, x10, x12
  cinc x14, x14, hs
  ucvtf.2d v10, v10
  ucvtf.2d v7, v7
  mov.16b v23, v21
  mul x12, x3, x5
  fmla.2d v23, v0, v4
  fsub.2d v24, v22, v23
  umulh x5, x3, x5
  fmla.2d v24, v0, v4
  add.2d v13, v13, v23
  add.2d v11, v11, v24
  adds x12, x12, x14
  cinc x5, x5, hs
  mov.16b v23, v21
  fmla.2d v23, v0, v5
  fsub.2d v24, v22, v23
  adds x12, x12, x4
  cinc x5, x5, hs
  fmla.2d v24, v0, v5
  add.2d v15, v15, v23
  mul x4, x0, x6
  add.2d v13, v13, v24
  mov.16b v23, v21
  fmla.2d v23, v0, v6
  umulh x14, x0, x6
  fsub.2d v24, v22, v23
  fmla.2d v24, v0, v6
  add.2d v17, v17, v23
  adds x4, x4, x11
  cinc x14, x14, hs
  add.2d v15, v15, v24
  mov.16b v23, v21
  fmla.2d v23, v0, v10
  mul x11, x1, x6
  fsub.2d v24, v22, v23
  fmla.2d v24, v0, v10
  umulh x15, x1, x6
  add.2d v19, v19, v23
  add.2d v17, v17, v24
  mov.16b v23, v21
  adds x11, x11, x14
  cinc x15, x15, hs
  fmla.2d v23, v0, v7
  fsub.2d v24, v22, v23
  fmla.2d v24, v0, v7
  adds x11, x11, x10
  cinc x15, x15, hs
  add.2d v0, v20, v23
  add.2d v19, v19, v24
  mul x10, x2, x6
  mov.16b v20, v21
  fmla.2d v20, v1, v4
  fsub.2d v23, v22, v20
  umulh x14, x2, x6
  fmla.2d v23, v1, v4
  add.2d v15, v15, v20
  add.2d v13, v13, v23
  adds x10, x10, x15
  cinc x14, x14, hs
  mov.16b v20, v21
  fmla.2d v20, v1, v5
  adds x10, x10, x12
  cinc x14, x14, hs
  fsub.2d v23, v22, v20
  fmla.2d v23, v1, v5
  add.2d v17, v17, v20
  mul x12, x3, x6
  add.2d v15, v15, v23
  mov.16b v20, v21
  fmla.2d v20, v1, v6
  umulh x6, x3, x6
  fsub.2d v23, v22, v20
  fmla.2d v23, v1, v6
  add.2d v19, v19, v20
  adds x12, x12, x14
  cinc x6, x6, hs
  add.2d v17, v17, v23
  mov.16b v20, v21
  adds x12, x12, x5
  cinc x6, x6, hs
  fmla.2d v20, v1, v10
  fsub.2d v23, v22, v20
  fmla.2d v23, v1, v10
  mul x5, x0, x7
  add.2d v0, v0, v20
  add.2d v19, v19, v23
  mov.16b v20, v21
  umulh x0, x0, x7
  fmla.2d v20, v1, v7
  fsub.2d v23, v22, v20
  adds x5, x5, x11
  cinc x0, x0, hs
  fmla.2d v23, v1, v7
  add.2d v1, v18, v20
  add.2d v0, v0, v23
  mul x11, x1, x7
  mov.16b v18, v21
  fmla.2d v18, v2, v4
  fsub.2d v20, v22, v18
  umulh x1, x1, x7
  fmla.2d v20, v2, v4
  add.2d v17, v17, v18
  adds x11, x11, x0
  cinc x1, x1, hs
  add.2d v15, v15, v20
  mov.16b v18, v21
  fmla.2d v18, v2, v5
  adds x11, x11, x10
  cinc x1, x1, hs
  fsub.2d v20, v22, v18
  fmla.2d v20, v2, v5
  add.2d v18, v19, v18
  mul x0, x2, x7
  add.2d v17, v17, v20
  mov.16b v19, v21
  fmla.2d v19, v2, v6
  umulh x2, x2, x7
  fsub.2d v20, v22, v19
  fmla.2d v20, v2, v6
  adds x0, x0, x1
  cinc x2, x2, hs
  add.2d v0, v0, v19
  add.2d v18, v18, v20
  mov.16b v19, v21
  adds x0, x0, x12
  cinc x2, x2, hs
  fmla.2d v19, v2, v10
  fsub.2d v20, v22, v19
  fmla.2d v20, v2, v10
  mul x1, x3, x7
  add.2d v1, v1, v19
  add.2d v0, v0, v20
  umulh x3, x3, x7
  mov.16b v19, v21
  fmla.2d v19, v2, v7
  fsub.2d v20, v22, v19
  adds x1, x1, x2
  cinc x3, x3, hs
  fmla.2d v20, v2, v7
  add.2d v2, v16, v19
  add.2d v1, v1, v20
  adds x1, x1, x6
  cinc x3, x3, hs
  mov.16b v16, v21
  fmla.2d v16, v9, v4
  mov x2, #48718
  fsub.2d v19, v22, v16
  fmla.2d v19, v9, v4
  add.2d v16, v18, v16
  movk x2, #4732, lsl 16
  add.2d v17, v17, v19
  mov.16b v18, v21
  fmla.2d v18, v9, v5
  movk x2, #45078, lsl 32
  fsub.2d v19, v22, v18
  fmla.2d v19, v9, v5
  add.2d v0, v0, v18
  movk x2, #39852, lsl 48
  add.2d v16, v16, v19
  mov.16b v18, v21
  mov x6, #16676
  fmla.2d v18, v9, v6
  fsub.2d v19, v22, v18
  fmla.2d v19, v9, v6
  movk x6, #12692, lsl 16
  add.2d v1, v1, v18
  add.2d v0, v0, v19
  mov.16b v18, v21
  movk x6, #20986, lsl 32
  fmla.2d v18, v9, v10
  fsub.2d v19, v22, v18
  movk x6, #2848, lsl 48
  fmla.2d v19, v9, v10
  add.2d v2, v2, v18
  add.2d v1, v1, v19
  mov x7, #51052
  mov.16b v18, v21
  fmla.2d v18, v9, v7
  fsub.2d v19, v22, v18
  movk x7, #24721, lsl 16
  fmla.2d v19, v9, v7
  add.2d v9, v14, v18
  movk x7, #61092, lsl 32
  add.2d v2, v2, v19
  mov.16b v14, v21
  fmla.2d v14, v3, v4
  movk x7, #45156, lsl 48
  fsub.2d v18, v22, v14
  fmla.2d v18, v3, v4
  add.2d v0, v0, v14
  mov x10, #3197
  add.2d v4, v16, v18
  mov.16b v14, v21
  fmla.2d v14, v3, v5
  movk x10, #18936, lsl 16
  fsub.2d v16, v22, v14
  fmla.2d v16, v3, v5
  movk x10, #10922, lsl 32
  add.2d v1, v1, v14
  add.2d v0, v0, v16
  mov.16b v5, v21
  movk x10, #11014, lsl 48
  fmla.2d v5, v3, v6
  fsub.2d v14, v22, v5
  fmla.2d v14, v3, v6
  mul x12, x2, x9
  add.2d v2, v2, v5
  add.2d v1, v1, v14
  umulh x2, x2, x9
  mov.16b v5, v21
  fmla.2d v5, v3, v10
  fsub.2d v6, v22, v5
  adds x12, x12, x5
  cinc x2, x2, hs
  fmla.2d v6, v3, v10
  add.2d v5, v9, v5
  add.2d v2, v2, v6
  mul x5, x6, x9
  mov.16b v6, v21
  fmla.2d v6, v3, v7
  umulh x6, x6, x9
  fsub.2d v9, v22, v6
  fmla.2d v9, v3, v7
  add.2d v3, v12, v6
  adds x5, x5, x2
  cinc x6, x6, hs
  add.2d v5, v5, v9
  usra.2d v13, v11, #52
  usra.2d v15, v13, #52
  adds x5, x5, x11
  cinc x6, x6, hs
  usra.2d v17, v15, #52
  usra.2d v4, v17, #52
  and.16b v6, v11, v8
  mul x2, x7, x9
  and.16b v7, v13, v8
  and.16b v9, v15, v8
  umulh x7, x7, x9
  and.16b v8, v17, v8
  ucvtf.2d v6, v6
  mov x11, #37864
  adds x2, x2, x6
  cinc x7, x7, hs
  movk x11, #1815, lsl 16
  movk x11, #28960, lsl 32
  movk x11, #17153, lsl 48
  adds x2, x2, x0
  cinc x7, x7, hs
  dup.2d v10, x11
  mov.16b v11, v21
  mul x0, x10, x9
  fmla.2d v11, v6, v10
  fsub.2d v12, v22, v11
  fmla.2d v12, v6, v10
  umulh x6, x10, x9
  add.2d v0, v0, v11
  add.2d v4, v4, v12
  mov x9, #46128
  adds x0, x0, x7
  cinc x6, x6, hs
  movk x9, #29964, lsl 16
  movk x9, #7587, lsl 32
  adds x0, x0, x1
  cinc x6, x6, hs
  movk x9, #17161, lsl 48
  dup.2d v10, x9
  mov.16b v11, v21
  add x1, x3, x6
  fmla.2d v11, v6, v10
  fsub.2d v12, v22, v11
  fmla.2d v12, v6, v10
  mov x3, #56431
  add.2d v1, v1, v11
  add.2d v0, v0, v12
  mov x6, #52826
  movk x3, #30457, lsl 16
  movk x6, #57790, lsl 16
  movk x6, #55431, lsl 32
  movk x3, #30012, lsl 32
  movk x6, #17196, lsl 48
  dup.2d v10, x6
  mov.16b v11, v21
  movk x3, #6382, lsl 48
  fmla.2d v11, v6, v10
  fsub.2d v12, v22, v11
  fmla.2d v12, v6, v10
  mov x6, #59151
  add.2d v2, v2, v11
  add.2d v1, v1, v12
  movk x6, #41769, lsl 16
  mov x7, #31276
  movk x7, #21262, lsl 16
  movk x7, #2304, lsl 32
  movk x6, #32276, lsl 32
  movk x7, #17182, lsl 48
  dup.2d v10, x7
  mov.16b v11, v21
  movk x6, #21677, lsl 48
  fmla.2d v11, v6, v10
  fsub.2d v12, v22, v11
  mov x7, #34015
  fmla.2d v12, v6, v10
  add.2d v5, v5, v11
  add.2d v2, v2, v12
  movk x7, #20342, lsl 16
  mov x9, #28672
  movk x9, #24515, lsl 16
  movk x9, #54929, lsl 32
  movk x7, #13935, lsl 32
  movk x9, #17064, lsl 48
  dup.2d v10, x9
  mov.16b v11, v21
  movk x7, #11030, lsl 48
  fmla.2d v11, v6, v10
  fsub.2d v12, v22, v11
  mov x9, #13689
  fmla.2d v12, v6, v10
  add.2d v3, v3, v11
  add.2d v5, v5, v12
  movk x9, #8159, lsl 16
  ucvtf.2d v6, v7
  mov x10, #44768
  movk x10, #51919, lsl 16
  movk x9, #215, lsl 32
  movk x10, #6346, lsl 32
  movk x10, #17133, lsl 48
  movk x9, #4913, lsl 48
  dup.2d v7, x10
  mov.16b v10, v21
  fmla.2d v10, v6, v7
  mul x10, x3, x13
  fsub.2d v11, v22, v10
  fmla.2d v11, v6, v7
  add.2d v0, v0, v10
  umulh x3, x3, x13
  add.2d v4, v4, v11
  mov x11, #47492
  adds x10, x10, x12
  cinc x3, x3, hs
  movk x11, #23630, lsl 16
  movk x11, #49985, lsl 32
  movk x11, #17168, lsl 48
  mul x12, x6, x13
  dup.2d v7, x11
  mov.16b v10, v21
  fmla.2d v10, v6, v7
  umulh x6, x6, x13
  fsub.2d v11, v22, v10
  fmla.2d v11, v6, v7
  add.2d v1, v1, v10
  adds x12, x12, x3
  cinc x6, x6, hs
  add.2d v0, v0, v11
  mov x3, #57936
  adds x12, x12, x5
  cinc x6, x6, hs
  movk x3, #54828, lsl 16
  movk x3, #18292, lsl 32
  movk x3, #17197, lsl 48
  mul x5, x7, x13
  dup.2d v7, x3
  mov.16b v10, v21
  fmla.2d v10, v6, v7
  umulh x3, x7, x13
  fsub.2d v11, v22, v10
  fmla.2d v11, v6, v7
  adds x5, x5, x6
  cinc x3, x3, hs
  add.2d v2, v2, v10
  add.2d v1, v1, v11
  mov x6, #17708
  adds x5, x5, x2
  cinc x3, x3, hs
  movk x6, #43915, lsl 16
  movk x6, #64348, lsl 32
  movk x6, #17188, lsl 48
  mul x2, x9, x13
  dup.2d v7, x6
  mov.16b v10, v21
  umulh x6, x9, x13
  fmla.2d v10, v6, v7
  fsub.2d v11, v22, v10
  fmla.2d v11, v6, v7
  adds x2, x2, x3
  cinc x6, x6, hs
  add.2d v5, v5, v10
  add.2d v2, v2, v11
  mov x3, #29184
  adds x2, x2, x0
  cinc x6, x6, hs
  movk x3, #20789, lsl 16
  movk x3, #19197, lsl 32
  movk x3, #17083, lsl 48
  add x0, x1, x6
  dup.2d v7, x3
  mov.16b v10, v21
  mov x1, #61005
  fmla.2d v10, v6, v7
  fsub.2d v11, v22, v10
  fmla.2d v11, v6, v7
  movk x1, #58262, lsl 16
  add.2d v3, v3, v10
  add.2d v5, v5, v11
  ucvtf.2d v6, v9
  movk x1, #32851, lsl 32
  mov x3, #58856
  movk x3, #14953, lsl 16
  movk x1, #11582, lsl 48
  movk x3, #15155, lsl 32
  movk x3, #17181, lsl 48
  dup.2d v7, x3
  mov x3, #37581
  mov.16b v9, v21
  fmla.2d v9, v6, v7
  fsub.2d v10, v22, v9
  movk x3, #43836, lsl 16
  fmla.2d v10, v6, v7
  add.2d v0, v0, v9
  movk x3, #36286, lsl 32
  add.2d v4, v4, v10
  mov x6, #35392
  movk x6, #12477, lsl 16
  movk x3, #51783, lsl 48
  movk x6, #56780, lsl 32
  movk x6, #17142, lsl 48
  dup.2d v7, x6
  mov x6, #10899
  mov.16b v9, v21
  fmla.2d v9, v6, v7
  fsub.2d v10, v22, v9
  movk x6, #30709, lsl 16
  fmla.2d v10, v6, v7
  add.2d v1, v1, v9
  movk x6, #61551, lsl 32
  add.2d v0, v0, v10
  mov x7, #9848
  movk x7, #54501, lsl 16
  movk x6, #45784, lsl 48
  movk x7, #31540, lsl 32
  movk x7, #17170, lsl 48
  dup.2d v7, x7
  mov x7, #36612
  mov.16b v9, v21
  fmla.2d v9, v6, v7
  movk x7, #63402, lsl 16
  fsub.2d v10, v22, v9
  fmla.2d v10, v6, v7
  add.2d v2, v2, v9
  movk x7, #47623, lsl 32
  add.2d v1, v1, v10
  mov x9, #9584
  movk x9, #63883, lsl 16
  movk x7, #9430, lsl 48
  movk x9, #18253, lsl 32
  movk x9, #17190, lsl 48
  mul x11, x1, x4
  dup.2d v7, x9
  mov.16b v9, v21
  fmla.2d v9, v6, v7
  umulh x1, x1, x4
  fsub.2d v10, v22, v9
  fmla.2d v10, v6, v7
  add.2d v5, v5, v9
  adds x11, x11, x10
  cinc x1, x1, hs
  add.2d v2, v2, v10
  mov x9, #51712
  movk x9, #16093, lsl 16
  mul x10, x3, x4
  movk x9, #30633, lsl 32
  movk x9, #17068, lsl 48
  umulh x3, x3, x4
  dup.2d v7, x9
  mov.16b v9, v21
  fmla.2d v9, v6, v7
  adds x10, x10, x1
  cinc x3, x3, hs
  fsub.2d v10, v22, v9
  fmla.2d v10, v6, v7
  add.2d v3, v3, v9
  adds x10, x10, x12
  cinc x3, x3, hs
  add.2d v5, v5, v10
  ucvtf.2d v6, v8
  mul x1, x6, x4
  mov x9, #34724
  movk x9, #40393, lsl 16
  movk x9, #23752, lsl 32
  umulh x6, x6, x4
  movk x9, #17184, lsl 48
  dup.2d v7, x9
  mov.16b v8, v21
  adds x1, x1, x3
  cinc x6, x6, hs
  fmla.2d v8, v6, v7
  fsub.2d v9, v22, v8
  adds x1, x1, x5
  cinc x6, x6, hs
  fmla.2d v9, v6, v7
  add.2d v0, v0, v8
  add.2d v4, v4, v9
  mul x3, x7, x4
  mov x5, #25532
  movk x5, #31025, lsl 16
  movk x5, #10002, lsl 32
  umulh x4, x7, x4
  movk x5, #17199, lsl 48
  dup.2d v7, x5
  mov.16b v8, v21
  adds x3, x3, x6
  cinc x4, x4, hs
  fmla.2d v8, v6, v7
  fsub.2d v9, v22, v8
  adds x3, x3, x2
  cinc x4, x4, hs
  fmla.2d v9, v6, v7
  add.2d v1, v1, v8
  add.2d v0, v0, v9
  add x0, x0, x4
  mov x2, #18830
  movk x2, #2465, lsl 16
  movk x2, #36348, lsl 32
  mov x4, #65535
  movk x2, #17194, lsl 48
  dup.2d v7, x2
  movk x4, #61439, lsl 16
  mov.16b v8, v21
  fmla.2d v8, v6, v7
  fsub.2d v9, v22, v8
  movk x4, #62867, lsl 32
  fmla.2d v9, v6, v7
  add.2d v2, v2, v8
  add.2d v1, v1, v9
  movk x4, #49889, lsl 48
  mov x2, #21566
  movk x2, #43708, lsl 16
  mul x4, x4, x11
  movk x2, #57685, lsl 32
  movk x2, #17185, lsl 48
  dup.2d v7, x2
  mov x2, #1
  mov.16b v8, v21
  fmla.2d v8, v6, v7
  fsub.2d v9, v22, v8
  movk x2, #61440, lsl 16
  fmla.2d v9, v6, v7
  add.2d v5, v5, v8
  add.2d v2, v2, v9
  movk x2, #62867, lsl 32
  mov x5, #3072
  movk x5, #8058, lsl 16
  movk x2, #17377, lsl 48
  movk x5, #46097, lsl 32
  movk x5, #17047, lsl 48
  dup.2d v7, x5
  mov x5, #28817
  mov.16b v8, v21
  fmla.2d v8, v6, v7
  fsub.2d v9, v22, v8
  movk x5, #31161, lsl 16
  fmla.2d v9, v6, v7
  add.2d v3, v3, v8
  movk x5, #59464, lsl 32
  add.2d v5, v5, v9
  mov x6, #65535
  movk x6, #61439, lsl 16
  movk x5, #10291, lsl 48
  movk x6, #62867, lsl 32
  movk x6, #1, lsl 48
  umov x7, v4.d[0]
  mov x9, #22621
  umov x12, v4.d[1]
  mul x7, x7, x6
  movk x9, #33153, lsl 16
  mul x6, x12, x6
  and x7, x7, x8
  and x6, x6, x8
  movk x9, #17846, lsl 32
  ins v6.d[0], x7
  ins v6.d[1], x6
  ucvtf.2d v6, v6
  mov x6, #16
  movk x9, #47184, lsl 48
  movk x6, #22847, lsl 32
  movk x6, #17151, lsl 48
  dup.2d v7, x6
  mov x6, #41001
  mov.16b v8, v21
  fmla.2d v8, v6, v7
  movk x6, #57649, lsl 16
  fsub.2d v9, v22, v8
  fmla.2d v9, v6, v7
  add.2d v0, v0, v8
  movk x6, #20082, lsl 32
  add.2d v4, v4, v9
  mov x7, #20728
  movk x7, #23588, lsl 16
  movk x6, #12388, lsl 48
  movk x7, #7790, lsl 32
  movk x7, #17170, lsl 48
  mul x8, x2, x4
  dup.2d v7, x7
  mov.16b v8, v21
  fmla.2d v8, v6, v7
  umulh x2, x2, x4
  fsub.2d v9, v22, v8
  fmla.2d v9, v6, v7
  add.2d v1, v1, v8
  cmn x8, x11
  cinc x2, x2, hs
  add.2d v0, v0, v9
  mov x7, #16000
  mul x8, x5, x4
  movk x7, #53891, lsl 16
  movk x7, #5509, lsl 32
  movk x7, #17144, lsl 48
  umulh x5, x5, x4
  dup.2d v7, x7
  mov.16b v8, v21
  fmla.2d v8, v6, v7
  adds x8, x8, x2
  cinc x5, x5, hs
  fsub.2d v9, v22, v8
  fmla.2d v9, v6, v7
  add.2d v2, v2, v8
  adds x8, x8, x10
  cinc x5, x5, hs
  add.2d v1, v1, v9
  mov x2, #46800
  mul x7, x9, x4
  movk x2, #2568, lsl 16
  movk x2, #1335, lsl 32
  movk x2, #17188, lsl 48
  umulh x9, x9, x4
  dup.2d v7, x2
  mov.16b v8, v21
  fmla.2d v8, v6, v7
  adds x7, x7, x5
  cinc x9, x9, hs
  fsub.2d v9, v22, v8
  fmla.2d v9, v6, v7
  adds x7, x7, x1
  cinc x9, x9, hs
  add.2d v5, v5, v8
  add.2d v2, v2, v9
  mov x1, #39040
  mul x2, x6, x4
  movk x1, #14704, lsl 16
  movk x1, #12839, lsl 32
  movk x1, #17096, lsl 48
  umulh x4, x6, x4
  dup.2d v7, x1
  mov.16b v8, v21
  adds x2, x2, x9
  cinc x4, x4, hs
  fmla.2d v8, v6, v7
  fsub.2d v9, v22, v8
  fmla.2d v9, v6, v7
  adds x2, x2, x3
  cinc x4, x4, hs
  add.2d v3, v3, v8
  add.2d v5, v5, v9
  mov x1, #140737488355328
  add x0, x0, x4
  dup.2d v6, x1
  and.16b v6, v3, v6
  cmeq.2d v6, v6, #0
  mov x1, #2
  mov x3, #2
  movk x3, #57344, lsl 16
  movk x1, #57344, lsl 16
  movk x3, #60199, lsl 32
  movk x3, #3, lsl 48
  dup.2d v7, x3
  movk x1, #60199, lsl 32
  bic.16b v7, v7, v6
  mov x3, #10364
  movk x3, #11794, lsl 16
  movk x1, #34755, lsl 48
  movk x3, #3895, lsl 32
  movk x3, #9, lsl 48
  mov x4, #57634
  dup.2d v8, x3
  bic.16b v8, v8, v6
  mov x3, #26576
  movk x4, #62322, lsl 16
  movk x3, #47696, lsl 16
  movk x3, #688, lsl 32
  movk x3, #3, lsl 48
  movk x4, #53392, lsl 32
  dup.2d v9, x3
  bic.16b v9, v9, v6
  movk x4, #20583, lsl 48
  mov x3, #46800
  movk x3, #2568, lsl 16
  movk x3, #1335, lsl 32
  mov x5, #45242
  movk x3, #4, lsl 48
  dup.2d v10, x3
  bic.16b v10, v10, v6
  movk x5, #770, lsl 16
  mov x3, #49763
  movk x3, #40165, lsl 16
  movk x3, #24776, lsl 32
  movk x5, #35693, lsl 32
  dup.2d v11, x3
  bic.16b v6, v11, v6
  movk x5, #28832, lsl 48
  sub.2d v0, v0, v7
  ssra.2d v0, v4, #52
  sub.2d v1, v1, v8
  mov x3, #16467
  ssra.2d v1, v0, #52
  sub.2d v2, v2, v9
  ssra.2d v2, v1, #52
  movk x3, #49763, lsl 16
  sub.2d v4, v5, v10
  ssra.2d v4, v2, #52
  movk x3, #40165, lsl 32
  sub.2d v3, v3, v6
  ssra.2d v3, v4, #52
  ushr.2d v5, v1, #12
  movk x3, #24776, lsl 48
  ushr.2d v6, v2, #24
  ushr.2d v7, v4, #36
  sli.2d v0, v1, #52
  subs x1, x8, x1
  sbcs x4, x7, x4
  sbcs x5, x2, x5
  sbcs x3, x0, x3
  sli.2d v5, v2, #40
  sli.2d v6, v4, #28
  sli.2d v7, v3, #16
  tst x0, #9223372036854775808
  csel x1, x1, x8, mi
  csel x4, x4, x7, mi
  csel x2, x5, x2, mi
  csel x0, x3, x0, mi
ret
