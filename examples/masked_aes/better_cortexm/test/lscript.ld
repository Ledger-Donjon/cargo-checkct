/* https://jacobmossberg.se/posts/2018/08/11/run-c-program-bare-metal-on-arm-cortex-m3.html */

SECTIONS
{
    .  = 0x0;         /* From 0x00000000 */

    .text : 
    {
        *(vectors)    /* Vector table */
        *(.text)      /* Program code */
        *(.text_masked) /* Masked and code */
    }
    .rodata :
    {
        *(.rodata)    /* Read only data */
    }

    _DATA_ROM_START = .;
    .  = 0x20000000;
    _DATA_RAM_START = .;

    .data :
    {
        *(.data)      /* Data memory */
    }
    _DATA_RAM_END = .;

    .data : AT(_DATA_ROM_START)
    {
        *(.data)      /* Data memory */
    }
    _DATA_RAM_END = .;
}
