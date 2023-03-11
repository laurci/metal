.section .text
.globl _start
_start:
    li x5, 0x1
    li x28, 0xA
    li x6, 0x10000000
    sw x28, 0(x6)

