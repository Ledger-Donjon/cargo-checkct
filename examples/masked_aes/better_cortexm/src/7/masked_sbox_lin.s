.syntax unified
.thumb

.macro l1_load_bs_reg off, d
    .set addr, \off
    ldrh r2, [r1, #addr]
    .set addr, 1*2*(\d+1)  + \off
    ldrh r3, [r1, #addr]
    .set addr, 2*2*(\d+1)  + \off
    ldrh r4, [r1, #addr]
    .set addr, 3*2*(\d+1)  + \off
    ldrh r5, [r1, #addr]
    .set addr, 4*2*(\d+1)  + \off
    ldrh r6, [r1, #addr]
    .set addr, 5*2*(\d+1)  + \off
    ldrh r7, [r1, #addr]
    .set addr, 6*2*(\d+1)  + \off
    ldrh r8, [r1, #addr]
    .set addr, 7*2*(\d+1)  + \off
    ldrh r9, [r1, #addr]
.endm



// Require:
// Registers (Ui = bitslice register i):
//  r0:  "all_tmp_addr" (callee saved)
//  r1:  "state_addr" (callee saved)
//  r2:  "U0"
//  r3:  "U1"
//  r4:  "U2"
//  r5:  "U3"
//  r6:  "U4"
//  r7:  "U5"
//  r8:  "U6",
//  r9:  "U7",
// r10:  "X",
// r11:  "X",
// r12:  "X",
// r14:  "X",

// Require a spot to store a word at sp + 0

.macro l1 off, d
    l1_load_bs_reg \off, \d
    str r1, [sp, #0]

    eor  r1,  r5,  r7    //Exec T14 = U3 ^ U5 into r1
    eor r10,  r2,  r8    //Exec T13 = U0 ^ U6 into r10
    eor r11, r10,  r1    //Exec T12 = T13 ^ T14 into r11
    eor r12,  r6, r11    //Exec x1 = U4 ^ T12 into r12
    eor r14, r12,  r7    //Exec T15 = x1 ^ U5 into r14
    .set addr, 13*2*(\d + 1) + \off 
    strh  r1, [r0, #addr] //Store r1/T14 on stack
    eor  r1, r14,  r9    //Exec T6 = T15 ^ U7 into r1
    .set addr, 5*2*(\d + 1) + \off 
    strh  r1, [r0, #addr] //Store r1/T6 on stack
    eor  r1, r12,  r3    //Exec T20 = x1 ^ U1 into r1
    eor  r6,  r2,  r5    //Exec T9 = U0 ^ U3 into r6
    .set addr, 19*2*(\d + 1) + \off 
    strh  r1, [r0, #addr] //Store r1/T20 on stack
    eor  r1,  r1,  r6    //Exec T11 = T20 ^ T9 into r1
    .set addr, 8*2*(\d + 1) + \off 
    strh  r6, [r0, #addr] //Store r6/T9 on stack
    eor  r6,  r9,  r1    //Exec T7 = U7 ^ T11 into r6
    .set addr, 6*2*(\d + 1) + \off 
    strh  r6, [r0, #addr] //Store r6/T7 on stack
    eor  r6,  r2,  r7    //Exec T8 = U0 ^ U5 into r6
    eor  r3,  r3,  r4    //Exec x0 = U1 ^ U2 into r3
    eor  r4, r14,  r3    //Exec T10 = T15 ^ x0 into r4
    eor  r7,  r4,  r1    //Exec T17 = T10 ^ T11 into r7
    .set addr, 9*2*(\d + 1) + \off 
    strh  r4, [r0, #addr] //Store r4/T10 on stack
    eor  r4,  r4,  r6    //Exec T19 = T10 ^ T8 into r4
    .set addr, 10*2*(\d + 1) + \off 
    strh  r1, [r0, #addr] //Store r1/T11 on stack
    eor  r1,  r3,  r1    //Exec T16 = x0 ^ T11 into r1
    .set addr, 18*2*(\d + 1) + \off 
    strh  r4, [r0, #addr] //Store r4/T19 on stack
    eor  r4, r10,  r1    //Exec T21 = T13 ^ T16 into r4
    .set addr, 15*2*(\d + 1) + \off 
    strh  r1, [r0, #addr] //Store r1/T16 on stack
    eor  r1,  r2,  r1    //Exec T18 = U0 ^ T16 into r1
    .set addr, 17*2*(\d + 1) + \off 
    strh  r1, [r0, #addr] //Store r1/T18 on stack
    eor  r1,  r3,  r9    //Exec T1 = x0 ^ U7 into r1
    eor  r3,  r1,  r5    //Exec T4 = T1 ^ U3 into r3
    eor  r2,  r1,  r2    //Exec T2 = T1 ^ U0 into r2
    .set addr, 0*2*(\d + 1) + \off 
    strh  r1, [r0, #addr] //Store r1/T1 on stack
    eor  r1,  r1,  r8    //Exec T5 = T1 ^ U6 into r1
    .set addr, 4*2*(\d + 1) + \off 
    strh  r1, [r0, #addr] //Store r1/T5 on stack
    eor  r1,  r1,  r6    //Exec T3 = T5 ^ T8 into r1

    .set addr, 2*2*(\d + 1) + \off 
    strh  r1, [r0, #addr] //Store r1/T3 on stack
    .set addr, 1*2*(\d + 1) + \off 
    strh  r2, [r0, #addr] //Store r2/T2 on stack
    .set addr, 3*2*(\d + 1) + \off 
    strh  r3, [r0, #addr] //Store r3/T4 on stack
    .set addr, 20*2*(\d + 1) + \off 
    strh  r4, [r0, #addr] //Store r4/T21 on stack
    .set addr, 7*2*(\d + 1) + \off 
    strh  r6, [r0, #addr] //Store r6/T8 on stack
    .set addr, 16*2*(\d + 1) + \off 
    strh  r7, [r0, #addr] //Store r7/T17 on stack
    .set addr, 12*2*(\d + 1) + \off 
    strh  r10, [r0, #addr] //Store r10/T13 on stack
    .set addr, 11*2*(\d + 1) + \off 
    strh  r11, [r0, #addr] //Store r11/T12 on stack
    .set addr, 14*2*(\d + 1) + \off 
    strh  r14, [r0, #addr] //Store r14/T15 on stack

    ldr r1, [sp, #0]
.endm


.macro l2_load_bs_reg off, d
    .set addr, 5*2*(\d+1)  + \off
    ldrh r14, [r1, #addr]
    strh r14, [sp, #10]
    .set addr, 12*2*(\d+1)  + \off
    ldrh r14, [r1, #addr]
    strh r14, [sp, #8]
    .set addr, 13*2*(\d+1)  + \off
    ldrh r14, [r1, #addr]
    strh r14, [sp, #6]
    .set addr, 14*2*(\d+1)  + \off
    ldrh r14, [r1, #addr]
    strh r14, [sp, #4]
    .set addr, 16*2*(\d+1)  + \off
    ldrh r14, [r1, #addr]
    strh r14, [sp, #2]
    .set addr, 17*2*(\d+1)  + \off
    ldrh r14, [r1, #addr]
    strh r14, [sp, #0]

    .set addr, \off
    ldrh r2, [r1, #addr]
    .set addr, 1*2*(\d+1)  + \off
    ldrh r3, [r1, #addr]
    .set addr, 2*2*(\d+1)  + \off
    ldrh r4, [r1, #addr]
    .set addr, 3*2*(\d+1)  + \off
    ldrh r5, [r1, #addr]
    .set addr, 4*2*(\d+1)  + \off
    ldrh r6, [r1, #addr]
    .set addr, 15*2*(\d+1)  + \off
    ldrh r7, [r1, #addr]
    .set addr, 6*2*(\d+1)  + \off
    ldrh r8, [r1, #addr]
    .set addr, 7*2*(\d+1)  + \off
    ldrh r9, [r1, #addr]
    .set addr, 8*2*(\d+1)  + \off
    ldrh r10, [r1, #addr]
    .set addr, 9*2*(\d+1)  + \off
    ldrh r11, [r1, #addr]
    .set addr, 10*2*(\d+1)  + \off
    ldrh r12, [r1, #addr]
    .set addr, 11*2*(\d+1)  + \off
    ldrh r14, [r1, #addr]

.endm

// Require:
// Registers (Ui = bitslice register i):
//  r0:  "state_addr"
//  r1:  "all_tmp_addr"
//  r2:  "U0"
//  r3:  "U1"
//  r4:  "U2"
//  r5:  "U3"
//  r6:  "U4"
//  r7:  "U15
//  r8:  "U6"
//  r9:  "U7"
// r10:  "U8"
// r11:  "U9"
// r12:  "U10"
// r14:  "U11"

// Stack during execution (values stored during l2_load_bs_reg):
// sp + 12: r1 (full word)
// sp + 10: U5
// sp +  8: U12
// sp +  6: U13
// sp +  4: U14
// sp +  2: U16
// sp +  0: U17

// Require space on the stack between sp and sp + 15 (included)
.macro l2 off, d

    l2_load_bs_reg \off, \d // After this, r1 is free
    str r1, [sp, #12]

    ldrh  r1, [sp, #2] //Load U16 into r1
    eor  r1,  r7,  r1    //Exec l1 = U15 ^ U16 into r1
    eor r12, r12,  r1    //Exec l2 = U10 ^ l1 into r12
    eor r11, r11, r12    //Exec l3 = U9 ^ l2 into r11
    eor r12, r12, r14    //Exec l21 = l2 ^ U11 into r12
    ldrh r14, [sp, #6 ] //Load U13 into r14
    eor  r1, r14,  r1    //Exec l13 = U13 ^ l1 into r1
    eor  r4,  r2,  r4    //Exec l4 = U0 ^ U2 into r4
    ldrh r14, [sp, #8  ] //Load U12 into r14
    eor r14, r14,  r4    //Exec l7 = U12 ^ l4 into r14
    eor r10, r10, r14    //Exec l9 = U8 ^ l7 into r10
    eor  r2,  r3,  r2    //Exec l5 = U1 ^ U0 into r2
    eor  r3,  r5,  r6    //Exec l6 = U3 ^ U4 into r3
    eor  r2,  r3,  r2    //Exec l11 = l6 ^ l5 into r2
    eor  r2, r11,  r2    //Exec T3 = l3 ^ l11 into r2
    eor  r3,  r9,  r3    //Exec l8 = U7 ^ l6 into r3
    eor  r6,  r3, r10    //Exec l10 = l8 ^ l9 into r6
    ldrh  r9, [sp, #4 ] //Load U14 into r9
    eor  r9,  r9,  r6    //Exec l17 = U14 ^ l10 into r9
    eor r10, r12,  r9    //Exec T5 = l21 ^ l17 into r10
    eor  r3,  r8,  r3    //Exec l16 = U6 ^ l8 into r3
    eor  r8,  r2,  r3    //Exec T1 = T3 ^ l16 into r8
    .set addr, 1*2*(\d + 1) + \off 
    strh  r8, [r0, #addr] //Store r8/T1 on stack
    eor  r8, r11,  r3    //Exec T0 = l3 ^ l16 into r8
    eor  r3,  r7,  r3    //Exec l20 = U15 ^ l16 into r3
    eor  r3,  r9,  r3    //Exec l26 = l17 ^ l20 into r3
    ldrh  r7, [sp, #0] //Load U17 into r7
    eor  r3,  r3,  r7    //Exec T2 = l26 ^ U17 into r3
    .set addr, 2*2*(\d + 1) + \off 
    strh  r3, [r0, #addr] //Store r3/T2 on stack
    ldrh  r3, [sp, #10 ] //Load U5 into r3
    eor  r3,  r5,  r3    //Exec l12 = U3 ^ U5 into r3
    eor  r3,  r4,  r3    //Exec l14 = l4 ^ l12 into r3
    .set addr, 3*2*(\d + 1) + \off 
    strh  r2, [r0, #addr] //Store r2/T3 on stack
    eor  r2,  r3,  r2    //Exec T4 = l14 ^ T3 into r2
    eor  r1,  r1,  r3    //Exec l18 = l13 ^ l14 into r1
    .set addr, 4*2*(\d + 1) + \off 
    strh  r2, [r0, #addr] //Store r2/T4 on stack
    eor  r2,  r6,  r1    //Exec T6 = l10 ^ l18 into r2
    .set addr, 6*2*(\d + 1) + \off 
    strh  r2, [r0, #addr] //Store r2/T6 on stack
    ldrh  r2, [sp, #8 ] //Load U12 into r2
    eor  r1,  r2,  r1    //Exec T7 = U12 ^ l18 into r1

    .set addr, 7*2*(\d + 1) + \off 
    strh  r1, [r0, #addr] //Store r2/T7 on stack
    .set addr, 0*2*(\d + 1) + \off 
    strh  r8, [r0, #addr] //Store r2/T0 on stack
    .set addr, 5*2*(\d + 1) + \off 
    strh  r10, [r0, #addr] //Store r2/T5 on stack

    ldr r1, [sp, #12]
.endm

.macro l_int0_load_bs_reg off, d
    .set addr, \off
    ldrh r2, [r0, #addr]
    .set addr, 1*2*(\d+1)  + \off
    ldrh r3, [r0, #addr]
    .set addr, 2*2*(\d+1)  + \off
    ldrh r4, [r0, #addr]
    .set addr, 3*2*(\d+1)  + \off
    ldrh r5, [r0, #addr]
    .set addr, 4*2*(\d+1)  + \off
    ldrh r6, [r0, #addr]
    .set addr, 5*2*(\d+1)  + \off
    ldrh r7, [r0, #addr]
    .set addr, 6*2*(\d+1)  + \off
    ldrh r8, [r0, #addr]
    .set addr, 0*2*(\d+1)  + \off
    ldrh r9, [r1, #addr]
    .set addr, 1*2*(\d+1)  + \off
    ldrh r10, [r1, #addr]
.endm

// Require:
// Registers (Ui = bitslice register i):
//  r0:  "state_addr"
//  r1:  "all_tmp_addr"
//  r2:  "X"
//  r3:  "X"
//  r4:  "X"
//  r5:  "X"
//  r6:  "X"
//  r7:  "X"
//  r8:  "X"
//  r9:  "X"
// r10:  "X"
// r11:  "X"
// r12:  "X"
// r14:  "X"
.macro l_int0 off, d
    l_int0_load_bs_reg \off, \d // After this, r1 is free

    eor r5, r2, r5 // t16 = t15 ^ t12
    eor r8, r8, r5 // t18 = t6 ^ t16
    eor r6, r6, r5 // t20 = t11 ^ t16
    eor r8, r8, r10 // t22 = t18 ^ y19
    eor r6, r6, r9 // t24 = t20 ^ y18
    eor r7, r7, r8 // t25 = t21 ^ t22
    eor r5, r6, r3 // t27 = t24 ^ t26
    eor r2, r4, r6 // t30 = t23 ^ t24
    eor r3, r8, r3 // t31 = t22 ^ t26

    .set addr, \off
    strh r2, [r0, #addr]
    .set addr, 1*2*(\d+1)  + \off
    strh r3, [r0, #addr]
    .set addr, 3*2*(\d+1)  + \off
    strh r5, [r0, #addr]
    .set addr, 4*2*(\d+1)  + \off
    strh r6, [r0, #addr]
    .set addr, 5*2*(\d+1)  + \off
    strh r7, [r0, #addr]
    .set addr, 6*2*(\d+1)  + \off
    strh r8, [r0, #addr]
.endm

.macro prologue lastreg
    push {r4-\lastreg, lr} // Save registers of caller, somehow r12 is a scratch register
.endm

.macro epilogue lastreg
    pop {r4-\lastreg, pc} // Restore saved registers of caller and resume inside caller
.endm

// Require:
// Registers (Ui = bitslice register i):
//  r0:  "all_tmp_addr"
//  r1:  "state_addr"
//  r2:  "X"
//  r3:  "X"
//  r4:  "X"
//  r5:  "X"
//  r6:  "X"
//  r7:  "X"
//  r8:  "X"
//  r9:  "X"
// r10:  "X"
// r11:  "X"
// r12:  "X"
// r14:  "X"
.globl masked_sbox_l1
.type masked_sbox_l1,%function
masked_sbox_l1:
    prologue r11
    sub sp, #4
    l1 0, 7
    l1 2, 7
    l1 4, 7
    l1 6, 7
    l1 8, 7
    l1 10, 7
    l1 12, 7
    l1 14, 7
    add sp, #4
    epilogue r11
.size masked_sbox_l1,.-masked_sbox_l1

// Require:
// Registers (Ui = bitslice register i):
//  r0:  "state_addr"
//  r1:  "all_tmp_addr"
//  r2:  "X"
//  r3:  "X"
//  r4:  "X"
//  r5:  "X"
//  r6:  "X"
//  r7:  "X"
//  r8:  "X"
//  r9:  "X"
// r10:  "X"
// r11:  "X"
// r12:  "X"
// r14:  "X"
.globl masked_sbox_l2
.type masked_sbox_l2,%function
masked_sbox_l2:
    prologue r11
    sub sp, #16
    l2 0, 7
    l2 2, 7
    l2 4, 7
    l2 6, 7
    l2 8, 7
    l2 10, 7
    l2 12, 7
    l2 14, 7
    add sp, #16
    epilogue r11
.size masked_sbox_l2,.-masked_sbox_l2



// Require:
// Registers (Ui = bitslice register i):
//  r0:  "state_addr"
//  r1:  "all_tmp_addr"
//  r2:  "X"
//  r3:  "X"
//  r4:  "X"
//  r5:  "X"
//  r6:  "X"
//  r7:  "X"
//  r8:  "X"
//  r9:  "X"
// r10:  "X"
// r11:  "X"
// r12:  "X"
// r14:  "X"
.globl masked_sbox_l_int0
.type masked_sbox_l_int0,%function
masked_sbox_l_int0:
    prologue r10
    l_int0 0, 7
    l_int0 2, 7
    l_int0 4, 7
    l_int0 6, 7
    l_int0 8, 7
    l_int0 10, 7
    l_int0 12, 7
    l_int0 14, 7
    epilogue r10
.size masked_sbox_l_int0,.-masked_sbox_l_int0
