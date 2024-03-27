.syntax unified
.thumb


// Given in that contains 16 "useful" bits as 4 groups of 4 bits, gather them
// and store them
.macro gather_and_store in, m, dst, off, tmp0, tmp1
    and   \tmp0, \m, \in
    and   \tmp1, \m, \in, LSR #8
    orr   \tmp0, \tmp0, \tmp1, LSL #4
    and   \tmp1, \m, \in, LSR #16
    orr   \tmp0, \tmp0, \tmp1, LSL #8
    and   \tmp1, \m, \in, LSR #24
    orr   \tmp0, \tmp0, \tmp1, LSL #12

    strh  \tmp0, [\dst, \off]
.endm

// Given the 4 32-bits in[i], select bit off of each byte
// At the end, out contains the 16 bits as 4 groups of 4 bits
.macro partial_pack out, in0, in1, in2, in3, m, off, tmp
    and  \out, \m,   \in3, LSR \off

    and  \tmp, \m,   \in2, LSR \off
    orr  \out, \out, \tmp, LSL #1

    and  \tmp, \m,   \in1, LSR \off
    orr  \out, \out, \tmp, LSL #2

    and  \tmp, \m,   \in0, LSR \off
    orr  \out, \out, \tmp, LSL #3
.endm

// Require:
// - State as input in r0-r3
// - dst in sp + 4*(nb_saved_registers)
.global bitslice 
.type bitslice,%function
bitslice:
    push {r5-r8, lr}
    movw r12, #0x0101
    movt r12, #0x0101             // r12 = 0x01010101
    movw  r7, #0xf
    ldr  r14, [sp, #20]

    partial_pack r5, r0, r1, r2, r3, r12, #0, r6
    // At this point r5 contains 4 groups of 4 "useful" bits with 0s in between
    gather_and_store r5, r7, r14, #14, r6, r8

    // Next registers

    partial_pack r5, r0, r1, r2, r3, r12, #1, r6
    gather_and_store r5, r7, r14, #12, r6, r8

    partial_pack r5, r0, r1, r2, r3, r12, #2, r6
    gather_and_store r5, r7, r14, #10, r6, r8

    partial_pack r5, r0, r1, r2, r3, r12, #3, r6
    gather_and_store r5, r7, r14, #8, r6, r8

    partial_pack r5, r0, r1, r2, r3, r12, #4, r6
    gather_and_store r5, r7, r14, #6, r6, r8

    partial_pack r5, r0, r1, r2, r3, r12, #5, r6
    gather_and_store r5, r7, r14, #4, r6, r8

    partial_pack r5, r0, r1, r2, r3, r12, #6, r6
    gather_and_store r5, r7, r14, #2, r6, r8

    partial_pack r5, r0, r1, r2, r3, r12, #7, r6
    gather_and_store r5, r7, r14, #0, r6, r8

    pop {r5-r8, pc}
.size bitslice,.-bitslice

.macro select_pack_and_scatter_first_half dst, in0, in1, in2, in3, m0, m1, tmp0, tmp1, off, shift
    and \tmp0,  \in0,   \m0, LSR \off
    and \tmp1,  \in1,   \m0, LSR \off
    orr \tmp0, \tmp0, \tmp1, LSL #1
    and \tmp1,  \in2,   \m0, LSR \off
    orr \tmp0, \tmp0, \tmp1, LSL #2
    and \tmp1,  \in3,   \m0, LSR \off
    orr \tmp0, \tmp0, \tmp1, LSL #3

    lsl \tmp0, \tmp0, \shift

    and  \dst,   \m1, \tmp0, LSR #4

    and \tmp1,   \m1, \tmp0, LSR #8
    orr  \dst,  \dst, \tmp1, LSL #8

    and \tmp1,   \m1, \tmp0, LSR #12
    orr  \dst,  \dst, \tmp1, LSL #16

    and \tmp1,   \m1, \tmp0, LSR #16
    orr  \dst,  \dst, \tmp1, LSL #24
.endm

.macro select_pack_and_scatter_second_half dst, in0, in1, in2, in3, m0, m1, tmp0, tmp1, off, shift
    and \tmp0,  \in0,   \m0, LSR \off
    and \tmp1,  \in1,   \m0, LSR \off
    orr \tmp0, \tmp0, \tmp1, LSL #1
    and \tmp1,  \in2,   \m0, LSR \off
    orr \tmp0, \tmp0, \tmp1, LSL #2
    and \tmp1,  \in3,   \m0, LSR \off
    orr \tmp0, \tmp0, \tmp1, LSL #3

    lsl \tmp0, \tmp0, \shift

    and  \tmp1,   \m1, \tmp0, LSR #4
    orr   \dst,  \dst, \tmp1, LSL #4

    and \tmp1,   \m1, \tmp0, LSR #8
    orr  \dst,  \dst, \tmp1, LSL #12

    and \tmp1,   \m1, \tmp0, LSR #12
    orr  \dst,  \dst, \tmp1, LSL #20

    and \tmp1,   \m1, \tmp0, LSR #16
    orr  \dst,  \dst, \tmp1, LSL #28
.endm

// Require:
// - r0 containing bitsliced state address
// - r1 containing destination address
.global unbitslice 
.type unbitslice,%function
unbitslice:
    push {r4-r11, lr}

    movw r12, #0x8888
    movw r14, #0xf
    
    ldrh r2, [r0, #12]
    ldrh r3, [r0, #10]
    ldrh r4, [r0, #8 ]
    ldrh r5, [r0, #6 ]
    ldrh r6, [r0, #4 ]
    ldrh r7, [r0, #2 ]
    ldrh r8, [r0, #0 ]
    ldrh r0, [r0, #14]

    select_pack_and_scatter_first_half  r9, r0, r2, r3, r4, r12, r14, r10, r11, #0, #1
    select_pack_and_scatter_second_half r9, r5, r6, r7, r8, r12, r14, r10, r11, #0, #1
    str r9, [r1, #0 ]

    select_pack_and_scatter_first_half  r9, r0, r2, r3, r4, r12, r14, r10, r11, #1, #2
    select_pack_and_scatter_second_half r9, r5, r6, r7, r8, r12, r14, r10, r11, #1, #2
    str r9, [r1, #4 ]

    select_pack_and_scatter_first_half  r9, r0, r2, r3, r4, r12, r14, r10, r11, #2, #3
    select_pack_and_scatter_second_half r9, r5, r6, r7, r8, r12, r14, r10, r11, #2, #3
    str r9, [r1, #8 ]

    select_pack_and_scatter_first_half  r9, r0, r2, r3, r4, r12, r14, r10, r11, #3, #4
    select_pack_and_scatter_second_half r9, r5, r6, r7, r8, r12, r14, r10, r11, #3, #4
    str r9, [r1, #12]

    pop {r4-r11, pc}
.size unbitslice,.-unbitslice
