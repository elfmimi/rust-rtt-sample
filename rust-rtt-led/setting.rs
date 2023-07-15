// Option Function Select 0
const OFS0: u32 = 0xFFFF_FFFF;
// Option Function Select 1
const OFS1: u32 = 0xFFFF_FFFF;

// Special data which has to be placed at specific address.
#[link_section = ".option_setting"]
#[no_mangle]
static __OPTION_SETTING: [u32; 2] = [
    OFS0,
    OFS1,
];
