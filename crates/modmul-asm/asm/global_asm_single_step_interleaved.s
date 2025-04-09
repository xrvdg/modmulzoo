//in("x0") _, in("x1") _, in("x2") _, in("x3") _, in("x4") _, in("x5") _, in("x6") _, in("x7") _, in("v8") _, in("v9") _, in("v10") _, in("v11") _, in("v12") _, in("v13") _, in("v14") _, in("v15") _,
//lateout("x2") out0[0], lateout("x3") out0[1], lateout("x1") out0[2], lateout("x0") out0[3], lateout("v3") out1[0], lateout("v5") out1[1], lateout("v7") out1[2], lateout("v0") out1[3],
//lateout("v1") _, lateout("v2") _, lateout("x4") _, lateout("v4") _, lateout("x5") _, lateout("x6") _, lateout("v6") _, lateout("x7") _, lateout("x8") _, lateout("v8") _, lateout("x9") _, lateout("v9") _, lateout("x10") _, lateout("v10") _, lateout("x11") _, lateout("v11") _, lateout("x12") _, lateout("v12") _, lateout("x13") _, lateout("v13") _, lateout("x14") _, lateout("v14") _, lateout("x15") _, lateout("v15") _, lateout("x16") _, lateout("v16") _, lateout("x17") _, lateout("v17") _, lateout("v18") _, lateout("v19") _, lateout("v20") _, lateout("v21") _, lateout("v22") _, lateout("v23") _, lateout("v24") _,
//lateout("lr") _
.global _single_step_interleaved
.align 4
.text
_single_step_interleaved:
  mov x8, #4503599627370495
  dup.2d v0, x8
  shl.2d v1, v9, #14
  mul x8, x0, x4
  shl.2d v2, v10, #26
  shl.2d v3, v11, #38
  shl.2d v4, v8, #2
  umulh x9, x0, x4
  usra.2d v1, v8, #50
  usra.2d v2, v9, #38
  usra.2d v3, v10, #26
  mul x10, x1, x4
  and.16b v4, v4, v0
  and.16b v1, v1, v0
  and.16b v2, v2, v0
  umulh x11, x1, x4
  and.16b v0, v3, v0
  ushr.2d v3, v11, #14
  mov x12, #4503599627370495
  adds x10, x10, x9
  cinc x11, x11, hs
  dup.2d v5, x12
  shl.2d v6, v13, #14
  shl.2d v7, v14, #26
  shl.2d v8, v15, #38
  mul x9, x2, x4
  shl.2d v9, v12, #2
  usra.2d v6, v12, #50
  usra.2d v7, v13, #38
  umulh x12, x2, x4
  usra.2d v8, v14, #26
  and.16b v9, v9, v5
  and.16b v6, v6, v5
  adds x9, x9, x11
  cinc x12, x12, hs
  and.16b v7, v7, v5
  and.16b v5, v8, v5
  ushr.2d v8, v15, #14
  mul x11, x3, x4
  mov x13, #13605374474286268416
  dup.2d v10, x13
  mov x13, #6440147467139809280
  umulh x4, x3, x4
  dup.2d v11, x13
  mov x13, #3688448094816436224
  dup.2d v12, x13
  mov x13, #9209861237972664320
  adds x11, x11, x12
  cinc x4, x4, hs
  dup.2d v13, x13
  mov x12, #12218265789056155648
  dup.2d v14, x12
  mul x12, x0, x5
  mov x13, #17739678932212383744
  dup.2d v15, x13
  mov x13, #2301339409586323456
  umulh x14, x0, x5
  dup.2d v16, x13
  mov x13, #7822752552742551552
  dup.2d v17, x13
  adds x12, x12, x10
  cinc x14, x14, hs
  mov x10, #5071053180419178496
  dup.2d v18, x10
  mov x10, #16352570246982270976
  mul x13, x1, x5
  dup.2d v19, x10
  mov x10, #5075556780046548992
  dup.2d v20, x10
  mov x10, #1
  umulh x15, x1, x5
  movk x10, #18032, lsl 48
  dup.2d v21, x10
  ucvtf.2d v4, v4
  adds x13, x13, x14
  cinc x15, x15, hs
  ucvtf.2d v22, v9
  mov.16b v23, v20
  mov.16b v24, v21
  adds x13, x13, x9
  cinc x15, x15, hs
  fmla.2d v23, v4, v22
  fsub.2d v24, v24, v23
  fmla.2d v24, v4, v22
  mul x9, x2, x5
  add.2d v12, v12, v23
  add.2d v10, v10, v24
  ucvtf.2d v22, v6
  umulh x10, x2, x5
  mov.16b v23, v20
  mov.16b v24, v21
  fmla.2d v23, v4, v22
  fsub.2d v24, v24, v23
  adds x9, x9, x15
  cinc x10, x10, hs
  fmla.2d v24, v4, v22
  add.2d v14, v14, v23
  add.2d v12, v12, v24
  adds x9, x9, x11
  cinc x10, x10, hs
  ucvtf.2d v22, v7
  mov.16b v23, v20
  mov.16b v24, v21
  mul x11, x3, x5
  fmla.2d v23, v4, v22
  fsub.2d v24, v24, v23
  fmla.2d v24, v4, v22
  umulh x5, x3, x5
  add.2d v16, v16, v23
  add.2d v14, v14, v24
  ucvtf.2d v22, v5
  adds x11, x11, x10
  cinc x5, x5, hs
  mov.16b v23, v20
  mov.16b v24, v21
  fmla.2d v23, v4, v22
  fsub.2d v24, v24, v23
  adds x11, x11, x4
  cinc x5, x5, hs
  fmla.2d v24, v4, v22
  add.2d v18, v18, v23
  add.2d v16, v16, v24
  mul x4, x0, x6
  ucvtf.2d v22, v8
  mov.16b v23, v20
  mov.16b v24, v21
  umulh x10, x0, x6
  fmla.2d v23, v4, v22
  fsub.2d v24, v24, v23
  fmla.2d v24, v4, v22
  adds x4, x4, x13
  cinc x10, x10, hs
  add.2d v4, v19, v23
  add.2d v18, v18, v24
  ucvtf.2d v1, v1
  mul x13, x1, x6
  ucvtf.2d v19, v9
  mov.16b v22, v20
  mov.16b v23, v21
  umulh x14, x1, x6
  fmla.2d v22, v1, v19
  fsub.2d v23, v23, v22
  fmla.2d v23, v1, v19
  add.2d v14, v14, v22
  adds x13, x13, x10
  cinc x14, x14, hs
  add.2d v12, v12, v23
  ucvtf.2d v19, v6
  mov.16b v22, v20
  adds x13, x13, x9
  cinc x14, x14, hs
  mov.16b v23, v21
  fmla.2d v22, v1, v19
  fsub.2d v23, v23, v22
  mul x9, x2, x6
  fmla.2d v23, v1, v19
  add.2d v16, v16, v22
  add.2d v14, v14, v23
  umulh x10, x2, x6
  ucvtf.2d v19, v7
  mov.16b v22, v20
  mov.16b v23, v21
  adds x9, x9, x14
  cinc x10, x10, hs
  fmla.2d v22, v1, v19
  fsub.2d v23, v23, v22
  fmla.2d v23, v1, v19
  add.2d v18, v18, v22
  adds x9, x9, x11
  cinc x10, x10, hs
  add.2d v16, v16, v23
  ucvtf.2d v19, v5
  mov.16b v22, v20
  mul x11, x3, x6
  mov.16b v23, v21
  fmla.2d v22, v1, v19
  fsub.2d v23, v23, v22
  umulh x6, x3, x6
  fmla.2d v23, v1, v19
  add.2d v4, v4, v22
  add.2d v18, v18, v23
  adds x11, x11, x10
  cinc x6, x6, hs
  ucvtf.2d v19, v8
  mov.16b v22, v20
  mov.16b v23, v21
  adds x11, x11, x5
  cinc x6, x6, hs
  fmla.2d v22, v1, v19
  fsub.2d v23, v23, v22
  fmla.2d v23, v1, v19
  add.2d v1, v17, v22
  mul x5, x0, x7
  add.2d v4, v4, v23
  ucvtf.2d v2, v2
  ucvtf.2d v17, v9
  umulh x0, x0, x7
  mov.16b v19, v20
  mov.16b v22, v21
  fmla.2d v19, v2, v17
  adds x5, x5, x13
  cinc x0, x0, hs
  fsub.2d v22, v22, v19
  fmla.2d v22, v2, v17
  add.2d v16, v16, v19
  mul x10, x1, x7
  add.2d v14, v14, v22
  ucvtf.2d v17, v6
  mov.16b v19, v20
  umulh x1, x1, x7
  mov.16b v22, v21
  fmla.2d v19, v2, v17
  fsub.2d v22, v22, v19
  fmla.2d v22, v2, v17
  adds x10, x10, x0
  cinc x1, x1, hs
  add.2d v17, v18, v19
  add.2d v16, v16, v22
  ucvtf.2d v18, v7
  adds x10, x10, x9
  cinc x1, x1, hs
  mov.16b v19, v20
  mov.16b v22, v21
  fmla.2d v19, v2, v18
  mul x0, x2, x7
  fsub.2d v22, v22, v19
  fmla.2d v22, v2, v18
  add.2d v4, v4, v19
  umulh x2, x2, x7
  add.2d v17, v17, v22
  ucvtf.2d v18, v5
  mov.16b v19, v20
  adds x0, x0, x1
  cinc x2, x2, hs
  mov.16b v22, v21
  fmla.2d v19, v2, v18
  fsub.2d v22, v22, v19
  fmla.2d v22, v2, v18
  adds x0, x0, x11
  cinc x2, x2, hs
  add.2d v1, v1, v19
  add.2d v4, v4, v22
  ucvtf.2d v18, v8
  mul x1, x3, x7
  mov.16b v19, v20
  mov.16b v22, v21
  fmla.2d v19, v2, v18
  umulh x3, x3, x7
  fsub.2d v22, v22, v19
  fmla.2d v22, v2, v18
  add.2d v2, v15, v19
  adds x1, x1, x2
  cinc x3, x3, hs
  add.2d v1, v1, v22
  ucvtf.2d v0, v0
  ucvtf.2d v15, v9
  adds x1, x1, x6
  cinc x3, x3, hs
  mov.16b v18, v20
  mov.16b v19, v21
  fmla.2d v18, v0, v15
  fsub.2d v19, v19, v18
  mov x2, #48718
  fmla.2d v19, v0, v15
  add.2d v15, v17, v18
  add.2d v16, v16, v19
  movk x2, #4732, lsl 16
  ucvtf.2d v17, v6
  mov.16b v18, v20
  mov.16b v19, v21
  movk x2, #45078, lsl 32
  fmla.2d v18, v0, v17
  fsub.2d v19, v19, v18
  fmla.2d v19, v0, v17
  movk x2, #39852, lsl 48
  add.2d v4, v4, v18
  add.2d v15, v15, v19
  ucvtf.2d v17, v7
  mov x6, #16676
  mov.16b v18, v20
  mov.16b v19, v21
  fmla.2d v18, v0, v17
  movk x6, #12692, lsl 16
  fsub.2d v19, v19, v18
  fmla.2d v19, v0, v17
  add.2d v1, v1, v18
  add.2d v4, v4, v19
  movk x6, #20986, lsl 32
  ucvtf.2d v17, v5
  mov.16b v18, v20
  mov.16b v19, v21
  movk x6, #2848, lsl 48
  fmla.2d v18, v0, v17
  fsub.2d v19, v19, v18
  fmla.2d v19, v0, v17
  mov x7, #51052
  add.2d v2, v2, v18
  add.2d v1, v1, v19
  ucvtf.2d v17, v8
  movk x7, #24721, lsl 16
  mov.16b v18, v20
  mov.16b v19, v21
  fmla.2d v18, v0, v17
  movk x7, #61092, lsl 32
  fsub.2d v19, v19, v18
  fmla.2d v19, v0, v17
  add.2d v0, v13, v18
  add.2d v2, v2, v19
  movk x7, #45156, lsl 48
  ucvtf.2d v3, v3
  ucvtf.2d v9, v9
  mov.16b v13, v20
  mov x9, #3197
  mov.16b v17, v21
  fmla.2d v13, v3, v9
  fsub.2d v17, v17, v13
  movk x9, #18936, lsl 16
  fmla.2d v17, v3, v9
  add.2d v4, v4, v13
  add.2d v9, v15, v17
  movk x9, #10922, lsl 32
  ucvtf.2d v6, v6
  mov.16b v13, v20
  mov.16b v15, v21
  movk x9, #11014, lsl 48
  fmla.2d v13, v3, v6
  fsub.2d v15, v15, v13
  fmla.2d v15, v3, v6
  add.2d v1, v1, v13
  mul x11, x2, x8
  add.2d v4, v4, v15
  ucvtf.2d v6, v7
  mov.16b v7, v20
  umulh x2, x2, x8
  mov.16b v13, v21
  fmla.2d v7, v3, v6
  fsub.2d v13, v13, v7
  adds x11, x11, x5
  cinc x2, x2, hs
  fmla.2d v13, v3, v6
  add.2d v2, v2, v7
  add.2d v1, v1, v13
  mul x5, x6, x8
  ucvtf.2d v5, v5
  mov.16b v6, v20
  mov.16b v7, v21
  umulh x6, x6, x8
  fmla.2d v6, v3, v5
  fsub.2d v7, v7, v6
  fmla.2d v7, v3, v5
  add.2d v0, v0, v6
  adds x5, x5, x2
  cinc x6, x6, hs
  add.2d v2, v2, v7
  ucvtf.2d v5, v8
  mov.16b v6, v20
  adds x5, x5, x10
  cinc x6, x6, hs
  mov.16b v7, v21
  fmla.2d v6, v3, v5
  fsub.2d v7, v7, v6
  mul x2, x7, x8
  fmla.2d v7, v3, v5
  add.2d v3, v11, v6
  add.2d v0, v0, v7
  umulh x7, x7, x8
  usra.2d v12, v10, #52
  usra.2d v14, v12, #52
  usra.2d v16, v14, #52
  adds x2, x2, x6
  cinc x7, x7, hs
  usra.2d v9, v16, #52
  mov x6, #4503599627370495
  dup.2d v5, x6
  and.16b v6, v10, v5
  adds x2, x2, x0
  cinc x7, x7, hs
  and.16b v7, v12, v5
  and.16b v8, v14, v5
  and.16b v5, v16, v5
  mul x0, x9, x8
  mov x10, #62077
  movk x10, #226, lsl 16
  movk x10, #11812, lsl 32
  umulh x8, x9, x8
  movk x10, #2, lsl 48
  mov x9, #38534
  movk x9, #28321, lsl 16
  adds x0, x0, x7
  cinc x8, x8, hs
  movk x9, #9140, lsl 32
  movk x9, #3, lsl 48
  mov x7, #26413
  adds x0, x0, x1
  cinc x8, x8, hs
  movk x7, #61663, lsl 16
  movk x7, #27715, lsl 32
  movk x7, #14, lsl 48
  mov x1, #40587
  add x3, x3, x8
  movk x1, #5315, lsl 16
  movk x1, #33344, lsl 32
  movk x1, #7, lsl 48
  mov x8, #56431
  mov x13, #57784
  movk x13, #18607, lsl 16
  movk x13, #3179, lsl 32
  movk x8, #30457, lsl 16
  mov x14, #5075556780046548992
  dup.2d v10, x14
  mov x14, #1
  movk x8, #30012, lsl 32
  movk x14, #18032, lsl 48
  dup.2d v11, x14
  ucvtf.2d v6, v6
  movk x8, #6382, lsl 48
  ucvtf d12, x10
  mov.16b v13, v10
  mov.16b v14, v11
  mov x10, #59151
  fmla.2d v13, v6, v12[0]
  fsub.2d v14, v14, v13
  fmla.2d v14, v6, v12[0]
  add.2d v4, v4, v13
  movk x10, #41769, lsl 16
  add.2d v9, v9, v14
  ucvtf d12, x9
  mov.16b v13, v10
  movk x10, #32276, lsl 32
  mov.16b v14, v11
  fmla.2d v13, v6, v12[0]
  fsub.2d v14, v14, v13
  movk x10, #21677, lsl 48
  fmla.2d v14, v6, v12[0]
  add.2d v1, v1, v13
  add.2d v4, v4, v14
  mov x9, #34015
  ucvtf d12, x7
  mov.16b v13, v10
  mov.16b v14, v11
  movk x9, #20342, lsl 16
  fmla.2d v13, v6, v12[0]
  fsub.2d v14, v14, v13
  fmla.2d v14, v6, v12[0]
  add.2d v2, v2, v13
  movk x9, #13935, lsl 32
  add.2d v1, v1, v14
  ucvtf d12, x1
  mov.16b v13, v10
  movk x9, #11030, lsl 48
  mov.16b v14, v11
  fmla.2d v13, v6, v12[0]
  fsub.2d v14, v14, v13
  mov x1, #13689
  fmla.2d v14, v6, v12[0]
  add.2d v0, v0, v13
  add.2d v2, v2, v14
  movk x1, #8159, lsl 16
  ucvtf d12, x13
  mov.16b v10, v10
  mov.16b v11, v11
  movk x1, #215, lsl 32
  fmla.2d v10, v6, v12[0]
  fsub.2d v11, v11, v10
  fmla.2d v11, v6, v12[0]
  add.2d v3, v3, v10
  movk x1, #4913, lsl 48
  add.2d v0, v0, v11
  mov x7, #32119
  movk x7, #22102, lsl 16
  mul x13, x8, x12
  movk x7, #59590, lsl 32
  mov x14, #44641
  movk x14, #22291, lsl 16
  umulh x8, x8, x12
  movk x14, #12496, lsl 32
  movk x14, #4, lsl 48
  mov x15, #28968
  adds x13, x13, x11
  cinc x8, x8, hs
  movk x15, #27414, lsl 16
  movk x15, #41914, lsl 32
  movk x15, #14, lsl 48
  mul x11, x10, x12
  mov x16, #41622
  movk x16, #21957, lsl 16
  movk x16, #32174, lsl 32
  movk x16, #10, lsl 48
  umulh x10, x10, x12
  mov x17, #13682
  movk x17, #64849, lsl 16
  movk x17, #6986, lsl 32
  adds x11, x11, x8
  cinc x10, x10, hs
  mov x8, #5075556780046548992
  dup.2d v6, x8
  mov x8, #1
  adds x11, x11, x5
  cinc x10, x10, hs
  movk x8, #18032, lsl 48
  dup.2d v10, x8
  ucvtf.2d v7, v7
  mul x5, x9, x12
  ucvtf d11, x7
  mov.16b v12, v6
  mov.16b v13, v10
  umulh x7, x9, x12
  fmla.2d v12, v7, v11[0]
  fsub.2d v13, v13, v12
  fmla.2d v13, v7, v11[0]
  add.2d v4, v4, v12
  adds x5, x5, x10
  cinc x7, x7, hs
  add.2d v9, v9, v13
  ucvtf d11, x14
  mov.16b v12, v6
  adds x5, x5, x2
  cinc x7, x7, hs
  mov.16b v13, v10
  fmla.2d v12, v7, v11[0]
  fsub.2d v13, v13, v12
  mul x2, x1, x12
  fmla.2d v13, v7, v11[0]
  add.2d v1, v1, v12
  add.2d v4, v4, v13
  umulh x1, x1, x12
  ucvtf d11, x15
  mov.16b v12, v6
  mov.16b v13, v10
  adds x2, x2, x7
  cinc x1, x1, hs
  fmla.2d v12, v7, v11[0]
  fsub.2d v13, v13, v12
  fmla.2d v13, v7, v11[0]
  adds x2, x2, x0
  cinc x1, x1, hs
  add.2d v2, v2, v12
  add.2d v1, v1, v13
  ucvtf d11, x16
  mov.16b v12, v6
  add x0, x3, x1
  mov.16b v13, v10
  fmla.2d v12, v7, v11[0]
  fsub.2d v13, v13, v12
  mov x1, #61005
  fmla.2d v13, v7, v11[0]
  add.2d v0, v0, v12
  add.2d v2, v2, v13
  movk x1, #58262, lsl 16
  ucvtf d11, x17
  mov.16b v6, v6
  mov.16b v10, v10
  movk x1, #32851, lsl 32
  fmla.2d v6, v7, v11[0]
  fsub.2d v10, v10, v6
  fmla.2d v10, v7, v11[0]
  movk x1, #11582, lsl 48
  add.2d v3, v3, v6
  add.2d v0, v0, v10
  mov x3, #31098
  movk x3, #52890, lsl 16
  mov x7, #37581
  movk x3, #20172, lsl 32
  movk x3, #7, lsl 48
  mov x8, #55460
  movk x7, #43836, lsl 16
  movk x8, #49931, lsl 16
  movk x8, #28124, lsl 32
  movk x8, #1, lsl 48
  movk x7, #36286, lsl 32
  mov x9, #18846
  movk x9, #13625, lsl 16
  movk x9, #40653, lsl 32
  movk x7, #51783, lsl 48
  movk x9, #4, lsl 48
  mov x10, #37560
  movk x10, #64709, lsl 16
  mov x12, #10899
  movk x10, #9126, lsl 32
  movk x10, #11, lsl 48
  mov x14, #28389
  movk x14, #54431, lsl 16
  movk x12, #30709, lsl 16
  movk x14, #3643, lsl 32
  mov x15, #5075556780046548992
  dup.2d v6, x15
  movk x12, #61551, lsl 32
  mov x15, #1
  movk x15, #18032, lsl 48
  dup.2d v7, x15
  movk x12, #45784, lsl 48
  ucvtf.2d v8, v8
  ucvtf d10, x3
  mov.16b v11, v6
  mov x3, #36612
  mov.16b v12, v7
  fmla.2d v11, v8, v10[0]
  fsub.2d v12, v12, v11
  movk x3, #63402, lsl 16
  fmla.2d v12, v8, v10[0]
  add.2d v4, v4, v11
  add.2d v9, v9, v12
  ucvtf d10, x8
  movk x3, #47623, lsl 32
  mov.16b v11, v6
  mov.16b v12, v7
  fmla.2d v11, v8, v10[0]
  movk x3, #9430, lsl 48
  fsub.2d v12, v12, v11
  fmla.2d v12, v8, v10[0]
  add.2d v1, v1, v11
  mul x8, x1, x4
  add.2d v4, v4, v12
  ucvtf d10, x9
  mov.16b v11, v6
  umulh x1, x1, x4
  mov.16b v12, v7
  fmla.2d v11, v8, v10[0]
  fsub.2d v12, v12, v11
  adds x8, x8, x13
  cinc x1, x1, hs
  fmla.2d v12, v8, v10[0]
  add.2d v2, v2, v11
  add.2d v1, v1, v12
  ucvtf d10, x10
  mul x9, x7, x4
  mov.16b v11, v6
  mov.16b v12, v7
  fmla.2d v11, v8, v10[0]
  umulh x7, x7, x4
  fsub.2d v12, v12, v11
  fmla.2d v12, v8, v10[0]
  add.2d v0, v0, v11
  adds x9, x9, x1
  cinc x7, x7, hs
  add.2d v2, v2, v12
  ucvtf d10, x14
  mov.16b v6, v6
  adds x9, x9, x11
  cinc x7, x7, hs
  mov.16b v7, v7
  fmla.2d v6, v8, v10[0]
  fsub.2d v7, v7, v6
  mul x1, x12, x4
  fmla.2d v7, v8, v10[0]
  add.2d v3, v3, v6
  add.2d v0, v0, v7
  mov x10, #50130
  umulh x11, x12, x4
  movk x10, #20196, lsl 16
  movk x10, #11876, lsl 32
  movk x10, #8, lsl 48
  adds x1, x1, x7
  cinc x11, x11, hs
  mov x7, #45534
  movk x7, #15512, lsl 16
  movk x7, #37769, lsl 32
  adds x1, x1, x5
  cinc x11, x11, hs
  movk x7, #15, lsl 48
  mov x5, #42183
  movk x5, #1232, lsl 16
  mul x12, x3, x4
  movk x5, #18174, lsl 32
  movk x5, #13, lsl 48
  mov x13, #10783
  umulh x3, x3, x4
  movk x13, #54622, lsl 16
  movk x13, #61610, lsl 32
  movk x13, #8, lsl 48
  adds x12, x12, x11
  cinc x3, x3, hs
  mov x4, #56963
  movk x4, #1095, lsl 16
  movk x4, #1517, lsl 32
  mov x11, #5075556780046548992
  adds x12, x12, x2
  cinc x3, x3, hs
  dup.2d v6, x11
  mov x2, #1
  movk x2, #18032, lsl 48
  add x0, x0, x3
  dup.2d v7, x2
  ucvtf.2d v5, v5
  ucvtf d8, x10
  mov x2, #65535
  mov.16b v10, v6
  mov.16b v11, v7
  fmla.2d v10, v5, v8[0]
  movk x2, #61439, lsl 16
  fsub.2d v11, v11, v10
  fmla.2d v11, v5, v8[0]
  add.2d v4, v4, v10
  movk x2, #62867, lsl 32
  add.2d v8, v9, v11
  ucvtf d9, x7
  mov.16b v10, v6
  mov.16b v11, v7
  movk x2, #49889, lsl 48
  fmla.2d v10, v5, v9[0]
  fsub.2d v11, v11, v10
  fmla.2d v11, v5, v9[0]
  mul x2, x2, x8
  add.2d v1, v1, v10
  add.2d v4, v4, v11
  ucvtf d9, x5
  mov x3, #1
  mov.16b v10, v6
  mov.16b v11, v7
  fmla.2d v10, v5, v9[0]
  movk x3, #61440, lsl 16
  fsub.2d v11, v11, v10
  fmla.2d v11, v5, v9[0]
  add.2d v2, v2, v10
  movk x3, #62867, lsl 32
  add.2d v1, v1, v11
  ucvtf d9, x13
  mov.16b v10, v6
  mov.16b v11, v7
  movk x3, #17377, lsl 48
  fmla.2d v10, v5, v9[0]
  fsub.2d v11, v11, v10
  fmla.2d v11, v5, v9[0]
  mov x5, #28817
  add.2d v0, v0, v10
  add.2d v2, v2, v11
  ucvtf d9, x4
  movk x5, #31161, lsl 16
  mov.16b v6, v6
  mov.16b v7, v7
  fmla.2d v6, v5, v9[0]
  movk x5, #59464, lsl 32
  fsub.2d v7, v7, v6
  fmla.2d v7, v5, v9[0]
  add.2d v3, v3, v6
  movk x5, #10291, lsl 48
  add.2d v0, v0, v7
  mov x4, #65535
  movk x4, #61439, lsl 16
  movk x4, #62867, lsl 32
  mov x7, #22621
  movk x4, #1, lsl 48
  umov x10, v8.d[0]
  umov x11, v8.d[1]
  movk x7, #33153, lsl 16
  mul x10, x10, x4
  mul x4, x11, x4
  and x10, x10, x6
  movk x7, #17846, lsl 32
  and x4, x4, x6
  ins v5.d[0], x10
  ins v5.d[1], x4
  mov x4, #1
  movk x7, #47184, lsl 48
  movk x4, #61440, lsl 16
  movk x4, #62867, lsl 32
  movk x4, #1, lsl 48
  mov x6, #41001
  mov x10, #5182
  movk x10, #38665, lsl 16
  movk x10, #34715, lsl 32
  movk x10, #4, lsl 48
  movk x6, #57649, lsl 16
  mov x11, #13288
  movk x11, #23848, lsl 16
  movk x11, #33112, lsl 32
  movk x6, #20082, lsl 32
  movk x11, #1, lsl 48
  mov x13, #23400
  movk x13, #34052, lsl 16
  movk x6, #12388, lsl 48
  movk x13, #667, lsl 32
  movk x13, #10, lsl 48
  mov x14, #57649
  mul x15, x3, x2
  movk x14, #20082, lsl 16
  movk x14, #12388, lsl 32
  mov x16, #5075556780046548992
  umulh x3, x3, x2
  dup.2d v6, x16
  mov x16, #1
  movk x16, #18032, lsl 48
  dup.2d v7, x16
  cmn x15, x8
  cinc x3, x3, hs
  ucvtf.2d v5, v5
  ucvtf d9, x4
  mov.16b v10, v6
  mul x4, x5, x2
  mov.16b v11, v7
  fmla.2d v10, v5, v9[0]
  fsub.2d v11, v11, v10
  umulh x5, x5, x2
  fmla.2d v11, v5, v9[0]
  add.2d v4, v4, v10
  add.2d v8, v8, v11
  adds x4, x4, x3
  cinc x5, x5, hs
  ucvtf d9, x10
  mov.16b v10, v6
  mov.16b v11, v7
  adds x4, x4, x9
  cinc x5, x5, hs
  fmla.2d v10, v5, v9[0]
  fsub.2d v11, v11, v10
  fmla.2d v11, v5, v9[0]
  mul x3, x7, x2
  add.2d v1, v1, v10
  add.2d v4, v4, v11
  ucvtf d9, x11
  mov.16b v10, v6
  umulh x7, x7, x2
  mov.16b v11, v7
  fmla.2d v10, v5, v9[0]
  fsub.2d v11, v11, v10
  adds x3, x3, x5
  cinc x7, x7, hs
  fmla.2d v11, v5, v9[0]
  add.2d v2, v2, v10
  add.2d v1, v1, v11
  adds x3, x3, x1
  cinc x7, x7, hs
  ucvtf d9, x13
  mov.16b v10, v6
  mov.16b v11, v7
  mul x1, x6, x2
  fmla.2d v10, v5, v9[0]
  fsub.2d v11, v11, v10
  fmla.2d v11, v5, v9[0]
  umulh x2, x6, x2
  add.2d v0, v0, v10
  add.2d v2, v2, v11
  ucvtf d9, x14
  mov.16b v6, v6
  adds x1, x1, x7
  cinc x2, x2, hs
  mov.16b v7, v7
  fmla.2d v6, v5, v9[0]
  fsub.2d v7, v7, v6
  adds x1, x1, x12
  cinc x2, x2, hs
  fmla.2d v7, v5, v9[0]
  add.2d v3, v3, v6
  add.2d v0, v0, v7
  add x0, x0, x2
  mov x2, #140737488355328
  dup.2d v5, x2
  and.16b v5, v3, v5
  mov x2, #2
  cmeq.2d v5, v5, #0
  mov x5, #2
  movk x5, #57344, lsl 16
  movk x2, #57344, lsl 16
  movk x5, #60199, lsl 32
  movk x5, #3, lsl 48
  dup.2d v6, x5
  bic.16b v6, v6, v5
  movk x2, #60199, lsl 32
  mov x5, #10364
  movk x5, #11794, lsl 16
  movk x5, #3895, lsl 32
  movk x2, #34755, lsl 48
  movk x5, #9, lsl 48
  dup.2d v7, x5
  bic.16b v7, v7, v5
  mov x5, #57634
  mov x6, #26576
  movk x6, #47696, lsl 16
  movk x6, #688, lsl 32
  movk x5, #62322, lsl 16
  movk x6, #3, lsl 48
  dup.2d v9, x6
  bic.16b v9, v9, v5
  movk x5, #53392, lsl 32
  mov x6, #46800
  movk x6, #2568, lsl 16
  movk x6, #1335, lsl 32
  movk x6, #4, lsl 48
  movk x5, #20583, lsl 48
  dup.2d v10, x6
  bic.16b v10, v10, v5
  mov x6, #49763
  mov x7, #45242
  movk x6, #40165, lsl 16
  movk x6, #24776, lsl 32
  dup.2d v11, x6
  movk x7, #770, lsl 16
  bic.16b v5, v11, v5
  mov x6, #4503599627370495
  dup.2d v11, x6
  movk x7, #35693, lsl 32
  sub.2d v4, v4, v6
  ssra.2d v4, v8, #52
  and.16b v6, v4, v11
  movk x7, #28832, lsl 48
  sub.2d v1, v1, v7
  ssra.2d v1, v4, #52
  and.16b v4, v1, v11
  sub.2d v2, v2, v9
  mov x6, #16467
  ssra.2d v2, v1, #52
  and.16b v1, v2, v11
  sub.2d v0, v0, v10
  movk x6, #49763, lsl 16
  ssra.2d v0, v2, #52
  and.16b v2, v0, v11
  sub.2d v3, v3, v5
  movk x6, #40165, lsl 32
  ssra.2d v3, v0, #52
  and.16b v0, v3, v11
  shl.2d v3, v4, #52
  movk x6, #24776, lsl 48
  shl.2d v5, v1, #40
  shl.2d v7, v2, #28
  shl.2d v0, v0, #16
  subs x2, x4, x2
  sbcs x5, x3, x5
  sbcs x7, x1, x7
  sbcs x6, x0, x6
  orr.16b v3, v6, v3
  usra.2d v5, v4, #12
  usra.2d v7, v1, #24
  usra.2d v0, v2, #36
  tst x0, #9223372036854775808
  csel x2, x2, x4, mi
  csel x3, x5, x3, mi
  csel x1, x7, x1, mi
  csel x0, x6, x0, mi
ret
