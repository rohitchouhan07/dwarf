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

