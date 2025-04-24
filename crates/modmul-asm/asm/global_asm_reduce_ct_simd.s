//in("v0") in0[0], in("v1") in0[1], in("v2") in0[2], in("v3") in0[3], in("v4") in0[4], in("v5") in0[5],
//lateout("v0") out0[0], lateout("v1") out0[1], lateout("v2") out0[2], lateout("v3") out0[3], lateout("v4") out0[4],
//lateout("x0") _, lateout("v5") _, lateout("v6") _, lateout("v7") _, lateout("v8") _, lateout("v9") _, lateout("v10") _, lateout("v11") _, lateout("v12") _,
//lateout("lr") _
        .global _reduce_ct_simd
.align 4
.text
_reduce_ct_simd:
  mov x0, #4503599627370495
  dup.2d v6, x0
  mov x0, #140737488355328
  dup.2d v7, x0
  and.16b v7, v5, v7
  cmeq.2d v7, v7, #0
  mov x0, #2
  movk x0, #57344, lsl 16
  movk x0, #60199, lsl 32
  movk x0, #3, lsl 48
  dup.2d v8, x0
  bic.16b v8, v8, v7
  mov x0, #10364
  movk x0, #11794, lsl 16
  movk x0, #3895, lsl 32
  movk x0, #9, lsl 48
  dup.2d v9, x0
  bic.16b v9, v9, v7
  mov x0, #26576
  movk x0, #47696, lsl 16
  movk x0, #688, lsl 32
  movk x0, #3, lsl 48
  dup.2d v10, x0
  bic.16b v10, v10, v7
  mov x0, #46800
  movk x0, #2568, lsl 16
  movk x0, #1335, lsl 32
  movk x0, #4, lsl 48
  dup.2d v11, x0
  bic.16b v11, v11, v7
  mov x0, #49763
  movk x0, #40165, lsl 16
  movk x0, #24776, lsl 32
  dup.2d v12, x0
  bic.16b v7, v12, v7
  sub.2d v1, v1, v8
  ssra.2d v1, v0, #52
  sub.2d v2, v2, v9
  ssra.2d v2, v1, #52
  sub.2d v3, v3, v10
  ssra.2d v3, v2, #52
  sub.2d v4, v4, v11
  ssra.2d v4, v3, #52
  sub.2d v5, v5, v7
  ssra.2d v5, v4, #52
  and.16b v0, v1, v6
  and.16b v1, v2, v6
  and.16b v2, v3, v6
  and.16b v3, v4, v6
  and.16b v4, v5, v6
  ret