#[repr(u8)]
#[derive(Debug)]
pub enum EIData {
    NONE = 0,
    LSB = 1,
    MSB = 2,
}
