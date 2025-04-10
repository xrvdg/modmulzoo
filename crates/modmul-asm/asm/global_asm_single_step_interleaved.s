//in("x0") in0[0], in("x1") in0[1], in("x2") in0[2], in("x3") in0[3], in("x4") in1[0], in("x5") in1[1], in("x6") in1[2], in("x7") in1[3], in("v0") in2[0], in("v1") in2[1], in("v2") in2[2], in("v3") in2[3], in("v4") in3[0], in("v5") in3[1], in("v6") in3[2], in("v7") in3[3],
//lateout("x2") out0[0], lateout("x3") out0[1], lateout("x4") out0[2], lateout("x0") out0[3], lateout("v4") out1[0], lateout("v6") out1[1], lateout("v7") out1[2], lateout("v3") out1[3],
//lateout("v0") _, lateout("x1") _, lateout("v1") _, lateout("v2") _, lateout("x5") _, lateout("v5") _, lateout("x6") _, lateout("x7") _, lateout("x8") _, lateout("v8") _, lateout("x9") _, lateout("v9") _, lateout("x10") _, lateout("v10") _, lateout("x11") _, lateout("v11") _, lateout("x12") _, lateout("v12") _, lateout("x13") _, lateout("v13") _, lateout("x14") _, lateout("v14") _, lateout("x15") _, lateout("v15") _, lateout("x16") _, lateout("v16") _, lateout("v17") _, lateout("v18") _, lateout("v19") _, lateout("v20") _, lateout("v21") _, lateout("v22") _, lateout("v23") _, lateout("v24") _,
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
  and.16b v1, v9, v8
  umulh x12, x1, x4
  and.16b v2, v10, v8
  and.16b v9, v11, v8
  ushr.2d v3, v3, #14
  adds x11, x11, x10
  cinc x12, x12, hs
  shl.2d v10, v5, #14
  shl.2d v11, v6, #26
  shl.2d v12, v7, #38
  mul x10, x2, x4
  shl.2d v13, v4, #2
  usra.2d v10, v4, #50
  usra.2d v11, v5, #38
  umulh x13, x2, x4
  usra.2d v12, v6, #26
  and.16b v4, v13, v8
  and.16b v5, v10, v8
  adds x10, x10, x12
  cinc x13, x13, hs
  and.16b v6, v11, v8
  and.16b v10, v12, v8
  ushr.2d v7, v7, #14
  mul x12, x3, x4
  mov x14, #13605374474286268416
  dup.2d v11, x14
  mov x14, #6440147467139809280
  umulh x4, x3, x4
  dup.2d v12, x14
  mov x14, #3688448094816436224
  dup.2d v13, x14
  adds x12, x12, x13
  cinc x4, x4, hs
  mov x13, #9209861237972664320
  dup.2d v14, x13
  mov x13, #12218265789056155648
  mul x14, x0, x5
  dup.2d v15, x13
  mov x13, #17739678932212383744
  dup.2d v16, x13
  umulh x13, x0, x5
  mov x15, #2301339409586323456
  dup.2d v17, x15
  mov x15, #7822752552742551552
  adds x14, x14, x11
  cinc x13, x13, hs
  dup.2d v18, x15
  mov x11, #5071053180419178496
  dup.2d v19, x11
  mul x11, x1, x5
  mov x15, #16352570246982270976
  dup.2d v20, x15
  mov x15, #5075556780046548992
  umulh x16, x1, x5
  dup.2d v21, x15
  mov x15, #1
  movk x15, #18032, lsl 48
  adds x11, x11, x13
  cinc x16, x16, hs
  dup.2d v22, x15
  ucvtf.2d v0, v0
  ucvtf.2d v1, v1
  adds x11, x11, x10
  cinc x16, x16, hs
  ucvtf.2d v2, v2
  ucvtf.2d v9, v9
  ucvtf.2d v3, v3
  mul x10, x2, x5
  ucvtf.2d v4, v4
  ucvtf.2d v5, v5
  ucvtf.2d v6, v6
  umulh x13, x2, x5
  ucvtf.2d v10, v10
  ucvtf.2d v7, v7
  mov.16b v23, v21
  adds x10, x10, x16
  cinc x13, x13, hs
  mov.16b v24, v22
  fmla.2d v23, v0, v4
  fsub.2d v24, v24, v23
  adds x10, x10, x12
  cinc x13, x13, hs
  fmla.2d v24, v0, v4
  add.2d v13, v13, v23
  add.2d v11, v11, v24
  mul x12, x3, x5
  mov.16b v23, v21
  mov.16b v24, v22
  fmla.2d v23, v0, v5
  umulh x5, x3, x5
  fsub.2d v24, v24, v23
  fmla.2d v24, v0, v5
  add.2d v15, v15, v23
  adds x12, x12, x13
  cinc x5, x5, hs
  add.2d v13, v13, v24
  mov.16b v23, v21
  mov.16b v24, v22
  adds x12, x12, x4
  cinc x5, x5, hs
  fmla.2d v23, v0, v6
  fsub.2d v24, v24, v23
  fmla.2d v24, v0, v6
  mul x4, x0, x6
  add.2d v17, v17, v23
  add.2d v15, v15, v24
  mov.16b v23, v21
  umulh x13, x0, x6
  mov.16b v24, v22
  fmla.2d v23, v0, v10
  fsub.2d v24, v24, v23
  adds x4, x4, x11
  cinc x13, x13, hs
  fmla.2d v24, v0, v10
  add.2d v19, v19, v23
  add.2d v17, v17, v24
  mul x11, x1, x6
  mov.16b v23, v21
  mov.16b v24, v22
  fmla.2d v23, v0, v7
  umulh x15, x1, x6
  fsub.2d v24, v24, v23
  fmla.2d v24, v0, v7
  add.2d v0, v20, v23
  adds x11, x11, x13
  cinc x15, x15, hs
  add.2d v19, v19, v24
  mov.16b v20, v21
  mov.16b v23, v22
  adds x11, x11, x10
  cinc x15, x15, hs
  fmla.2d v20, v1, v4
  fsub.2d v23, v23, v20
  fmla.2d v23, v1, v4
  mul x10, x2, x6
  add.2d v15, v15, v20
  add.2d v13, v13, v23
  umulh x13, x2, x6
  mov.16b v20, v21
  mov.16b v23, v22
  fmla.2d v20, v1, v5
  adds x10, x10, x15
  cinc x13, x13, hs
  fsub.2d v23, v23, v20
  fmla.2d v23, v1, v5
  add.2d v17, v17, v20
  adds x10, x10, x12
  cinc x13, x13, hs
  add.2d v15, v15, v23
  mov.16b v20, v21
  mov.16b v23, v22
  mul x12, x3, x6
  fmla.2d v20, v1, v6
  fsub.2d v23, v23, v20
  fmla.2d v23, v1, v6
  umulh x6, x3, x6
  add.2d v19, v19, v20
  add.2d v17, v17, v23
  mov.16b v20, v21
  adds x12, x12, x13
  cinc x6, x6, hs
  mov.16b v23, v22
  fmla.2d v20, v1, v10
  fsub.2d v23, v23, v20
  adds x12, x12, x5
  cinc x6, x6, hs
  fmla.2d v23, v1, v10
  add.2d v0, v0, v20
  add.2d v19, v19, v23
  mul x5, x0, x7
  mov.16b v20, v21
  mov.16b v23, v22
  fmla.2d v20, v1, v7
  umulh x0, x0, x7
  fsub.2d v23, v23, v20
  fmla.2d v23, v1, v7
  add.2d v1, v18, v20
  adds x5, x5, x11
  cinc x0, x0, hs
  add.2d v0, v0, v23
  mov.16b v18, v21
  mov.16b v20, v22
  mul x11, x1, x7
  fmla.2d v18, v2, v4
  fsub.2d v20, v20, v18
  fmla.2d v20, v2, v4
  umulh x1, x1, x7
  add.2d v17, v17, v18
  add.2d v15, v15, v20
  mov.16b v18, v21
  adds x11, x11, x0
  cinc x1, x1, hs
  mov.16b v20, v22
  fmla.2d v18, v2, v5
  fsub.2d v20, v20, v18
  adds x11, x11, x10
  cinc x1, x1, hs
  fmla.2d v20, v2, v5
  add.2d v18, v19, v18
  add.2d v17, v17, v20
  mul x0, x2, x7
  mov.16b v19, v21
  mov.16b v20, v22
  fmla.2d v19, v2, v6
  umulh x2, x2, x7
  fsub.2d v20, v20, v19
  fmla.2d v20, v2, v6
  add.2d v0, v0, v19
  adds x0, x0, x1
  cinc x2, x2, hs
  add.2d v18, v18, v20
  mov.16b v19, v21
  mov.16b v20, v22
  adds x0, x0, x12
  cinc x2, x2, hs
  fmla.2d v19, v2, v10
  fsub.2d v20, v20, v19
  fmla.2d v20, v2, v10
  mul x1, x3, x7
  add.2d v1, v1, v19
  add.2d v0, v0, v20
  mov.16b v19, v21
  umulh x3, x3, x7
  mov.16b v20, v22
  fmla.2d v19, v2, v7
  fsub.2d v20, v20, v19
  adds x1, x1, x2
  cinc x3, x3, hs
  fmla.2d v20, v2, v7
  add.2d v2, v16, v19
  add.2d v1, v1, v20
  adds x1, x1, x6
  cinc x3, x3, hs
  mov.16b v16, v21
  mov.16b v19, v22
  fmla.2d v16, v9, v4
  mov x2, #48718
  fsub.2d v19, v19, v16
  fmla.2d v19, v9, v4
  add.2d v16, v18, v16
  movk x2, #4732, lsl 16
  add.2d v17, v17, v19
  mov.16b v18, v21
  mov.16b v19, v22
  movk x2, #45078, lsl 32
  fmla.2d v18, v9, v5
  fsub.2d v19, v19, v18
  fmla.2d v19, v9, v5
  movk x2, #39852, lsl 48
  add.2d v0, v0, v18
  add.2d v16, v16, v19
  mov.16b v18, v21
  mov x6, #16676
  mov.16b v19, v22
  fmla.2d v18, v9, v6
  fsub.2d v19, v19, v18
  movk x6, #12692, lsl 16
  fmla.2d v19, v9, v6
  add.2d v1, v1, v18
  add.2d v0, v0, v19
  movk x6, #20986, lsl 32
  mov.16b v18, v21
  mov.16b v19, v22
  fmla.2d v18, v9, v10
  movk x6, #2848, lsl 48
  fsub.2d v19, v19, v18
  fmla.2d v19, v9, v10
  add.2d v2, v2, v18
  mov x7, #51052
  add.2d v1, v1, v19
  mov.16b v18, v21
  mov.16b v19, v22
  movk x7, #24721, lsl 16
  fmla.2d v18, v9, v7
  fsub.2d v19, v19, v18
  fmla.2d v19, v9, v7
  movk x7, #61092, lsl 32
  add.2d v9, v14, v18
  add.2d v2, v2, v19
  mov.16b v14, v21
  movk x7, #45156, lsl 48
  mov.16b v18, v22
  fmla.2d v14, v3, v4
  fsub.2d v18, v18, v14
  mov x10, #3197
  fmla.2d v18, v3, v4
  add.2d v0, v0, v14
  movk x10, #18936, lsl 16
  add.2d v4, v16, v18
  mov.16b v14, v21
  mov.16b v16, v22
  movk x10, #10922, lsl 32
  fmla.2d v14, v3, v5
  fsub.2d v16, v16, v14
  fmla.2d v16, v3, v5
  movk x10, #11014, lsl 48
  add.2d v1, v1, v14
  add.2d v0, v0, v16
  mov.16b v5, v21
  mul x12, x2, x9
  mov.16b v14, v22
  fmla.2d v5, v3, v6
  fsub.2d v14, v14, v5
  umulh x2, x2, x9
  fmla.2d v14, v3, v6
  add.2d v2, v2, v5
  add.2d v1, v1, v14
  adds x12, x12, x5
  cinc x2, x2, hs
  mov.16b v5, v21
  mov.16b v6, v22
  fmla.2d v5, v3, v10
  mul x5, x6, x9
  fsub.2d v6, v6, v5
  fmla.2d v6, v3, v10
  add.2d v5, v9, v5
  umulh x6, x6, x9
  add.2d v2, v2, v6
  mov.16b v6, v21
  mov.16b v9, v22
  adds x5, x5, x2
  cinc x6, x6, hs
  fmla.2d v6, v3, v7
  fsub.2d v9, v9, v6
  fmla.2d v9, v3, v7
  adds x5, x5, x11
  cinc x6, x6, hs
  add.2d v3, v12, v6
  add.2d v5, v5, v9
  usra.2d v13, v11, #52
  mul x2, x7, x9
  usra.2d v15, v13, #52
  usra.2d v17, v15, #52
  usra.2d v4, v17, #52
  umulh x7, x7, x9
  and.16b v6, v11, v8
  and.16b v7, v13, v8
  and.16b v9, v15, v8
  adds x2, x2, x6
  cinc x7, x7, hs
  and.16b v10, v17, v8
  ucvtf.2d v6, v6
  mov x6, #37864
  adds x2, x2, x0
  cinc x7, x7, hs
  movk x6, #1815, lsl 16
  movk x6, #28960, lsl 32
  movk x6, #17153, lsl 48
  mul x0, x10, x9
  dup.2d v11, x6
  mov.16b v12, v21
  mov.16b v13, v22
  umulh x6, x10, x9
  fmla.2d v12, v6, v11
  fsub.2d v13, v13, v12
  fmla.2d v13, v6, v11
  adds x0, x0, x7
  cinc x6, x6, hs
  add.2d v0, v0, v12
  add.2d v4, v4, v13
  mov x7, #46128
  adds x0, x0, x1
  cinc x6, x6, hs
  movk x7, #29964, lsl 16
  movk x7, #7587, lsl 32
  movk x7, #17161, lsl 48
  add x1, x3, x6
  dup.2d v11, x7
  mov.16b v12, v21
  mov.16b v13, v22
  mov x3, #56431
  fmla.2d v12, v6, v11
  fsub.2d v13, v13, v12
  fmla.2d v13, v6, v11
  movk x3, #30457, lsl 16
  add.2d v1, v1, v12
  add.2d v0, v0, v13
  mov x6, #52826
  movk x3, #30012, lsl 32
  movk x6, #57790, lsl 16
  movk x6, #55431, lsl 32
  movk x6, #17196, lsl 48
  movk x3, #6382, lsl 48
  dup.2d v11, x6
  mov.16b v12, v21
  mov.16b v13, v22
  mov x6, #59151
  fmla.2d v12, v6, v11
  fsub.2d v13, v13, v12
  fmla.2d v13, v6, v11
  movk x6, #41769, lsl 16
  add.2d v2, v2, v12
  add.2d v1, v1, v13
  mov x7, #31276
  movk x6, #32276, lsl 32
  movk x7, #21262, lsl 16
  movk x7, #2304, lsl 32
  movk x7, #17182, lsl 48
  movk x6, #21677, lsl 48
  dup.2d v11, x7
  mov.16b v12, v21
  mov.16b v13, v22
  mov x7, #34015
  fmla.2d v12, v6, v11
  fsub.2d v13, v13, v12
  fmla.2d v13, v6, v11
  movk x7, #20342, lsl 16
  add.2d v5, v5, v12
  add.2d v2, v2, v13
  mov x9, #28672
  movk x7, #13935, lsl 32
  movk x9, #24515, lsl 16
  movk x9, #54929, lsl 32
  movk x9, #17064, lsl 48
  movk x7, #11030, lsl 48
  dup.2d v11, x9
  mov.16b v12, v21
  mov.16b v13, v22
  mov x9, #13689
  fmla.2d v12, v6, v11
  fsub.2d v13, v13, v12
  fmla.2d v13, v6, v11
  movk x9, #8159, lsl 16
  add.2d v3, v3, v12
  add.2d v5, v5, v13
  ucvtf.2d v6, v7
  movk x9, #215, lsl 32
  mov x10, #44768
  movk x10, #51919, lsl 16
  movk x9, #4913, lsl 48
  movk x10, #6346, lsl 32
  movk x10, #17133, lsl 48
  dup.2d v7, x10
  mul x10, x3, x14
  mov.16b v11, v21
  mov.16b v12, v22
  fmla.2d v11, v6, v7
  umulh x3, x3, x14
  fsub.2d v12, v12, v11
  fmla.2d v12, v6, v7
  add.2d v0, v0, v11
  adds x10, x10, x12
  cinc x3, x3, hs
  add.2d v4, v4, v12
  mov x11, #47492
  movk x11, #23630, lsl 16
  mul x12, x6, x14
  movk x11, #49985, lsl 32
  movk x11, #17168, lsl 48
  dup.2d v7, x11
  umulh x6, x6, x14
  mov.16b v11, v21
  mov.16b v12, v22
  fmla.2d v11, v6, v7
  adds x12, x12, x3
  cinc x6, x6, hs
  fsub.2d v12, v12, v11
  fmla.2d v12, v6, v7
  add.2d v1, v1, v11
  adds x12, x12, x5
  cinc x6, x6, hs
  add.2d v0, v0, v12
  mov x3, #57936
  movk x3, #54828, lsl 16
  mul x5, x7, x14
  movk x3, #18292, lsl 32
  movk x3, #17197, lsl 48
  dup.2d v7, x3
  umulh x3, x7, x14
  mov.16b v11, v21
  mov.16b v12, v22
  fmla.2d v11, v6, v7
  adds x5, x5, x6
  cinc x3, x3, hs
  fsub.2d v12, v12, v11
  fmla.2d v12, v6, v7
  add.2d v2, v2, v11
  adds x5, x5, x2
  cinc x3, x3, hs
  add.2d v1, v1, v12
  mov x2, #17708
  movk x2, #43915, lsl 16
  mul x6, x9, x14
  movk x2, #64348, lsl 32
  movk x2, #17188, lsl 48
  dup.2d v7, x2
  umulh x2, x9, x14
  mov.16b v11, v21
  mov.16b v12, v22
  fmla.2d v11, v6, v7
  adds x6, x6, x3
  cinc x2, x2, hs
  fsub.2d v12, v12, v11
  fmla.2d v12, v6, v7
  add.2d v5, v5, v11
  adds x6, x6, x0
  cinc x2, x2, hs
  add.2d v2, v2, v12
  mov x0, #29184
  movk x0, #20789, lsl 16
  add x1, x1, x2
  movk x0, #19197, lsl 32
  movk x0, #17083, lsl 48
  dup.2d v7, x0
  mov x0, #61005
  mov.16b v11, v21
  mov.16b v12, v22
  fmla.2d v11, v6, v7
  movk x0, #58262, lsl 16
  fsub.2d v12, v12, v11
  fmla.2d v12, v6, v7
  add.2d v3, v3, v11
  movk x0, #32851, lsl 32
  add.2d v5, v5, v12
  ucvtf.2d v6, v9
  mov x2, #58856
  movk x0, #11582, lsl 48
  movk x2, #14953, lsl 16
  movk x2, #15155, lsl 32
  movk x2, #17181, lsl 48
  mov x3, #37581
  dup.2d v7, x2
  mov.16b v9, v21
  mov.16b v11, v22
  movk x3, #43836, lsl 16
  fmla.2d v9, v6, v7
  fsub.2d v11, v11, v9
  fmla.2d v11, v6, v7
  movk x3, #36286, lsl 32
  add.2d v0, v0, v9
  add.2d v4, v4, v11
  mov x2, #35392
  movk x3, #51783, lsl 48
  movk x2, #12477, lsl 16
  movk x2, #56780, lsl 32
  movk x2, #17142, lsl 48
  mov x7, #10899
  dup.2d v7, x2
  mov.16b v9, v21
  mov.16b v11, v22
  movk x7, #30709, lsl 16
  fmla.2d v9, v6, v7
  fsub.2d v11, v11, v9
  fmla.2d v11, v6, v7
  movk x7, #61551, lsl 32
  add.2d v1, v1, v9
  add.2d v0, v0, v11
  mov x2, #9848
  movk x7, #45784, lsl 48
  movk x2, #54501, lsl 16
  movk x2, #31540, lsl 32
  movk x2, #17170, lsl 48
  mov x9, #36612
  dup.2d v7, x2
  mov.16b v9, v21
  mov.16b v11, v22
  movk x9, #63402, lsl 16
  fmla.2d v9, v6, v7
  fsub.2d v11, v11, v9
  fmla.2d v11, v6, v7
  movk x9, #47623, lsl 32
  add.2d v2, v2, v9
  add.2d v1, v1, v11
  mov x2, #9584
  movk x9, #9430, lsl 48
  movk x2, #63883, lsl 16
  movk x2, #18253, lsl 32
  movk x2, #17190, lsl 48
  mul x11, x0, x4
  dup.2d v7, x2
  mov.16b v9, v21
  mov.16b v11, v22
  umulh x0, x0, x4
  fmla.2d v9, v6, v7
  fsub.2d v11, v11, v9
  adds x11, x11, x10
  cinc x0, x0, hs
  fmla.2d v11, v6, v7
  add.2d v5, v5, v9
  add.2d v2, v2, v11
  mul x2, x3, x4
  mov x10, #51712
  movk x10, #16093, lsl 16
  movk x10, #30633, lsl 32
  umulh x3, x3, x4
  movk x10, #17068, lsl 48
  dup.2d v7, x10
  mov.16b v9, v21
  adds x2, x2, x0
  cinc x3, x3, hs
  mov.16b v11, v22
  fmla.2d v9, v6, v7
  fsub.2d v11, v11, v9
  adds x2, x2, x12
  cinc x3, x3, hs
  fmla.2d v11, v6, v7
  add.2d v3, v3, v9
  add.2d v5, v5, v11
  mul x0, x7, x4
  ucvtf.2d v6, v10
  mov x10, #34724
  movk x10, #40393, lsl 16
  umulh x7, x7, x4
  movk x10, #23752, lsl 32
  movk x10, #17184, lsl 48
  dup.2d v7, x10
  adds x0, x0, x3
  cinc x7, x7, hs
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7
  adds x0, x0, x5
  cinc x7, x7, hs
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7
  add.2d v0, v0, v9
  mul x3, x9, x4
  add.2d v4, v4, v10
  mov x5, #25532
  movk x5, #31025, lsl 16
  umulh x4, x9, x4
  movk x5, #10002, lsl 32
  movk x5, #17199, lsl 48
  dup.2d v7, x5
  adds x3, x3, x7
  cinc x4, x4, hs
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7
  adds x3, x3, x6
  cinc x4, x4, hs
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7
  add.2d v1, v1, v9
  add x1, x1, x4
  add.2d v0, v0, v10
  mov x4, #18830
  movk x4, #2465, lsl 16
  mov x5, #65535
  movk x4, #36348, lsl 32
  movk x4, #17194, lsl 48
  dup.2d v7, x4
  movk x5, #61439, lsl 16
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7
  movk x5, #62867, lsl 32
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7
  add.2d v2, v2, v9
  movk x5, #49889, lsl 48
  add.2d v1, v1, v10
  mov x4, #21566
  movk x4, #43708, lsl 16
  mul x5, x5, x11
  movk x4, #57685, lsl 32
  movk x4, #17185, lsl 48
  dup.2d v7, x4
  mov x4, #1
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7
  movk x4, #61440, lsl 16
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7
  add.2d v5, v5, v9
  movk x4, #62867, lsl 32
  add.2d v2, v2, v10
  mov x6, #3072
  movk x6, #8058, lsl 16
  movk x4, #17377, lsl 48
  movk x6, #46097, lsl 32
  movk x6, #17047, lsl 48
  dup.2d v7, x6
  mov x6, #28817
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7
  movk x6, #31161, lsl 16
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7
  add.2d v3, v3, v9
  movk x6, #59464, lsl 32
  add.2d v5, v5, v10
  mov x7, #65535
  movk x7, #61439, lsl 16
  movk x6, #10291, lsl 48
  movk x7, #62867, lsl 32
  movk x7, #1, lsl 48
  umov x9, v4.d[0]
  mov x10, #22621
  umov x12, v4.d[1]
  mul x9, x9, x7
  mul x7, x12, x7
  movk x10, #33153, lsl 16
  and x9, x9, x8
  and x7, x7, x8
  ins v6.d[0], x9
  ins v6.d[1], x7
  movk x10, #17846, lsl 32
  ucvtf.2d v6, v6
  mov x7, #16
  movk x7, #22847, lsl 32
  movk x10, #47184, lsl 48
  movk x7, #17151, lsl 48
  dup.2d v7, x7
  mov.16b v9, v21
  mov x7, #41001
  mov.16b v10, v22
  fmla.2d v9, v6, v7
  fsub.2d v10, v10, v9
  movk x7, #57649, lsl 16
  fmla.2d v10, v6, v7
  add.2d v0, v0, v9
  add.2d v4, v4, v10
  movk x7, #20082, lsl 32
  mov x8, #20728
  movk x8, #23588, lsl 16
  movk x7, #12388, lsl 48
  movk x8, #7790, lsl 32
  movk x8, #17170, lsl 48
  dup.2d v7, x8
  mul x8, x4, x5
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7
  umulh x4, x4, x5
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7
  add.2d v1, v1, v9
  cmn x8, x11
  cinc x4, x4, hs
  add.2d v0, v0, v10
  mov x8, #16000
  movk x8, #53891, lsl 16
  mul x9, x6, x5
  movk x8, #5509, lsl 32
  movk x8, #17144, lsl 48
  dup.2d v7, x8
  umulh x6, x6, x5
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7
  adds x9, x9, x4
  cinc x6, x6, hs
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7
  add.2d v2, v2, v9
  adds x9, x9, x2
  cinc x6, x6, hs
  add.2d v1, v1, v10
  mov x2, #46800
  movk x2, #2568, lsl 16
  mul x4, x10, x5
  movk x2, #1335, lsl 32
  movk x2, #17188, lsl 48
  dup.2d v7, x2
  umulh x2, x10, x5
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7
  adds x4, x4, x6
  cinc x2, x2, hs
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7
  add.2d v5, v5, v9
  adds x4, x4, x0
  cinc x2, x2, hs
  add.2d v2, v2, v10
  mov x0, #39040
  movk x0, #14704, lsl 16
  mul x6, x7, x5
  movk x0, #12839, lsl 32
  movk x0, #17096, lsl 48
  dup.2d v7, x0
  umulh x0, x7, x5
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7
  adds x6, x6, x2
  cinc x0, x0, hs
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7
  add.2d v3, v3, v9
  adds x6, x6, x3
  cinc x0, x0, hs
  add.2d v5, v5, v10
  mov x2, #140737488355328
  dup.2d v6, x2
  add x0, x1, x0
  and.16b v6, v3, v6
  cmeq.2d v6, v6, #0
  mov x1, #2
  mov x2, #2
  movk x1, #57344, lsl 16
  movk x1, #60199, lsl 32
  movk x1, #3, lsl 48
  movk x2, #57344, lsl 16
  dup.2d v7, x1
  bic.16b v7, v7, v6
  mov x1, #10364
  movk x2, #60199, lsl 32
  movk x1, #11794, lsl 16
  movk x1, #3895, lsl 32
  movk x1, #9, lsl 48
  movk x2, #34755, lsl 48
  dup.2d v9, x1
  bic.16b v9, v9, v6
  mov x1, #26576
  mov x3, #57634
  movk x1, #47696, lsl 16
  movk x1, #688, lsl 32
  movk x1, #3, lsl 48
  movk x3, #62322, lsl 16
  dup.2d v10, x1
  bic.16b v10, v10, v6
  mov x1, #46800
  movk x3, #53392, lsl 32
  movk x1, #2568, lsl 16
  movk x1, #1335, lsl 32
  movk x1, #4, lsl 48
  movk x3, #20583, lsl 48
  dup.2d v11, x1
  bic.16b v11, v11, v6
  mov x1, #49763
  mov x5, #45242
  movk x1, #40165, lsl 16
  movk x1, #24776, lsl 32
  dup.2d v12, x1
  movk x5, #770, lsl 16
  bic.16b v6, v12, v6
  sub.2d v0, v0, v7
  ssra.2d v0, v4, #52
  movk x5, #35693, lsl 32
  and.16b v4, v0, v8
  sub.2d v1, v1, v9
  ssra.2d v1, v0, #52
  movk x5, #28832, lsl 48
  and.16b v0, v1, v8
  sub.2d v2, v2, v10
  ssra.2d v2, v1, #52
  mov x1, #16467
  and.16b v1, v2, v8
  sub.2d v5, v5, v11
  ssra.2d v5, v2, #52
  movk x1, #49763, lsl 16
  and.16b v2, v5, v8
  sub.2d v3, v3, v6
  ssra.2d v3, v5, #52
  movk x1, #40165, lsl 32
  and.16b v3, v3, v8
  shl.2d v5, v0, #52
  shl.2d v6, v1, #40
  movk x1, #24776, lsl 48
  shl.2d v7, v2, #28
  shl.2d v3, v3, #16
  orr.16b v4, v4, v5
  subs x2, x9, x2
  sbcs x3, x4, x3
  sbcs x5, x6, x5
  sbcs x1, x0, x1
  usra.2d v6, v0, #12
  usra.2d v7, v1, #24
  usra.2d v3, v2, #36
  tst x0, #9223372036854775808
  csel x2, x2, x9, mi
  csel x3, x3, x4, mi
  csel x4, x5, x6, mi
  csel x0, x1, x0, mi
ret
