.syntax unified
.thumb

// Masked xor between shared inputs at offset a and b, result stored at offset ret
// Use registers tmp1 and tmp2
// ret can be equal to a or b if needed
// Version with 8 shares (order d = 7)
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

    ldr \tmp0, [\a, #8]
    ldr \tmp1, [\b, #8]
    eor \tmp0, \tmp0, \tmp1
    str \tmp0, [\ret, #8]

    ldr \tmp0, [\a, #12]
    ldr \tmp1, [\b, #12]
    eor \tmp0, \tmp0, \tmp1
    str \tmp0, [\ret, #12]
.endm

// Masked xor between shared inputs at offset a and b, result stored at offset ret
// Use registers tmp1 and tmp2
// ret can be equal to a or b if needed
// Version with 8 shares (order d = 7)
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

    ldrh \tmp0, [\a, #8]
    ldrh \tmp1, [\b, #8]
    eor \tmp0, \tmp0, \tmp1
    strh \tmp0, [\ret, #8]

    ldrh \tmp0, [\a, #10]
    ldrh \tmp1, [\b, #10]
    eor \tmp0, \tmp0, \tmp1
    strh \tmp0, [\ret, #10]

    ldrh \tmp0, [\a, #12]
    ldrh \tmp1, [\b, #12]
    eor \tmp0, \tmp0, \tmp1
    strh \tmp0, [\ret, #12]

    ldrh \tmp0, [\a, #14]
    ldrh \tmp1, [\b, #14]
    eor \tmp0, \tmp0, \tmp1
    strh \tmp0, [\ret, #14]
.endm

// Use multiple register load and store
// Require a lot of tmp register (as many as the number of shares)
// /!\ Not tested, stmia register order is not safe /!\
.macro optim_masked_eor ret, a, b, tmp0, tmp1, tmp2, tmp3, tmp4, tmp5, tmp6, tmp7
    ldmia \a, {\tmp0, \tmp1, \tmp2, \tmp3}
    ldmia \b, {\tmp4, \tmp5, \tmp6, \tmp7}
    eor \tmp0, \tmp0, \tmp4
    eor \tmp0, \tmp1, \tmp5
    eor \tmp0, \tmp2, \tmp6
    eor \tmp0, \tmp3, \tmp7
    stmia \ret, {\tmp0, \tmp1, \tmp2, \tmp3}
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
