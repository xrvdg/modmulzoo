//in("x0") in0[0], in("x1") in0[1], in("x2") in0[2], in("x3") in0[3], in("x4") in0[4], in("x5") in1[0], in("x6") in1[1], in("x7") in1[2], in("x8") in1[3], in("x9") in2[0],
//lateout("x0") out0[0], lateout("x1") out0[1], lateout("x2") out0[2], lateout("x3") out0[3], lateout("x4") out0[4],
//lateout("x5") _, lateout("x6") _, lateout("x7") _, lateout("x8") _, lateout("x9") _, lateout("x10") _,
//lateout("lr") _
.global _smul_add
.align 4
.text
_smul_add:
  mul x10, x5, x9
  umulh x5, x5, x9
  adds x0, x10, x0
  cinc x5, x5, hs
  mul x10, x6, x9
  umulh x6, x6, x9
  adds x5, x10, x5
  cinc x6, x6, hs
  adds x1, x5, x1
  cinc x5, x6, hs
  mul x6, x7, x9
  umulh x7, x7, x9
  adds x5, x6, x5
  cinc x6, x7, hs
  adds x2, x5, x2
  cinc x5, x6, hs
  mul x6, x8, x9
  umulh x7, x8, x9
  adds x5, x6, x5
  cinc x6, x7, hs
  adds x3, x5, x3
  cinc x5, x6, hs
  add x4, x4, x5
ret
