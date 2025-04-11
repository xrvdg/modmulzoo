//in("x0") in0[0], in("x1") in0[1], in("x2") in0[2], in("x3") in0[3], in("x4") in1[0], in("x5") in1[1], in("x6") in1[2], in("x7") in1[3],
//lateout("x8") out0[0], lateout("x9") out0[1], lateout("x10") out0[2], lateout("x11") out0[3], lateout("x0") out0[4], lateout("x1") out0[5], lateout("x2") out0[6], lateout("x3") out0[7],
//lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _, lateout("x12") _, lateout("x13") _, lateout("x14") _,
//lateout("lr") _
.global _school_method
.align 4
.text
_school_method:
  mul x8, x0, x4
  umulh x9, x0, x4
  mul x10, x1, x4
  umulh x11, x1, x4
  adds x9, x10, x9
  cinc x10, x11, hs
  mul x11, x2, x4
  umulh x12, x2, x4
  adds x10, x11, x10
  cinc x11, x12, hs
  mul x12, x3, x4
  umulh x4, x3, x4
  adds x11, x12, x11
  cinc x4, x4, hs
  mul x12, x0, x5
  umulh x13, x0, x5
  adds x9, x12, x9
  cinc x12, x13, hs
  mul x13, x1, x5
  umulh x14, x1, x5
  adds x12, x13, x12
  cinc x13, x14, hs
  adds x10, x12, x10
  cinc x12, x13, hs
  mul x13, x2, x5
  umulh x14, x2, x5
  adds x12, x13, x12
  cinc x13, x14, hs
  adds x11, x12, x11
  cinc x12, x13, hs
  mul x13, x3, x5
  umulh x5, x3, x5
  adds x12, x13, x12
  cinc x5, x5, hs
  adds x4, x12, x4
  cinc x5, x5, hs
  mul x12, x0, x6
  umulh x13, x0, x6
  adds x10, x12, x10
  cinc x12, x13, hs
  mul x13, x1, x6
  umulh x14, x1, x6
  adds x12, x13, x12
  cinc x13, x14, hs
  adds x11, x12, x11
  cinc x12, x13, hs
  mul x13, x2, x6
  umulh x14, x2, x6
  adds x12, x13, x12
  cinc x13, x14, hs
  adds x4, x12, x4
  cinc x12, x13, hs
  mul x13, x3, x6
  umulh x6, x3, x6
  adds x12, x13, x12
  cinc x6, x6, hs
  adds x5, x12, x5
  cinc x6, x6, hs
  mul x12, x0, x7
  umulh x0, x0, x7
  adds x11, x12, x11
  cinc x0, x0, hs
  mul x12, x1, x7
  umulh x1, x1, x7
  adds x0, x12, x0
  cinc x1, x1, hs
  adds x0, x0, x4
  cinc x1, x1, hs
  mul x4, x2, x7
  umulh x2, x2, x7
  adds x1, x4, x1
  cinc x2, x2, hs
  adds x1, x1, x5
  cinc x2, x2, hs
  mul x4, x3, x7
  umulh x3, x3, x7
  adds x2, x4, x2
  cinc x3, x3, hs
  adds x2, x2, x6
  cinc x3, x3, hs
ret
