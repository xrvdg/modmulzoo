//in("v0") _, in("v1") _, in("v2") _, in("v3") _, in("v4") _, in("v5") _, in("v6") _, in("v7") _,
//lateout("v4") out[0], lateout("v6") out[1], lateout("v7") out[2], lateout("v3") out[3],
//lateout("x0") _, lateout("v0") _, lateout("x1") _, lateout("v1") _, lateout("x2") _, lateout("v2") _, lateout("x3") _, lateout("x4") _, lateout("v4[0]") _, lateout("v4[1]") _, lateout("x5") _, lateout("v5") _, lateout("x6") _, lateout("v6[0]") _, lateout("v6[1]") _, lateout("x7") _, lateout("x8") _, lateout("v8") _, lateout("x9") _, lateout("v9") _, lateout("v9[0]") _, lateout("d9") _, lateout("x10") _, lateout("v10") _, lateout("v10[0]") _, lateout("d10") _, lateout("x11") _, lateout("v11") _, lateout("v11[0]") _, lateout("d11") _, lateout("x12") _, lateout("v12") _, lateout("v12[0]") _, lateout("d12") _, lateout("x13") _, lateout("v13") _, lateout("x14") _, lateout("v14") _, lateout("x15") _, lateout("v15") _, lateout("x16") _, lateout("v16") _, lateout("x17") _, lateout("v17") _, lateout("x18") _, lateout("v18") _, lateout("x19") _, lateout("v19") _, lateout("x20") _, lateout("v20") _, lateout("x21") _, lateout("v21") _, lateout("v22") _, lateout("v23") _, lateout("v24") _,
//lateout("lr") _
.global _single_step_simd
.align 4
.text
_single_step_simd:
  mov x0, #4503599627370495
  dup.2d v8, x0
  shl.2d v9, v1, #14
  shl.2d v10, v2, #26
  shl.2d v11, v3, #38
  shl.2d v12, v0, #2
  usra.2d v9, v0, #50
  usra.2d v10, v1, #38
  usra.2d v11, v2, #26
  and.16b v0, v12, v8
  and.16b v1, v9, v8
  and.16b v2, v10, v8
  and.16b v8, v11, v8
  ushr.2d v3, v3, #14
  mov x0, #4503599627370495
  dup.2d v9, x0
  shl.2d v10, v5, #14
  shl.2d v11, v6, #26
  shl.2d v12, v7, #38
  shl.2d v13, v4, #2
  usra.2d v10, v4, #50
  usra.2d v11, v5, #38
  usra.2d v12, v6, #26
  and.16b v4, v13, v9
  and.16b v5, v10, v9
  and.16b v6, v11, v9
  and.16b v9, v12, v9
  ushr.2d v7, v7, #14
  mov x0, #13605374474286268416
  dup.2d v10, x0
  mov x0, #6440147467139809280
  dup.2d v11, x0
  mov x0, #3688448094816436224
  dup.2d v12, x0
  mov x0, #9209861237972664320
  dup.2d v13, x0
  mov x0, #12218265789056155648
  dup.2d v14, x0
  mov x0, #17739678932212383744
  dup.2d v15, x0
  mov x0, #2301339409586323456
  dup.2d v16, x0
  mov x0, #7822752552742551552
  dup.2d v17, x0
  mov x0, #5071053180419178496
  dup.2d v18, x0
  mov x0, #16352570246982270976
  dup.2d v19, x0
  mov x0, #5075556780046548992
  dup.2d v20, x0
  mov x0, #1
  movk x0, #18032, lsl 48
  dup.2d v21, x0
  ucvtf.2d v0, v0
  ucvtf.2d v22, v4
  mov.16b v23, v20
  mov.16b v24, v21
  fmla.2d v23, v0, v22
  fsub.2d v24, v24, v23
  fmla.2d v24, v0, v22
  add.2d v12, v12, v23
  add.2d v10, v10, v24
  ucvtf.2d v22, v5
  mov.16b v23, v20
  mov.16b v24, v21
  fmla.2d v23, v0, v22
  fsub.2d v24, v24, v23
  fmla.2d v24, v0, v22
  add.2d v14, v14, v23
  add.2d v12, v12, v24
  ucvtf.2d v22, v6
  mov.16b v23, v20
  mov.16b v24, v21
  fmla.2d v23, v0, v22
  fsub.2d v24, v24, v23
  fmla.2d v24, v0, v22
  add.2d v16, v16, v23
  add.2d v14, v14, v24
  ucvtf.2d v22, v9
  mov.16b v23, v20
  mov.16b v24, v21
  fmla.2d v23, v0, v22
  fsub.2d v24, v24, v23
  fmla.2d v24, v0, v22
  add.2d v18, v18, v23
  add.2d v16, v16, v24
  ucvtf.2d v22, v7
  mov.16b v23, v20
  mov.16b v24, v21
  fmla.2d v23, v0, v22
  fsub.2d v24, v24, v23
  fmla.2d v24, v0, v22
  add.2d v0, v19, v23
  add.2d v18, v18, v24
  ucvtf.2d v1, v1
  ucvtf.2d v19, v4
  mov.16b v22, v20
  mov.16b v23, v21
  fmla.2d v22, v1, v19
  fsub.2d v23, v23, v22
  fmla.2d v23, v1, v19
  add.2d v14, v14, v22
  add.2d v12, v12, v23
  ucvtf.2d v19, v5
  mov.16b v22, v20
  mov.16b v23, v21
  fmla.2d v22, v1, v19
  fsub.2d v23, v23, v22
  fmla.2d v23, v1, v19
  add.2d v16, v16, v22
  add.2d v14, v14, v23
  ucvtf.2d v19, v6
  mov.16b v22, v20
  mov.16b v23, v21
  fmla.2d v22, v1, v19
  fsub.2d v23, v23, v22
  fmla.2d v23, v1, v19
  add.2d v18, v18, v22
  add.2d v16, v16, v23
  ucvtf.2d v19, v9
  mov.16b v22, v20
  mov.16b v23, v21
  fmla.2d v22, v1, v19
  fsub.2d v23, v23, v22
  fmla.2d v23, v1, v19
  add.2d v0, v0, v22
  add.2d v18, v18, v23
  ucvtf.2d v19, v7
  mov.16b v22, v20
  mov.16b v23, v21
  fmla.2d v22, v1, v19
  fsub.2d v23, v23, v22
  fmla.2d v23, v1, v19
  add.2d v1, v17, v22
  add.2d v0, v0, v23
  ucvtf.2d v2, v2
  ucvtf.2d v17, v4
  mov.16b v19, v20
  mov.16b v22, v21
  fmla.2d v19, v2, v17
  fsub.2d v22, v22, v19
  fmla.2d v22, v2, v17
  add.2d v16, v16, v19
  add.2d v14, v14, v22
  ucvtf.2d v17, v5
  mov.16b v19, v20
  mov.16b v22, v21
  fmla.2d v19, v2, v17
  fsub.2d v22, v22, v19
  fmla.2d v22, v2, v17
  add.2d v17, v18, v19
  add.2d v16, v16, v22
  ucvtf.2d v18, v6
  mov.16b v19, v20
  mov.16b v22, v21
  fmla.2d v19, v2, v18
  fsub.2d v22, v22, v19
  fmla.2d v22, v2, v18
  add.2d v0, v0, v19
  add.2d v17, v17, v22
  ucvtf.2d v18, v9
  mov.16b v19, v20
  mov.16b v22, v21
  fmla.2d v19, v2, v18
  fsub.2d v22, v22, v19
  fmla.2d v22, v2, v18
  add.2d v1, v1, v19
  add.2d v0, v0, v22
  ucvtf.2d v18, v7
  mov.16b v19, v20
  mov.16b v22, v21
  fmla.2d v19, v2, v18
  fsub.2d v22, v22, v19
  fmla.2d v22, v2, v18
  add.2d v2, v15, v19
  add.2d v1, v1, v22
  ucvtf.2d v8, v8
  ucvtf.2d v15, v4
  mov.16b v18, v20
  mov.16b v19, v21
  fmla.2d v18, v8, v15
  fsub.2d v19, v19, v18
  fmla.2d v19, v8, v15
  add.2d v15, v17, v18
  add.2d v16, v16, v19
  ucvtf.2d v17, v5
  mov.16b v18, v20
  mov.16b v19, v21
  fmla.2d v18, v8, v17
  fsub.2d v19, v19, v18
  fmla.2d v19, v8, v17
  add.2d v0, v0, v18
  add.2d v15, v15, v19
  ucvtf.2d v17, v6
  mov.16b v18, v20
  mov.16b v19, v21
  fmla.2d v18, v8, v17
  fsub.2d v19, v19, v18
  fmla.2d v19, v8, v17
  add.2d v1, v1, v18
  add.2d v0, v0, v19
  ucvtf.2d v17, v9
  mov.16b v18, v20
  mov.16b v19, v21
  fmla.2d v18, v8, v17
  fsub.2d v19, v19, v18
  fmla.2d v19, v8, v17
  add.2d v2, v2, v18
  add.2d v1, v1, v19
  ucvtf.2d v17, v7
  mov.16b v18, v20
  mov.16b v19, v21
  fmla.2d v18, v8, v17
  fsub.2d v19, v19, v18
  fmla.2d v19, v8, v17
  add.2d v8, v13, v18
  add.2d v2, v2, v19
  ucvtf.2d v3, v3
  ucvtf.2d v4, v4
  mov.16b v13, v20
  mov.16b v17, v21
  fmla.2d v13, v3, v4
  fsub.2d v17, v17, v13
  fmla.2d v17, v3, v4
  add.2d v0, v0, v13
  add.2d v4, v15, v17
  ucvtf.2d v5, v5
  mov.16b v13, v20
  mov.16b v15, v21
  fmla.2d v13, v3, v5
  fsub.2d v15, v15, v13
  fmla.2d v15, v3, v5
  add.2d v1, v1, v13
  add.2d v0, v0, v15
  ucvtf.2d v5, v6
  mov.16b v6, v20
  mov.16b v13, v21
  fmla.2d v6, v3, v5
  fsub.2d v13, v13, v6
  fmla.2d v13, v3, v5
  add.2d v2, v2, v6
  add.2d v1, v1, v13
  ucvtf.2d v5, v9
  mov.16b v6, v20
  mov.16b v9, v21
  fmla.2d v6, v3, v5
  fsub.2d v9, v9, v6
  fmla.2d v9, v3, v5
  add.2d v5, v8, v6
  add.2d v2, v2, v9
  ucvtf.2d v6, v7
  mov.16b v7, v20
  mov.16b v8, v21
  fmla.2d v7, v3, v6
  fsub.2d v8, v8, v7
  fmla.2d v8, v3, v6
  add.2d v3, v11, v7
  add.2d v5, v5, v8
  usra.2d v12, v10, #52
  usra.2d v14, v12, #52
  usra.2d v16, v14, #52
  usra.2d v4, v16, #52
  mov x0, #4503599627370495
  dup.2d v6, x0
  and.16b v7, v10, v6
  and.16b v8, v12, v6
  and.16b v9, v14, v6
  and.16b v6, v16, v6
  mov x1, #62077
  movk x1, #226, lsl 16
  movk x1, #11812, lsl 32
  movk x1, #2, lsl 48
  mov x2, #38534
  movk x2, #28321, lsl 16
  movk x2, #9140, lsl 32
  movk x2, #3, lsl 48
  mov x3, #26413
  movk x3, #61663, lsl 16
  movk x3, #27715, lsl 32
  movk x3, #14, lsl 48
  mov x4, #40587
  movk x4, #5315, lsl 16
  movk x4, #33344, lsl 32
  movk x4, #7, lsl 48
  mov x5, #57784
  movk x5, #18607, lsl 16
  movk x5, #3179, lsl 32
  mov x6, #32119
  movk x6, #22102, lsl 16
  movk x6, #59590, lsl 32
  mov x7, #44641
  movk x7, #22291, lsl 16
  movk x7, #12496, lsl 32
  movk x7, #4, lsl 48
  mov x8, #28968
  movk x8, #27414, lsl 16
  movk x8, #41914, lsl 32
  movk x8, #14, lsl 48
  mov x9, #41622
  movk x9, #21957, lsl 16
  movk x9, #32174, lsl 32
  movk x9, #10, lsl 48
  mov x10, #13682
  movk x10, #64849, lsl 16
  movk x10, #6986, lsl 32
  mov x11, #31098
  movk x11, #52890, lsl 16
  movk x11, #20172, lsl 32
  movk x11, #7, lsl 48
  mov x12, #55460
  movk x12, #49931, lsl 16
  movk x12, #28124, lsl 32
  movk x12, #1, lsl 48
  mov x13, #18846
  movk x13, #13625, lsl 16
  movk x13, #40653, lsl 32
  movk x13, #4, lsl 48
  mov x14, #37560
  movk x14, #64709, lsl 16
  movk x14, #9126, lsl 32
  movk x14, #11, lsl 48
  mov x15, #28389
  movk x15, #54431, lsl 16
  movk x15, #3643, lsl 32
  mov x16, #50130
  movk x16, #20196, lsl 16
  movk x16, #11876, lsl 32
  movk x16, #8, lsl 48
  mov x17, #45534
  movk x17, #15512, lsl 16
  movk x17, #37769, lsl 32
  movk x17, #15, lsl 48
  mov x18, #42183
  movk x18, #1232, lsl 16
  movk x18, #18174, lsl 32
  movk x18, #13, lsl 48
  mov x19, #10783
  movk x19, #54622, lsl 16
  movk x19, #61610, lsl 32
  movk x19, #8, lsl 48
  mov x20, #56963
  movk x20, #1095, lsl 16
  movk x20, #1517, lsl 32
  mov x21, #5075556780046548992
  dup.2d v10, x21
  mov x21, #1
  movk x21, #18032, lsl 48
  dup.2d v11, x21
  ucvtf.2d v7, v7
  ucvtf d12, x1
  mov.16b v13, v10
  mov.16b v14, v11
  fmla.2d v13, v7, v12[0]
  fsub.2d v14, v14, v13
  fmla.2d v14, v7, v12[0]
  add.2d v0, v0, v13
  add.2d v4, v4, v14
  ucvtf d12, x2
  mov.16b v13, v10
  mov.16b v14, v11
  fmla.2d v13, v7, v12[0]
  fsub.2d v14, v14, v13
  fmla.2d v14, v7, v12[0]
  add.2d v1, v1, v13
  add.2d v0, v0, v14
  ucvtf d12, x3
  mov.16b v13, v10
  mov.16b v14, v11
  fmla.2d v13, v7, v12[0]
  fsub.2d v14, v14, v13
  fmla.2d v14, v7, v12[0]
  add.2d v2, v2, v13
  add.2d v1, v1, v14
  ucvtf d12, x4
  mov.16b v13, v10
  mov.16b v14, v11
  fmla.2d v13, v7, v12[0]
  fsub.2d v14, v14, v13
  fmla.2d v14, v7, v12[0]
  add.2d v5, v5, v13
  add.2d v2, v2, v14
  ucvtf d12, x5
  mov.16b v10, v10
  mov.16b v11, v11
  fmla.2d v10, v7, v12[0]
  fsub.2d v11, v11, v10
  fmla.2d v11, v7, v12[0]
  add.2d v3, v3, v10
  add.2d v5, v5, v11
  mov x1, #5075556780046548992
  dup.2d v7, x1
  mov x1, #1
  movk x1, #18032, lsl 48
  dup.2d v10, x1
  ucvtf.2d v8, v8
  ucvtf d11, x6
  mov.16b v12, v7
  mov.16b v13, v10
  fmla.2d v12, v8, v11[0]
  fsub.2d v13, v13, v12
  fmla.2d v13, v8, v11[0]
  add.2d v0, v0, v12
  add.2d v4, v4, v13
  ucvtf d11, x7
  mov.16b v12, v7
  mov.16b v13, v10
  fmla.2d v12, v8, v11[0]
  fsub.2d v13, v13, v12
  fmla.2d v13, v8, v11[0]
  add.2d v1, v1, v12
  add.2d v0, v0, v13
  ucvtf d11, x8
  mov.16b v12, v7
  mov.16b v13, v10
  fmla.2d v12, v8, v11[0]
  fsub.2d v13, v13, v12
  fmla.2d v13, v8, v11[0]
  add.2d v2, v2, v12
  add.2d v1, v1, v13
  ucvtf d11, x9
  mov.16b v12, v7
  mov.16b v13, v10
  fmla.2d v12, v8, v11[0]
  fsub.2d v13, v13, v12
  fmla.2d v13, v8, v11[0]
  add.2d v5, v5, v12
  add.2d v2, v2, v13
  ucvtf d11, x10
  mov.16b v7, v7
  mov.16b v10, v10
  fmla.2d v7, v8, v11[0]
  fsub.2d v10, v10, v7
  fmla.2d v10, v8, v11[0]
  add.2d v3, v3, v7
  add.2d v5, v5, v10
  mov x1, #5075556780046548992
  dup.2d v7, x1
  mov x1, #1
  movk x1, #18032, lsl 48
  dup.2d v8, x1
  ucvtf.2d v9, v9
  ucvtf d10, x11
  mov.16b v11, v7
  mov.16b v12, v8
  fmla.2d v11, v9, v10[0]
  fsub.2d v12, v12, v11
  fmla.2d v12, v9, v10[0]
  add.2d v0, v0, v11
  add.2d v4, v4, v12
  ucvtf d10, x12
  mov.16b v11, v7
  mov.16b v12, v8
  fmla.2d v11, v9, v10[0]
  fsub.2d v12, v12, v11
  fmla.2d v12, v9, v10[0]
  add.2d v1, v1, v11
  add.2d v0, v0, v12
  ucvtf d10, x13
  mov.16b v11, v7
  mov.16b v12, v8
  fmla.2d v11, v9, v10[0]
  fsub.2d v12, v12, v11
  fmla.2d v12, v9, v10[0]
  add.2d v2, v2, v11
  add.2d v1, v1, v12
  ucvtf d10, x14
  mov.16b v11, v7
  mov.16b v12, v8
  fmla.2d v11, v9, v10[0]
  fsub.2d v12, v12, v11
  fmla.2d v12, v9, v10[0]
  add.2d v5, v5, v11
  add.2d v2, v2, v12
  ucvtf d10, x15
  mov.16b v7, v7
  mov.16b v8, v8
  fmla.2d v7, v9, v10[0]
  fsub.2d v8, v8, v7
  fmla.2d v8, v9, v10[0]
  add.2d v3, v3, v7
  add.2d v5, v5, v8
  mov x1, #5075556780046548992
  dup.2d v7, x1
  mov x1, #1
  movk x1, #18032, lsl 48
  dup.2d v8, x1
  ucvtf.2d v6, v6
  ucvtf d9, x16
  mov.16b v10, v7
  mov.16b v11, v8
  fmla.2d v10, v6, v9[0]
  fsub.2d v11, v11, v10
  fmla.2d v11, v6, v9[0]
  add.2d v0, v0, v10
  add.2d v4, v4, v11
  ucvtf d9, x17
  mov.16b v10, v7
  mov.16b v11, v8
  fmla.2d v10, v6, v9[0]
  fsub.2d v11, v11, v10
  fmla.2d v11, v6, v9[0]
  add.2d v1, v1, v10
  add.2d v0, v0, v11
  ucvtf d9, x18
  mov.16b v10, v7
  mov.16b v11, v8
  fmla.2d v10, v6, v9[0]
  fsub.2d v11, v11, v10
  fmla.2d v11, v6, v9[0]
  add.2d v2, v2, v10
  add.2d v1, v1, v11
  ucvtf d9, x19
  mov.16b v10, v7
  mov.16b v11, v8
  fmla.2d v10, v6, v9[0]
  fsub.2d v11, v11, v10
  fmla.2d v11, v6, v9[0]
  add.2d v5, v5, v10
  add.2d v2, v2, v11
  ucvtf d9, x20
  mov.16b v7, v7
  mov.16b v8, v8
  fmla.2d v7, v6, v9[0]
  fsub.2d v8, v8, v7
  fmla.2d v8, v6, v9[0]
  add.2d v3, v3, v7
  add.2d v5, v5, v8
  mov x1, #65535
  movk x1, #61439, lsl 16
  movk x1, #62867, lsl 32
  movk x1, #1, lsl 48
  umov x2, v4[0]
  umov x3, v4[1]
  mul x2, x2, x1
  mul x1, x3, x1
  and x2, x2, x0
  and x0, x1, x0
  ins v6[0], x2
  ins v6[1], x0
  mov x0, #1
  movk x0, #61440, lsl 16
  movk x0, #62867, lsl 32
  movk x0, #1, lsl 48
  mov x1, #5182
  movk x1, #38665, lsl 16
  movk x1, #34715, lsl 32
  movk x1, #4, lsl 48
  mov x2, #13288
  movk x2, #23848, lsl 16
  movk x2, #33112, lsl 32
  movk x2, #1, lsl 48
  mov x3, #23400
  movk x3, #34052, lsl 16
  movk x3, #667, lsl 32
  movk x3, #10, lsl 48
  mov x4, #57649
  movk x4, #20082, lsl 16
  movk x4, #12388, lsl 32
  mov x5, #5075556780046548992
  dup.2d v7, x5
  mov x5, #1
  movk x5, #18032, lsl 48
  dup.2d v8, x5
  ucvtf.2d v6, v6
  ucvtf d9, x0
  mov.16b v10, v7
  mov.16b v11, v8
  fmla.2d v10, v6, v9[0]
  fsub.2d v11, v11, v10
  fmla.2d v11, v6, v9[0]
  add.2d v0, v0, v10
  add.2d v4, v4, v11
  ucvtf d9, x1
  mov.16b v10, v7
  mov.16b v11, v8
  fmla.2d v10, v6, v9[0]
  fsub.2d v11, v11, v10
  fmla.2d v11, v6, v9[0]
  add.2d v1, v1, v10
  add.2d v0, v0, v11
  ucvtf d9, x2
  mov.16b v10, v7
  mov.16b v11, v8
  fmla.2d v10, v6, v9[0]
  fsub.2d v11, v11, v10
  fmla.2d v11, v6, v9[0]
  add.2d v2, v2, v10
  add.2d v1, v1, v11
  ucvtf d9, x3
  mov.16b v10, v7
  mov.16b v11, v8
  fmla.2d v10, v6, v9[0]
  fsub.2d v11, v11, v10
  fmla.2d v11, v6, v9[0]
  add.2d v5, v5, v10
  add.2d v2, v2, v11
  ucvtf d9, x4
  mov.16b v7, v7
  mov.16b v8, v8
  fmla.2d v7, v6, v9[0]
  fsub.2d v8, v8, v7
  fmla.2d v8, v6, v9[0]
  add.2d v3, v3, v7
  add.2d v5, v5, v8
  mov x0, #140737488355328
  dup.2d v6, x0
  and.16b v6, v3, v6
  cmeq.2d v6, v6, #0
  mov x0, #2
  movk x0, #57344, lsl 16
  movk x0, #60199, lsl 32
  movk x0, #3, lsl 48
  dup.2d v7, x0
  bic.16b v7, v7, v6
  mov x0, #10364
  movk x0, #11794, lsl 16
  movk x0, #3895, lsl 32
  movk x0, #9, lsl 48
  dup.2d v8, x0
  bic.16b v8, v8, v6
  mov x0, #26576
  movk x0, #47696, lsl 16
  movk x0, #688, lsl 32
  movk x0, #3, lsl 48
  dup.2d v9, x0
  bic.16b v9, v9, v6
  mov x0, #46800
  movk x0, #2568, lsl 16
  movk x0, #1335, lsl 32
  movk x0, #4, lsl 48
  dup.2d v10, x0
  bic.16b v10, v10, v6
  mov x0, #49763
  movk x0, #40165, lsl 16
  movk x0, #24776, lsl 32
  dup.2d v11, x0
  bic.16b v6, v11, v6
  mov x0, #4503599627370495
  dup.2d v11, x0
  sub.2d v0, v0, v7
  ssra.2d v0, v4, #52
  and.16b v4, v0, v11
  sub.2d v1, v1, v8
  ssra.2d v1, v0, #52
  and.16b v0, v1, v11
  sub.2d v2, v2, v9
  ssra.2d v2, v1, #52
  and.16b v1, v2, v11
  sub.2d v5, v5, v10
  ssra.2d v5, v2, #52
  and.16b v2, v5, v11
  sub.2d v3, v3, v6
  ssra.2d v3, v5, #52
  and.16b v3, v3, v11
  shl.2d v5, v0, #52
  shl.2d v6, v1, #40
  shl.2d v7, v2, #28
  shl.2d v3, v3, #16
  orr.16b v4, v4, v5
  usra.2d v6, v0, #12
  usra.2d v7, v1, #24
  usra.2d v3, v2, #36
ret
