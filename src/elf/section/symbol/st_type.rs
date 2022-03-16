#[repr(u8)]
#[derive(Debug)]
pub enum STType {
    NOTYPE = 0,
    OBJECT = 1,
    FUNC = 2,
    SECTION = 3,
    FILE = 4,
    COMMON = 5,
    TLS = 6,
    LOOS = 10,
    HIOS = 12,
    LOPROC = 13,
    HIPROC = 15,
}
