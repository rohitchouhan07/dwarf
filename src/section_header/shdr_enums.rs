#[derive(Debug, Default)]
pub enum SType {
    #[default]
    NULL,
    PROGBITS,
    SYMTAB,
    STRTAB,
    RELA,
    HASH,
    DYNAMIC,
    NOTE,
    NOBITS,
    REL,
    SHLIB,
    DYNSYM,
    INIT_ARRAY,
    FINI_ARRAY,
    PREINIT_ARRAY,
    GROUP,
    SYMTAB_SHNDX,
    NUM,
    NONE,
}

#[derive(Debug, Default)]
pub enum SFlags {
    #[default]
    NULL,
    WRITE,
    NONE,
}
