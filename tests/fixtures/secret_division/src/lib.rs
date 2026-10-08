#![no_std]

/// Branch-free, but division instructions are not constant-time on most CPUs.
pub fn div(dividend: u32, divisor: u32) -> u32 {
    // Make the divisor non-zero, so that the compiler does not insert a branch to panic
    dividend / (divisor | 1)
}
