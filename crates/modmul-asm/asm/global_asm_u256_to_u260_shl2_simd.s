//in("v0") in0[0], in("v1") in0[1], in("v2") in0[2], in("v3") in0[3],
//lateout("v0") out0[0], lateout("v1") out0[1], lateout("v2") out0[2], lateout("v4") out0[3], lateout("v3") out0[4],
//lateout("x0") _, lateout("v5") _, lateout("v6") _, lateout("v7") _, lateout("v8") _,
//lateout("lr") _
.global _u256_to_u260_shl2_simd
.align 4
.text
_u256_to_u260_shl2_simd:
  mov x0, #4503599627370495
  dup.2d v4, x0
  shl.2d v5, v1, #14
  shl.2d v6, v2, #26
  shl.2d v7, v3, #38
  shl.2d v8, v0, #2
  usra.2d v5, v0, #50
  usra.2d v6, v1, #38
  usra.2d v7, v2, #26
  and.16b v0, v8, v4
  and.16b v1, v5, v4
  and.16b v2, v6, v4
  and.16b v4, v7, v4
  ushr.2d v3, v3, #14
ret
