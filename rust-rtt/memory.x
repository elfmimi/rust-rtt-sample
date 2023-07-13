MEMORY {
    FLASH : ORIGIN = 0x00000000, LENGTH = 256K
    RAM   : ORIGIN = 0x20000000, LENGTH = 32K
}

/* Option-Setting Memory has to be placed at this specific address. */
SECTIONS {
    .option_setting 0x00000400 : {
        LONG(0xFFFFFFFF); /* OFS0 */
        LONG(0xFFFFFFFF); /* OFS1 */
    } > FLASH
}
INSERT AFTER .vector_table

/* Push .text section abit forward. */
_stext = 0x00000440;
