//in("x0") a[0], in("x1") a[1], in("x2") a[2], in("x3") a[3],
//in("x4") b[0], in("x5") b[1], in("x6") b[2], in("x7") b[3],
//lateout("x8") out[0], lateout("x9") out[1], lateout("x10") out[2], lateout("x11") out[3], lateout("x4") out[4], lateout("x5") out[5], lateout("x6") out[6], lateout("x7") out[7],
//lateout("x0") _, lateout("x1") _, lateout("x2") _, lateout("x3") _, lateout("x12") _, lateout("x13") _, lateout("x14") _,
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
  umulh x13, x0, x7
  adds x11, x12, x11
  cinc x12, x13, hs
  mul x13, x1, x7
  umulh x14, x1, x7
  adds x12, x13, x12
  cinc x13, x14, hs
  adds x4, x12, x4
  cinc x12, x13, hs
  mul x13, x2, x7
  umulh x14, x2, x7
  adds x12, x13, x12
  cinc x13, x14, hs
  adds x5, x12, x5
  cinc x12, x13, hs
  mul x13, x3, x7
  umulh x7, x3, x7
  adds x12, x13, x12
  cinc x7, x7, hs
  adds x6, x12, x6
  cinc x7, x7, hs
  ret