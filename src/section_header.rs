use crate::{Class, Endian};
use crate::{BigEndian, LittleEndian, ReadBytesExt};
pub mod shdr_enums;
use self::shdr_enums::{SType, SFlags};

#[derive(Debug, Default)]
struct SHeader {
    sname: String,
    stype: SType,
    sflags: SFlags,
    vaddr: u64,
    offset: u64,
    size: u64,
    link: u32,
    info: u32,
    align: u64,
    entsize: u64,
}

pub fn parse(content: &Vec<u8>,
             shdr_offset: u64,
             class: Class,
             endian: Endian,
             strtab_start: u64,
             ) -> Result<(), &'static str> {
    
    // cursor
    let mut cursor: usize = shdr_offset as usize;
    let width: usize;
    let mut buff: &[u8];

    if class == Class::X32Bit {
        width = 0x04;
    }
    else {
        width = 0x08;
    }

    let mut s_header: SHeader = SHeader {
        ..Default::default()
    };

    // read the name of section
    buff = &content[cursor..(cursor + 0x04)];
    let name_offset: u32 = match endian {
        Endian::Little => buff.read_u32::<LittleEndian>().unwrap(),
        Endian::Big => buff.read_u32::<BigEndian>().unwrap(),
        _ => return Err("Unkown error"),
    };
    
    let mut byte: u8 = content[(strtab_start + name_offset as u64)
                               as usize];
    let mut i: usize = 1;
    let mut section_name: String = String::from("");
    while byte != 0x00 {
        section_name.push(byte as char);
        byte = content[(strtab_start + name_offset as u64) as usize + i];
        i += 1;
    }
    s_header.sname = section_name; 
    cursor += 0x04;
    
    // section type
    buff = &content[cursor..(cursor + 0x04)];
    let stype: u32 = match endian {
        Endian::Little => buff.read_u32::<LittleEndian>().unwrap(),
        Endian::Big => buff.read_u32::<BigEndian>().unwrap(),
        _ => return Err("Unkown error"),
    };

    s_header.stype = match stype {
       0x00 => SType::NULL,
       0x01 => SType::PROGBITS,
       0x02 => SType::SYMTAB,
       0x03 => SType::STRTAB,
       0x04 => SType::RELA,
       0x05 => SType::HASH,
       0x06 => SType::DYNAMIC,
       0x07 => SType::NOTE,
       0x08 => SType::NOBITS,
       0x09 => SType::REL,
       0x0a => SType::SHLIB,
       0x0b => SType::DYNSYM,
       0x0e => SType::INIT_ARRAY,
       0x0f => SType::FINI_ARRAY,
       0x10 => SType::PREINIT_ARRAY,
       0x11 => SType::GROUP,
       0x12 => SType::SYMTAB_SHNDX,
       0x13 => SType::NUM,
       _ => SType::NONE,
    };
    cursor += 0x04;

    // Flags
    buff = &content[cursor..(cursor + width)];
    let sflags: u64;
    if class == Class::X64Bit {
        sflags = match endian {
            Endian::Little => buff.read_u64::<LittleEndian>().unwrap(),
            Endian::Big => buff.read_u64::<BigEndian>().unwrap(),
            _ => return Err("TBD"),
        };
    }
    else {
        sflags = match endian {
            Endian::Little => buff.read_u32::<LittleEndian>().unwrap()
                as u64,
            Endian::Big => buff.read_u32::<BigEndian>().unwrap() as u64,
            _ => return Err("TBD"),
        };
    }
    s_header.sflags = match sflags {
        0x01 => SFlags::WRITE,
        0x02 => SFlags::ALLOC,
        0x04 => SFlags::EXECINSTR,
        0x10 => SFlags::MERGE,
        0x20 => SFlags::STRINGS,
        0x40 => SFlags::INFO_LINK,
        0x80 => SFlags::LINK_ORDER,
        0x100 => SFlags::OS_NONCONFORMING,
        0x200 => SFlags::GROUP,
        0x400 => SFlags::TLS,
        0x0ff00000 => SFlags::MASKOS,
        0xf0000000 => SFlags::MASKPROC,
        0x4000000 => SFlags::ORDERED,
        0x8000000 => SFlags::EXCLUDE,
        0x03 => SFlags::WRTIE_ALLOC,
        0x06 => SFlags::EXECINSTR_ALLOC,
        0x30 => SFlags::MERGE_STRINGS,
        _ => SFlags::NONE,
    };
    cursor += width;

    // vaddr
    buff = &content[cursor..(cursor + width)];
    s_header.vaddr = match class {
        Class::X64Bit  => {
            match endian {
                Endian::Little => buff.read_u64::<LittleEndian>().unwrap(),
                Endian::Big => buff.read_u64::<BigEndian>().unwrap(),
                _ => return Err("TBD"),
            }
        },
        Class::X32Bit => {
            match endian {
                Endian::Little => buff.read_u32::<LittleEndian>().unwrap()
                as u64,
                Endian::Big => buff.read_u32::<BigEndian>().unwrap() as u64,
                _ => return Err("TBD"),
            }
        },
        _ => return Err("TBD"),
    };
    cursor += width;

    // offset
    buff = &content[cursor..(cursor + width)];
    s_header.offset = match class {
        Class::X64Bit  => {
            match endian {
                Endian::Little => buff.read_u64::<LittleEndian>().unwrap(),
                Endian::Big => buff.read_u64::<BigEndian>().unwrap(),
                _ => return Err("TBD"),
            }
        },
        Class::X32Bit => {
            match endian {
                Endian::Little => buff.read_u32::<LittleEndian>().unwrap()
                as u64,
                Endian::Big => buff.read_u32::<BigEndian>().unwrap() as u64,
                _ => return Err("TBD"),
            }
        },
        _ => return Err("TBD"),
    };
    cursor += width;

    // size
    buff = &content[cursor..(cursor + width)];
    s_header.size = match class {
        Class::X64Bit  => {
            match endian {
                Endian::Little => buff.read_u64::<LittleEndian>().unwrap(),
                Endian::Big => buff.read_u64::<BigEndian>().unwrap(),
                _ => return Err("TBD"),
            }
        },
        Class::X32Bit => {
            match endian {
                Endian::Little => buff.read_u32::<LittleEndian>().unwrap()
                as u64,
                Endian::Big => buff.read_u32::<BigEndian>().unwrap() as u64,
                _ => return Err("TBD"),
            }
        },
        _ => return Err("TBD"),
    };
    cursor += width;
    
    // link
    buff = &content[cursor..(cursor + 0x04)];
    s_header.link = match endian {
        Endian::Little => buff.read_u32::<LittleEndian>().unwrap(),
        Endian::Big => buff.read_u32::<BigEndian>().unwrap(),
        _ => return Err("TBD"),
    };
    cursor += 0x04;

    // info
    buff = &content[cursor..(cursor + 0x04)];
    s_header.info = match endian {
        Endian::Little => buff.read_u32::<LittleEndian>().unwrap(),
        Endian::Big => buff.read_u32::<BigEndian>().unwrap(),
        _ => return Err("TBD"),
    };
    cursor += 0x04;

    // align
    buff = &content[cursor..(cursor + width)];
    s_header.align = match class {
        Class::X64Bit  => {
            match endian {
                Endian::Little => buff.read_u64::<LittleEndian>().unwrap(),
                Endian::Big => buff.read_u64::<BigEndian>().unwrap(),
                _ => return Err("TBD"),
            }
        },
        Class::X32Bit => {
            match endian {
                Endian::Little => buff.read_u32::<LittleEndian>().unwrap()
                as u64,
                Endian::Big => buff.read_u32::<BigEndian>().unwrap() as u64,
                _ => return Err("TBD"),
            }
        },
        _ => return Err("TBD"),
    };
    cursor += width;
   
    // entsize
    buff = &content[cursor..(cursor + width)];
    s_header.entsize = match class {
        Class::X64Bit  => {
            match endian {
                Endian::Little => buff.read_u64::<LittleEndian>().unwrap(),
                Endian::Big => buff.read_u64::<BigEndian>().unwrap(),
                _ => return Err("TBD"),
            }
        },
        Class::X32Bit => {
            match endian {
                Endian::Little => buff.read_u32::<LittleEndian>().unwrap()
                as u64,
                Endian::Big => buff.read_u32::<BigEndian>().unwrap() as u64,
                _ => return Err("TBD"),
            }
        },
        _ => return Err("TBD"),
    };
    cursor += width;

    // end of program header
    dbg!(s_header);
    Ok(())
}
