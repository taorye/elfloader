#[repr(usize)]
#[derive(Debug)]
pub enum SHFlags {
    WRITE = 0x1,
    ALLOC = 0x2,
    EXECINSTR = 0x4,
    MERGE = 0x10,
    STRINGS = 0x20,
    INFOLINK = 0x40,
    LINKORDER = 0x80,
    OSNONCONFORMING = 0x100,
    GROUP = 0x200,
    TLS = 0x400,
    COMPRESSED = 0x800,
    MASKOS = 0x0ff00000,
    MASKPROC = 0xf0000000,
}
