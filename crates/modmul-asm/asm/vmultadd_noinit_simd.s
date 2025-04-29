//in("v0") t[0], in("v1") t[1], in("v2") t[2], in("v3") t[3], in("v4") t[4], in("v5") t[5], in("v6") t[6], in("v7") t[7], in("v8") t[8], in("v9") t[9],
//in("v10") a[0], in("v11") a[1], in("v12") a[2], in("v13") a[3], in("v14") a[4],
//in("v15") b[0], in("v16") b[1], in("v17") b[2], in("v18") b[3], in("v19") b[4],
//lateout("v0") out[0], lateout("v1") out[1], lateout("v2") out[2], lateout("v3") out[3], lateout("v4") out[4], lateout("v5") out[5], lateout("v6") out[6], lateout("v7") out[7], lateout("v8") out[8], lateout("v9") out[9],
//lateout("x0") _, lateout("v10") _, lateout("v11") _, lateout("v12") _, lateout("v13") _, lateout("v14") _, lateout("v15") _, lateout("v16") _, lateout("v17") _, lateout("v18") _, lateout("v19") _, lateout("v20") _, lateout("v21") _, lateout("v22") _, lateout("v23") _,
//lateout("lr") _
        
.global _vmultadd_noinit_simd
.align 4
.text
_vmultadd_noinit_simd:
  mov x0, #5075556780046548992
  dup.2d v20, x0
  mov x0, #1
  movk x0, #18032, lsl 48
  dup.2d v21, x0
  ucvtf.2d v10, v10
  ucvtf.2d v11, v11
  ucvtf.2d v12, v12
  ucvtf.2d v13, v13
  ucvtf.2d v14, v14
  ucvtf.2d v15, v15
  ucvtf.2d v16, v16
  ucvtf.2d v17, v17
  ucvtf.2d v18, v18
  ucvtf.2d v19, v19
  mov.16b v22, v20
  fmla.2d v22, v10, v15
  fsub.2d v23, v21, v22
  fmla.2d v23, v10, v15
  add.2d v1, v1, v22
  add.2d v0, v0, v23
  mov.16b v22, v20
  fmla.2d v22, v10, v16
  fsub.2d v23, v21, v22
  fmla.2d v23, v10, v16
  add.2d v2, v2, v22
  add.2d v1, v1, v23
  mov.16b v22, v20
  fmla.2d v22, v10, v17
  fsub.2d v23, v21, v22
  fmla.2d v23, v10, v17
  add.2d v3, v3, v22
  add.2d v2, v2, v23
  mov.16b v22, v20
  fmla.2d v22, v10, v18
  fsub.2d v23, v21, v22
  fmla.2d v23, v10, v18
  add.2d v4, v4, v22
  add.2d v3, v3, v23
  mov.16b v22, v20
  fmla.2d v22, v10, v19
  fsub.2d v23, v21, v22
  fmla.2d v23, v10, v19
  add.2d v5, v5, v22
  add.2d v4, v4, v23
  mov.16b v10, v20
  fmla.2d v10, v11, v15
  fsub.2d v22, v21, v10
  fmla.2d v22, v11, v15
  add.2d v2, v2, v10
  add.2d v1, v1, v22
  mov.16b v10, v20
  fmla.2d v10, v11, v16
  fsub.2d v22, v21, v10
  fmla.2d v22, v11, v16
  add.2d v3, v3, v10
  add.2d v2, v2, v22
  mov.16b v10, v20
  fmla.2d v10, v11, v17
  fsub.2d v22, v21, v10
  fmla.2d v22, v11, v17
  add.2d v4, v4, v10
  add.2d v3, v3, v22
  mov.16b v10, v20
  fmla.2d v10, v11, v18
  fsub.2d v22, v21, v10
  fmla.2d v22, v11, v18
  add.2d v5, v5, v10
  add.2d v4, v4, v22
  mov.16b v10, v20
  fmla.2d v10, v11, v19
  fsub.2d v22, v21, v10
  fmla.2d v22, v11, v19
  add.2d v6, v6, v10
  add.2d v5, v5, v22
  mov.16b v10, v20
  fmla.2d v10, v12, v15
  fsub.2d v11, v21, v10
  fmla.2d v11, v12, v15
  add.2d v3, v3, v10
  add.2d v2, v2, v11
  mov.16b v10, v20
  fmla.2d v10, v12, v16
  fsub.2d v11, v21, v10
  fmla.2d v11, v12, v16
  add.2d v4, v4, v10
  add.2d v3, v3, v11
  mov.16b v10, v20
  fmla.2d v10, v12, v17
  fsub.2d v11, v21, v10
  fmla.2d v11, v12, v17
  add.2d v5, v5, v10
  add.2d v4, v4, v11
  mov.16b v10, v20
  fmla.2d v10, v12, v18
  fsub.2d v11, v21, v10
  fmla.2d v11, v12, v18
  add.2d v6, v6, v10
  add.2d v5, v5, v11
  mov.16b v10, v20
  fmla.2d v10, v12, v19
  fsub.2d v11, v21, v10
  fmla.2d v11, v12, v19
  add.2d v7, v7, v10
  add.2d v6, v6, v11
  mov.16b v10, v20
  fmla.2d v10, v13, v15
  fsub.2d v11, v21, v10
  fmla.2d v11, v13, v15
  add.2d v4, v4, v10
  add.2d v3, v3, v11
  mov.16b v10, v20
  fmla.2d v10, v13, v16
  fsub.2d v11, v21, v10
  fmla.2d v11, v13, v16
  add.2d v5, v5, v10
  add.2d v4, v4, v11
  mov.16b v10, v20
  fmla.2d v10, v13, v17
  fsub.2d v11, v21, v10
  fmla.2d v11, v13, v17
  add.2d v6, v6, v10
  add.2d v5, v5, v11
  mov.16b v10, v20
  fmla.2d v10, v13, v18
  fsub.2d v11, v21, v10
  fmla.2d v11, v13, v18
  add.2d v7, v7, v10
  add.2d v6, v6, v11
  mov.16b v10, v20
  fmla.2d v10, v13, v19
  fsub.2d v11, v21, v10
  fmla.2d v11, v13, v19
  add.2d v8, v8, v10
  add.2d v7, v7, v11
  mov.16b v10, v20
  fmla.2d v10, v14, v15
  fsub.2d v11, v21, v10
  fmla.2d v11, v14, v15
  add.2d v5, v5, v10
  add.2d v4, v4, v11
  mov.16b v10, v20
  fmla.2d v10, v14, v16
  fsub.2d v11, v21, v10
  fmla.2d v11, v14, v16
  add.2d v6, v6, v10
  add.2d v5, v5, v11
  mov.16b v10, v20
  fmla.2d v10, v14, v17
  fsub.2d v11, v21, v10
  fmla.2d v11, v14, v17
  add.2d v7, v7, v10
  add.2d v6, v6, v11
  mov.16b v10, v20
  fmla.2d v10, v14, v18
  fsub.2d v11, v21, v10
  fmla.2d v11, v14, v18
  add.2d v8, v8, v10
  add.2d v7, v7, v11
  mov.16b v10, v20
  fmla.2d v10, v14, v19
  fsub.2d v11, v21, v10
  fmla.2d v11, v14, v19
  add.2d v9, v9, v10
  add.2d v8, v8, v11
  ret