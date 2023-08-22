#[derive(Debug, Default)]
pub enum SType {
    #[default]
    NONE,
    PROGBITS,
}

#[derive(Debug, Default)]
pub enum SFlags {
    #[default]
    NONE,
    WRITE,
}
