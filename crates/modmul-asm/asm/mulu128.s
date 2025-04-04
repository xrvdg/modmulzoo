.text
.global _mulu128
.align 4
_mulu128:
    mul x2, x0, x1
    umulh x3, x0, x1
    ret
