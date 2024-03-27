.syntax unified
.thumb

// Masked xor between shared inputs at offset a and b, result stored at offset ret
// Use registers tmp1 and tmp2
// ret can be equal to a or b if needed
// Version with 4 shares (order d = 3)
.macro masked_eor_old ret, a, b, tmp0, tmp1
    // One xor every two shares
    ldr \tmp0, [\a, #0]
    ldr \tmp1, [\b, #0]
    eor \tmp0, \tmp0, \tmp1
    str \tmp0, [\ret, #0]

    ldr \tmp0, [\a, #4]
    ldr \tmp1, [\b, #4]
    eor \tmp0, \tmp0, \tmp1
    str \tmp0, [\ret, #4]
.endm

// Masked xor between shared inputs at offset a and b, result stored at offset ret
// Use registers tmp1 and tmp2
// ret can be equal to a or b if needed
// Version with 4 shares (order d = 3)
// Never two shares in the same register
.macro masked_eor ret, a, b, tmp0, tmp1
    ldrh \tmp0, [\a, #0]
    ldrh \tmp1, [\b, #0]
    eor \tmp0, \tmp0, \tmp1
    strh \tmp0, [\ret, #0]

    ldrh \tmp0, [\a, #2]
    ldrh \tmp1, [\b, #2]
    eor \tmp0, \tmp0, \tmp1
    strh \tmp0, [\ret, #2]

    ldrh \tmp0, [\a, #4]
    ldrh \tmp1, [\b, #4]
    eor \tmp0, \tmp0, \tmp1
    strh \tmp0, [\ret, #4]

    ldrh \tmp0, [\a, #6]
    ldrh \tmp1, [\b, #6]
    eor \tmp0, \tmp0, \tmp1
    strh \tmp0, [\ret, #6]
.endm

// Require:
// - r0 = operand0
// - r1 = operand1
// - r2 = result 
.globl masked_xor
.type masked_xor,%function
masked_xor:
    // Don't need to save r3 or r12, they are scratch registers

    masked_eor r2, r0, r1, r12, r3
    bx lr
.size masked_xor,.-masked_xor
