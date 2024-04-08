mod driver;
mod rng;
use driver::checkct;

#[no_mangle]
#[inline(never)]
fn __checkct() {
    core::hint::black_box(rng::__checkct_public_rand());
    checkct()
}

fn main() {
    __checkct();
}
