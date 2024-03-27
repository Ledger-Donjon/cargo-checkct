.syntax unified
.thumb

// Rotate a share accordingly
.macro rotshare src, mask
    and  \src, \mask, \src, ROR #1
    orr  \src,  \src, \src, LSR #16
    uxth \src,  \src
.endm

// r1 contain addres of the bitsliced and masked state
// The correct rotation is applied to all shares of a single bitslice register
.macro rotbit off, mask
    .set addr, 0   + \off
    ldrh r2, [r1, #addr]
    .set addr, 2   + \off
    ldrh r3, [r1, #addr]
    .set addr, 4   + \off
    ldrh r4, [r1, #addr]
    .set addr, 6   + \off
    ldrh r5, [r1, #addr]
    .set addr, 8   + \off
    ldrh r6, [r1, #addr]
    .set addr, 10  + \off
    ldrh r7, [r1, #addr]
    .set addr, 12  + \off
    ldrh r8, [r1, #addr]
    .set addr, 14  + \off
    ldrh r9, [r1, #addr]

    rotshare r2, \mask
    rotshare r3, \mask
    rotshare r4, \mask
    rotshare r5, \mask
    rotshare r6, \mask
    rotshare r7, \mask
    rotshare r8, \mask
    rotshare r9, \mask

    .set addr, 0   + \off
    strh r2, [r1, #addr]
    .set addr, 2   + \off
    strh r3, [r1, #addr]
    .set addr, 4   + \off
    strh r4, [r1, #addr]
    .set addr, 6   + \off
    strh r5, [r1, #addr]
    .set addr, 8   + \off
    strh r6, [r1, #addr]
    .set addr, 10  + \off
    strh r7, [r1, #addr]
    .set addr, 12  + \off
    strh r8, [r1, #addr]
    .set addr, 14  + \off
    strh r9, [r1, #addr]
.endm

// Xor the correct column from the previous rkey  with result of previous
// computation
// This macro is for only one share of only one bitslice register
.macro xorcols_share dst, prev, tmp0, tmp1, mask
    // First column: data already in dst, no shift needed
    and \tmp0, \prev, \mask            // Select column 0 of previous rkey
    eor  \dst,  \dst, \tmp0            // Xor it with subword-rot column

    // For columns 1-3: Select previous, xor and store
    and \tmp1, \prev, \mask, LSR #1    // Select column 1 of previous rkey
    eor \tmp0, \tmp1,  \dst, LSR #1    // Xor it with column 0 of current rkey
    orr  \dst,  \dst, \tmp0            // Store the result in column 1

    and \tmp1, \prev, \mask, LSR #2    // Select column 2 of previous rkey
    eor \tmp0, \tmp1, \tmp0, LSR #1    // Xor it with column 1 of current rkey
    orr  \dst,  \dst, \tmp0            // Store the result in column 2

    and \tmp1, \prev, \mask, LSR #3    // Select column 3 of previous rkey
    eor \tmp0, \tmp1, \tmp0, LSR #1    // Xor it with column 2 of current rkey
    orr  \dst,  \dst, \tmp0            // Store the result in column 3
.endm

// r0: address of previous rkey
// r1: address of current rkey
// For each share of a single bitslice register (specified by off), apply
// the xoring process of the columns to compute the current rkey
.macro xorcols_bit off, mask
    // First half of the shares
    .set addr, 0   + \off
    ldrh r2, [r1, #addr]
    ldrh r6, [r0, #addr]
    .set addr, 2   + \off
    ldrh r3, [r1, #addr]
    ldrh r7, [r0, #addr]
    .set addr, 4   + \off
    ldrh r4, [r1, #addr]
    ldrh r8, [r0, #addr]
    .set addr, 6  + \off
    ldrh r5, [r1, #addr]
    ldrh r9, [r0, #addr ]

    xorcols_share r2, r6, r10, r14, \mask
    xorcols_share r3, r7, r10, r14, \mask
    xorcols_share r4, r8, r10, r14, \mask
    xorcols_share r5, r9, r10, r14, \mask

    .set addr, 0   + \off
    strh r2, [r1, #addr]
    .set addr, 2   + \off
    strh r3, [r1, #addr]
    .set addr, 4   + \off
    strh r4, [r1, #addr]
    .set addr, 6  + \off
    strh r5, [r1, #addr]

    // Second half of the shares
    .set addr, 8   + \off
    ldrh r2, [r1, #addr]
    ldrh r6, [r0, #addr]
    .set addr, 10   + \off
    ldrh r3, [r1, #addr]
    ldrh r7, [r0, #addr]
    .set addr, 12   + \off
    ldrh r4, [r1, #addr]
    ldrh r8, [r0, #addr]
    .set addr, 14  + \off
    ldrh r5, [r1, #addr]
    ldrh r9, [r0, #addr ]

    xorcols_share r2, r6, r10, r14, \mask
    xorcols_share r3, r7, r10, r14, \mask
    xorcols_share r4, r8, r10, r14, \mask
    xorcols_share r5, r9, r10, r14, \mask

    .set addr, 8   + \off
    strh r2, [r1, #addr]
    .set addr, 10   + \off
    strh r3, [r1, #addr]
    .set addr, 12   + \off
    strh r4, [r1, #addr]
    .set addr, 14  + \off
    strh r5, [r1, #addr]
.endm

// Require:
// - r0 containing previous (masked bitslice) rkey address
// - r1 containing next (masked bitslice) rkey address already subword (
.globl masked_rotword_xorcol
.type masked_rotword_xorcol,%function
masked_rotword_xorcol:
    push {r4-r10, lr}

    // Here, we need to rotword and put in column 0 what is in column 3 of
    // the state given by r1

    movw r12, #0x0888 // Mask select column 3 after being rotated 1 bit right
    movt r12, #0x8000

    rotbit 0,   r12
    rotbit 16,  r12
    rotbit 32,  r12
    rotbit 48,  r12
    rotbit 64,  r12
    rotbit 80,  r12
    rotbit 96,  r12
    rotbit 112, r12

    // Now r1 is a state with only column 0 set.
    // Can start xor

    movw r12, #0x8888 // Mask select column 0
    movt r12, #0x8888

    xorcols_bit 0,   r12
    xorcols_bit 16,  r12
    xorcols_bit 32,  r12
    xorcols_bit 48,  r12
    xorcols_bit 64,  r12
    xorcols_bit 80,  r12
    xorcols_bit 96,  r12
    xorcols_bit 112, r12

    pop {r4-r10, pc}
.size masked_rotword_xorcol,.-masked_rotword_xorcol
