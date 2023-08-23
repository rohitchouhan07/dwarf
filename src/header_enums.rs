#[derive(Debug, Default)]
pub enum BinType {
    #[default]
    NONE,
    REL,
    EXEC,
    DYN,
    CORE,
    LOOS,
    HIOS,
    LOPROC,
    HIPROC,
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub enum Class {
    #[default]
    NONE,
    X32Bit,
    X64Bit,
} 

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub enum Endian {
    #[default]
    NONE,
    Little,
    Big,
}

#[derive(Debug, Default)]
pub enum Abi {
    #[default]
    NONE,
    SystemV,
    HpUX,
    NetBSD,
    Linux,
    GnuHurd,
    Solaris,
    AIX,
    IRIX,
    FreeBSD,
    Tru64,
    Novell,
    OpenBSD,
    OpenVMS,
    NonStop,
    AROS,
    FernixOS,
    Nuxi,
    Status,
}

#[derive(Debug, Default)]
pub enum Machine {
    #[default]
    NONE,
    X86,
    AMD64,
}


