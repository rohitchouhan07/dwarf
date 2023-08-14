#[derive(Default)]
pub enum PType {
    #[default]
    NONE,
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

#[derive(Default)]
pub enum PFlags {
    #[default]
    NONE,
    X,
    W,
    R,
}

