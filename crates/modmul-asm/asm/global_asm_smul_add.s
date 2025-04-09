//in("x0") _, in("x1") _, in("x2") _, in("x3") _, in("x4") _, in("x5") _, in("x6") _, in("x7") _, in("x8") _, in("x9") _,
//lateout("x10") out0[0], lateout("x0") out0[1], lateout("x1") out0[2], lateout("x2") out0[3], lateout("x3") out0[4],
//lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _, lateout("x8") _, lateout("x9") _,
//lateout("lr") _
.global _smul_add
.align 4
.text
_smul_add:
  mul x10, x5, x9
  umulh x5, x5, x9
  adds x10, x10, x0
  cinc x5, x5, hs
  mul x0, x6, x9
  umulh x6, x6, x9
  adds x0, x0, x5
  cinc x6, x6, hs
  adds x0, x0, x1
  cinc x6, x6, hs
  mul x1, x7, x9
  umulh x5, x7, x9
  adds x1, x1, x6
  cinc x5, x5, hs
  adds x1, x1, x2
  cinc x5, x5, hs
  mul x2, x8, x9
  umulh x6, x8, x9
  adds x2, x2, x5
  cinc x6, x6, hs
  adds x2, x2, x3
  cinc x6, x6, hs
  add x3, x4, x6
ret
