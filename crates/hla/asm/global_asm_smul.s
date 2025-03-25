.text
_smul:
  mul x5, x1, x0
  umulh x1, x1, x0
  mul x6, x2, x0
  umulh x2, x2, x0
  adds x1, x1, x6
  cinc x2, x2, hs
  mul x6, x3, x0
  umulh x3, x3, x0
  adds x2, x2, x6
  cinc x3, x3, hs
  mul x6, x4, x0
  umulh x0, x4, x0
  adds x3, x3, x6
  cinc x0, x0, hs
ret
