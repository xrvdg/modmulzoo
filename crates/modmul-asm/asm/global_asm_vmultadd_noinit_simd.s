//in("v0") _, in("v1") _, in("v2") _, in("v3") _, in("v4") _, in("v5") _, in("v6") _, in("v7") _, in("v8") _, in("v9") _, in("v10") _, in("v11") _, in("v12") _, in("v13") _, in("v14") _, in("v15") _, in("v16") _, in("v17") _, in("v18") _, in("v19") _,
//lateout("v0") out[0], lateout("v1") out[1], lateout("v2") out[2], lateout("v3") out[3], lateout("v4") out[4], lateout("v5") out[5], lateout("v6") out[6], lateout("v7") out[7], lateout("v8") out[8], lateout("v9") out[9],
//lateout("x0") _, lateout("v10") _, lateout("v11") _, lateout("v12") _, lateout("v13") _, lateout("v14") _, lateout("v15") _, lateout("v16") _, lateout("v17") _, lateout("v18") _, lateout("v19") _, lateout("v20") _, lateout("v21") _, lateout("v22") _, lateout("v23") _, lateout("v24") _,
//lateout("lr") _
.global _vmultadd_noinit_simd
.align 4
.text
_vmultadd_noinit_simd:
  mov x0, #5075556780046548992
  dup.2d v20, x0
  movk x0, #1, lsl 0
  movk x0, #18032, lsl 48
  dup.2d v21, x0
  ucvtf.2d v10, v10
  ucvtf.2d v22, v15
  mov.16b v23, v20
  mov.16b v24, v21
  fmla.2d v23, v10, v22
  fsub.2d v24, v24, v23
  fmla.2d v24, v10, v22
  add.2d v1, v1, v23
  add.2d v0, v0, v24
  ucvtf.2d v22, v16
  mov.16b v23, v20
  mov.16b v24, v21
  fmla.2d v23, v10, v22
  fsub.2d v24, v24, v23
  fmla.2d v24, v10, v22
  add.2d v2, v2, v23
  add.2d v1, v1, v24
  ucvtf.2d v22, v17
  mov.16b v23, v20
  mov.16b v24, v21
  fmla.2d v23, v10, v22
  fsub.2d v24, v24, v23
  fmla.2d v24, v10, v22
  add.2d v3, v3, v23
  add.2d v2, v2, v24
  ucvtf.2d v22, v18
  mov.16b v23, v20
  mov.16b v24, v21
  fmla.2d v23, v10, v22
  fsub.2d v24, v24, v23
  fmla.2d v24, v10, v22
  add.2d v4, v4, v23
  add.2d v3, v3, v24
  ucvtf.2d v22, v19
  mov.16b v23, v20
  mov.16b v24, v21
  fmla.2d v23, v10, v22
  fsub.2d v24, v24, v23
  fmla.2d v24, v10, v22
  add.2d v5, v5, v23
  add.2d v4, v4, v24
  ucvtf.2d v10, v11
  ucvtf.2d v11, v15
  mov.16b v22, v20
  mov.16b v23, v21
  fmla.2d v22, v10, v11
  fsub.2d v23, v23, v22
  fmla.2d v23, v10, v11
  add.2d v2, v2, v22
  add.2d v1, v1, v23
  ucvtf.2d v11, v16
  mov.16b v22, v20
  mov.16b v23, v21
  fmla.2d v22, v10, v11
  fsub.2d v23, v23, v22
  fmla.2d v23, v10, v11
  add.2d v3, v3, v22
  add.2d v2, v2, v23
  ucvtf.2d v11, v17
  mov.16b v22, v20
  mov.16b v23, v21
  fmla.2d v22, v10, v11
  fsub.2d v23, v23, v22
  fmla.2d v23, v10, v11
  add.2d v4, v4, v22
  add.2d v3, v3, v23
  ucvtf.2d v11, v18
  mov.16b v22, v20
  mov.16b v23, v21
  fmla.2d v22, v10, v11
  fsub.2d v23, v23, v22
  fmla.2d v23, v10, v11
  add.2d v5, v5, v22
  add.2d v4, v4, v23
  ucvtf.2d v11, v19
  mov.16b v22, v20
  mov.16b v23, v21
  fmla.2d v22, v10, v11
  fsub.2d v23, v23, v22
  fmla.2d v23, v10, v11
  add.2d v6, v6, v22
  add.2d v5, v5, v23
  ucvtf.2d v10, v12
  ucvtf.2d v11, v15
  mov.16b v12, v20
  mov.16b v22, v21
  fmla.2d v12, v10, v11
  fsub.2d v22, v22, v12
  fmla.2d v22, v10, v11
  add.2d v3, v3, v12
  add.2d v2, v2, v22
  ucvtf.2d v11, v16
  mov.16b v12, v20
  mov.16b v22, v21
  fmla.2d v12, v10, v11
  fsub.2d v22, v22, v12
  fmla.2d v22, v10, v11
  add.2d v4, v4, v12
  add.2d v3, v3, v22
  ucvtf.2d v11, v17
  mov.16b v12, v20
  mov.16b v22, v21
  fmla.2d v12, v10, v11
  fsub.2d v22, v22, v12
  fmla.2d v22, v10, v11
  add.2d v5, v5, v12
  add.2d v4, v4, v22
  ucvtf.2d v11, v18
  mov.16b v12, v20
  mov.16b v22, v21
  fmla.2d v12, v10, v11
  fsub.2d v22, v22, v12
  fmla.2d v22, v10, v11
  add.2d v6, v6, v12
  add.2d v5, v5, v22
  ucvtf.2d v11, v19
  mov.16b v12, v20
  mov.16b v22, v21
  fmla.2d v12, v10, v11
  fsub.2d v22, v22, v12
  fmla.2d v22, v10, v11
  add.2d v7, v7, v12
  add.2d v6, v6, v22
  ucvtf.2d v10, v13
  ucvtf.2d v11, v15
  mov.16b v12, v20
  mov.16b v13, v21
  fmla.2d v12, v10, v11
  fsub.2d v13, v13, v12
  fmla.2d v13, v10, v11
  add.2d v4, v4, v12
  add.2d v3, v3, v13
  ucvtf.2d v11, v16
  mov.16b v12, v20
  mov.16b v13, v21
  fmla.2d v12, v10, v11
  fsub.2d v13, v13, v12
  fmla.2d v13, v10, v11
  add.2d v5, v5, v12
  add.2d v4, v4, v13
  ucvtf.2d v11, v17
  mov.16b v12, v20
  mov.16b v13, v21
  fmla.2d v12, v10, v11
  fsub.2d v13, v13, v12
  fmla.2d v13, v10, v11
  add.2d v6, v6, v12
  add.2d v5, v5, v13
  ucvtf.2d v11, v18
  mov.16b v12, v20
  mov.16b v13, v21
  fmla.2d v12, v10, v11
  fsub.2d v13, v13, v12
  fmla.2d v13, v10, v11
  add.2d v7, v7, v12
  add.2d v6, v6, v13
  ucvtf.2d v11, v19
  mov.16b v12, v20
  mov.16b v13, v21
  fmla.2d v12, v10, v11
  fsub.2d v13, v13, v12
  fmla.2d v13, v10, v11
  add.2d v8, v8, v12
  add.2d v7, v7, v13
  ucvtf.2d v10, v14
  ucvtf.2d v11, v15
  mov.16b v12, v20
  mov.16b v13, v21
  fmla.2d v12, v10, v11
  fsub.2d v13, v13, v12
  fmla.2d v13, v10, v11
  add.2d v5, v5, v12
  add.2d v4, v4, v13
  ucvtf.2d v11, v16
  mov.16b v12, v20
  mov.16b v13, v21
  fmla.2d v12, v10, v11
  fsub.2d v13, v13, v12
  fmla.2d v13, v10, v11
  add.2d v6, v6, v12
  add.2d v5, v5, v13
  ucvtf.2d v11, v17
  mov.16b v12, v20
  mov.16b v13, v21
  fmla.2d v12, v10, v11
  fsub.2d v13, v13, v12
  fmla.2d v13, v10, v11
  add.2d v7, v7, v12
  add.2d v6, v6, v13
  ucvtf.2d v11, v18
  mov.16b v12, v20
  mov.16b v13, v21
  fmla.2d v12, v10, v11
  fsub.2d v13, v13, v12
  fmla.2d v13, v10, v11
  add.2d v8, v8, v12
  add.2d v7, v7, v13
  ucvtf.2d v11, v19
  mov.16b v12, v20
  mov.16b v13, v21
  fmla.2d v12, v10, v11
  fsub.2d v13, v13, v12
  fmla.2d v13, v10, v11
  add.2d v9, v9, v12
  add.2d v8, v8, v13
ret
