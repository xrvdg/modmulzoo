//in("v0") _, in("v1") _, in("v2") _, in("v3") _, in("v4") _,
//lateout("v0") out[0], lateout("v6") out[1], lateout("v7") out[2], lateout("v4") out[3],
//lateout("v1") _, lateout("v2") _, lateout("v3") _, lateout("v5") _,
//lateout("lr") _
.global _u260_to_u256_simd
.align 4
.text
_u260_to_u256_simd:
  shl.2d v5, v1, #52
  shl.2d v6, v2, #40
  shl.2d v7, v3, #28
  shl.2d v4, v4, #16
  orr.16b v0, v0, v5
  usra.2d v6, v1, #12
  usra.2d v7, v2, #24
  usra.2d v4, v3, #36
ret
