//in("v0") in0[0], in("v1") in0[1], in("v2") in0[2], in("v3") in0[3], in("v4") in1[0], in("v5") in1[1], in("v6") in1[2], in("v7") in1[3],
//lateout("v4") out0[0], lateout("v6") out0[1], lateout("v7") out0[2], lateout("v3") out0[3],
//lateout("x0") _, lateout("v0") _, lateout("x1") _, lateout("v1") _, lateout("x2") _, lateout("v2") _, lateout("x3") _, lateout("v5") _, lateout("v8") _, lateout("v9") _, lateout("v10") _, lateout("v11") _, lateout("v12") _, lateout("v13") _, lateout("v14") _, lateout("v15") _, lateout("v16") _, lateout("v17") _, lateout("v18") _, lateout("v19") _, lateout("v20") _, lateout("v21") _, lateout("v22") _, lateout("v23") _, lateout("v24") _,
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
  ucvtf.2d v1, v1
  ucvtf.2d v2, v2
  ucvtf.2d v9, v9
  ucvtf.2d v3, v3
  ucvtf.2d v4, v4
  ucvtf.2d v5, v5
  ucvtf.2d v6, v6
  ucvtf.2d v10, v10
  ucvtf.2d v7, v7
  mov.16b v23, v21
  mov.16b v24, v22
  fmla.2d v23, v0, v4
  fsub.2d v24, v24, v23
  fmla.2d v24, v0, v4
  add.2d v13, v13, v23
  add.2d v11, v11, v24
  mov.16b v23, v21
  mov.16b v24, v22
  fmla.2d v23, v0, v5
  fsub.2d v24, v24, v23
  fmla.2d v24, v0, v5
  add.2d v15, v15, v23
  add.2d v13, v13, v24
  mov.16b v23, v21
  mov.16b v24, v22
  fmla.2d v23, v0, v6
  fsub.2d v24, v24, v23
  fmla.2d v24, v0, v6
  add.2d v17, v17, v23
  add.2d v15, v15, v24
  mov.16b v23, v21
  mov.16b v24, v22
  fmla.2d v23, v0, v10
  fsub.2d v24, v24, v23
  fmla.2d v24, v0, v10
  add.2d v19, v19, v23
  add.2d v17, v17, v24
  mov.16b v23, v21
  mov.16b v24, v22
  fmla.2d v23, v0, v7
  fsub.2d v24, v24, v23
  fmla.2d v24, v0, v7
  add.2d v0, v20, v23
  add.2d v19, v19, v24
  mov.16b v20, v21
  mov.16b v23, v22
  fmla.2d v20, v1, v4
  fsub.2d v23, v23, v20
  fmla.2d v23, v1, v4
  add.2d v15, v15, v20
  add.2d v13, v13, v23
  mov.16b v20, v21
  mov.16b v23, v22
  fmla.2d v20, v1, v5
  fsub.2d v23, v23, v20
  fmla.2d v23, v1, v5
  add.2d v17, v17, v20
  add.2d v15, v15, v23
  mov.16b v20, v21
  mov.16b v23, v22
  fmla.2d v20, v1, v6
  fsub.2d v23, v23, v20
  fmla.2d v23, v1, v6
  add.2d v19, v19, v20
  add.2d v17, v17, v23
  mov.16b v20, v21
  mov.16b v23, v22
  fmla.2d v20, v1, v10
  fsub.2d v23, v23, v20
  fmla.2d v23, v1, v10
  add.2d v0, v0, v20
  add.2d v19, v19, v23
  mov.16b v20, v21
  mov.16b v23, v22
  fmla.2d v20, v1, v7
  fsub.2d v23, v23, v20
  fmla.2d v23, v1, v7
  add.2d v1, v18, v20
  add.2d v0, v0, v23
  mov.16b v18, v21
  mov.16b v20, v22
  fmla.2d v18, v2, v4
  fsub.2d v20, v20, v18
  fmla.2d v20, v2, v4
  add.2d v17, v17, v18
  add.2d v15, v15, v20
  mov.16b v18, v21
  mov.16b v20, v22
  fmla.2d v18, v2, v5
  fsub.2d v20, v20, v18
  fmla.2d v20, v2, v5
  add.2d v18, v19, v18
  add.2d v17, v17, v20
  mov.16b v19, v21
  mov.16b v20, v22
  fmla.2d v19, v2, v6
  fsub.2d v20, v20, v19
  fmla.2d v20, v2, v6
  add.2d v0, v0, v19
  add.2d v18, v18, v20
  mov.16b v19, v21
  mov.16b v20, v22
  fmla.2d v19, v2, v10
  fsub.2d v20, v20, v19
  fmla.2d v20, v2, v10
  add.2d v1, v1, v19
  add.2d v0, v0, v20
  mov.16b v19, v21
  mov.16b v20, v22
  fmla.2d v19, v2, v7
  fsub.2d v20, v20, v19
  fmla.2d v20, v2, v7
  add.2d v2, v16, v19
  add.2d v1, v1, v20
  mov.16b v16, v21
  mov.16b v19, v22
  fmla.2d v16, v9, v4
  fsub.2d v19, v19, v16
  fmla.2d v19, v9, v4
  add.2d v16, v18, v16
  add.2d v17, v17, v19
  mov.16b v18, v21
  mov.16b v19, v22
  fmla.2d v18, v9, v5
  fsub.2d v19, v19, v18
  fmla.2d v19, v9, v5
  add.2d v0, v0, v18
  add.2d v16, v16, v19
  mov.16b v18, v21
  mov.16b v19, v22
  fmla.2d v18, v9, v6
  fsub.2d v19, v19, v18
  fmla.2d v19, v9, v6
  add.2d v1, v1, v18
  add.2d v0, v0, v19
  mov.16b v18, v21
  mov.16b v19, v22
  fmla.2d v18, v9, v10
  fsub.2d v19, v19, v18
  fmla.2d v19, v9, v10
  add.2d v2, v2, v18
  add.2d v1, v1, v19
  mov.16b v18, v21
  mov.16b v19, v22
  fmla.2d v18, v9, v7
  fsub.2d v19, v19, v18
  fmla.2d v19, v9, v7
  add.2d v9, v14, v18
  add.2d v2, v2, v19
  mov.16b v14, v21
  mov.16b v18, v22
  fmla.2d v14, v3, v4
  fsub.2d v18, v18, v14
  fmla.2d v18, v3, v4
  add.2d v0, v0, v14
  add.2d v4, v16, v18
  mov.16b v14, v21
  mov.16b v16, v22
  fmla.2d v14, v3, v5
  fsub.2d v16, v16, v14
  fmla.2d v16, v3, v5
  add.2d v1, v1, v14
  add.2d v0, v0, v16
  mov.16b v5, v21
  mov.16b v14, v22
  fmla.2d v5, v3, v6
  fsub.2d v14, v14, v5
  fmla.2d v14, v3, v6
  add.2d v2, v2, v5
  add.2d v1, v1, v14
  mov.16b v5, v21
  mov.16b v6, v22
  fmla.2d v5, v3, v10
  fsub.2d v6, v6, v5
  fmla.2d v6, v3, v10
  add.2d v5, v9, v5
  add.2d v2, v2, v6
  mov.16b v6, v21
  mov.16b v9, v22
  fmla.2d v6, v3, v7
  fsub.2d v9, v9, v6
  fmla.2d v9, v3, v7
  add.2d v3, v12, v6
  add.2d v5, v5, v9
  usra.2d v13, v11, #52
  usra.2d v15, v13, #52
  usra.2d v17, v15, #52
  usra.2d v4, v17, #52
  and.16b v6, v11, v8
  and.16b v7, v13, v8
  and.16b v9, v15, v8
  and.16b v10, v17, v8
  ucvtf.2d v6, v6
  mov x1, #37864
  movk x1, #1815, lsl 16
  movk x1, #28960, lsl 32
  movk x1, #17153, lsl 48
  dup.2d v11, x1
  mov.16b v12, v21
  mov.16b v13, v22
  fmla.2d v12, v6, v11
  fsub.2d v13, v13, v12
  fmla.2d v13, v6, v11
  add.2d v0, v0, v12
  add.2d v4, v4, v13
  mov x1, #46128
  movk x1, #29964, lsl 16
  movk x1, #7587, lsl 32
  movk x1, #17161, lsl 48
  dup.2d v11, x1
  mov.16b v12, v21
  mov.16b v13, v22
  fmla.2d v12, v6, v11
  fsub.2d v13, v13, v12
  fmla.2d v13, v6, v11
  add.2d v1, v1, v12
  add.2d v0, v0, v13
  mov x1, #52826
  movk x1, #57790, lsl 16
  movk x1, #55431, lsl 32
  movk x1, #17196, lsl 48
  dup.2d v11, x1
  mov.16b v12, v21
  mov.16b v13, v22
  fmla.2d v12, v6, v11
  fsub.2d v13, v13, v12
  fmla.2d v13, v6, v11
  add.2d v2, v2, v12
  add.2d v1, v1, v13
  mov x1, #31276
  movk x1, #21262, lsl 16
  movk x1, #2304, lsl 32
  movk x1, #17182, lsl 48
  dup.2d v11, x1
  mov.16b v12, v21
  mov.16b v13, v22
  fmla.2d v12, v6, v11
  fsub.2d v13, v13, v12
  fmla.2d v13, v6, v11
  add.2d v5, v5, v12
  add.2d v2, v2, v13
  mov x1, #28672
  movk x1, #24515, lsl 16
  movk x1, #54929, lsl 32
  movk x1, #17064, lsl 48
  dup.2d v11, x1
  mov.16b v12, v21
  mov.16b v13, v22
  fmla.2d v12, v6, v11
  fsub.2d v13, v13, v12
  fmla.2d v13, v6, v11
  add.2d v3, v3, v12
  add.2d v5, v5, v13
  ucvtf.2d v6, v7
  mov x1, #44768
  movk x1, #51919, lsl 16
  movk x1, #6346, lsl 32
  movk x1, #17133, lsl 48
  dup.2d v7, x1
  mov.16b v11, v21
  mov.16b v12, v22
  fmla.2d v11, v6, v7
  fsub.2d v12, v12, v11
  fmla.2d v12, v6, v7
  add.2d v0, v0, v11
  add.2d v4, v4, v12
  mov x1, #47492
  movk x1, #23630, lsl 16
  movk x1, #49985, lsl 32
  movk x1, #17168, lsl 48
  dup.2d v7, x1
  mov.16b v11, v21
  mov.16b v12, v22
  fmla.2d v11, v6, v7
  fsub.2d v12, v12, v11
  fmla.2d v12, v6, v7
  add.2d v1, v1, v11
  add.2d v0, v0, v12
  mov x1, #57936
  movk x1, #54828, lsl 16
  movk x1, #18292, lsl 32
  movk x1, #17197, lsl 48
  dup.2d v7, x1
  mov.16b v11, v21
  mov.16b v12, v22
  fmla.2d v11, v6, v7
  fsub.2d v12, v12, v11
  fmla.2d v12, v6, v7
  add.2d v2, v2, v11
  add.2d v1, v1, v12
  mov x1, #17708
  movk x1, #43915, lsl 16
  movk x1, #64348, lsl 32
  movk x1, #17188, lsl 48
  dup.2d v7, x1
  mov.16b v11, v21
  mov.16b v12, v22
  fmla.2d v11, v6, v7
  fsub.2d v12, v12, v11
  fmla.2d v12, v6, v7
  add.2d v5, v5, v11
  add.2d v2, v2, v12
  mov x1, #29184
  movk x1, #20789, lsl 16
  movk x1, #19197, lsl 32
  movk x1, #17083, lsl 48
  dup.2d v7, x1
  mov.16b v11, v21
  mov.16b v12, v22
  fmla.2d v11, v6, v7
  fsub.2d v12, v12, v11
  fmla.2d v12, v6, v7
  add.2d v3, v3, v11
  add.2d v5, v5, v12
  ucvtf.2d v6, v9
  mov x1, #58856
  movk x1, #14953, lsl 16
  movk x1, #15155, lsl 32
  movk x1, #17181, lsl 48
  dup.2d v7, x1
  mov.16b v9, v21
  mov.16b v11, v22
  fmla.2d v9, v6, v7
  fsub.2d v11, v11, v9
  fmla.2d v11, v6, v7
  add.2d v0, v0, v9
  add.2d v4, v4, v11
  mov x1, #35392
  movk x1, #12477, lsl 16
  movk x1, #56780, lsl 32
  movk x1, #17142, lsl 48
  dup.2d v7, x1
  mov.16b v9, v21
  mov.16b v11, v22
  fmla.2d v9, v6, v7
  fsub.2d v11, v11, v9
  fmla.2d v11, v6, v7
  add.2d v1, v1, v9
  add.2d v0, v0, v11
  mov x1, #9848
  movk x1, #54501, lsl 16
  movk x1, #31540, lsl 32
  movk x1, #17170, lsl 48
  dup.2d v7, x1
  mov.16b v9, v21
  mov.16b v11, v22
  fmla.2d v9, v6, v7
  fsub.2d v11, v11, v9
  fmla.2d v11, v6, v7
  add.2d v2, v2, v9
  add.2d v1, v1, v11
  mov x1, #9584
  movk x1, #63883, lsl 16
  movk x1, #18253, lsl 32
  movk x1, #17190, lsl 48
  dup.2d v7, x1
  mov.16b v9, v21
  mov.16b v11, v22
  fmla.2d v9, v6, v7
  fsub.2d v11, v11, v9
  fmla.2d v11, v6, v7
  add.2d v5, v5, v9
  add.2d v2, v2, v11
  mov x1, #51712
  movk x1, #16093, lsl 16
  movk x1, #30633, lsl 32
  movk x1, #17068, lsl 48
  dup.2d v7, x1
  mov.16b v9, v21
  mov.16b v11, v22
  fmla.2d v9, v6, v7
  fsub.2d v11, v11, v9
  fmla.2d v11, v6, v7
  add.2d v3, v3, v9
  add.2d v5, v5, v11
  ucvtf.2d v6, v10
  mov x1, #34724
  movk x1, #40393, lsl 16
  movk x1, #23752, lsl 32
  movk x1, #17184, lsl 48
  dup.2d v7, x1
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7
  add.2d v0, v0, v9
  add.2d v4, v4, v10
  mov x1, #25532
  movk x1, #31025, lsl 16
  movk x1, #10002, lsl 32
  movk x1, #17199, lsl 48
  dup.2d v7, x1
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7
  add.2d v1, v1, v9
  add.2d v0, v0, v10
  mov x1, #18830
  movk x1, #2465, lsl 16
  movk x1, #36348, lsl 32
  movk x1, #17194, lsl 48
  dup.2d v7, x1
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7
  add.2d v2, v2, v9
  add.2d v1, v1, v10
  mov x1, #21566
  movk x1, #43708, lsl 16
  movk x1, #57685, lsl 32
  movk x1, #17185, lsl 48
  dup.2d v7, x1
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7
  add.2d v5, v5, v9
  add.2d v2, v2, v10
  mov x1, #3072
  movk x1, #8058, lsl 16
  movk x1, #46097, lsl 32
  movk x1, #17047, lsl 48
  dup.2d v7, x1
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7
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
  ucvtf.2d v6, v6
  mov x0, #16
  movk x0, #22847, lsl 32
  movk x0, #17151, lsl 48
  dup.2d v7, x0
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7
  add.2d v0, v0, v9
  add.2d v4, v4, v10
  mov x0, #20728
  movk x0, #23588, lsl 16
  movk x0, #7790, lsl 32
  movk x0, #17170, lsl 48
  dup.2d v7, x0
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7
  add.2d v1, v1, v9
  add.2d v0, v0, v10
  mov x0, #16000
  movk x0, #53891, lsl 16
  movk x0, #5509, lsl 32
  movk x0, #17144, lsl 48
  dup.2d v7, x0
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7
  add.2d v2, v2, v9
  add.2d v1, v1, v10
  mov x0, #46800
  movk x0, #2568, lsl 16
  movk x0, #1335, lsl 32
  movk x0, #17188, lsl 48
  dup.2d v7, x0
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7
  add.2d v5, v5, v9
  add.2d v2, v2, v10
  mov x0, #39040
  movk x0, #14704, lsl 16
  movk x0, #12839, lsl 32
  movk x0, #17096, lsl 48
  dup.2d v7, x0
  mov.16b v9, v21
  mov.16b v10, v22
  fmla.2d v9, v6, v7
  fsub.2d v10, v10, v9
  fmla.2d v10, v6, v7
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
