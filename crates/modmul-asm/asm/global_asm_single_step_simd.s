//in("v0") in0[0], in("v1") in0[1], in("v2") in0[2], in("v3") in0[3], in("v4") in1[0], in("v5") in1[1], in("v6") in1[2], in("v7") in1[3],
//lateout("v4") out0[0], lateout("v6") out0[1], lateout("v7") out0[2], lateout("v3") out0[3],
//lateout("x0") _, lateout("v0") _, lateout("x1") _, lateout("v1") _, lateout("x2") _, lateout("v2") _, lateout("x3") _, lateout("x4") _, lateout("x5") _, lateout("v5") _, lateout("v8") _, lateout("v9") _, lateout("v10") _, lateout("v11") _, lateout("v12") _, lateout("v13") _, lateout("v14") _, lateout("v15") _, lateout("v16") _, lateout("v17") _, lateout("v18") _, lateout("v19") _, lateout("v20") _, lateout("v21") _, lateout("v22") _, lateout("v23") _, lateout("v24") _, lateout("v25") _,
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
  and.16b v9, v11, v8
  ushr.2d v3, v3, #14
  shl.2d v10, v5, #14
  shl.2d v11, v6, #26
  shl.2d v12, v7, #38
  shl.2d v13, v4, #2
  usra.2d v10, v4, #50
  usra.2d v11, v5, #38
  usra.2d v12, v6, #26
  and.16b v4, v13, v8
  and.16b v5, v10, v8
  and.16b v6, v11, v8
  and.16b v10, v12, v8
  ushr.2d v7, v7, #14
  mov x1, #13605374474286268416
  dup.2d v11, x1
  mov x1, #6440147467139809280
  dup.2d v12, x1
  mov x1, #3688448094816436224
  dup.2d v13, x1
  mov x1, #9209861237972664320
  dup.2d v14, x1
  mov x1, #12218265789056155648
  dup.2d v15, x1
  mov x1, #17739678932212383744
  dup.2d v16, x1
  mov x1, #2301339409586323456
  dup.2d v17, x1
  mov x1, #7822752552742551552
  dup.2d v18, x1
  mov x1, #5071053180419178496
  dup.2d v19, x1
  mov x1, #16352570246982270976
  dup.2d v20, x1
  mov x1, #5075556780046548992
  dup.2d v21, x1
  mov x1, #1
  movk x1, #18032, lsl 48
  dup.2d v22, x1
  ucvtf.2d v0, v0
  ucvtf.2d v23, v4
  mov.16b v24, v21
  mov.16b v25, v22
  fmla.2d v24, v0, v23
  fsub.2d v25, v25, v24
  fmla.2d v25, v0, v23
  add.2d v13, v13, v24
  add.2d v11, v11, v25
  ucvtf.2d v23, v5
  mov.16b v24, v21
  mov.16b v25, v22
  fmla.2d v24, v0, v23
  fsub.2d v25, v25, v24
  fmla.2d v25, v0, v23
  add.2d v15, v15, v24
  add.2d v13, v13, v25
  ucvtf.2d v23, v6
  mov.16b v24, v21
  mov.16b v25, v22
  fmla.2d v24, v0, v23
  fsub.2d v25, v25, v24
  fmla.2d v25, v0, v23
  add.2d v17, v17, v24
  add.2d v15, v15, v25
  ucvtf.2d v23, v10
  mov.16b v24, v21
  mov.16b v25, v22
  fmla.2d v24, v0, v23
  fsub.2d v25, v25, v24
  fmla.2d v25, v0, v23
  add.2d v19, v19, v24
  add.2d v17, v17, v25
  ucvtf.2d v23, v7
  mov.16b v24, v21
  mov.16b v25, v22
  fmla.2d v24, v0, v23
  fsub.2d v25, v25, v24
  fmla.2d v25, v0, v23
  add.2d v0, v20, v24
  add.2d v19, v19, v25
  ucvtf.2d v1, v1
  ucvtf.2d v20, v4
  mov.16b v23, v21
  mov.16b v24, v22
  fmla.2d v23, v1, v20
  fsub.2d v24, v24, v23
  fmla.2d v24, v1, v20
  add.2d v15, v15, v23
  add.2d v13, v13, v24
  ucvtf.2d v20, v5
  mov.16b v23, v21
  mov.16b v24, v22
  fmla.2d v23, v1, v20
  fsub.2d v24, v24, v23
  fmla.2d v24, v1, v20
  add.2d v17, v17, v23
  add.2d v15, v15, v24
  ucvtf.2d v20, v6
  mov.16b v23, v21
  mov.16b v24, v22
  fmla.2d v23, v1, v20
  fsub.2d v24, v24, v23
  fmla.2d v24, v1, v20
  add.2d v19, v19, v23
  add.2d v17, v17, v24
  ucvtf.2d v20, v10
  mov.16b v23, v21
  mov.16b v24, v22
  fmla.2d v23, v1, v20
  fsub.2d v24, v24, v23
  fmla.2d v24, v1, v20
  add.2d v0, v0, v23
  add.2d v19, v19, v24
  ucvtf.2d v20, v7
  mov.16b v23, v21
  mov.16b v24, v22
  fmla.2d v23, v1, v20
  fsub.2d v24, v24, v23
  fmla.2d v24, v1, v20
  add.2d v1, v18, v23
  add.2d v0, v0, v24
  ucvtf.2d v2, v2
  ucvtf.2d v18, v4
  mov.16b v20, v21
  mov.16b v23, v22
  fmla.2d v20, v2, v18
  fsub.2d v23, v23, v20
  fmla.2d v23, v2, v18
  add.2d v17, v17, v20
  add.2d v15, v15, v23
  ucvtf.2d v18, v5
  mov.16b v20, v21
  mov.16b v23, v22
  fmla.2d v20, v2, v18
  fsub.2d v23, v23, v20
  fmla.2d v23, v2, v18
  add.2d v18, v19, v20
  add.2d v17, v17, v23
  ucvtf.2d v19, v6
  mov.16b v20, v21
  mov.16b v23, v22
  fmla.2d v20, v2, v19
  fsub.2d v23, v23, v20
  fmla.2d v23, v2, v19
  add.2d v0, v0, v20
  add.2d v18, v18, v23
  ucvtf.2d v19, v10
  mov.16b v20, v21
  mov.16b v23, v22
  fmla.2d v20, v2, v19
  fsub.2d v23, v23, v20
  fmla.2d v23, v2, v19
  add.2d v1, v1, v20
  add.2d v0, v0, v23
  ucvtf.2d v19, v7
  mov.16b v20, v21
  mov.16b v23, v22
  fmla.2d v20, v2, v19
  fsub.2d v23, v23, v20
  fmla.2d v23, v2, v19
  add.2d v2, v16, v20
  add.2d v1, v1, v23
  ucvtf.2d v9, v9
  ucvtf.2d v16, v4
  mov.16b v19, v21
  mov.16b v20, v22
  fmla.2d v19, v9, v16
  fsub.2d v20, v20, v19
  fmla.2d v20, v9, v16
  add.2d v16, v18, v19
  add.2d v17, v17, v20
  ucvtf.2d v18, v5
  mov.16b v19, v21
  mov.16b v20, v22
  fmla.2d v19, v9, v18
  fsub.2d v20, v20, v19
  fmla.2d v20, v9, v18
  add.2d v0, v0, v19
  add.2d v16, v16, v20
  ucvtf.2d v18, v6
  mov.16b v19, v21
  mov.16b v20, v22
  fmla.2d v19, v9, v18
  fsub.2d v20, v20, v19
  fmla.2d v20, v9, v18
  add.2d v1, v1, v19
  add.2d v0, v0, v20
  ucvtf.2d v18, v10
  mov.16b v19, v21
  mov.16b v20, v22
  fmla.2d v19, v9, v18
  fsub.2d v20, v20, v19
  fmla.2d v20, v9, v18
  add.2d v2, v2, v19
  add.2d v1, v1, v20
  ucvtf.2d v18, v7
  mov.16b v19, v21
  mov.16b v20, v22
  fmla.2d v19, v9, v18
  fsub.2d v20, v20, v19
  fmla.2d v20, v9, v18
  add.2d v9, v14, v19
  add.2d v2, v2, v20
  ucvtf.2d v3, v3
  ucvtf.2d v4, v4
  mov.16b v14, v21
  mov.16b v18, v22
  fmla.2d v14, v3, v4
  fsub.2d v18, v18, v14
  fmla.2d v18, v3, v4
  add.2d v0, v0, v14
  add.2d v4, v16, v18
  ucvtf.2d v5, v5
  mov.16b v14, v21
  mov.16b v16, v22
  fmla.2d v14, v3, v5
  fsub.2d v16, v16, v14
  fmla.2d v16, v3, v5
  add.2d v1, v1, v14
  add.2d v0, v0, v16
  ucvtf.2d v5, v6
  mov.16b v6, v21
  mov.16b v14, v22
  fmla.2d v6, v3, v5
  fsub.2d v14, v14, v6
  fmla.2d v14, v3, v5
  add.2d v2, v2, v6
  add.2d v1, v1, v14
  ucvtf.2d v5, v10
  mov.16b v6, v21
  mov.16b v10, v22
  fmla.2d v6, v3, v5
  fsub.2d v10, v10, v6
  fmla.2d v10, v3, v5
  add.2d v5, v9, v6
  add.2d v2, v2, v10
  ucvtf.2d v6, v7
  mov.16b v7, v21
  mov.16b v9, v22
  fmla.2d v7, v3, v6
  fsub.2d v9, v9, v7
  fmla.2d v9, v3, v6
  add.2d v3, v12, v7
  add.2d v5, v5, v9
  usra.2d v13, v11, #52
  usra.2d v15, v13, #52
  usra.2d v17, v15, #52
  usra.2d v4, v17, #52
  and.16b v6, v11, v8
  and.16b v7, v13, v8
  and.16b v9, v15, v8
  and.16b v10, v17, v8
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
  ucvtf.2d v6, v6
  ucvtf d11, x1
  mov.16b v12, v21
  mov.16b v13, v22
  fmla.2d v12, v6, v11[0]
  fsub.2d v13, v13, v12
  fmla.2d v13, v6, v11[0]
  add.2d v0, v0, v12
  add.2d v4, v4, v13
  ucvtf d11, x2
  mov.16b v12, v21
  mov.16b v13, v22
  fmla.2d v12, v6, v11[0]
  fsub.2d v13, v13, v12
  fmla.2d v13, v6, v11[0]
  add.2d v1, v1, v12
  add.2d v0, v0, v13
  ucvtf d11, x3
  mov.16b v12, v21
  mov.16b v13, v22
  fmla.2d v12, v6, v11[0]
  fsub.2d v13, v13, v12
  fmla.2d v13, v6, v11[0]
  add.2d v2, v2, v12
  add.2d v1, v1, v13
  ucvtf d11, x4
  mov.16b v12, v21
  mov.16b v13, v22
  fmla.2d v12, v6, v11[0]
  fsub.2d v13, v13, v12
  fmla.2d v13, v6, v11[0]
  add.2d v5, v5, v12
  add.2d v2, v2, v13
  ucvtf d11, x5
  mov.16b v12, v21
  mov.16b v13, v22
  fmla.2d v12, v6, v11[0]
  fsub.2d v13, v13, v12
  fmla.2d v13, v6, v11[0]
  add.2d v3, v3, v12
  add.2d v5, v5, v13
  mov x1, #32119
  movk x1, #22102, lsl 16
  movk x1, #59590, lsl 32
  mov x2, #44641
  movk x2, #22291, lsl 16
  movk x2, #12496, lsl 32
  movk x2, #4, lsl 48
  mov x3, #28968
  movk x3, #27414, lsl 16
  movk x3, #41914, lsl 32
  movk x3, #14, lsl 48
  mov x4, #41622
  movk x4, #21957, lsl 16
  movk x4, #32174, lsl 32
  movk x4, #10, lsl 48
  mov x5, #13682
  movk x5, #64849, lsl 16
  movk x5, #6986, lsl 32
  ucvtf.2d v6, v7
  ucvtf d7, x1
  mov.16b v11, v21
  mov.16b v12, v22
  fmla.2d v11, v6, v7[0]
  fsub.2d v12, v12, v11
  fmla.2d v12, v6, v7[0]
  add.2d v0, v0, v11
  add.2d v4, v4, v12
  ucvtf d7, x2
  mov.16b v11, v21
  mov.16b v12, v22
  fmla.2d v11, v6, v7[0]
  fsub.2d v12, v12, v11
  fmla.2d v12, v6, v7[0]
  add.2d v1, v1, v11
  add.2d v0, v0, v12
  ucvtf d7, x3
  mov.16b v11, v21
  mov.16b v12, v22
  fmla.2d v11, v6, v7[0]
  fsub.2d v12, v12, v11
  fmla.2d v12, v6, v7[0]
  add.2d v2, v2, v11
  add.2d v1, v1, v12
  ucvtf d7, x4
  mov.16b v11, v21
  mov.16b v12, v22
  fmla.2d v11, v6, v7[0]
  fsub.2d v12, v12, v11
  fmla.2d v12, v6, v7[0]
  add.2d v5, v5, v11
  add.2d v2, v2, v12
  ucvtf d7, x5
  mov.16b v11, v21
  mov.16b v12, v22
  fmla.2d v11, v6, v7[0]
  fsub.2d v12, v12, v11
  fmla.2d v12, v6, v7[0]
  add.2d v3, v3, v11
  add.2d v5, v5, v12
  mov x1, #31098
  movk x1, #52890, lsl 16
  movk x1, #20172, lsl 32
  movk x1, #7, lsl 48
  mov x2, #55460
  movk x2, #49931, lsl 16
  movk x2, #28124, lsl 32
  movk x2, #1, lsl 48
  mov x3, #18846
  movk x3, #13625, lsl 16
  movk x3, #40653, lsl 32
  movk x3, #4, lsl 48
  mov x4, #37560
  movk x4, #64709, lsl 16
  movk x4, #9126, lsl 32
  movk x4, #11, lsl 48
  mov x5, #28389
  movk x5, #54431, lsl 16
  movk x5, #3643, lsl 32
  ucvtf.2d v6, v9
  ucvtf d7, x1
  mov.16b v9, v21
  mov.16b v11, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v11, v11, v9
  fmla.2d v11, v6, v7[0]
  add.2d v0, v0, v9
  add.2d v4, v4, v11
  ucvtf d7, x2
  mov.16b v9, v21
  mov.16b v11, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v11, v11, v9
  fmla.2d v11, v6, v7[0]
  add.2d v1, v1, v9
  add.2d v0, v0, v11
  ucvtf d7, x3
  mov.16b v9, v21
  mov.16b v11, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v11, v11, v9
  fmla.2d v11, v6, v7[0]
  add.2d v2, v2, v9
  add.2d v1, v1, v11
  ucvtf d7, x4
  mov.16b v9, v21
  mov.16b v11, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v11, v11, v9
  fmla.2d v11, v6, v7[0]
  add.2d v5, v5, v9
  add.2d v2, v2, v11
  ucvtf d7, x5
  mov.16b v9, v21
  mov.16b v11, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v11, v11, v9
  fmla.2d v11, v6, v7[0]
  add.2d v3, v3, v9
  add.2d v5, v5, v11
  mov x1, #50130
  movk x1, #20196, lsl 16
  movk x1, #11876, lsl 32
  movk x1, #8, lsl 48
  mov x2, #45534
  movk x2, #15512, lsl 16
  movk x2, #37769, lsl 32
  movk x2, #15, lsl 48
  mov x3, #42183
  movk x3, #1232, lsl 16
  movk x3, #18174, lsl 32
  movk x3, #13, lsl 48
  mov x4, #10783
  movk x4, #54622, lsl 16
  movk x4, #61610, lsl 32
  movk x4, #8, lsl 48
  mov x5, #56963
  movk x5, #1095, lsl 16
  movk x5, #1517, lsl 32
  ucvtf.2d v6, v10
  ucvtf d7, x1
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  add.2d v0, v0, v9
  add.2d v4, v4, v10
  ucvtf d7, x2
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  add.2d v1, v1, v9
  add.2d v0, v0, v10
  ucvtf d7, x3
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  add.2d v2, v2, v9
  add.2d v1, v1, v10
  ucvtf d7, x4
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  add.2d v5, v5, v9
  add.2d v2, v2, v10
  ucvtf d7, x5
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  add.2d v3, v3, v9
  add.2d v5, v5, v10
  mov x1, #65535
  movk x1, #61439, lsl 16
  movk x1, #62867, lsl 32
  movk x1, #1, lsl 48
  umov x2, v4.d[0]
  umov x3, v4.d[1]
  mul x2, x2, x1
  mul x1, x3, x1
  and x2, x2, x0
  and x0, x1, x0
  ins v6.d[0], x2
  ins v6.d[1], x0
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
  ucvtf.2d v6, v6
  ucvtf d7, x0
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  add.2d v0, v0, v9
  add.2d v4, v4, v10
  ucvtf d7, x1
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  add.2d v1, v1, v9
  add.2d v0, v0, v10
  ucvtf d7, x2
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  add.2d v2, v2, v9
  add.2d v1, v1, v10
  ucvtf d7, x3
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  add.2d v5, v5, v9
  add.2d v2, v2, v10
  ucvtf d7, x4
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7[0]
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7[0]
  add.2d v3, v3, v9
  add.2d v5, v5, v10
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
  dup.2d v9, x0
  bic.16b v9, v9, v6
  mov x0, #26576
  movk x0, #47696, lsl 16
  movk x0, #688, lsl 32
  movk x0, #3, lsl 48
  dup.2d v10, x0
  bic.16b v10, v10, v6
  mov x0, #46800
  movk x0, #2568, lsl 16
  movk x0, #1335, lsl 32
  movk x0, #4, lsl 48
  dup.2d v11, x0
  bic.16b v11, v11, v6
  mov x0, #49763
  movk x0, #40165, lsl 16
  movk x0, #24776, lsl 32
  dup.2d v12, x0
  bic.16b v6, v12, v6
  sub.2d v0, v0, v7
  ssra.2d v0, v4, #52
  and.16b v4, v0, v8
  sub.2d v1, v1, v9
  ssra.2d v1, v0, #52
  and.16b v0, v1, v8
  sub.2d v2, v2, v10
  ssra.2d v2, v1, #52
  and.16b v1, v2, v8
  sub.2d v5, v5, v11
  ssra.2d v5, v2, #52
  and.16b v2, v5, v8
  sub.2d v3, v3, v6
  ssra.2d v3, v5, #52
  and.16b v3, v3, v8
  shl.2d v5, v0, #52
  shl.2d v6, v1, #40
  shl.2d v7, v2, #28
  shl.2d v3, v3, #16
  orr.16b v4, v4, v5
  usra.2d v6, v0, #12
  usra.2d v7, v1, #24
  usra.2d v3, v2, #36
ret
