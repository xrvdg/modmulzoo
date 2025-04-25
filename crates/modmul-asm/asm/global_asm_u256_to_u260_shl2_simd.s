//in("v0") limbs[0], in("v1") limbs[1], in("v2") limbs[2], in("v3") limbs[3],
//lateout("v0") out[0], lateout("v1") out[1], lateout("v2") out[2], lateout("v3") out[3], lateout("v4") out[4],
//lateout("x0") _, lateout("v5") _, lateout("v6") _, lateout("v7") _, lateout("v8") _,
//lateout("lr") _
        .global _u256_to_u260_shl2_simd
.align 4
.text
_u256_to_u260_shl2_simd:
  mov x0, #4503599627370495
  dup.2d v5, x0
  shl.2d v6, v1, #14
  shl.2d v7, v2, #26
  shl.2d v8, v3, #38
  ushr.2d v4, v3, #14
  shl.2d v3, v0, #2
  usra.2d v6, v0, #50
  usra.2d v7, v1, #38
  usra.2d v8, v2, #26
  and.16b v0, v3, v5
  and.16b v1, v6, v5
  and.16b v2, v7, v5
  and.16b v3, v8, v5
  ret