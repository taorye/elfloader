mod ei_class;
mod ei_data;
mod ei_magic;
mod ei_version;

use core::fmt::Debug;

use self::ei_class::EIClass;
use self::ei_data::EIData;
use self::ei_magic::EIMagic;
use self::ei_version::EIVersion;

#[repr(C)]
pub struct EIdent {
    /* byte 0-3 */
    pub magic: EIMagic,
    /* byte 4 */
    pub class: EIClass,
    /* byte 5 */
    pub data: EIData,
    /* byte 6 */
    pub version: EIVersion,
    /* byte 7 */
    pub osabi: u8,
    /* byte 8 */
    pub abiversion: u8,
    /* byte 9-15 */
    _pad: [u8; 7],
}

impl EIdent {
    pub const fn is_valid(&self) -> bool {
        self.magic.is_valid()
            && match self.version {
                EIVersion::CURRENT => true,
                _ => false,
            }
    }
}

impl Debug for EIdent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("")
            .field("magic", &self.magic)
            .field("class", &self.class)
            .field("data", &self.data)
            .field("version", &self.version)
            .field("osabi", &self.osabi)
            .field("abiversion", &self.abiversion)
            .finish()
    }
}
