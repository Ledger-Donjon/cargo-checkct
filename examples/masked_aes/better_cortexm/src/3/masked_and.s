.syntax unified
.thumb


.macro prologue
    push {r4-r11, lr} // Save registers of caller, somehow r12 is a scratch register
.endm

.macro epilogue
    pop {r4-r11, pc} // Restore saved registers of caller and resume inside caller
.endm

// intertwined operands to prepare for masked and
// Require:
//  - r0, r1, r2, r3 containing the address of the operands (r0 & r1 and r2 & r3)

// /!\ Every gp registers (r0 - r15) will be used /!\
// /!\ except for r15 (PC) and r13 (SP) /!\

// Result:
// ai = (r0[i] << 16) || r2[i]
// bi = (r1[i] << 16) || r3[i]
// Layout at the end:
// Registers:
// r0:  X
// r1:  X
// r2:  X
// r3:  X
// r4:  a0
// r5:  a1
// r6:  a2
// r7:  a3
// r8:  b0
// r9:  b1
// r10: b2
// r11: b3
// r12: X
// r14: X

.macro load_operands_masked_and
    ldrh r4,  [r2, #0] // half of a0
    ldrh r5,  [r2, #2] // half of a1
    ldrh r6,  [r2, #4] // half of a2
    ldrh r7,  [r2, #6] // half of a3
    ldrh r8,  [r3, #0] // half of b0
    ldrh r9,  [r3, #2] // half of b1
    ldrh r10, [r3, #4] // half of b2
    ldrh r11, [r3, #6] // half of b3

    ldrh r12, [r0, #0] // other half of a0
    ldrh r14, [r0, #2] // other half of a1
    ldrh r2,  [r0, #4] // other half of a2
    ldrh r3,  [r0, #6] // other half of a3

    orr  r4, r4, r12,  LSL #16 // combine a0
    orr  r5, r5, r14,  LSL #16 // combine a1
    orr  r6, r6, r2,   LSL #16 // combine a2
    orr  r7, r7, r3,   LSL #16 // combine a3

    ldrh r12, [r1, #0] // other half of b0
    ldrh r14, [r1, #2] // other half of b1
    ldrh r2,  [r1, #4] // other half of b2
    ldrh r3,  [r1, #6] // other half of b3

    orr  r8,  r8, r12, LSL #16 // combine b0
    orr  r9,  r9, r14, LSL #16 // combine b1
    orr  r10, r10, r2, LSL #16 // combine b2
    orr  r11, r11, r3, LSL #16 // combine b3
.endm

// Require:
// - tmp, a temporary register
// - addr of fresh randomness at sp + 52
// Result:
// - Stack:
// sp + 4: fresh_rands[1]
// sp    : fresh_rands[0]
// Load rand00, rand01 and rand10 directly in registers out0, out1, out2
.macro load_random tmp0, tmp1, out0, out1, out2
    ldr \tmp0, [sp, #52] // Load array address

    ldr \tmp1, [\tmp0, #0] // Load data
    str \tmp1, [sp, #0] // Store it

    ldr \tmp1, [\tmp0, #4]
    str \tmp1, [sp, #4] // Store it

    ldr \out0, [\tmp0, #8]
    ldr \out1, [\tmp0, #12]
    ldr \out2, [\tmp0, #16]
.endm

// Require:
//  - One tmp register
// 
// Result:
//  - Stack:
// sp -  28: 0 
// ...
// sp - 104: 0
// TODO: More generic zeroing out memory
.macro zero_random tmp
    eor \tmp, \tmp, \tmp // \tmp = 0
    str \tmp, [sp, #0]
    str \tmp, [sp, #4]
    eor r0, r0, r0
    eor r1, r1, r1
    eor r2, r2, r2 
.endm

// TODO: Write macro gathering random


// According to reference, str are always one cycle because pipelined with
// next instruction
// (https://developer.arm.com/documentation/ddi0439/b/Programmers-Model/Instruction-set-summary/Load-store-timings)
//
// src and tmp registers will be trashed
.macro split_and_store dst0, dst1, src, tmp, mask, off
    lsr  \tmp, \src, #16        // First half (res0) 
    strh \tmp, [\dst0, \off]
    and  \src, \src, \mask      // Other half (res1)
    strh \src, [\dst1, \off]
.endm

// Require:
//  - Address of res0 and res1 at (respectively) sp+36 and sp+40
//  - Shares of intertwined results:
//      * T0 at sp - 108
//      * T1 at sp - 132
//      * T2 at sp - 148
//      * T3 at sp - 40
//      * T4 in r12
//      * T5 in r4
//      * T6 in r3
//      * T7 in r0

// Result:
//  - res0 and res1 array correctly filled with result of masked_and
.macro store_results_masked
                        // T0 already in r14
                        // T1 already in r12
                        // T2 already in r10
                        // T3 already in r0

    ldr r7, [sp, #44  ] // Load res0 addr
    ldr r8, [sp, #48  ] // Load res1 addr

    mov r11, #0xFFFF
    split_and_store r7, r8,  r14, r9, r11, #0
    split_and_store r7, r8,  r12, r9, r11, #2
    split_and_store r7, r8,  r10, r9, r11, #4
    split_and_store r7, r8,   r0, r9, r11, #6
.endm



// Require this layout at the start:

// Registers will be:
// r0:  X
// r1:  X
// r2:  X
// r3:  X
// r4:  a0
// r5:  a1
// r6:  a2
// r7:  a3
// r8:  b0
// r9:  b1
// r10: b2
// r11: b3
// r12: X
// r14: X

// Stack layout during execution:
// sp +  52: fresh_randoms array addr
// sp +  48: res1 array addr
// sp +  44: res0 array addr
// sp +  40: start of saved registers
// ...
// sp +   8: end of saved registers
// sp +   4: rand03
// sp      : rand02

// Result:
// - Results (c0, c1, c2, c3) in: r14, r12, r10, r0
.globl masked_and
.type masked_and,%function
masked_and:
    prologue

    sub sp, #8

    load_operands_masked_and
    load_random r12, r14, r0, r1, r2

    and  r3,  r4,  r8    //Exec s00 = a0 & b0 into r3
    eor r12,  r3,  r0    //Exec y01 = s00 ^ rand00 into r12
    and r14,  r4,  r9    //Exec s01 = a0 & b1 into r14
    eor  r3, r12, r14    //Exec y02 = y01 ^ s01 into r3
    and r12,  r4, r10    //Exec s02 = a0 & b2 into r12
    and  r4,  r4, r11    //Exec s03 = a0 & b3 into r4
    and r14,  r5,  r8    //Exec s10 = a1 & b0 into r14
    eor  r3,  r3, r14    //Exec y03 = y02 ^ s10 into r3
    eor  r3,  r3,  r1    //Exec y04 = y03 ^ rand01 into r3
    eor  r3,  r3, r12    //Exec y05 = y04 ^ s02 into r3
    and r12,  r5,  r9    //Exec s11 = a1 & b1 into r12
    eor  r1, r12,  r1    //Exec y11 = s11 ^ rand01 into r1
    and r12,  r5, r10    //Exec s12 = a1 & b2 into r12
    eor  r1,  r1, r12    //Exec y12 = y11 ^ s12 into r1
    and  r5,  r5, r11    //Exec s13 = a1 & b3 into r5
    and r12,  r6,  r8    //Exec s20 = a2 & b0 into r12
    eor  r3,  r3, r12    //Exec y06 = y05 ^ s20 into r3
    eor  r14,  r3,  r2    //Exec T0  = y06 ^ rand10 into r3
    //str  r14, [sp, #-12 ] //Store r3/T0 on stack
    and  r3,  r6,  r9    //Exec s21 = a2 & b1 into r3
    eor  r1,  r1,  r3    //Exec y13 = y12 ^ s21 into r1
    ldr  r3, [sp, #0  ] //Load rand02 into r3
    eor  r1,  r1,  r3    //Exec y14 = y13 ^ rand02 into r1
    eor  r1,  r1,  r5    //Exec y15 = y14 ^ s13 into r1
    and  r5,  r6, r10    //Exec s22 = a2 & b2 into r5
    eor  r3,  r5,  r3    //Exec y21 = s22 ^ rand02 into r3
    and  r5,  r6, r11    //Exec s23 = a2 & b3 into r5
    eor  r3,  r3,  r5    //Exec y22 = y21 ^ s23 into r3
    and  r5,  r7,  r8    //Exec s30 = a3 & b0 into r5
    and  r6,  r7,  r9    //Exec s31 = a3 & b1 into r6
    eor  r1,  r1,  r6    //Exec y16 = y15 ^ s31 into r1
    eor  r12,  r1,  r2    //Exec T1  = y16 ^ rand10 into r1
    //str  r12, [sp, #0  ] //Store r1/T1 on stack
    and  r1,  r7, r10    //Exec s32 = a3 & b2 into r1
    eor  r1,  r3,  r1    //Exec y23 = y22 ^ s32 into r1
    ldr  r3, [sp,  #4  ] //Load rand03 into r3
    eor  r1,  r1,  r3    //Exec y24 = y23 ^ rand03 into r1
    eor  r10,  r1,  r2    //Exec T2  = y24 ^ rand10 into r1
    //str  r10, [sp, #-16 ] //Store r1/T2 on stack
    and  r1,  r7, r11    //Exec s33 = a3 & b3 into r1
    eor  r1,  r1,  r3    //Exec y31 = s33 ^ rand03 into r1
    eor  r1,  r1,  r5    //Exec y32 = y31 ^ s30 into r1
    eor  r1,  r1,  r4    //Exec y33 = y32 ^ s03 into r1
    eor  r0,  r1,  r0    //Exec y34 = y33 ^ rand00 into r0
    eor  r0,  r0,  r2    //Exec T3  = y34 ^ rand10 into r0
    
    store_results_masked
    add sp, #8
    epilogue
.size masked_and,.-masked_and
