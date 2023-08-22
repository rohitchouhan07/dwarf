#[derive(Default, Debug)]
pub enum PType {
    #[default]
    NONE,
    NULL,
    LOAD,
    DYNAMIC,
    INTERP,
    NOTE,
    SHLIB,
    PHDR,
    TLS,
    LOOS,
    HIOS,
    LOPROC,
    HIPROC,
    GNU_EH_FRAME,
    GNU_STACK,
    GNU_RELRO,
    GNU_PROPERTY,
    GNU_SFRAME,
}

#[derive(Default, Debug)]
pub enum PFlags {
    #[default]
    NONE,
    X,
    W,
    R,
    RW,
    RX,
    RWX,
}

