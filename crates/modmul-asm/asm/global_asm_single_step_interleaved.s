//in("x0") in0[0], in("x1") in0[1], in("x2") in0[2], in("x3") in0[3], in("x4") in1[0], in("x5") in1[1], in("x6") in1[2], in("x7") in1[3], in("v0") in2[0], in("v1") in2[1], in("v2") in2[2], in("v3") in2[3], in("v4") in3[0], in("v5") in3[1], in("v6") in3[2], in("v7") in3[3],
//lateout("x4") out0[0], lateout("x2") out0[1], lateout("x3") out0[2], lateout("x0") out0[3], lateout("v4") out1[0], lateout("v6") out1[1], lateout("v7") out1[2], lateout("v3") out1[3],
//lateout("v0") _, lateout("x1") _, lateout("v1") _, lateout("v2") _, lateout("x5") _, lateout("v5") _, lateout("x6") _, lateout("x7") _, lateout("x8") _, lateout("v8") _, lateout("x9") _, lateout("v9") _, lateout("x10") _, lateout("v10") _, lateout("x11") _, lateout("v11") _, lateout("x12") _, lateout("v12") _, lateout("x13") _, lateout("v13") _, lateout("x14") _, lateout("v14") _, lateout("x15") _, lateout("v15") _, lateout("x16") _, lateout("v16") _, lateout("x17") _, lateout("v17") _, lateout("v18") _, lateout("v19") _, lateout("v20") _, lateout("v21") _, lateout("v22") _, lateout("v23") _, lateout("v24") _,
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
  mul x10, x2, x5
  ucvtf.2d v3, v3
  ucvtf.2d v4, v4
  ucvtf.2d v5, v5
  umulh x13, x2, x5
  ucvtf.2d v6, v6
  ucvtf.2d v10, v10
  ucvtf.2d v7, v7
  adds x10, x10, x16
  cinc x13, x13, hs
  mov.16b v23, v21
  mov.16b v24, v22
  fmla.2d v23, v0, v4
  adds x10, x10, x12
  cinc x13, x13, hs
  fsub.2d v24, v24, v23
  fmla.2d v24, v0, v4
  add.2d v13, v13, v23
  mul x12, x3, x5
  add.2d v11, v11, v24
  mov.16b v23, v21
  mov.16b v24, v22
  umulh x5, x3, x5
  fmla.2d v23, v0, v5
  fsub.2d v24, v24, v23
  fmla.2d v24, v0, v5
  adds x12, x12, x13
  cinc x5, x5, hs
  add.2d v15, v15, v23
  add.2d v13, v13, v24
  mov.16b v23, v21
  adds x12, x12, x4
  cinc x5, x5, hs
  mov.16b v24, v22
  fmla.2d v23, v0, v6
  fsub.2d v24, v24, v23
  mul x4, x0, x6
  fmla.2d v24, v0, v6
  add.2d v17, v17, v23
  add.2d v15, v15, v24
  umulh x13, x0, x6
  mov.16b v23, v21
  mov.16b v24, v22
  fmla.2d v23, v0, v10
  adds x4, x4, x11
  cinc x13, x13, hs
  fsub.2d v24, v24, v23
  fmla.2d v24, v0, v10
  add.2d v19, v19, v23
  mul x11, x1, x6
  add.2d v17, v17, v24
  mov.16b v23, v21
  mov.16b v24, v22
  umulh x15, x1, x6
  fmla.2d v23, v0, v7
  fsub.2d v24, v24, v23
  fmla.2d v24, v0, v7
  adds x11, x11, x13
  cinc x15, x15, hs
  add.2d v0, v20, v23
  add.2d v19, v19, v24
  mov.16b v20, v21
  adds x11, x11, x10
  cinc x15, x15, hs
  mov.16b v23, v22
  fmla.2d v20, v1, v4
  fsub.2d v23, v23, v20
  mul x10, x2, x6
  fmla.2d v23, v1, v4
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
  mul x12, x3, x6
  mov.16b v23, v22
  fmla.2d v20, v1, v6
  fsub.2d v23, v23, v20
  umulh x6, x3, x6
  fmla.2d v23, v1, v6
  add.2d v19, v19, v20
  add.2d v17, v17, v23
  adds x12, x12, x13
  cinc x6, x6, hs
  mov.16b v20, v21
  mov.16b v23, v22
  fmla.2d v20, v1, v10
  adds x12, x12, x5
  cinc x6, x6, hs
  fsub.2d v23, v23, v20
  fmla.2d v23, v1, v10
  add.2d v0, v0, v20
  mul x5, x0, x7
  add.2d v19, v19, v23
  mov.16b v20, v21
  mov.16b v23, v22
  umulh x0, x0, x7
  fmla.2d v20, v1, v7
  fsub.2d v23, v23, v20
  fmla.2d v23, v1, v7
  adds x5, x5, x11
  cinc x0, x0, hs
  add.2d v1, v18, v20
  add.2d v0, v0, v23
  mov.16b v18, v21
  mul x11, x1, x7
  mov.16b v20, v22
  fmla.2d v18, v2, v4
  fsub.2d v20, v20, v18
  umulh x1, x1, x7
  fmla.2d v20, v2, v4
  add.2d v17, v17, v18
  add.2d v15, v15, v20
  adds x11, x11, x0
  cinc x1, x1, hs
  mov.16b v18, v21
  mov.16b v20, v22
  fmla.2d v18, v2, v5
  adds x11, x11, x10
  cinc x1, x1, hs
  fsub.2d v20, v20, v18
  fmla.2d v20, v2, v5
  add.2d v18, v19, v18
  mul x0, x2, x7
  add.2d v17, v17, v20
  mov.16b v19, v21
  mov.16b v20, v22
  umulh x2, x2, x7
  fmla.2d v19, v2, v6
  fsub.2d v20, v20, v19
  fmla.2d v20, v2, v6
  adds x0, x0, x1
  cinc x2, x2, hs
  add.2d v0, v0, v19
  add.2d v18, v18, v20
  mov.16b v19, v21
  adds x0, x0, x12
  cinc x2, x2, hs
  mov.16b v20, v22
  fmla.2d v19, v2, v10
  fsub.2d v20, v20, v19
  mul x1, x3, x7
  fmla.2d v20, v2, v10
  add.2d v1, v1, v19
  add.2d v0, v0, v20
  umulh x3, x3, x7
  mov.16b v19, v21
  mov.16b v20, v22
  fmla.2d v19, v2, v7
  adds x1, x1, x2
  cinc x3, x3, hs
  fsub.2d v20, v20, v19
  fmla.2d v20, v2, v7
  add.2d v2, v16, v19
  adds x1, x1, x6
  cinc x3, x3, hs
  add.2d v1, v1, v20
  mov.16b v16, v21
  mov x2, #48718
  mov.16b v19, v22
  fmla.2d v16, v9, v4
  fsub.2d v19, v19, v16
  movk x2, #4732, lsl 16
  fmla.2d v19, v9, v4
  add.2d v16, v18, v16
  add.2d v17, v17, v19
  movk x2, #45078, lsl 32
  mov.16b v18, v21
  mov.16b v19, v22
  fmla.2d v18, v9, v5
  movk x2, #39852, lsl 48
  fsub.2d v19, v19, v18
  fmla.2d v19, v9, v5
  add.2d v0, v0, v18
  mov x6, #16676
  add.2d v16, v16, v19
  mov.16b v18, v21
  mov.16b v19, v22
  movk x6, #12692, lsl 16
  fmla.2d v18, v9, v6
  fsub.2d v19, v19, v18
  fmla.2d v19, v9, v6
  movk x6, #20986, lsl 32
  add.2d v1, v1, v18
  add.2d v0, v0, v19
  mov.16b v18, v21
  movk x6, #2848, lsl 48
  mov.16b v19, v22
  fmla.2d v18, v9, v10
  fsub.2d v19, v19, v18
  mov x7, #51052
  fmla.2d v19, v9, v10
  add.2d v2, v2, v18
  add.2d v1, v1, v19
  movk x7, #24721, lsl 16
  mov.16b v18, v21
  mov.16b v19, v22
  fmla.2d v18, v9, v7
  movk x7, #61092, lsl 32
  fsub.2d v19, v19, v18
  fmla.2d v19, v9, v7
  add.2d v9, v14, v18
  movk x7, #45156, lsl 48
  add.2d v2, v2, v19
  mov.16b v14, v21
  mov.16b v18, v22
  mov x10, #3197
  fmla.2d v14, v3, v4
  fsub.2d v18, v18, v14
  fmla.2d v18, v3, v4
  movk x10, #18936, lsl 16
  add.2d v0, v0, v14
  add.2d v4, v16, v18
  mov.16b v14, v21
  movk x10, #10922, lsl 32
  mov.16b v16, v22
  fmla.2d v14, v3, v5
  fsub.2d v16, v16, v14
  movk x10, #11014, lsl 48
  fmla.2d v16, v3, v5
  add.2d v1, v1, v14
  add.2d v0, v0, v16
  mul x12, x2, x9
  mov.16b v5, v21
  mov.16b v14, v22
  fmla.2d v5, v3, v6
  umulh x2, x2, x9
  fsub.2d v14, v14, v5
  fmla.2d v14, v3, v6
  add.2d v2, v2, v5
  adds x12, x12, x5
  cinc x2, x2, hs
  add.2d v1, v1, v14
  mov.16b v5, v21
  mul x5, x6, x9
  mov.16b v6, v22
  fmla.2d v5, v3, v10
  fsub.2d v6, v6, v5
  umulh x6, x6, x9
  fmla.2d v6, v3, v10
  add.2d v5, v9, v5
  add.2d v2, v2, v6
  adds x5, x5, x2
  cinc x6, x6, hs
  mov.16b v6, v21
  mov.16b v9, v22
  fmla.2d v6, v3, v7
  adds x5, x5, x11
  cinc x6, x6, hs
  fsub.2d v9, v9, v6
  fmla.2d v9, v3, v7
  add.2d v3, v12, v6
  mul x2, x7, x9
  add.2d v5, v5, v9
  usra.2d v13, v11, #52
  usra.2d v15, v13, #52
  umulh x7, x7, x9
  usra.2d v17, v15, #52
  usra.2d v4, v17, #52
  and.16b v6, v11, v8
  adds x2, x2, x6
  cinc x7, x7, hs
  and.16b v7, v13, v8
  and.16b v9, v15, v8
  and.16b v10, v17, v8
  adds x2, x2, x0
  cinc x7, x7, hs
  mov x0, #62077
  movk x0, #226, lsl 16
  movk x0, #11812, lsl 32
  mul x6, x10, x9
  movk x0, #2, lsl 48
  mov x11, #38534
  movk x11, #28321, lsl 16
  umulh x9, x10, x9
  movk x11, #9140, lsl 32
  movk x11, #3, lsl 48
  mov x10, #26413
  adds x6, x6, x7
  cinc x9, x9, hs
  movk x10, #61663, lsl 16
  movk x10, #27715, lsl 32
  movk x10, #14, lsl 48
  adds x6, x6, x1
  cinc x9, x9, hs
  mov x1, #40587
  movk x1, #5315, lsl 16
  movk x1, #33344, lsl 32
  add x3, x3, x9
  movk x1, #7, lsl 48
  mov x7, #57784
  movk x7, #18607, lsl 16
  mov x9, #56431
  movk x7, #3179, lsl 32
  ucvtf.2d v6, v6
  ucvtf d11, x0
  movk x9, #30457, lsl 16
  mov.16b v12, v21
  mov.16b v13, v22
  fmla.2d v12, v6, v11[0]
  movk x9, #30012, lsl 32
  fsub.2d v13, v13, v12
  fmla.2d v13, v6, v11[0]
  add.2d v0, v0, v12
  movk x9, #6382, lsl 48
  add.2d v4, v4, v13
  ucvtf d11, x11
  mov.16b v12, v21
  mov x0, #59151
  mov.16b v13, v22
  fmla.2d v12, v6, v11[0]
  fsub.2d v13, v13, v12
  movk x0, #41769, lsl 16
  fmla.2d v13, v6, v11[0]
  add.2d v1, v1, v12
  movk x0, #32276, lsl 32
  add.2d v0, v0, v13
  ucvtf d11, x10
  mov.16b v12, v21
  movk x0, #21677, lsl 48
  mov.16b v13, v22
  fmla.2d v12, v6, v11[0]
  fsub.2d v13, v13, v12
  mov x10, #34015
  fmla.2d v13, v6, v11[0]
  add.2d v2, v2, v12
  add.2d v1, v1, v13
  movk x10, #20342, lsl 16
  ucvtf d11, x1
  mov.16b v12, v21
  mov.16b v13, v22
  movk x10, #13935, lsl 32
  fmla.2d v12, v6, v11[0]
  fsub.2d v13, v13, v12
  fmla.2d v13, v6, v11[0]
  movk x10, #11030, lsl 48
  add.2d v5, v5, v12
  add.2d v2, v2, v13
  ucvtf d11, x7
  mov x1, #13689
  mov.16b v12, v21
  mov.16b v13, v22
  fmla.2d v12, v6, v11[0]
  movk x1, #8159, lsl 16
  fsub.2d v13, v13, v12
  fmla.2d v13, v6, v11[0]
  add.2d v3, v3, v12
  movk x1, #215, lsl 32
  add.2d v5, v5, v13
  mov x7, #32119
  movk x7, #22102, lsl 16
  movk x1, #4913, lsl 48
  movk x7, #59590, lsl 32
  mov x11, #44641
  movk x11, #22291, lsl 16
  mul x13, x9, x14
  movk x11, #12496, lsl 32
  movk x11, #4, lsl 48
  mov x15, #28968
  umulh x9, x9, x14
  movk x15, #27414, lsl 16
  movk x15, #41914, lsl 32
  movk x15, #14, lsl 48
  adds x13, x13, x12
  cinc x9, x9, hs
  mov x12, #41622
  movk x12, #21957, lsl 16
  movk x12, #32174, lsl 32
  mul x16, x0, x14
  movk x12, #10, lsl 48
  mov x17, #13682
  movk x17, #64849, lsl 16
  umulh x0, x0, x14
  movk x17, #6986, lsl 32
  ucvtf.2d v6, v7
  ucvtf d7, x7
  adds x16, x16, x9
  cinc x0, x0, hs
  mov.16b v11, v21
  mov.16b v12, v22
  fmla.2d v11, v6, v7[0]
  adds x16, x16, x5
  cinc x0, x0, hs
  fsub.2d v12, v12, v11
  fmla.2d v12, v6, v7[0]
  add.2d v0, v0, v11
  mul x5, x10, x14
  add.2d v4, v4, v12
  ucvtf d7, x11
  umulh x7, x10, x14
  mov.16b v11, v21
  mov.16b v12, v22
  fmla.2d v11, v6, v7[0]
  adds x5, x5, x0
  cinc x7, x7, hs
  fsub.2d v12, v12, v11
  fmla.2d v12, v6, v7[0]
  add.2d v1, v1, v11
  adds x5, x5, x2
  cinc x7, x7, hs
  add.2d v0, v0, v12
  ucvtf d7, x15
  mov.16b v11, v21
  mul x0, x1, x14
  mov.16b v12, v22
  fmla.2d v11, v6, v7[0]
  fsub.2d v12, v12, v11
  umulh x1, x1, x14
  fmla.2d v12, v6, v7[0]
  add.2d v2, v2, v11
  add.2d v1, v1, v12
  adds x0, x0, x7
  cinc x1, x1, hs
  ucvtf d7, x12
  mov.16b v11, v21
  mov.16b v12, v22
  adds x0, x0, x6
  cinc x1, x1, hs
  fmla.2d v11, v6, v7[0]
  fsub.2d v12, v12, v11
  fmla.2d v12, v6, v7[0]
  add x1, x3, x1
  add.2d v5, v5, v11
  add.2d v2, v2, v12
  ucvtf d7, x17
  mov x2, #61005
  mov.16b v11, v21
  mov.16b v12, v22
  fmla.2d v11, v6, v7[0]
  movk x2, #58262, lsl 16
  fsub.2d v12, v12, v11
  fmla.2d v12, v6, v7[0]
  add.2d v3, v3, v11
  movk x2, #32851, lsl 32
  add.2d v5, v5, v12
  mov x3, #31098
  movk x3, #52890, lsl 16
  movk x2, #11582, lsl 48
  movk x3, #20172, lsl 32
  movk x3, #7, lsl 48
  mov x6, #55460
  mov x7, #37581
  movk x6, #49931, lsl 16
  movk x6, #28124, lsl 32
  movk x6, #1, lsl 48
  movk x7, #43836, lsl 16
  mov x9, #18846
  movk x9, #13625, lsl 16
  movk x9, #40653, lsl 32
  movk x7, #36286, lsl 32
  movk x9, #4, lsl 48
  mov x10, #37560
  movk x10, #64709, lsl 16
  movk x7, #51783, lsl 48
  movk x10, #9126, lsl 32
  movk x10, #11, lsl 48
  mov x11, #28389
  mov x12, #10899
  movk x11, #54431, lsl 16
  movk x11, #3643, lsl 32
  ucvtf.2d v6, v9
  movk x12, #30709, lsl 16
  ucvtf d7, x3
  mov.16b v9, v21
  mov.16b v11, v22
  movk x12, #61551, lsl 32
  fmla.2d v9, v6, v7[0]
  fsub.2d v11, v11, v9
  movk x12, #45784, lsl 48
  fmla.2d v11, v6, v7[0]
  add.2d v0, v0, v9
  add.2d v4, v4, v11
  mov x3, #36612
  ucvtf d7, x6
  mov.16b v9, v21
  mov.16b v11, v22
  movk x3, #63402, lsl 16
  fmla.2d v9, v6, v7[0]
  fsub.2d v11, v11, v9
  fmla.2d v11, v6, v7[0]
  movk x3, #47623, lsl 32
  add.2d v1, v1, v9
  add.2d v0, v0, v11
  ucvtf d7, x9
  movk x3, #9430, lsl 48
  mov.16b v9, v21
  mov.16b v11, v22
  fmla.2d v9, v6, v7[0]
  mul x6, x2, x4
  fsub.2d v11, v11, v9
  fmla.2d v11, v6, v7[0]
  add.2d v2, v2, v9
  umulh x2, x2, x4
  add.2d v1, v1, v11
  ucvtf d7, x10
  mov.16b v9, v21
  adds x6, x6, x13
  cinc x2, x2, hs
  mov.16b v11, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v11, v11, v9
  mul x9, x7, x4
  fmla.2d v11, v6, v7[0]
  add.2d v5, v5, v9
  add.2d v2, v2, v11
  umulh x7, x7, x4
  ucvtf d7, x11
  mov.16b v9, v21
  mov.16b v11, v22
  adds x9, x9, x2
  cinc x7, x7, hs
  fmla.2d v9, v6, v7[0]
  fsub.2d v11, v11, v9
  fmla.2d v11, v6, v7[0]
  adds x9, x9, x16
  cinc x7, x7, hs
  add.2d v3, v3, v9
  add.2d v5, v5, v11
  mov x2, #50130
  mul x10, x12, x4
  movk x2, #20196, lsl 16
  movk x2, #11876, lsl 32
  movk x2, #8, lsl 48
  umulh x11, x12, x4
  mov x12, #45534
  movk x12, #15512, lsl 16
  movk x12, #37769, lsl 32
  adds x10, x10, x7
  cinc x11, x11, hs
  movk x12, #15, lsl 48
  mov x7, #42183
  movk x7, #1232, lsl 16
  adds x10, x10, x5
  cinc x11, x11, hs
  movk x7, #18174, lsl 32
  movk x7, #13, lsl 48
  mov x5, #10783
  mul x13, x3, x4
  movk x5, #54622, lsl 16
  movk x5, #61610, lsl 32
  movk x5, #8, lsl 48
  umulh x3, x3, x4
  mov x4, #56963
  movk x4, #1095, lsl 16
  movk x4, #1517, lsl 32
  adds x13, x13, x11
  cinc x3, x3, hs
  ucvtf.2d v6, v10
  ucvtf d7, x2
  adds x13, x13, x0
  cinc x3, x3, hs
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  add x0, x1, x3
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  add.2d v0, v0, v9
  mov x1, #65535
  add.2d v4, v4, v10
  ucvtf d7, x12
  mov.16b v9, v21
  movk x1, #61439, lsl 16
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  movk x1, #62867, lsl 32
  fmla.2d v10, v6, v7[0]
  add.2d v1, v1, v9
  add.2d v0, v0, v10
  movk x1, #49889, lsl 48
  ucvtf d7, x7
  mov.16b v9, v21
  mov.16b v10, v22
  mul x1, x1, x6
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  mov x2, #1
  add.2d v2, v2, v9
  add.2d v1, v1, v10
  ucvtf d7, x5
  movk x2, #61440, lsl 16
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  movk x2, #62867, lsl 32
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  add.2d v5, v5, v9
  movk x2, #17377, lsl 48
  add.2d v2, v2, v10
  ucvtf d7, x4
  mov.16b v9, v21
  mov x3, #28817
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  movk x3, #31161, lsl 16
  fmla.2d v10, v6, v7[0]
  add.2d v3, v3, v9
  add.2d v5, v5, v10
  movk x3, #59464, lsl 32
  mov x4, #65535
  movk x4, #61439, lsl 16
  movk x4, #62867, lsl 32
  movk x3, #10291, lsl 48
  movk x4, #1, lsl 48
  umov x5, v4.d[0]
  umov x7, v4.d[1]
  mov x11, #22621
  mul x5, x5, x4
  mul x4, x7, x4
  and x5, x5, x8
  movk x11, #33153, lsl 16
  and x4, x4, x8
  ins v6.d[0], x5
  ins v6.d[1], x4
  mov x4, #1
  movk x11, #17846, lsl 32
  movk x4, #61440, lsl 16
  movk x4, #62867, lsl 32
  movk x4, #1, lsl 48
  movk x11, #47184, lsl 48
  mov x5, #5182
  movk x5, #38665, lsl 16
  mov x7, #41001
  movk x5, #34715, lsl 32
  movk x5, #4, lsl 48
  mov x8, #13288
  movk x7, #57649, lsl 16
  movk x8, #23848, lsl 16
  movk x8, #33112, lsl 32
  movk x8, #1, lsl 48
  movk x7, #20082, lsl 32
  mov x12, #23400
  movk x12, #34052, lsl 16
  movk x12, #667, lsl 32
  movk x7, #12388, lsl 48
  movk x12, #10, lsl 48
  mov x14, #57649
  movk x14, #20082, lsl 16
  mul x15, x2, x1
  movk x14, #12388, lsl 32
  ucvtf.2d v6, v6
  ucvtf d7, x4
  umulh x2, x2, x1
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  cmn x15, x6
  cinc x2, x2, hs
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  add.2d v0, v0, v9
  mul x4, x3, x1
  add.2d v4, v4, v10
  ucvtf d7, x5
  mov.16b v9, v21
  umulh x3, x3, x1
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  adds x4, x4, x2
  cinc x3, x3, hs
  fmla.2d v10, v6, v7[0]
  add.2d v1, v1, v9
  add.2d v0, v0, v10
  adds x4, x4, x9
  cinc x3, x3, hs
  ucvtf d7, x8
  mov.16b v9, v21
  mov.16b v10, v22
  mul x2, x11, x1
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  umulh x5, x11, x1
  add.2d v2, v2, v9
  add.2d v1, v1, v10
  ucvtf d7, x12
  adds x2, x2, x3
  cinc x5, x5, hs
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  adds x2, x2, x10
  cinc x5, x5, hs
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  add.2d v5, v5, v9
  mul x3, x7, x1
  add.2d v2, v2, v10
  ucvtf d7, x14
  mov.16b v9, v21
  umulh x1, x7, x1
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  adds x3, x3, x5
  cinc x1, x1, hs
  fmla.2d v10, v6, v7[0]
  add.2d v3, v3, v9
  add.2d v5, v5, v10
  adds x3, x3, x13
  cinc x1, x1, hs
  mov x5, #140737488355328
  dup.2d v6, x5
  add x0, x0, x1
  and.16b v6, v3, v6
  cmeq.2d v6, v6, #0
  mov x1, #2
  mov x5, #2
  movk x1, #57344, lsl 16
  movk x1, #60199, lsl 32
  movk x1, #3, lsl 48
  movk x5, #57344, lsl 16
  dup.2d v7, x1
  bic.16b v7, v7, v6
  mov x1, #10364
  movk x5, #60199, lsl 32
  movk x1, #11794, lsl 16
  movk x1, #3895, lsl 32
  movk x1, #9, lsl 48
  movk x5, #34755, lsl 48
  dup.2d v9, x1
  bic.16b v9, v9, v6
  mov x1, #26576
  mov x6, #57634
  movk x1, #47696, lsl 16
  movk x1, #688, lsl 32
  movk x1, #3, lsl 48
  movk x6, #62322, lsl 16
  dup.2d v10, x1
  bic.16b v10, v10, v6
  mov x1, #46800
  movk x6, #53392, lsl 32
  movk x1, #2568, lsl 16
  movk x1, #1335, lsl 32
  movk x1, #4, lsl 48
  movk x6, #20583, lsl 48
  dup.2d v11, x1
  bic.16b v11, v11, v6
  mov x1, #49763
  mov x7, #45242
  movk x1, #40165, lsl 16
  movk x1, #24776, lsl 32
  dup.2d v12, x1
  movk x7, #770, lsl 16
  bic.16b v6, v12, v6
  sub.2d v0, v0, v7
  ssra.2d v0, v4, #52
  movk x7, #35693, lsl 32
  and.16b v4, v0, v8
  sub.2d v1, v1, v9
  ssra.2d v1, v0, #52
  movk x7, #28832, lsl 48
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
  subs x5, x4, x5
  sbcs x6, x2, x6
  sbcs x7, x3, x7
  sbcs x1, x0, x1
  usra.2d v6, v0, #12
  usra.2d v7, v1, #24
  usra.2d v3, v2, #36
  tst x0, #9223372036854775808
  csel x4, x5, x4, mi
  csel x2, x6, x2, mi
  csel x3, x7, x3, mi
  csel x0, x1, x0, mi
ret
