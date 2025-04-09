//in("x0") in0[0], in("x1") in0[1], in("x2") in0[2], in("x3") in0[3], in("x4") in1[0], in("x5") in1[1], in("x6") in1[2], in("x7") in1[3], in("v0") in2[0], in("v1") in2[1], in("v2") in2[2], in("v3") in2[3], in("v4") in3[0], in("v5") in3[1], in("v6") in3[2], in("v7") in3[3],
//lateout("x2") out0[0], lateout("x3") out0[1], lateout("x1") out0[2], lateout("x0") out0[3], lateout("v4") out1[0], lateout("v6") out1[1], lateout("v7") out1[2], lateout("v3") out1[3],
//lateout("v0") _, lateout("v1") _, lateout("v2") _, lateout("x4") _, lateout("x5") _, lateout("v5") _, lateout("x6") _, lateout("x7") _, lateout("x8") _, lateout("v8") _, lateout("x9") _, lateout("v9") _, lateout("x10") _, lateout("v10") _, lateout("x11") _, lateout("v11") _, lateout("x12") _, lateout("v12") _, lateout("x13") _, lateout("v13") _, lateout("x14") _, lateout("v14") _, lateout("x15") _, lateout("v15") _, lateout("x16") _, lateout("v16") _, lateout("v17") _, lateout("v18") _, lateout("v19") _, lateout("v20") _, lateout("v21") _, lateout("v22") _, lateout("v23") _, lateout("v24") _, lateout("v25") _,
//lateout("lr") _
.global _single_step_interleaved
.align 4
.text
_single_step_interleaved:
  mov x8, #4503599627370495
  dup.2d v8, x8
  shl.2d v9, v1, #14
  mul x9, x0, x4
  shl.2d v10, v2, #26
  shl.2d v11, v3, #38
  shl.2d v12, v0, #2
  umulh x10, x0, x4
  usra.2d v9, v0, #50
  usra.2d v10, v1, #38
  usra.2d v11, v2, #26
  mul x11, x1, x4
  and.16b v0, v12, v8
  and.16b v1, v9, v8
  and.16b v2, v10, v8
  umulh x12, x1, x4
  and.16b v9, v11, v8
  ushr.2d v3, v3, #14
  shl.2d v10, v5, #14
  adds x11, x11, x10
  cinc x12, x12, hs
  shl.2d v11, v6, #26
  shl.2d v12, v7, #38
  shl.2d v13, v4, #2
  mul x10, x2, x4
  usra.2d v10, v4, #50
  usra.2d v11, v5, #38
  usra.2d v12, v6, #26
  umulh x13, x2, x4
  and.16b v4, v13, v8
  and.16b v5, v10, v8
  and.16b v6, v11, v8
  adds x10, x10, x12
  cinc x13, x13, hs
  and.16b v10, v12, v8
  ushr.2d v7, v7, #14
  mov x12, #13605374474286268416
  mul x14, x3, x4
  dup.2d v11, x12
  mov x12, #6440147467139809280
  dup.2d v12, x12
  umulh x4, x3, x4
  mov x12, #3688448094816436224
  dup.2d v13, x12
  mov x12, #9209861237972664320
  adds x14, x14, x13
  cinc x4, x4, hs
  dup.2d v14, x12
  mov x12, #12218265789056155648
  dup.2d v15, x12
  mul x12, x0, x5
  mov x13, #17739678932212383744
  dup.2d v16, x13
  mov x13, #2301339409586323456
  umulh x15, x0, x5
  dup.2d v17, x13
  mov x13, #7822752552742551552
  dup.2d v18, x13
  adds x12, x12, x11
  cinc x15, x15, hs
  mov x11, #5071053180419178496
  dup.2d v19, x11
  mov x11, #16352570246982270976
  mul x13, x1, x5
  dup.2d v20, x11
  mov x11, #5075556780046548992
  dup.2d v21, x11
  umulh x11, x1, x5
  mov x16, #1
  movk x16, #18032, lsl 48
  dup.2d v22, x16
  adds x13, x13, x15
  cinc x11, x11, hs
  ucvtf.2d v0, v0
  ucvtf.2d v23, v4
  mov.16b v24, v21
  adds x13, x13, x10
  cinc x11, x11, hs
  mov.16b v25, v22
  fmla.2d v24, v0, v23
  fsub.2d v25, v25, v24
  mul x10, x2, x5
  fmla.2d v25, v0, v23
  add.2d v13, v13, v24
  add.2d v11, v11, v25
  umulh x15, x2, x5
  ucvtf.2d v23, v5
  mov.16b v24, v21
  mov.16b v25, v22
  adds x10, x10, x11
  cinc x15, x15, hs
  fmla.2d v24, v0, v23
  fsub.2d v25, v25, v24
  fmla.2d v25, v0, v23
  adds x10, x10, x14
  cinc x15, x15, hs
  add.2d v15, v15, v24
  add.2d v13, v13, v25
  ucvtf.2d v23, v6
  mov.16b v24, v21
  mul x11, x3, x5
  mov.16b v25, v22
  fmla.2d v24, v0, v23
  fsub.2d v25, v25, v24
  umulh x5, x3, x5
  fmla.2d v25, v0, v23
  add.2d v17, v17, v24
  add.2d v15, v15, v25
  adds x11, x11, x15
  cinc x5, x5, hs
  ucvtf.2d v23, v10
  mov.16b v24, v21
  mov.16b v25, v22
  adds x11, x11, x4
  cinc x5, x5, hs
  fmla.2d v24, v0, v23
  fsub.2d v25, v25, v24
  fmla.2d v25, v0, v23
  mul x4, x0, x6
  add.2d v19, v19, v24
  add.2d v17, v17, v25
  ucvtf.2d v23, v7
  umulh x14, x0, x6
  mov.16b v24, v21
  mov.16b v25, v22
  fmla.2d v24, v0, v23
  adds x4, x4, x13
  cinc x14, x14, hs
  fsub.2d v25, v25, v24
  fmla.2d v25, v0, v23
  add.2d v0, v20, v24
  mul x13, x1, x6
  add.2d v19, v19, v25
  ucvtf.2d v1, v1
  ucvtf.2d v20, v4
  umulh x15, x1, x6
  mov.16b v23, v21
  mov.16b v24, v22
  fmla.2d v23, v1, v20
  adds x13, x13, x14
  cinc x15, x15, hs
  fsub.2d v24, v24, v23
  fmla.2d v24, v1, v20
  add.2d v15, v15, v23
  adds x13, x13, x10
  cinc x15, x15, hs
  add.2d v13, v13, v24
  ucvtf.2d v20, v5
  mov.16b v23, v21
  mul x10, x2, x6
  mov.16b v24, v22
  fmla.2d v23, v1, v20
  fsub.2d v24, v24, v23
  umulh x14, x2, x6
  fmla.2d v24, v1, v20
  add.2d v17, v17, v23
  add.2d v15, v15, v24
  adds x10, x10, x15
  cinc x14, x14, hs
  ucvtf.2d v20, v6
  mov.16b v23, v21
  mov.16b v24, v22
  adds x10, x10, x11
  cinc x14, x14, hs
  fmla.2d v23, v1, v20
  fsub.2d v24, v24, v23
  fmla.2d v24, v1, v20
  mul x11, x3, x6
  add.2d v19, v19, v23
  add.2d v17, v17, v24
  ucvtf.2d v20, v10
  umulh x6, x3, x6
  mov.16b v23, v21
  mov.16b v24, v22
  fmla.2d v23, v1, v20
  adds x11, x11, x14
  cinc x6, x6, hs
  fsub.2d v24, v24, v23
  fmla.2d v24, v1, v20
  add.2d v0, v0, v23
  adds x11, x11, x5
  cinc x6, x6, hs
  add.2d v19, v19, v24
  ucvtf.2d v20, v7
  mov.16b v23, v21
  mul x5, x0, x7
  mov.16b v24, v22
  fmla.2d v23, v1, v20
  fsub.2d v24, v24, v23
  umulh x0, x0, x7
  fmla.2d v24, v1, v20
  add.2d v1, v18, v23
  add.2d v0, v0, v24
  adds x5, x5, x13
  cinc x0, x0, hs
  ucvtf.2d v2, v2
  ucvtf.2d v18, v4
  mov.16b v20, v21
  mul x13, x1, x7
  mov.16b v23, v22
  fmla.2d v20, v2, v18
  fsub.2d v23, v23, v20
  fmla.2d v23, v2, v18
  umulh x1, x1, x7
  add.2d v17, v17, v20
  add.2d v15, v15, v23
  ucvtf.2d v18, v5
  adds x13, x13, x0
  cinc x1, x1, hs
  mov.16b v20, v21
  mov.16b v23, v22
  fmla.2d v20, v2, v18
  adds x13, x13, x10
  cinc x1, x1, hs
  fsub.2d v23, v23, v20
  fmla.2d v23, v2, v18
  add.2d v18, v19, v20
  mul x0, x2, x7
  add.2d v17, v17, v23
  ucvtf.2d v19, v6
  mov.16b v20, v21
  umulh x2, x2, x7
  mov.16b v23, v22
  fmla.2d v20, v2, v19
  fsub.2d v23, v23, v20
  adds x0, x0, x1
  cinc x2, x2, hs
  fmla.2d v23, v2, v19
  add.2d v0, v0, v20
  add.2d v18, v18, v23
  adds x0, x0, x11
  cinc x2, x2, hs
  ucvtf.2d v19, v10
  mov.16b v20, v21
  mov.16b v23, v22
  mul x1, x3, x7
  fmla.2d v20, v2, v19
  fsub.2d v23, v23, v20
  fmla.2d v23, v2, v19
  umulh x3, x3, x7
  add.2d v1, v1, v20
  add.2d v0, v0, v23
  ucvtf.2d v19, v7
  adds x1, x1, x2
  cinc x3, x3, hs
  mov.16b v20, v21
  mov.16b v23, v22
  fmla.2d v20, v2, v19
  adds x1, x1, x6
  cinc x3, x3, hs
  fsub.2d v23, v23, v20
  fmla.2d v23, v2, v19
  add.2d v2, v16, v20
  mov x2, #48718
  add.2d v1, v1, v23
  ucvtf.2d v9, v9
  ucvtf.2d v16, v4
  movk x2, #4732, lsl 16
  mov.16b v19, v21
  mov.16b v20, v22
  fmla.2d v19, v9, v16
  movk x2, #45078, lsl 32
  fsub.2d v20, v20, v19
  fmla.2d v20, v9, v16
  add.2d v16, v18, v19
  movk x2, #39852, lsl 48
  add.2d v17, v17, v20
  ucvtf.2d v18, v5
  mov.16b v19, v21
  mov x6, #16676
  mov.16b v20, v22
  fmla.2d v19, v9, v18
  fsub.2d v20, v20, v19
  movk x6, #12692, lsl 16
  fmla.2d v20, v9, v18
  add.2d v0, v0, v19
  add.2d v16, v16, v20
  movk x6, #20986, lsl 32
  ucvtf.2d v18, v6
  mov.16b v19, v21
  mov.16b v20, v22
  movk x6, #2848, lsl 48
  fmla.2d v19, v9, v18
  fsub.2d v20, v20, v19
  fmla.2d v20, v9, v18
  mov x7, #51052
  add.2d v1, v1, v19
  add.2d v0, v0, v20
  ucvtf.2d v18, v10
  movk x7, #24721, lsl 16
  mov.16b v19, v21
  mov.16b v20, v22
  fmla.2d v19, v9, v18
  movk x7, #61092, lsl 32
  fsub.2d v20, v20, v19
  fmla.2d v20, v9, v18
  add.2d v2, v2, v19
  movk x7, #45156, lsl 48
  add.2d v1, v1, v20
  ucvtf.2d v18, v7
  mov.16b v19, v21
  mov.16b v20, v22
  mov x10, #3197
  fmla.2d v19, v9, v18
  fsub.2d v20, v20, v19
  fmla.2d v20, v9, v18
  movk x10, #18936, lsl 16
  add.2d v9, v14, v19
  add.2d v2, v2, v20
  ucvtf.2d v3, v3
  movk x10, #10922, lsl 32
  ucvtf.2d v4, v4
  mov.16b v14, v21
  mov.16b v18, v22
  movk x10, #11014, lsl 48
  fmla.2d v14, v3, v4
  fsub.2d v18, v18, v14
  fmla.2d v18, v3, v4
  mul x11, x2, x9
  add.2d v0, v0, v14
  add.2d v4, v16, v18
  ucvtf.2d v5, v5
  umulh x2, x2, x9
  mov.16b v14, v21
  mov.16b v16, v22
  fmla.2d v14, v3, v5
  adds x11, x11, x5
  cinc x2, x2, hs
  fsub.2d v16, v16, v14
  fmla.2d v16, v3, v5
  add.2d v1, v1, v14
  mul x5, x6, x9
  add.2d v0, v0, v16
  ucvtf.2d v5, v6
  mov.16b v6, v21
  umulh x6, x6, x9
  mov.16b v14, v22
  fmla.2d v6, v3, v5
  fsub.2d v14, v14, v6
  adds x5, x5, x2
  cinc x6, x6, hs
  fmla.2d v14, v3, v5
  add.2d v2, v2, v6
  add.2d v1, v1, v14
  adds x5, x5, x13
  cinc x6, x6, hs
  ucvtf.2d v5, v10
  mov.16b v6, v21
  mov.16b v10, v22
  mul x2, x7, x9
  fmla.2d v6, v3, v5
  fsub.2d v10, v10, v6
  fmla.2d v10, v3, v5
  umulh x7, x7, x9
  add.2d v5, v9, v6
  add.2d v2, v2, v10
  ucvtf.2d v6, v7
  adds x2, x2, x6
  cinc x7, x7, hs
  mov.16b v7, v21
  mov.16b v9, v22
  fmla.2d v7, v3, v6
  adds x2, x2, x0
  cinc x7, x7, hs
  fsub.2d v9, v9, v7
  fmla.2d v9, v3, v6
  add.2d v3, v12, v7
  mul x0, x10, x9
  add.2d v5, v5, v9
  usra.2d v13, v11, #52
  usra.2d v15, v13, #52
  umulh x6, x10, x9
  usra.2d v17, v15, #52
  usra.2d v4, v17, #52
  and.16b v6, v11, v8
  adds x0, x0, x7
  cinc x6, x6, hs
  and.16b v7, v13, v8
  and.16b v9, v15, v8
  and.16b v10, v17, v8
  adds x0, x0, x1
  cinc x6, x6, hs
  mov x1, #62077
  movk x1, #226, lsl 16
  movk x1, #11812, lsl 32
  add x3, x3, x6
  movk x1, #2, lsl 48
  mov x6, #38534
  movk x6, #28321, lsl 16
  mov x7, #56431
  movk x6, #9140, lsl 32
  movk x6, #3, lsl 48
  mov x9, #26413
  movk x7, #30457, lsl 16
  movk x9, #61663, lsl 16
  movk x9, #27715, lsl 32
  movk x9, #14, lsl 48
  movk x7, #30012, lsl 32
  mov x10, #40587
  movk x10, #5315, lsl 16
  movk x10, #33344, lsl 32
  movk x10, #7, lsl 48
  movk x7, #6382, lsl 48
  mov x13, #57784
  movk x13, #18607, lsl 16
  movk x13, #3179, lsl 32
  mov x14, #59151
  ucvtf.2d v6, v6
  ucvtf d11, x1
  mov.16b v12, v21
  movk x14, #41769, lsl 16
  mov.16b v13, v22
  fmla.2d v12, v6, v11[0]
  fsub.2d v13, v13, v12
  movk x14, #32276, lsl 32
  fmla.2d v13, v6, v11[0]
  add.2d v0, v0, v12
  add.2d v4, v4, v13
  movk x14, #21677, lsl 48
  ucvtf d11, x6
  mov.16b v12, v21
  mov.16b v13, v22
  mov x1, #34015
  fmla.2d v12, v6, v11[0]
  fsub.2d v13, v13, v12
  fmla.2d v13, v6, v11[0]
  movk x1, #20342, lsl 16
  add.2d v1, v1, v12
  add.2d v0, v0, v13
  ucvtf d11, x9
  movk x1, #13935, lsl 32
  mov.16b v12, v21
  mov.16b v13, v22
  fmla.2d v12, v6, v11[0]
  movk x1, #11030, lsl 48
  fsub.2d v13, v13, v12
  fmla.2d v13, v6, v11[0]
  add.2d v2, v2, v12
  mov x6, #13689
  add.2d v1, v1, v13
  ucvtf d11, x10
  mov.16b v12, v21
  movk x6, #8159, lsl 16
  mov.16b v13, v22
  fmla.2d v12, v6, v11[0]
  fsub.2d v13, v13, v12
  movk x6, #215, lsl 32
  fmla.2d v13, v6, v11[0]
  add.2d v5, v5, v12
  add.2d v2, v2, v13
  movk x6, #4913, lsl 48
  ucvtf d11, x13
  mov.16b v12, v21
  mov.16b v13, v22
  mul x9, x7, x12
  fmla.2d v12, v6, v11[0]
  fsub.2d v13, v13, v12
  fmla.2d v13, v6, v11[0]
  umulh x7, x7, x12
  add.2d v3, v3, v12
  add.2d v5, v5, v13
  mov x10, #32119
  adds x9, x9, x11
  cinc x7, x7, hs
  movk x10, #22102, lsl 16
  movk x10, #59590, lsl 32
  mov x11, #44641
  mul x13, x14, x12
  movk x11, #22291, lsl 16
  movk x11, #12496, lsl 32
  movk x11, #4, lsl 48
  umulh x14, x14, x12
  mov x15, #28968
  movk x15, #27414, lsl 16
  movk x15, #41914, lsl 32
  adds x13, x13, x7
  cinc x14, x14, hs
  movk x15, #14, lsl 48
  mov x7, #41622
  movk x7, #21957, lsl 16
  adds x13, x13, x5
  cinc x14, x14, hs
  movk x7, #32174, lsl 32
  movk x7, #10, lsl 48
  mov x5, #13682
  mul x16, x1, x12
  movk x5, #64849, lsl 16
  movk x5, #6986, lsl 32
  ucvtf.2d v6, v7
  umulh x1, x1, x12
  ucvtf d7, x10
  mov.16b v11, v21
  mov.16b v12, v22
  adds x16, x16, x14
  cinc x1, x1, hs
  fmla.2d v11, v6, v7[0]
  fsub.2d v12, v12, v11
  fmla.2d v12, v6, v7[0]
  add.2d v0, v0, v11
  adds x16, x16, x2
  cinc x1, x1, hs
  add.2d v4, v4, v12
  ucvtf d7, x11
  mov.16b v11, v21
  mul x2, x6, x12
  mov.16b v12, v22
  fmla.2d v11, v6, v7[0]
  fsub.2d v12, v12, v11
  umulh x6, x6, x12
  fmla.2d v12, v6, v7[0]
  add.2d v1, v1, v11
  add.2d v0, v0, v12
  adds x2, x2, x1
  cinc x6, x6, hs
  ucvtf d7, x15
  mov.16b v11, v21
  mov.16b v12, v22
  adds x2, x2, x0
  cinc x6, x6, hs
  fmla.2d v11, v6, v7[0]
  fsub.2d v12, v12, v11
  fmla.2d v12, v6, v7[0]
  add x0, x3, x6
  add.2d v2, v2, v11
  add.2d v1, v1, v12
  ucvtf d7, x7
  mov x1, #61005
  mov.16b v11, v21
  mov.16b v12, v22
  fmla.2d v11, v6, v7[0]
  movk x1, #58262, lsl 16
  fsub.2d v12, v12, v11
  fmla.2d v12, v6, v7[0]
  add.2d v5, v5, v11
  movk x1, #32851, lsl 32
  add.2d v2, v2, v12
  ucvtf d7, x5
  mov.16b v11, v21
  movk x1, #11582, lsl 48
  mov.16b v12, v22
  fmla.2d v11, v6, v7[0]
  fsub.2d v12, v12, v11
  mov x3, #37581
  fmla.2d v12, v6, v7[0]
  add.2d v3, v3, v11
  add.2d v5, v5, v12
  movk x3, #43836, lsl 16
  mov x5, #31098
  movk x5, #52890, lsl 16
  movk x5, #20172, lsl 32
  movk x3, #36286, lsl 32
  movk x5, #7, lsl 48
  mov x6, #55460
  movk x6, #49931, lsl 16
  movk x3, #51783, lsl 48
  movk x6, #28124, lsl 32
  movk x6, #1, lsl 48
  mov x7, #18846
  mov x10, #10899
  movk x7, #13625, lsl 16
  movk x7, #40653, lsl 32
  movk x7, #4, lsl 48
  movk x10, #30709, lsl 16
  mov x11, #37560
  movk x11, #64709, lsl 16
  movk x11, #9126, lsl 32
  movk x10, #61551, lsl 32
  movk x11, #11, lsl 48
  mov x12, #28389
  movk x12, #54431, lsl 16
  movk x10, #45784, lsl 48
  movk x12, #3643, lsl 32
  ucvtf.2d v6, v9
  ucvtf d7, x5
  mov x5, #36612
  mov.16b v9, v21
  mov.16b v11, v22
  fmla.2d v9, v6, v7[0]
  movk x5, #63402, lsl 16
  fsub.2d v11, v11, v9
  fmla.2d v11, v6, v7[0]
  add.2d v0, v0, v9
  movk x5, #47623, lsl 32
  add.2d v4, v4, v11
  ucvtf d7, x6
  mov.16b v9, v21
  movk x5, #9430, lsl 48
  mov.16b v11, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v11, v11, v9
  mul x6, x1, x4
  fmla.2d v11, v6, v7[0]
  add.2d v1, v1, v9
  add.2d v0, v0, v11
  ucvtf d7, x7
  umulh x1, x1, x4
  mov.16b v9, v21
  mov.16b v11, v22
  fmla.2d v9, v6, v7[0]
  adds x6, x6, x9
  cinc x1, x1, hs
  fsub.2d v11, v11, v9
  fmla.2d v11, v6, v7[0]
  add.2d v2, v2, v9
  mul x7, x3, x4
  add.2d v1, v1, v11
  ucvtf d7, x11
  mov.16b v9, v21
  umulh x3, x3, x4
  mov.16b v11, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v11, v11, v9
  adds x7, x7, x1
  cinc x3, x3, hs
  fmla.2d v11, v6, v7[0]
  add.2d v5, v5, v9
  add.2d v2, v2, v11
  adds x7, x7, x13
  cinc x3, x3, hs
  ucvtf d7, x12
  mov.16b v9, v21
  mov.16b v11, v22
  mul x1, x10, x4
  fmla.2d v9, v6, v7[0]
  fsub.2d v11, v11, v9
  fmla.2d v11, v6, v7[0]
  umulh x9, x10, x4
  add.2d v3, v3, v9
  add.2d v5, v5, v11
  mov x10, #50130
  adds x1, x1, x3
  cinc x9, x9, hs
  movk x10, #20196, lsl 16
  movk x10, #11876, lsl 32
  movk x10, #8, lsl 48
  adds x1, x1, x16
  cinc x9, x9, hs
  mov x3, #45534
  movk x3, #15512, lsl 16
  movk x3, #37769, lsl 32
  mul x11, x5, x4
  movk x3, #15, lsl 48
  mov x12, #42183
  movk x12, #1232, lsl 16
  umulh x4, x5, x4
  movk x12, #18174, lsl 32
  movk x12, #13, lsl 48
  mov x5, #10783
  adds x11, x11, x9
  cinc x4, x4, hs
  movk x5, #54622, lsl 16
  movk x5, #61610, lsl 32
  movk x5, #8, lsl 48
  adds x11, x11, x2
  cinc x4, x4, hs
  mov x2, #56963
  movk x2, #1095, lsl 16
  movk x2, #1517, lsl 32
  add x0, x0, x4
  ucvtf.2d v6, v10
  ucvtf d7, x10
  mov.16b v9, v21
  mov x4, #65535
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  movk x4, #61439, lsl 16
  fmla.2d v10, v6, v7[0]
  add.2d v0, v0, v9
  add.2d v4, v4, v10
  movk x4, #62867, lsl 32
  ucvtf d7, x3
  mov.16b v9, v21
  mov.16b v10, v22
  movk x4, #49889, lsl 48
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  mul x3, x4, x6
  add.2d v1, v1, v9
  add.2d v0, v0, v10
  ucvtf d7, x12
  mov x4, #1
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  movk x4, #61440, lsl 16
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  add.2d v2, v2, v9
  movk x4, #62867, lsl 32
  add.2d v1, v1, v10
  ucvtf d7, x5
  mov.16b v9, v21
  mov.16b v10, v22
  movk x4, #17377, lsl 48
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  mov x5, #28817
  add.2d v5, v5, v9
  add.2d v2, v2, v10
  ucvtf d7, x2
  movk x5, #31161, lsl 16
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  movk x5, #59464, lsl 32
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  add.2d v3, v3, v9
  movk x5, #10291, lsl 48
  add.2d v5, v5, v10
  mov x2, #65535
  movk x2, #61439, lsl 16
  mov x9, #22621
  movk x2, #62867, lsl 32
  movk x2, #1, lsl 48
  umov x10, v4.d[0]
  movk x9, #33153, lsl 16
  umov x12, v4.d[1]
  mul x10, x10, x2
  mul x2, x12, x2
  movk x9, #17846, lsl 32
  and x10, x10, x8
  and x2, x2, x8
  ins v6.d[0], x10
  ins v6.d[1], x2
  movk x9, #47184, lsl 48
  mov x2, #1
  movk x2, #61440, lsl 16
  movk x2, #62867, lsl 32
  mov x8, #41001
  movk x2, #1, lsl 48
  mov x10, #5182
  movk x10, #38665, lsl 16
  movk x8, #57649, lsl 16
  movk x10, #34715, lsl 32
  movk x10, #4, lsl 48
  mov x12, #13288
  movk x8, #20082, lsl 32
  movk x12, #23848, lsl 16
  movk x12, #33112, lsl 32
  movk x12, #1, lsl 48
  movk x8, #12388, lsl 48
  mov x13, #23400
  movk x13, #34052, lsl 16
  movk x13, #667, lsl 32
  mul x14, x4, x3
  movk x13, #10, lsl 48
  mov x15, #57649
  movk x15, #20082, lsl 16
  umulh x4, x4, x3
  movk x15, #12388, lsl 32
  ucvtf.2d v6, v6
  ucvtf d7, x2
  cmn x14, x6
  cinc x4, x4, hs
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  mul x2, x5, x3
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  add.2d v0, v0, v9
  umulh x5, x5, x3
  add.2d v4, v4, v10
  ucvtf d7, x10
  mov.16b v9, v21
  adds x2, x2, x4
  cinc x5, x5, hs
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  adds x2, x2, x7
  cinc x5, x5, hs
  fmla.2d v10, v6, v7[0]
  add.2d v1, v1, v9
  add.2d v0, v0, v10
  mul x4, x9, x3
  ucvtf d7, x12
  mov.16b v9, v21
  mov.16b v10, v22
  umulh x6, x9, x3
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  adds x4, x4, x5
  cinc x6, x6, hs
  add.2d v2, v2, v9
  add.2d v1, v1, v10
  ucvtf d7, x13
  mov.16b v9, v21
  adds x4, x4, x1
  cinc x6, x6, hs
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  mul x1, x8, x3
  fmla.2d v10, v6, v7[0]
  add.2d v5, v5, v9
  add.2d v2, v2, v10
  umulh x3, x8, x3
  ucvtf d7, x15
  mov.16b v9, v21
  mov.16b v10, v22
  adds x1, x1, x6
  cinc x3, x3, hs
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  adds x1, x1, x11
  cinc x3, x3, hs
  add.2d v3, v3, v9
  add.2d v5, v5, v10
  mov x5, #140737488355328
  add x0, x0, x3
  dup.2d v6, x5
  and.16b v6, v3, v6
  cmeq.2d v6, v6, #0
  mov x3, #2
  mov x5, #2
  movk x5, #57344, lsl 16
  movk x5, #60199, lsl 32
  movk x3, #57344, lsl 16
  movk x5, #3, lsl 48
  dup.2d v7, x5
  bic.16b v7, v7, v6
  movk x3, #60199, lsl 32
  mov x5, #10364
  movk x5, #11794, lsl 16
  movk x5, #3895, lsl 32
  movk x3, #34755, lsl 48
  movk x5, #9, lsl 48
  dup.2d v9, x5
  bic.16b v9, v9, v6
  mov x5, #57634
  mov x6, #26576
  movk x6, #47696, lsl 16
  movk x6, #688, lsl 32
  movk x5, #62322, lsl 16
  movk x6, #3, lsl 48
  dup.2d v10, x6
  bic.16b v10, v10, v6
  movk x5, #53392, lsl 32
  mov x6, #46800
  movk x6, #2568, lsl 16
  movk x6, #1335, lsl 32
  movk x5, #20583, lsl 48
  movk x6, #4, lsl 48
  dup.2d v11, x6
  bic.16b v11, v11, v6
  mov x6, #45242
  mov x7, #49763
  movk x7, #40165, lsl 16
  movk x7, #24776, lsl 32
  movk x6, #770, lsl 16
  dup.2d v12, x7
  bic.16b v6, v12, v6
  sub.2d v0, v0, v7
  movk x6, #35693, lsl 32
  ssra.2d v0, v4, #52
  and.16b v4, v0, v8
  sub.2d v1, v1, v9
  movk x6, #28832, lsl 48
  ssra.2d v1, v0, #52
  and.16b v0, v1, v8
  sub.2d v2, v2, v10
  mov x7, #16467
  ssra.2d v2, v1, #52
  and.16b v1, v2, v8
  sub.2d v5, v5, v11
  movk x7, #49763, lsl 16
  ssra.2d v5, v2, #52
  and.16b v2, v5, v8
  sub.2d v3, v3, v6
  movk x7, #40165, lsl 32
  ssra.2d v3, v5, #52
  and.16b v3, v3, v8
  shl.2d v5, v0, #52
  movk x7, #24776, lsl 48
  shl.2d v6, v1, #40
  shl.2d v7, v2, #28
  shl.2d v3, v3, #16
  subs x3, x2, x3
  sbcs x5, x4, x5
  sbcs x6, x1, x6
  sbcs x7, x0, x7
  orr.16b v4, v4, v5
  usra.2d v6, v0, #12
  usra.2d v7, v1, #24
  usra.2d v3, v2, #36
  tst x0, #9223372036854775808
  csel x2, x3, x2, mi
  csel x3, x5, x4, mi
  csel x1, x6, x1, mi
  csel x0, x7, x0, mi
ret
