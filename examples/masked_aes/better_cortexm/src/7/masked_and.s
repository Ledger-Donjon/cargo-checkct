.syntax unified
.thumb


// Load inside a single register the share at offset off for both r0 and r1
// Require:
//  - One register to store the result (rd) which can be equal to r0 (but
//  trashed)
//  - One temporary register (tmp) which can be equal to r2 (but trashed)
//  - a0 and a1 containing the address of the shared value to load
//  - off the offset to load
// Result:
//  - rd contains (a0[off] << 16) || a1[off]
.macro double_ldr rd, a0, a1, off, tmp
    ldrh \rd,  [\a0, \off]
    ldrh \tmp, [\a1, \off]
    orr  \rd,  \tmp, \rd, LSL #16
.endm

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
// r12: b4
// r14: b5

// Stack:
// sp + 196: fresh_randoms array addr
// sp + 192: res1 array addr
// sp + 188: res0 array addr
// sp + 186: start of saved registers
// ...
// sp + 152: end of saved registers
// sp + 148: a4
// sp + 144: a5
// sp + 140: a6
// sp + 136: a7
// sp + 132: b6
// sp + 128: b7

// 4 blocks of load/store and start with load (should be pipelined with last random store)
.macro load_operands_masked_and

    ldrh r8,  [r2, #8]   // half of a4
    ldrh r9,  [r2, #10]  // half of a5
    ldrh r10, [r2, #12]  // half of a6
    ldrh r11, [r2, #14]  // half of a7


    ldrh r4, [r0, #8]   // other half of a4
    ldrh r5, [r0, #10]  // other half of a5
    ldrh r6, [r0, #12]  // other half of a6
    ldrh r7, [r0, #14]  // other half of a7

    orr  r4, r8, r4,  LSL #16 // combine a4
    orr  r5, r9, r5,  LSL #16 // combine a5

    ldrh r12,  [r3, #12]  // half of b6
    ldrh r14,  [r3, #14]  // half of b7

    ldrh r8,  [r1, #12]  // other half of b6
    ldrh r9,  [r1, #14]  // other half of b7

    orr  r6, r10, r6,  LSL #16 // combine a6
    orr  r7, r11, r7,  LSL #16 // combine a7

    orr  r8, r12, r8, LSL #16 // combine b6
    orr  r9, r14, r9, LSL #16 // combine b7

    //stmdb      sp, {r4, r5, r6, r7, r8, r9} // /!\ not in the right order

    // Store values computed before bewteen sp-4 and sp-24
    str  r4, [sp, #148]
    str  r5, [sp, #144]
    str  r6, [sp, #140]
    str  r7, [sp, #136]
    str  r8, [sp, #132]
    str  r9, [sp, #128]

    // Available registers : r4-r12, r14

    ldrh r4,  [r2,  #0] // Load r2[0] into r4
    ldrh r5,  [r2,  #2] // Load r2[1] into r5
    ldrh r6,  [r2,  #4] // Load r2[2] into r6
    ldrh r7,  [r2,  #6] // Load r2[3] into r7

    ldrh r8,  [r0, #0] // Load r0[0] into r8
    ldrh r9,  [r0, #2] // Load r0[1] into r9
    ldrh r10, [r0, #4] // Load r0[2] into r10
    ldrh r11, [r0, #6] // Load r0[3] into r11

    ldrh r0,  [r3, #0] // half of b0
    ldrh r2,  [r3, #2] // half of b1

    ldrh r12,  [r1, #0] // other half of b0
    ldrh r14,  [r1, #2] // other half of b1

    // Available registers: None

    // Load second part of the register (r0[i] << 16)
    orr  r4, r4,  r8, LSL #16 // combine a0
    orr  r5, r5,  r9, LSL #16 // combine a1
    orr  r6, r6, r10, LSL #16 // combine a2
    orr  r7, r7, r11, LSL #16 // combine a3

    orr  r8, r0, r12, LSL #16 // combine b0
    orr  r9, r2, r14, LSL #16 // combine b1

    // Available registers: r0, r2, r10, r11, r12, r14

    ldrh r10, [r3, #4] // half of b2
    ldrh r2,  [r3, #6] // half of b3
    ldrh r0,  [r3, #8] // half of b4
    ldrh r3,  [r3, #10]// half of b5

    ldrh r11, [r1, #4] // other half of b2
    ldrh r12, [r1, #6] // other half of b3
    ldrh r14, [r1, #8] // other half of b4
    ldrh r1,  [r1, #10]// other half of b5

    orr  r10, r10, r11, LSL #16 // combine b2
    orr  r11,  r2, r12, LSL #16 // combine b3
    orr  r12,  r0, r14, LSL #16 // combine b4
    orr  r14,  r3,  r1, LSL #16 // combine b5

    // Available register : r0, r1, r2, r3

.endm

// Require:
// - tmp, a temporary register
// - addr of fresh randomness at sp + 196
// Result:
// - Stack:
// sp + 124: fresh_rands[0]
// ...
// sp +  48: fresh_rands[19]
.macro load_random tmp0, tmp1
    ldr \tmp0, [sp, #196] // Load array address

    ldr \tmp1, [\tmp0, #0] // Load data
    str \tmp1, [sp, #124] // Store it

    ldr \tmp1, [\tmp0, #4]
    str \tmp1, [sp, #120]

    ldr \tmp1, [\tmp0, #8]
    str \tmp1, [sp, #116]

    ldr \tmp1, [\tmp0, #12]
    str \tmp1, [sp, #112]

    ldr \tmp1, [\tmp0, #16]
    str \tmp1, [sp, #108]

    ldr \tmp1, [\tmp0, #20]
    str \tmp1, [sp, #104]

    ldr \tmp1, [\tmp0, #24]
    str \tmp1, [sp, #100]

    ldr \tmp1, [\tmp0, #28]
    str \tmp1, [sp, #96]

    ldr \tmp1, [\tmp0, #32]
    str \tmp1, [sp, #92]

    ldr \tmp1, [\tmp0, #36]
    str \tmp1, [sp, #88]

    ldr \tmp1, [\tmp0, #40]
    str \tmp1, [sp, #84]

    ldr \tmp1, [\tmp0, #44]
    str \tmp1, [sp, #80]

    ldr \tmp1, [\tmp0, #48]
    str \tmp1, [sp, #76]

    ldr \tmp1, [\tmp0, #52]
    str \tmp1, [sp, #72]

    ldr \tmp1, [\tmp0, #56]
    str \tmp1, [sp, #68]

    ldr \tmp1, [\tmp0, #60]
    str \tmp1, [sp, #64]

    ldr \tmp1, [\tmp0, #64]
    str \tmp1, [sp, #60]

    ldr \tmp1, [\tmp0, #68]
    str \tmp1, [sp, #56]

    ldr \tmp1, [\tmp0, #72]
    str \tmp1, [sp, #52]

    ldr \tmp1, [\tmp0, #76]
    str \tmp1, [sp, #48]
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
    str \tmp, [sp, #124]
    str \tmp, [sp, #120]
    str \tmp, [sp, #116]
    str \tmp, [sp, #112]
    str \tmp, [sp, #108]
    str \tmp, [sp, #104]
    str \tmp, [sp, #100]
    str \tmp, [sp, #96 ]
    str \tmp, [sp, #92 ]
    str \tmp, [sp, #88 ]
    str \tmp, [sp, #84 ]
    str \tmp, [sp, #80 ]
    str \tmp, [sp, #76 ]
    str \tmp, [sp, #72 ]
    str \tmp, [sp, #68 ]
    str \tmp, [sp, #64 ]
    str \tmp, [sp, #60 ]
    str \tmp, [sp, #56 ]
    str \tmp, [sp, #52 ]
    str \tmp, [sp, #48 ]
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
    ldr r1, [sp, #44] // Load T0 in r1
    ldr r2, [sp, #20] // Load T1 in r2
    ldr r5, [sp, #4] // Load T2 in r5
    ldr r6, [sp, #112 ] // Load T3 in r6
                        // T4 already in r12
                        // T5 already in r4
                        // T6 already in r3
                        // T7 already in r0

    ldr r7, [sp, #188  ] // Load res0 addr
    ldr r8, [sp, #192  ] // Load res1 addr

    mov r10, #0xFFFF
    split_and_store r7, r8,  r1, r9, r10, #0
    split_and_store r7, r8,  r2, r9, r10, #2
    split_and_store r7, r8,  r5, r9, r10, #4
    split_and_store r7, r8,  r6, r9, r10, #6
    split_and_store r7, r8, r12, r9, r10, #8
    split_and_store r7, r8,  r4, r9, r10, #10
    split_and_store r7, r8,  r3, r9, r10, #12
    split_and_store r7, r8,  r0, r9, r10, #14
.endm



// Require this layout at the start:

// Registers:
// r0:  address of masked operand 0
// r1:  address of masked operand 1
// r2:  address of masked operand 2
// r3:  address of masked operand 3
// r4:  X
// r5:  X
// r6:  X
// r7:  X
// r8:  X
// r9:  X
// r10: X
// r11: X
// r12: X
// r14: X

// Stack during the function:
// sp + 196: fresh_randoms array addr
// sp + 192: res1 array addr
// sp + 188: res0 array addr
// sp + 184: start of saved registers
// ...
// sp + 152: end of saved registers
// sp + 148: a4
// sp + 144: a5
// sp + 140: a6
// sp + 136: a7
// sp + 132: b6
// sp + 128: b7
// sp + 124: rand00
// ...
// sp +  48: rand23
// sp +  44: tmp11
// sp      : tmp0

// Result:
// - Store result at sp + 92 -> sp + 123

// prologue: 1 block, 11 stores
// zero_random: 1 block, 20 stores
// load_operands_masked_and: 4 blocks, 38 loads
// Body: 57 loads, 30 stores
// store_results_masked: 1 block of 6 loads, 16 stores
// epilogue: 1 block, 11 loads

// Total macro: 77 stores, 112 loads (but a lot are pipelined and take only
// one cycle)

// TODO: HARDWARE LEAK during execution of a single instruction

.globl masked_and
.type masked_and,%function
masked_and:
    prologue

    sub sp, #152

    load_random r4, r5
    load_operands_masked_and

    and  r0,  r4,  r8    //Exec s00 = a0 & b0 into r0
    ldr  r1, [sp, #124 ] //Load rand00 into r1
    eor  r2,  r0,  r1    //Exec y01 = s00 ^ rand00 into r2
    and  r3,  r4,  r9    //Exec s01 = a0 & b1 into r3
    eor  r0,  r2,  r3    //Exec y02 = y01 ^ s01 into r0
    and  r2,  r4, r10    //Exec s02 = a0 & b2 into r2
    and  r3,  r4, r11    //Exec s03 = a0 & b3 into r3
    and  r1,  r4, r12    //Exec s04 = a0 & b4 into r1
    str  r1, [sp, #44] //Store r1/s04 on stack
    and  r1,  r4, r14    //Exec s05 = a0 & b5 into r1
    str  r1, [sp, #40] //Store r1/s05 on stack
    ldr  r1, [sp, #132 ] //Load b6 into r1
    str  r7, [sp, #36] //Store r7/a3 on stack
    and  r7,  r4,  r1    //Exec s06 = a0 & b6 into r7
    str  r7, [sp, #32] //Store r7/s06 on stack
    ldr  r7, [sp, #128 ] //Load b7 into r7
    and  r4,  r4,  r7    //Exec s07 = a0 & b7 into r4
    str  r4, [sp, #28] //Store r4/s07 on stack
    and  r4,  r5,  r8    //Exec s10 = a1 & b0 into r4
    eor  r0,  r0,  r4    //Exec y03 = y02 ^ s10 into r0
    ldr  r4, [sp, #120 ] //Load rand01 into r4
    eor  r0,  r0,  r4    //Exec y04 = y03 ^ rand01 into r0
    eor  r0,  r0,  r2    //Exec y05 = y04 ^ s02 into r0
    and  r2,  r5,  r9    //Exec s11 = a1 & b1 into r2
    eor  r2,  r2,  r4    //Exec y11 = s11 ^ rand01 into r2
    and  r4,  r5, r10    //Exec s12 = a1 & b2 into r4
    eor  r2,  r2,  r4    //Exec y12 = y11 ^ s12 into r2
    and  r4,  r5, r11    //Exec s13 = a1 & b3 into r4
    str r12, [sp, #120 ] //Store r12/b4 on stack
    and r12,  r5, r12    //Exec s14 = a1 & b4 into r12
    str r12, [sp, #24] //Store r12/s14 on stack
    and r12,  r5, r14    //Exec s15 = a1 & b5 into r12
    str r12, [sp, #20] //Store r12/s15 on stack
    and r12,  r5,  r1    //Exec s16 = a1 & b6 into r12
    and  r5,  r5,  r7    //Exec s17 = a1 & b7 into r5
    str  r5, [sp, #16] //Store r5/s17 on stack
    and  r5,  r6,  r8    //Exec s20 = a2 & b0 into r5
    eor  r0,  r0,  r5    //Exec y06 = y05 ^ s20 into r0
    ldr  r5, [sp, #92 ] //Load rand08 into r5
    eor  r0,  r0,  r5    //Exec y07 = y06 ^ rand08 into r0
    eor  r0,  r0,  r3    //Exec y08 = y07 ^ s03 into r0
    and  r3,  r6,  r9    //Exec s21 = a2 & b1 into r3
    eor  r2,  r2,  r3    //Exec y13 = y12 ^ s21 into r2
    ldr  r3, [sp, #116 ] //Load rand02 into r3
    eor  r2,  r2,  r3    //Exec y14 = y13 ^ rand02 into r2
    eor  r2,  r2,  r4    //Exec y15 = y14 ^ s13 into r2
    and  r4,  r6, r10    //Exec s22 = a2 & b2 into r4
    eor  r3,  r4,  r3    //Exec y21 = s22 ^ rand02 into r3
    and  r4,  r6, r11    //Exec s23 = a2 & b3 into r4
    eor  r3,  r3,  r4    //Exec y22 = y21 ^ s23 into r3
    ldr  r4, [sp, #120 ] //Load b4 into r4
    and  r5,  r6,  r4    //Exec s24 = a2 & b4 into r5
    str r12, [sp, #116 ] //Store r12/s16 on stack
    and r12,  r6, r14    //Exec s25 = a2 & b5 into r12
    str r12, [sp, #12] //Store r12/s25 on stack
    and r12,  r6,  r1    //Exec s26 = a2 & b6 into r12
    and  r6,  r6,  r7    //Exec s27 = a2 & b7 into r6
    str  r6, [sp, #8] //Store r6/s27 on stack
    ldr  r6, [sp, #36] //Load a3 into r6
    str r12, [sp, #4] //Store r12/s26 on stack
    and r12,  r6,  r8    //Exec s30 = a3 & b0 into r12
    eor  r0,  r0, r12    //Exec y09 = y08 ^ s30 into r0
    ldr r12, [sp, #88 ] //Load rand09 into r12
    eor  r0,  r0, r12    //Exec y0a = y09 ^ rand09 into r0
    str  r8, [sp, #0] //Store r8/b0 on stack
    ldr  r8, [sp, #44] //Load s04 into r8
    eor  r0,  r0,  r8    //Exec y0b = y0a ^ s04 into r0
    ldr  r8, [sp, #60 ] //Load rand20 into r8
    eor  r0,  r0,  r8    //Exec T0  = y0b ^ rand20 into r0
    str  r0, [sp, #44] //Store r0/T0 on stack
    and  r0,  r6,  r9    //Exec s31 = a3 & b1 into r0
    eor  r0,  r2,  r0    //Exec y16 = y15 ^ s31 into r0
    eor  r0,  r0, r12    //Exec y17 = y16 ^ rand09 into r0
    ldr  r2, [sp, #24] //Load s14 into r2
    eor  r0,  r0,  r2    //Exec y18 = y17 ^ s14 into r0
    and  r2,  r6, r10    //Exec s32 = a3 & b2 into r2
    eor  r2,  r3,  r2    //Exec y23 = y22 ^ s32 into r2
    ldr  r3, [sp, #112 ] //Load rand03 into r3
    eor  r2,  r2,  r3    //Exec y24 = y23 ^ rand03 into r2
    eor  r2,  r2,  r5    //Exec y25 = y24 ^ s24 into r2
    and  r5,  r6, r11    //Exec s33 = a3 & b3 into r5
    eor  r3,  r5,  r3    //Exec y31 = s33 ^ rand03 into r3
    and  r5,  r6,  r4    //Exec s34 = a3 & b4 into r5
    eor  r3,  r3,  r5    //Exec y32 = y31 ^ s34 into r3
    and  r5,  r6, r14    //Exec s35 = a3 & b5 into r5
    and r12,  r6,  r1    //Exec s36 = a3 & b6 into r12
    and  r6,  r6,  r7    //Exec s37 = a3 & b7 into r6
    ldr  r8, [sp, #148  ] //Load a4 into r8
    str  r6, [sp, #112 ] //Store r6/s37 on stack
    ldr  r6, [sp, #0] //Load b0 into r6
    str r12, [sp, #88 ] //Store r12/s36 on stack
    and r12,  r8,  r6    //Exec s40 = a4 & b0 into r12
    str r12, [sp, #36] //Store r12/s40 on stack
    and r12,  r8,  r9    //Exec s41 = a4 & b1 into r12
    eor  r0,  r0, r12    //Exec y19 = y18 ^ s41 into r0
    ldr r12, [sp, #84 ] //Load rand10 into r12
    eor  r0,  r0, r12    //Exec y1a = y19 ^ rand10 into r0
    str  r9, [sp, #24] //Store r9/b1 on stack
    ldr  r9, [sp, #20] //Load s15 into r9
    eor  r0,  r0,  r9    //Exec y1b = y1a ^ s15 into r0
    ldr  r9, [sp, #56 ] //Load rand21 into r9
    eor  r0,  r0,  r9    //Exec T1  = y1b ^ rand21 into r0
    str  r0, [sp, #20] //Store r0/T1 on stack
    and  r0,  r8, r10    //Exec s42 = a4 & b2 into r0
    eor  r0,  r2,  r0    //Exec y26 = y25 ^ s42 into r0
    eor  r0,  r0, r12    //Exec y27 = y26 ^ rand10 into r0
    ldr  r2, [sp, #12] //Load s25 into r2
    eor  r0,  r0,  r2    //Exec y28 = y27 ^ s25 into r0
    and  r2,  r8, r11    //Exec s43 = a4 & b3 into r2
    eor  r2,  r3,  r2    //Exec y33 = y32 ^ s43 into r2
    ldr  r3, [sp, #108 ] //Load rand04 into r3
    eor  r2,  r2,  r3    //Exec y34 = y33 ^ rand04 into r2
    eor  r2,  r2,  r5    //Exec y35 = y34 ^ s35 into r2
    and  r5,  r8,  r4    //Exec s44 = a4 & b4 into r5
    eor  r3,  r5,  r3    //Exec y41 = s44 ^ rand04 into r3
    and  r5,  r8, r14    //Exec s45 = a4 & b5 into r5
    eor  r3,  r3,  r5    //Exec y42 = y41 ^ s45 into r3
    and  r5,  r8,  r1    //Exec s46 = a4 & b6 into r5
    and  r8,  r8,  r7    //Exec s47 = a4 & b7 into r8
    ldr r12, [sp, #144  ] //Load a5 into r12 // TODO: THIS LEAKS... MUST CLEAR
    and  r9, r12,  r6    //Exec s50 = a5 & b0 into r9
    str  r9, [sp, #148  ] //Store r9/s50 on stack
    ldr  r9, [sp, #24] //Load b1 into r9
    str  r8, [sp, #108 ] //Store r8/s47 on stack
    and  r8, r12,  r9    //Exec s51 = a5 & b1 into r8
    str  r8, [sp, #84 ] //Store r8/s51 on stack
    and  r8, r12, r10    //Exec s52 = a5 & b2 into r8
    eor  r0,  r0,  r8    //Exec y29 = y28 ^ s52 into r0
    ldr  r8, [sp, #80 ] //Load rand11 into r8
    eor  r0,  r0,  r8    //Exec y2a = y29 ^ rand11 into r0
    str r10, [sp, #12] //Store r10/b2 on stack
    ldr r10, [sp, #4] //Load s26 into r10
    eor  r0,  r0, r10    //Exec y2b = y2a ^ s26 into r0
    ldr r10, [sp, #52] //Load rand22 into r10
    eor  r0,  r0, r10    //Exec T2  = y2b ^ rand22 into r0
    str  r0, [sp, #4] //Store r0/T2 on stack
    and  r0, r12, r11    //Exec s53 = a5 & b3 into r0
    eor  r0,  r2,  r0    //Exec y36 = y35 ^ s53 into r0
    eor  r0,  r0,  r8    //Exec y37 = y36 ^ rand11 into r0
    ldr  r2, [sp, #88 ] //Load s36 into r2
    eor  r0,  r0,  r2    //Exec y38 = y37 ^ s36 into r0
    and  r2, r12,  r4    //Exec s54 = a5 & b4 into r2
    eor  r2,  r3,  r2    //Exec y43 = y42 ^ s54 into r2
    ldr  r3, [sp, #104 ] //Load rand05 into r3
    eor  r2,  r2,  r3    //Exec y44 = y43 ^ rand05 into r2
    eor  r2,  r2,  r5    //Exec y45 = y44 ^ s46 into r2
    and  r5, r12, r14    //Exec s55 = a5 & b5 into r5
    eor  r3,  r5,  r3    //Exec y51 = s55 ^ rand05 into r3
    and  r5, r12,  r1    //Exec s56 = a5 & b6 into r5
    eor  r3,  r3,  r5    //Exec y52 = y51 ^ s56 into r3
    and  r5, r12,  r7    //Exec s57 = a5 & b7 into r5
    ldr  r8, [sp, #140 ] //Load a6 into r8
    and r12,  r8,  r6    //Exec s60 = a6 & b0 into r12
    and r10,  r8,  r9    //Exec s61 = a6 & b1 into r10
    str r10, [sp, #144  ] //Store r10/s61 on stack
    ldr r10, [sp, #12] //Load b2 into r10
    str r12, [sp, #104 ] //Store r12/s60 on stack
    and r12,  r8, r10    //Exec s62 = a6 & b2 into r12
    str r12, [sp, #88 ] //Store r12/s62 on stack
    and r12,  r8, r11    //Exec s63 = a6 & b3 into r12
    eor  r0,  r0, r12    //Exec y39 = y38 ^ s63 into r0
    ldr r12, [sp, #76 ] //Load rand12 into r12
    eor  r0,  r0, r12    //Exec y3a = y39 ^ rand12 into r0
    str r11, [sp, #80 ] //Store r11/b3 on stack
    ldr r11, [sp, #112 ] //Load s37 into r11
    eor  r0,  r0, r11    //Exec y3b = y3a ^ s37 into r0
    ldr r11, [sp, #48] //Load rand23 into r11
    eor  r0,  r0, r11    //Exec T3  = y3b ^ rand23 into r0
    str  r0, [sp, #112 ] //Store r0/T3 on stack
    and  r0,  r8,  r4    //Exec s64 = a6 & b4 into r0
    eor  r0,  r2,  r0    //Exec y46 = y45 ^ s64 into r0
    eor  r0,  r0, r12    //Exec y47 = y46 ^ rand12 into r0
    ldr  r2, [sp, #108 ] //Load s47 into r2
    eor  r0,  r0,  r2    //Exec y48 = y47 ^ s47 into r0
    and  r2,  r8, r14    //Exec s65 = a6 & b5 into r2
    eor  r2,  r3,  r2    //Exec y53 = y52 ^ s65 into r2
    ldr  r3, [sp, #100 ] //Load rand06 into r3
    eor  r2,  r2,  r3    //Exec y54 = y53 ^ rand06 into r2
    eor  r2,  r2,  r5    //Exec y55 = y54 ^ s57 into r2
    and  r5,  r8,  r1    //Exec s66 = a6 & b6 into r5
    eor  r3,  r5,  r3    //Exec y61 = s66 ^ rand06 into r3
    and  r5,  r8,  r7    //Exec s67 = a6 & b7 into r5
    eor  r3,  r3,  r5    //Exec y62 = y61 ^ s67 into r3
    ldr  r5, [sp, #136 ] //Load a7 into r5
    and  r6,  r5,  r6    //Exec s70 = a7 & b0 into r6
    and  r8,  r5,  r9    //Exec s71 = a7 & b1 into r8
    and  r9,  r5, r10    //Exec s72 = a7 & b2 into r9
    ldr r10, [sp, #80 ] //Load b3 into r10
    and r10,  r5, r10    //Exec s73 = a7 & b3 into r10
    and  r4,  r5,  r4    //Exec s74 = a7 & b4 into r4
    eor  r0,  r0,  r4    //Exec y49 = y48 ^ s74 into r0
    ldr  r4, [sp, #72 ] //Load rand13 into r4
    eor  r0,  r0,  r4    //Exec y4a = y49 ^ rand13 into r0
    ldr r12, [sp, #36] //Load s40 into r12
    eor  r0,  r0, r12    //Exec y4b = y4a ^ s40 into r0
    ldr r12, [sp, #60 ] //Load rand20 into r12
    eor  r12,  r0, r12    //Exec T4  = y4b ^ rand20 into r0
//  str  r12, [sp, #140 ] //Store r0/T4 on stack
    and  r0,  r5, r14    //Exec s75 = a7 & b5 into r0
    eor  r0,  r2,  r0    //Exec y56 = y55 ^ s75 into r0
    eor  r0,  r0,  r4    //Exec y57 = y56 ^ rand13 into r0
    ldr  r2, [sp, #148  ] //Load s50 into r2
    eor  r0,  r0,  r2    //Exec y58 = y57 ^ s50 into r0
    ldr  r2, [sp, #40] //Load s05 into r2
    eor  r0,  r0,  r2    //Exec y59 = y58 ^ s05 into r0
    ldr  r2, [sp, #68 ] //Load rand14 into r2
    eor  r0,  r0,  r2    //Exec y5a = y59 ^ rand14 into r0
    ldr  r4, [sp, #84 ] //Load s51 into r4
    eor  r0,  r0,  r4    //Exec y5b = y5a ^ s51 into r0
    ldr  r4, [sp, #56 ] //Load rand21 into r4
    eor  r4,  r0,  r4    //Exec T5  = y5b ^ rand21 into r0
//  str  r4, [sp, #148  ] //Store r0/T5 on stack
    and  r0,  r5,  r1    //Exec s76 = a7 & b6 into r0
    eor  r0,  r3,  r0    //Exec y63 = y62 ^ s76 into r0
    ldr  r1, [sp, #96 ] //Load rand07 into r1
    eor  r0,  r0,  r1    //Exec y64 = y63 ^ rand07 into r0
    ldr  r3, [sp, #104 ] //Load s60 into r3
    eor  r0,  r0,  r3    //Exec y65 = y64 ^ s60 into r0
    ldr  r3, [sp, #32] //Load s06 into r3
    eor  r0,  r0,  r3    //Exec y66 = y65 ^ s06 into r0
    eor  r0,  r0,  r2    //Exec y67 = y66 ^ rand14 into r0
    ldr  r2, [sp, #144  ] //Load s61 into r2
    eor  r0,  r0,  r2    //Exec y68 = y67 ^ s61 into r0
    ldr  r2, [sp, #116 ] //Load s16 into r2
    eor  r0,  r0,  r2    //Exec y69 = y68 ^ s16 into r0
    ldr  r2, [sp, #64 ] //Load rand15 into r2
    eor  r0,  r0,  r2    //Exec y6a = y69 ^ rand15 into r0
    ldr  r3, [sp, #88 ] //Load s62 into r3
    eor  r0,  r0,  r3    //Exec y6b = y6a ^ s62 into r0
    ldr  r3, [sp, #52] //Load rand22 into r3
    eor  r3,  r0,  r3    //Exec T6  = y6b ^ rand22 into r0
//  str  r3, [sp, #144  ] //Store r0/T6 on stack
    and  r0,  r5,  r7    //Exec s77 = a7 & b7 into r0
    eor  r0,  r0,  r1    //Exec y71 = s77 ^ rand07 into r0
    eor  r0,  r0,  r6    //Exec y72 = y71 ^ s70 into r0
    ldr  r1, [sp, #28] //Load s07 into r1
    eor  r0,  r0,  r1    //Exec y73 = y72 ^ s07 into r0
    ldr  r1, [sp, #124 ] //Load rand00 into r1
    eor  r0,  r0,  r1    //Exec y74 = y73 ^ rand00 into r0
    eor  r0,  r0,  r8    //Exec y75 = y74 ^ s71 into r0
    ldr  r1, [sp, #16] //Load s17 into r1
    eor  r0,  r0,  r1    //Exec y76 = y75 ^ s17 into r0
    eor  r0,  r0,  r2    //Exec y77 = y76 ^ rand15 into r0
    eor  r0,  r0,  r9    //Exec y78 = y77 ^ s72 into r0
    ldr  r1, [sp, #8] //Load s27 into r1
    eor  r0,  r0,  r1    //Exec y79 = y78 ^ s27 into r0
    ldr  r1, [sp, #92 ] //Load rand08 into r1
    eor  r0,  r0,  r1    //Exec y7a = y79 ^ rand08 into r0
    eor  r0,  r0, r10    //Exec y7b = y7a ^ s73 into r0
    eor  r0,  r0, r11    //Exec T7  = y7b ^ rand23 into r0

    store_results_masked
    add sp, #152
    epilogue
.size masked_and,.-masked_and
