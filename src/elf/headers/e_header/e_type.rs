#[repr(u16)]
#[derive(Debug)]
pub enum EType {
    NONE = 0,        /* No file type */
    REL = 1,         /* Relocatable file */
    EXEC = 2,        /* Executable file */
    DYN = 3,         /* Shared object file */
    CORE = 4,        /* Core file */
    LOOS = 0xfe00,   /* Operating system-specific */
    HIOS = 0xfeff,   /* Operating system-specific */
    LOPROC = 0xff00, /* Processor-specific */
    HIPROC = 0xffff, /* Processor-specific */
}
