// SPDX-FileCopyrightText: 2024 Ledger
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![no_std]

use core::ffi::c_int;

extern "C" {
    fn masked_aes_encrypt128(
        state: *mut u16,
        rkeys: *mut u16,
        rng_fill: unsafe extern "C" fn(*mut u8, c_int),
    );

    fn masked_aes_keyschedule128(
        key: *mut u16,
        rkeys: *mut u16,
        rng_fill: unsafe extern "C" fn(*mut u8, c_int),
    );

    fn mask_bitslice_state(
        state: *mut u8,
        masked_bs_state: *mut u16,
        rng_fill: unsafe extern "C" fn(*mut u8, c_int),
    );

    fn unbitslice_unmask_state(masked_bs_state: *mut u16, state: *mut u8);

    fn prng_init(seed: c_int);

    fn prng_fill(dst: *mut u8, size: c_int);
}

pub struct MaskedAes {
    key: [u8; 16],
}

impl MaskedAes {
    pub fn new(key: [u8; 16], seed: u32) -> Self {
        unsafe {
            prng_init(seed as i32);
        }
        Self { key }
    }

    pub fn encrypt(&mut self, block: [u8; 16]) -> [u8; 16] {
        let mut state = block.clone();
        let mut masked_bs_state = [0u16; 4 * 8];
        let mut masked_bs_key = [0u16; 4 * 8];
        let mut rkeys = [0u16; 4 * 8 * 11];

        unsafe {
            mask_bitslice_state(state.as_mut_ptr(), masked_bs_state.as_mut_ptr(), prng_fill);
            mask_bitslice_state(self.key.as_mut_ptr(), masked_bs_key.as_mut_ptr(), prng_fill);
            masked_aes_keyschedule128(masked_bs_key.as_mut_ptr(), rkeys.as_mut_ptr(), prng_fill);
            masked_aes_encrypt128(masked_bs_state.as_mut_ptr(), rkeys.as_mut_ptr(), prng_fill);
            unbitslice_unmask_state(masked_bs_state.as_mut_ptr(), state.as_mut_ptr());
        }

        state
    }
}
