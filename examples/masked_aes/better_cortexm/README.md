# AES-128 implementation masked at order 3 and 7 for Cortex-M

* Depends on GNU ARM Embedded Toolchain (`arm-non-eabi`)
* `make D=d` creates a library `libmasked_aes_<d>.a` where `d` is the masking order (default to `7`)
