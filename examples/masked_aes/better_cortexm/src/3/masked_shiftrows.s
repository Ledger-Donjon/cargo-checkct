.syntax unified
.thumb

.macro swapmove r, m, n, t
    eor \t, \r, \r, LSL \n
    and \t, \m
    eor \r, \t
    eor \r, \r, \t, LSR \n
.endm

// Require:
//  - r0 pointing at state[0]
//  - r1-r4 tmp register for each share
//  - r12 mask0
//  - r14 mask1
//  - r9 tmp register for swapmove
// 
// Apply shiftrow on two bitslice registers at a time
.macro masked_shiftrow off
    .set addr0, \off
    .set addr1, \off + 8
    ldrh r1, [r0, #addr0]
    ldrh r9, [r0, #addr1]
    orr  r1, r1, r9, LSL #16
    .set addr0, \off + 2
    .set addr1, \off + 2 + 8
    ldrh r2, [r0, #addr0]
    ldrh r9, [r0, #addr1]
    orr  r2, r2, r9, LSL #16
    .set addr0, \off + 4
    .set addr1, \off + 4 + 8
    ldrh r3, [r0, #addr0]
    ldrh r9, [r0, #addr1]
    orr  r3, r3, r9, LSL #16
    .set addr0, \off + 6
    .set addr1, \off + 6 + 8
    ldrh r4, [r0, #addr0]
    ldrh r9, [r0, #addr1]
    orr  r4, r4, r9, LSL #16

    swapmove r1, r12, #2, r9
    swapmove r2, r12, #2, r9
    swapmove r3, r12, #2, r9
    swapmove r4, r12, #2, r9

    swapmove r1, r14, #1, r9
    swapmove r2, r14, #1, r9
    swapmove r3, r14, #1, r9
    swapmove r4, r14, #1, r9

    .set addr0, \off
    .set addr1, \off + 8
    strh r1, [r0, #addr0]
    lsr  r1, r1, #16
    strh r1, [r0, #addr1]
    .set addr0, \off + 2
    .set addr1, \off + 2 + 8
    strh r2, [r0, #addr0]
    lsr  r2, r2, #16
    strh r2, [r0, #addr1]
    .set addr0, \off + 4
    .set addr1, \off + 4 + 8
    strh r3, [r0, #addr0]
    lsr  r3, r3, #16
    strh r3, [r0, #addr1]
    .set addr0, \off + 6
    .set addr1, \off + 6 + 8
    strh r4, [r0, #addr0]
    lsr  r4, r4, #16
    strh r4, [r0, #addr1]
.endm

// Require:
// - r0 = state pointer
.globl masked_shiftrows
.type masked_shiftrows,%function
masked_shiftrows:
    // Don't need to save r1, r2, r3 or r12, they are scratch registers
    // lr = r14
    push {r4, r9, lr}

    movw r12, #0x4C80
    movt r12, #0x4C80
    movw r14, #0xA0A0
    movt r14, #0xA0A0

    masked_shiftrow 0
    masked_shiftrow 16
    masked_shiftrow 32
    masked_shiftrow 48
    
    pop {r4, r9, pc}
.size masked_shiftrows,.-masked_shiftrows








// Sharesliced /!\
.macro masked_shiftrow_shareslice row, tmp0, tmp1, tmp2, tmp3, tmp4, tmp5, tmp6
    ldr \tmp0, [\row, #0 ]
    ldr \tmp1, [\row, #4 ]
    ldr \tmp2, [\row, #8 ]
    ldr \tmp3, [\row, #12]

    swapmove \tmp0, \tmp4, #2, \tmp6
    swapmove \tmp1, \tmp4, #2, \tmp6
    swapmove \tmp2, \tmp4, #2, \tmp6
    swapmove \tmp3, \tmp4, #2, \tmp6

    swapmove \tmp0, \tmp5, #1, \tmp6
    swapmove \tmp1, \tmp5, #1, \tmp6
    swapmove \tmp2, \tmp5, #1, \tmp6
    swapmove \tmp3, \tmp5, #1, \tmp6
    
    str \tmp0, [\row, #0 ]
    str \tmp1, [\row, #4 ]
    str \tmp2, [\row, #8 ]
    str \tmp3, [\row, #12]
.endm

// Require:
// - r0 = state pointer
.globl masked_shiftrows_shareslice
.type masked_shiftrows_shareslice,%function
masked_shiftrows_shareslice:
    // Don't need to save r1, r2, r3 or r12, they are scratch registers
    // lr = r14
    push {r4, r5, r6, lr}

    movw r6, #0x4C80
    movt r6, #0x4C80
    movw r2, #0xA0A0
    movt r2, #0xA0A0
    // r0 is already at the right value
    add r1, r0, #16
    masked_shiftrow_shareslice r0, r3, r12, r4, r5, r6, r2, r14 
    masked_shiftrow_shareslice r1, r3, r12, r4, r5, r6, r2, r14 
    add r0, r0, #32
    add r1, r1, #32
    masked_shiftrow_shareslice r0, r3, r12, r4, r5, r6, r2, r14 
    masked_shiftrow_shareslice r1, r3, r12, r4, r5, r6, r2, r14 
    add r0, r0, #32
    add r1, r1, #32
    masked_shiftrow_shareslice r0, r3, r12, r4, r5, r6, r2, r14 
    masked_shiftrow_shareslice r1, r3, r12, r4, r5, r6, r2, r14 
    add r0, r0, #32
    add r1, r1, #32
    masked_shiftrow_shareslice r0, r3, r12, r4, r5, r6, r2, r14 
    masked_shiftrow_shareslice r1, r3, r12, r4, r5, r6, r2, r14 
    
    pop {r4, r5, r6, pc}
.size masked_shiftrows_shareslice,.-masked_shiftrows_shareslice
