use crate::{Class, Endian};
use crate::{BigEndian, LittleEndian, ReadBytesExt};
pub mod phdr_enums;
use self::phdr_enums::{PType, PFlags};

#[derive(Debug, Default)]
struct PHeader {
    ptype: PType,
    pflags: PFlags,
    offset: u64,
    vaddr: u64,
    paddr: u64,
    filesz: u64,
    memsz: u64,
    align: u64,
}

pub fn parse(content: &Vec<u8>,
             phdr_offset: u64,
             class: Class,
             endian: Endian
             ) -> Result<(), &'static str> {
    
    // cursor
    let mut cursor: usize = phdr_offset as usize;
    let width: usize;
    let mut buff: &[u8];

    if class == Class::X32Bit {
        width = 0x04;
    }
    else {
        width = 0x08;
    }

    let mut p_header: PHeader = PHeader {
        ..Default::default()
    };

    // read the type of segment
    buff = &content[cursor..(cursor + 0x04)];
    let ptype: u32 = match endian {
        Endian::Little => buff.read_u32::<LittleEndian>().unwrap(),
        Endian::Big => buff.read_u32::<BigEndian>().unwrap(),
        _ => return Err("Unkown error"),
    };

    p_header.ptype = match ptype {
       0x00 => PType::NULL,
       0x01 => PType::LOAD,
       0x02 => PType::DYNAMIC,
       0x03 => PType::INTERP,
       0x04 => PType::NOTE,
       0x05 => PType::SHLIB,
       0x06 => PType::PHDR,
       0x07 => PType::TLS,
       0x60000000 => PType::LOOS,
       0x6FFFFFFF => PType::HIOS,
       0x70000000 => PType::LOPROC,
       0x7FFFFFFF => PType::HIPROC,
       _ => PType::NONE,
    };
    cursor += 0x04;

    // Flags
    if class == Class::X64Bit {
        buff = &content[cursor..(cursor + 0x04)];
        let pflags = match endian {
            Endian::Little => {
                buff.read_u32::<LittleEndian>().unwrap()
            },
            Endian::Big => buff.read_u32::<BigEndian>().unwrap(),
            _ => return Err("TBD")
        };

        p_header.pflags = match pflags {
            0x01 => PFlags::X,
            0x02 => PFlags::W,
            0x04 => PFlags::R,
            0x05 => PFlags::RX,
            0x06 => PFlags::RW,
            0x07 => PFlags::RWX,
            _ => PFlags::NONE,
        };
        cursor += 0x04
    }

    // offset
    buff = &content[cursor..(cursor + width)];
    p_header.offset = match endian {
        Endian::Little => buff.read_u32::<LittleEndian>().unwrap()
            as u64,
        Endian::Big => buff.read_u32::<BigEndian>().unwrap() as u64,
        _ => return Err("TBD")
    };
    cursor += width;

    // vaddr
    buff = &content[cursor..(cursor + width)];
    p_header.vaddr = match endian {
        Endian::Little => buff.read_u32::<LittleEndian>().unwrap()
            as u64,
        Endian::Big => buff.read_u32::<BigEndian>().unwrap() as u64,
        _ => return Err("TBD")
    };
    cursor += width;

   // paddr
    buff = &content[cursor..(cursor + width)];
    p_header.paddr = match endian {
        Endian::Little => buff.read_u32::<LittleEndian>().unwrap()
            as u64,
        Endian::Big => buff.read_u32::<BigEndian>().unwrap() as u64,
        _ => return Err("TBD")
    };
    cursor += width;

    // filesz
    buff = &content[cursor..(cursor + width)];
    p_header.filesz = match endian {
        Endian::Little => buff.read_u32::<LittleEndian>().unwrap()
            as u64,
        Endian::Big => buff.read_u32::<BigEndian>().unwrap() as u64,
        _ => return Err("TBD")
    };
    cursor += width;

    // memsz
    buff = &content[cursor..(cursor + width)];
    p_header.memsz = match endian {
        Endian::Little => buff.read_u32::<LittleEndian>().unwrap() as
            u64,
        Endian::Big => buff.read_u32::<BigEndian>().unwrap() as u64,
        _ => return Err("TBD")
    };
    cursor += width;

    // Flags for 32 bit
    if class == Class::X32Bit {
        buff = &content[cursor..(cursor + 0x04)];
        let pflags = match endian {
            Endian::Little => {
                buff.read_u32::<LittleEndian>().unwrap()
            },
            Endian::Big => buff.read_u32::<BigEndian>().unwrap(),
            _ => return Err("TBD")
        };

        p_header.pflags = match pflags {
            0x01 => PFlags::X,
            0x02 => PFlags::W,
            0x04 => PFlags::R,
            0x05 => PFlags::RX,
            0x06 => PFlags::RW,
            0x07 => PFlags::RWX,
            _ => PFlags::NONE,
        };
        cursor += 0x04
    }

    // align
    buff = &content[cursor..(cursor + width)];
    p_header.align = match endian {
        Endian::Little => buff.read_u32::<LittleEndian>().unwrap()
            as u64,
        Endian::Big => buff.read_u32::<BigEndian>().unwrap() as u64,
        _ => return Err("TBD")
    };
    cursor += width;

    // end of program header

    dbg!(p_header);
    Ok(())
}
