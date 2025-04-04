//in("v0") _, in("x1") _,
//lateout("v2") out[0],
//lateout("x0") _, lateout("v0") _, lateout("x1") _, lateout("v1[0]") _, lateout("d1") _,
//lateout("lr") _
.global _smul_noint_simd
.align 4
.text
_smul_noint_simd:
  ucvtf.2d v0, v0
  mov x0, #0
  ucvtf d1, x1
  dup.2d v2, x0
  mov.16b v2, v2
  fmla.2d v2, v0, v1[0]
ret
