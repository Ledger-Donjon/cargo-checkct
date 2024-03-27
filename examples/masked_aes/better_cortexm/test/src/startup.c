// https://jacobmossberg.se/posts/2018/08/11/run-c-program-bare-metal-on-arm-cortex-m3.html

extern unsigned int _DATA_ROM_START;
extern unsigned int _DATA_RAM_START;
extern unsigned int _DATA_RAM_END;

#define STACK_TOP 0x20008000
void startup();

unsigned int * myvectors[2]
__attribute__ ((section("vectors")))= {
    (unsigned int *)    STACK_TOP,  // stack pointer
    (unsigned int *)    startup     // code entry point
};

void main();

void startup()
{
    /* Copy data belonging to the `.data` section from its
     * load time position on flash (ROM) to its run time position
     * in SRAM.
     */
    unsigned int * data_rom_start_p = &_DATA_ROM_START;
    unsigned int * data_ram_start_p = &_DATA_RAM_START;
    unsigned int * data_ram_end_p = &_DATA_RAM_END;

    while(data_ram_start_p != data_ram_end_p)
    {
        *data_ram_start_p = *data_rom_start_p;
        data_ram_start_p++;
        data_rom_start_p++;
    }

    ///* Initialize data in the `.bss` section to zeros.
    // */
    //unsigned int * bss_start_p = &_BSS_START;
    //unsigned int * bss_end_p = &_BSS_END;

    //while(bss_start_p != bss_end_p)
    //{
    //    *bss_start_p = 0;
    //    bss_start_p++;
    //}


    /* Call the `main()` function.
     */
    main();
    __asm__("eor r0, r0, r0; swi 1;");
}
