//in("v0") in0[0], in("v1") in0[1], in("v2") in0[2], in("v3") in0[3], in("v4") in0[4],
//lateout("v0") out0[0], lateout("v5") out0[1], lateout("v6") out0[2], lateout("v7") out0[3],
//lateout("v1") _, lateout("v2") _, lateout("v3") _, lateout("v4") _,
//lateout("lr") _
.global _u260_to_u256_simd
.align 4
.text
_u260_to_u256_simd:
  ushr.2d v5, v1, #12
  ushr.2d v6, v2, #24
  ushr.2d v7, v3, #36
  sli.2d v0, v1, #52
  sli.2d v5, v2, #40
  sli.2d v6, v3, #28
  sli.2d v7, v4, #16
ret
