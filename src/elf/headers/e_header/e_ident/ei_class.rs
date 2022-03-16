#[repr(u8)]
#[derive(Debug)]
pub enum EIClass {
    CLASSNONE = 0,
    CLASS32 = 1,
    CLASS64 = 2,
}
