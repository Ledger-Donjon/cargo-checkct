#![no_std]

pub fn eq(left: &[u8], right: &[u8]) -> bool {
    for (l, r) in left.iter().zip(right.iter()) {
        if l != r {
            return false;
        }
    }

    true
}
