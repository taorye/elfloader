use core::fmt::Debug;

#[repr(C)]
pub struct EIMagic([u8; 4]);

impl EIMagic {
    const EIMAGIC_NUM: [u8; 4] = [0x7f, b'E', b'L', b'F'];

    pub const fn is_valid(&self) -> bool {
        self.0[0] == Self::EIMAGIC_NUM[0]
            && self.0[1] == Self::EIMAGIC_NUM[1]
            && self.0[2] == Self::EIMAGIC_NUM[2]
            && self.0[3] == Self::EIMAGIC_NUM[3]
    }
}

impl Debug for EIMagic {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[{:#x}, {:#x}, {:#x}, {:#x}]",
            self.0[0], self.0[1], self.0[2], self.0[3]
        )
    }
}
