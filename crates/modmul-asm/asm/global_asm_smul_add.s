.text
.global _smul_add
.align 4
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
