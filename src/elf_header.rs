use crate::{BigEndian, LittleEndian, ReadBytesExt};
pub mod header_enums;
use self::header_enums::{BinType, Class, Endian, Abi, Machine};

#[derive(Debug, Default)]
pub struct Header {
    pub class: Class,
    pub endian: Endian,
    pub version: u8,
    pub abi: Abi,
    pub abi_version: u8,
    pub bin_type: BinType,
    pub machine: Machine,
    pub misc_version: u32,
    pub entry_point: u64,
    pub phdr_offset: u64,
    pub shdr_offset: u64,
    pub flags: u32,
    pub hdr_sz: u16,
    pub phdr_entry_sz: u16,
    pub phdr_entries: u16,
    pub shdr_entry_sz: u16,
    pub shdr_entries: u16,
    pub shstr_idx: u16,
}

pub fn parse(content: &Vec<u8>,
                header: &mut Header) -> Result<(), &'static str> {
    // first we check whether it a valid ELF file or not
    const MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

    let magic: &[u8] = &content[0..4];
    if MAGIC != magic {
        return Err("Not an ELF binary.");
    }
    println!("Valid ELF binary.");

    // cursor
    let mut cursor: usize = 0x04;
    let mut width: usize = 0x08;
    let mut buff: &[u8];

    // reading the class
    match content[cursor] {
        0x01 => {
            header.class = Class::X32Bit;
            width = 0x04;
        },
        0x02 => header.class = Class::X64Bit,
        _ => header.class = Class::NONE,
    }
    cursor += 0x01;

    // reading endianness
    match content[cursor] {
        0x01 => header.endian = Endian::Little,
        0x02 => header.endian = Endian::Big,
        _ => header.endian = Endian::NONE,
    }
    cursor += 0x01;

    // read the version
    header.version = content[cursor];
    cursor += 0x01;

    // OS abi
    match content[cursor] {
        0x00 => header.abi = Abi::SystemV,
        0x01 => header.abi = Abi::HpUX,
        0x02 => header.abi = Abi::NetBSD,
        0x03 => header.abi = Abi::Linux,
        0x04 => header.abi = Abi::GnuHurd,
        0x06 => header.abi = Abi::Solaris,
        0x07 => header.abi = Abi::AIX,
        0x08 => header.abi = Abi::IRIX,
        0x09 => header.abi = Abi::FreeBSD,
        0x0A => header.abi = Abi::Tru64,
        0x0B => header.abi = Abi::Novell,
        0x0C => header.abi = Abi::OpenBSD,
        0x0D => header.abi = Abi::OpenVMS,
        0x0E => header.abi = Abi::NonStop,
        0x0F => header.abi = Abi::AROS,
        0x10 => header.abi = Abi::FernixOS,
        0x11 => header.abi = Abi::Nuxi,
        0x12 => header.abi = Abi::Status,
        _ => header.abi = Abi::NONE,
    }
    cursor += 0x01;

    // ABI version
    header.abi_version = content[cursor];
    cursor += 0x01;

    // Padding
    cursor += 0x07;

    // type of binary
    buff = &content[cursor..(cursor + 0x02)];
    let bin_type = match header.endian {
        Endian::Little => buff.read_u16::<LittleEndian>().unwrap(),
        Endian::Big => buff.read_u16::<BigEndian>().unwrap(),
        Endian::NONE => return Err("Endianess of the system not defined."),
    };

    match bin_type {
        0x01 => header.bin_type = BinType::REL,
        0x02 => header.bin_type = BinType::EXEC,
        0x03 => header.bin_type = BinType::DYN,
        0x04 => header.bin_type = BinType::CORE,
        0xFE00 => header.bin_type = BinType::LOOS,
        0xFEFF => header.bin_type = BinType::HIOS,
        0xFF00 => header.bin_type = BinType::LOPROC,
        0xFFFF => header.bin_type = BinType::HIPROC,
        _ => header.bin_type = BinType::NONE,
    }
    cursor += 0x02;

    // machine
    buff = &content[cursor..(cursor + 0x02)];
    let machine = match header.endian {
        Endian::Little => buff.read_u16::<LittleEndian>().unwrap(),
        Endian::Big => buff.read_u16::<BigEndian>().unwrap(),
        Endian::NONE => return Err("Endianness of the system not defined."),

    };
    match machine {
        0x03 => header.machine = Machine::X86,
        0x3E => header.machine = Machine::AMD64,
        _ =>  header.machine = Machine::NONE,
    }
    cursor += 0x02;

    // Another Version
    buff = &content[cursor..(cursor + 0x04)];
    header.misc_version = match header.endian {
        Endian::Little => buff.read_u32::<LittleEndian>().unwrap(),
        Endian::Big => buff.read_u32::<BigEndian>().unwrap(),
        Endian::NONE => return Err("Endianness of the system not defined."),
    };
    cursor += 0x04;

    // entry point
    buff = match header.class {
        Class::X32Bit | Class::X64Bit => &content[cursor..(cursor + width)],
        Class::NONE => return Err("Unknown binary type."),
    };

    header.entry_point = match header.endian {
        Endian::Little => {
            if header.class == Class::X64Bit {
                buff.read_u64::<LittleEndian>().unwrap()
            }
            else {
                buff.read_u32::<LittleEndian>().unwrap() as u64
            }
        },
        Endian::Big => {
            if header.class == Class::X64Bit {
                buff.read_u64::<BigEndian>().unwrap()
            }
            else {
                buff.read_u32::<BigEndian>().unwrap() as u64
            }
        },
        Endian::NONE => return Err("Endianness of the system not defined."),
    };
    cursor += width;

    // program header entry point
    buff = match header.class {
        Class::X32Bit | Class::X64Bit => &content[cursor..(cursor + width)],
        Class::NONE => return Err("Unknown binary type."),
    };

    header.phdr_offset = match header.endian {
        Endian::Little => {
            if header.class == Class::X64Bit {
                buff.read_u64::<LittleEndian>().unwrap()
            }
            else {
                buff.read_u32::<LittleEndian>().unwrap() as u64
            }
        },
        Endian::Big => {
            if header.class == Class::X64Bit {
                buff.read_u64::<BigEndian>().unwrap()
            }
            else {
                buff.read_u32::<BigEndian>().unwrap() as u64
            }
        },
        Endian::NONE => return Err("Endianness of the system not defined."),
    };
    cursor += width;

    // section header entry point
    buff = match header.class {
        Class::X32Bit | Class::X64Bit => &content[cursor..(cursor + width)],
        Class::NONE => return Err("Unknown binary type."),
    };

    header.shdr_offset = match header.endian {
        Endian::Little => {
            if header.class == Class::X64Bit {
                buff.read_u64::<LittleEndian>().unwrap()
            }
            else {
                buff.read_u32::<LittleEndian>().unwrap() as u64
            }
        },
        Endian::Big => {
            if header.class == Class::X64Bit {
                buff.read_u64::<BigEndian>().unwrap()
            }
            else {
                buff.read_u32::<BigEndian>().unwrap() as u64
            }
        },
        Endian::NONE => return Err("Endianness of the system not defined."),
    };
    cursor += width;

    // flags
    buff = &content[cursor..(cursor + 0x04)];
    header.flags = match header.endian {
        Endian::Little => buff.read_u32::<LittleEndian>().unwrap(),
        Endian::Big => buff.read_u32::<BigEndian>().unwrap(),
        Endian::NONE => return Err("Endianness of the system not defined."),

    };
    cursor += 0x04;

    // header size
    buff = &content[cursor..(cursor + 0x02)];
    header.hdr_sz = match header.endian {
        Endian::Little => buff.read_u16::<LittleEndian>().unwrap(),
        Endian::Big => buff.read_u16::<BigEndian>().unwrap(),
        Endian::NONE => return Err("Endianness of the system not defined."),

    };
    cursor += 0x02;

    // program header table entry size
    buff = &content[cursor..(cursor + 0x02)];
    header.phdr_entry_sz = match header.endian {
        Endian::Little => buff.read_u16::<LittleEndian>().unwrap(),
        Endian::Big => buff.read_u16::<BigEndian>().unwrap(),
        Endian::NONE => return Err("Endianness of the system not defined."),
    };
    cursor += 0x02;

    // program header table entries
    buff = &content[cursor..(cursor + 0x02)];
    header.phdr_entries = match header.endian {
        Endian::Little => buff.read_u16::<LittleEndian>().unwrap(),
        Endian::Big => buff.read_u16::<BigEndian>().unwrap(),
        Endian::NONE => return Err("Endianness of the system not defined."),
    };
    cursor += 0x02;

    // section header table entry size
    buff = &content[cursor..(cursor + 0x02)];
    header.shdr_entry_sz = match header.endian {
        Endian::Little => buff.read_u16::<LittleEndian>().unwrap(),
        Endian::Big => buff.read_u16::<BigEndian>().unwrap(),
        Endian::NONE => return Err("Endianness of the system not defined."),
    };
    cursor += 0x02;

    // section header table entries
    buff = &content[cursor..(cursor + 0x02)];
    header.shdr_entries = match header.endian {
        Endian::Little => buff.read_u16::<LittleEndian>().unwrap(),
        Endian::Big => buff.read_u16::<BigEndian>().unwrap(),
        Endian::NONE => return Err("Endianness of the system not defined."),
    };
    cursor += 0x02;

    // section header table entries
    buff = &content[cursor..(cursor + 0x02)];
    header.shstr_idx = match header.endian {
        Endian::Little => buff.read_u16::<LittleEndian>().unwrap(),
        Endian::Big => buff.read_u16::<BigEndian>().unwrap(),
        Endian::NONE => return Err("Endianness of the system not defined."),
    };

    // end of ELF header
    dbg!(header);
    Ok(())
}
