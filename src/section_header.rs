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

    //// Flags
    //if class == Class::X64Bit {
    //    buff = &content[cursor..(cursor + 0x04)];
    //    let pflags = match endian {
    //        Endian::Little => {
    //            buff.read_u32::<LittleEndian>().unwrap()
    //        },
    //        Endian::Big => buff.read_u32::<BigEndian>().unwrap(),
    //        _ => return Err("Undefined Endianess.")
    //    };

    //    p_header.pflags = match pflags {
    //        0x01 => PFlags::X,
    //        0x02 => PFlags::W,
    //        0x04 => PFlags::R,
    //        0x05 => PFlags::RX,
    //        0x06 => PFlags::RW,
    //        0x07 => PFlags::RWX,
    //        _ => PFlags::NONE,
    //    };
    //    cursor += 0x04
    //}

    //// offset
    //buff = &content[cursor..(cursor + width)];
    //p_header.offset = match endian {
    //    Endian::Little => buff.read_u32::<LittleEndian>().unwrap()
    //        as u64,
    //    Endian::Big => buff.read_u32::<BigEndian>().unwrap() as u64,
    //    _ => return Err("Undefined Endianess.")
    //};
    //cursor += width;

    //// vaddr
    //buff = &content[cursor..(cursor + width)];
    //p_header.vaddr = match endian {
    //    Endian::Little => buff.read_u32::<LittleEndian>().unwrap()
    //        as u64,
    //    Endian::Big => buff.read_u32::<BigEndian>().unwrap() as u64,
    //    _ => return Err("Undefined endianess.")
    //};
    //cursor += width;

   //// paddr
    //buff = &content[cursor..(cursor + width)];
    //p_header.paddr = match endian {
    //    Endian::Little => buff.read_u32::<LittleEndian>().unwrap()
    //        as u64,
    //    Endian::Big => buff.read_u32::<BigEndian>().unwrap() as u64,
    //    _ => return Err("Undefined endianess.")
    //};
    //cursor += width;

    //// filesz
    //buff = &content[cursor..(cursor + width)];
    //p_header.filesz = match endian {
    //    Endian::Little => buff.read_u32::<LittleEndian>().unwrap()
    //        as u64,
    //    Endian::Big => buff.read_u32::<BigEndian>().unwrap() as u64,
    //    _ => return Err("TBD")
    //};
    //cursor += width;

    //// memsz
    //buff = &content[cursor..(cursor + width)];
    //p_header.memsz = match endian {
    //    Endian::Little => buff.read_u32::<LittleEndian>().unwrap() as
    //        u64,
    //    Endian::Big => buff.read_u32::<BigEndian>().unwrap() as u64,
    //    _ => return Err("TBD")
    //};
    //cursor += width;

    //// Flags for 32 bit
    //if class == Class::X32Bit {
    //    buff = &content[cursor..(cursor + 0x04)];
    //    let pflags = match endian {
    //        Endian::Little => {
    //            buff.read_u32::<LittleEndian>().unwrap()
    //        },
    //        Endian::Big => buff.read_u32::<BigEndian>().unwrap(),
    //        _ => return Err("TBD")
    //    };

    //    p_header.pflags = match pflags {
    //        0x01 => PFlags::X,
    //        0x02 => PFlags::W,
    //        0x04 => PFlags::R,
    //        0x05 => PFlags::RX,
    //        0x06 => PFlags::RW,
    //        0x07 => PFlags::RWX,
    //        _ => PFlags::NONE,
    //    };
    //    cursor += 0x04
    //}

    //// align
    //buff = &content[cursor..(cursor + width)];
    //p_header.align = match endian {
    //    Endian::Little => buff.read_u32::<LittleEndian>().unwrap()
    //        as u64,
    //    Endian::Big => buff.read_u32::<BigEndian>().unwrap() as u64,
    //    _ => return Err("TBD")
    //};
    //cursor += width;

    // end of program header
    dbg!(s_header);
    Ok(())
}
