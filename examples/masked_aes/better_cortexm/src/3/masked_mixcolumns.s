.syntax unified
.thumb

// Require r10 containing 0xFFFF
// Compute dst := src0 + (src1 >>> 8)
.macro ror8_xor dst, src0, src1
    eor \dst, \src0, \src1, ROR #8
    eor \dst,  \dst,  \dst, LSR #16
    and \dst,  \dst, r10
.endm

// Require r10 containing 0xFFFF
// Compute dst := src0 + (src1 >>> 4) on 16 bits operands
.macro ror4_xor dst, src0, src1
    eor \dst, \src0, \src1, ROR #4
    eor \dst,  \dst,  \dst, LSR #16
    and \dst,  \dst, r10
.endm

// Require:
// - r0 pointing at state[0][i]
// - r10 containing 0xFFFF
// - r1-r9, r12, r14 will be used as tmp registers
.macro mixcolumn off
    .set addr, 0   + \off
    ldrh r1, [r0, #addr]
    .set addr, 8  + \off
    ldrh r2, [r0, #addr]
    .set addr, 16  + \off
    ldrh r3, [r0, #addr]
    .set addr, 24  + \off
    ldrh r4, [r0, #addr]
    .set addr, 32  + \off
    ldrh r5, [r0, #addr]
    .set addr, 40  + \off
    ldrh r6, [r0, #addr]
    .set addr, 48  + \off
    ldrh r7, [r0, #addr]
    .set addr, 56 + \off
    ldrh r8, [r0, #addr]

    ror4_xor  r9, r1, r1

    // R'0
    ror4_xor r12,  r2, r2

    ror4_xor  r1, r12, r1
    ror8_xor  r1,  r1, r9
    
    // R'1
    ror4_xor r14,  r3, r3

    ror4_xor  r2, r14, r2
    ror8_xor  r2,  r2, r12
    
    // R'2
    ror4_xor r12,  r4, r4

    ror4_xor  r3, r12, r3
    ror8_xor  r3,  r3, r14
    
    // R'3
    ror4_xor r14,  r5, r5

    ror4_xor  r4, r14, r4
    ror8_xor  r4,  r4, r12
    eor       r4,  r4, r9
    
    // R'4
    ror4_xor r12,  r6, r6

    ror4_xor  r5, r12, r5
    ror8_xor  r5,  r5, r14
    eor       r5,  r5, r9
    
    // R'5
    ror4_xor r14,  r7, r7

    ror4_xor  r6, r14, r6
    ror8_xor  r6,  r6, r12
    
    // R'6
    ror4_xor r12,  r8, r8

    ror4_xor  r7, r12, r7
    ror8_xor  r7,  r7, r14
    eor       r7,  r7, r9
    
    // R'7
    ror4_xor  r8,  r9, r8
    ror8_xor  r8,  r8, r12

    .set addr, 0   + \off
    strh r1, [r0, #addr]
    .set addr, 8  + \off
    strh r2, [r0, #addr]
    .set addr, 16  + \off
    strh r3, [r0, #addr]
    .set addr, 24  + \off
    strh r4, [r0, #addr]
    .set addr, 32  + \off
    strh r5, [r0, #addr]
    .set addr, 40  + \off
    strh r6, [r0, #addr]
    .set addr, 48  + \off
    strh r7, [r0, #addr]
    .set addr, 56 + \off
    strh r8, [r0, #addr]
.endm



    

// Require:
//  - r0 address of uint16_t state[4][8]
.globl masked_mixcolumns
.type masked_mixcolumns,%function
masked_mixcolumns:
    push {r4-r10, lr}
    
    movw r10, #0xFFFF

    mixcolumn 0
    mixcolumn 2
    mixcolumn 4
    mixcolumn 6

    pop {r4-r10, pc}
.size masked_mixcolumns,.-masked_mixcolumns



.globl masked_mixcolumns_shareslice
.type masked_mixcolumns_shareslice,%function
masked_mixcolumns_shareslice:
.size masked_mixcolumns_shareslice,.-masked_mixcolumns_shareslice
