#[repr(u8)]
#[derive(Debug)]
pub enum STVis {
    DEFAULT = 0,
    INTERNAL = 1,
    HIDDEN = 2,
    PROTECTED = 3,
}
