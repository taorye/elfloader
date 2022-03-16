#[allow(non_camel_case_types)]
#[repr(usize)]
#[derive(Debug)]
pub enum RelaType {
    /* Relocation types used by the dynamic linker */
    // NONE = 0,
    RISCV_32 = 1,
    RISCV_64 = 2,
    // RELATIVE = 3,
    // COPY = 4,
    // JUMP_SLOT = 5,
    // TLS_DTPMOD32 = 6,
    // TLS_DTPMOD64 = 7,
    // TLS_DTPREL32 = 8,
    // TLS_DTPREL64 = 9,
    // TLS_TPREL32 = 10,
    // TLS_TPREL64 = 11,
    /* Relocation types not used by the dynamic linker */
    BRANCH = 16,
    // JAL = 17,
    CALL = 18,
    CALL_PLT = 19,
    // GOT_HI20 = 20,
    // TLS_GOT_HI20 = 21,
    // TLS_GD_HI20 = 22,
    PCREL_HI20 = 23,
    PCREL_LO12_I = 24,
    PCREL_LO12_S = 25,
    HI20 = 26,
    LO12_I = 27,
    LO12_S = 28,
    // TPREL_HI20 = 29,
    // TPREL_LO12_I = 30,
    // TPREL_LO12_S = 31,
    // TPREL_ADD = 32,
    // ADD8 = 33,
    // ADD16 = 34,
    // ADD32 = 35,
    // ADD64 = 36,
    // SUB8 = 37,
    // SUB16 = 38,
    // SUB32 = 39,
    // SUB64 = 40,
    // GNU_VTINHERIT = 41,
    // GNU_VTENTRY = 42,
    // ALIGN = 43,
    RVC_BRANCH = 44,
    RVC_JUMP = 45,
    // RVC_LUI = 46,
    // GPREL_I = 47,
    // GPREL_S = 48,
    // TPREL_I = 49,
    // TPREL_S = 50,
    RELAX = 51,
    // SUB6 = 52,
    // SET6 = 53,
    // SET8 = 54,
    // SET16 = 55,
    // SET32 = 56,
    // RV32_PCREL = 57,
}