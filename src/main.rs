use clap::Parser;
use std::{error::Error, fs, process, str};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

fn parse_args() -> String {
    #[derive(Parser)]
    #[command(author, version, about, long_about = None)]
    struct Args {
        #[arg(long, short)]
        file_path: String,
    }
    let args: Args = Args::parse();

    args.file_path
}

#[derive(Debug, Default)]
enum BinType {
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

#[derive(Debug, Default, PartialEq)]
enum Class {
    #[default]
    NONE,
    X32Bit,
    X64Bit,
} 

#[derive(Debug, Default)]
enum Endian {
    #[default]
    NONE,
    Little,
    Big,
}

#[derive(Debug, Default)]
enum Abi {
    #[default]
    NONE,
    SystemV,
    HP_UX,
    NetBSD,
    Linux,
    GNU_Hurd,
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
enum Machine {
    #[default]
    NONE,
    X86,
    AMD64,
}

#[derive(Debug, Default)]
struct Header {
    class: Class,
    endian: Endian,
    version: u8,
    abi: Abi,
    bin_type: BinType,
    machine: Machine,
    misc_version: u32,
    entry_point: u64,
    phdr_offset: u64,
    shdr_offset: u64,
    flags: u32,
    hdr_sz: u16,
    phdr_entry_size: u16,
    phdr_entries: u16,
    shdr_entry_sz: u16,
    shdr_entries: u16,
    sec_entry_idx: u16,
}

fn main() {
    // get the command line arguments like the binary file name
    let binary_path: String = parse_args();
    println!("The binary to parse is: {binary_path}");

    // Now we need to read the binary ELF file
    if let Err(e) = run(binary_path) {
        println!("Error: {e}");
        process::exit(1);
    }
}

fn run(binary_path: String) -> Result<(), Box<dyn Error>> {
    let content: Vec<u8> = fs::read(binary_path)?;

    // we have the contents in a byte array, time to start parsing ELF header
    parse_header(&content)?;
    Ok(())
}

fn parse_header(content: &Vec<u8>) -> Result<(), &'static str> {
    // first we check whether it a valid ELF file or not
    const MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

    let magic: &[u8] = &content[0..4];
    if MAGIC != magic {
        return Err("Not an ELF binary");
    }
    println!("Valid ELF binary.");

    // now that we know the it is a valid ELF, we can move ahead
    let mut header: Header = Header {
        ..Default::default()
    };
    
    //cursor
    let mut cursor: usize = 0x04;
    let mut width: usize = 0x08;

    // reading the class
    match content[cursor] {
        1 => {
            header.class = Class::X32Bit;
            width = 0x04;    
        },
        2 => header.class = Class::X64Bit,
        _ => header.class = Class::NONE,
    }
    cursor += 0x01;

    // reading endianness
    match content[cursor] {
        1 => header.endian = Endian::Little,
        2 => header.endian = Endian::Big,
        _ => header.endian = Endian::NONE,
    }
    cursor += 0x01;

    // read the version
    header.version = content[cursor];
    cursor += 0x01;

    // OS abi
    match content[cursor] {
        0 => header.abi = Abi::SystemV,
        _ => header.abi = Abi::NONE,
    }
    cursor += 0x09;

    // type of binary
    let mut bin_type_arr: &[u8] = &content[cursor..(cursor + 0x02)];
    let bin_type = match header.endian {
        Endian::Little => bin_type_arr.read_u16::<LittleEndian>().unwrap(),
        Endian::Big => bin_type_arr.read_u16::<BigEndian>().unwrap(),
        Endian::NONE => return Err("Endianess of the system not defined."), 
    };

    match bin_type {
        1 => header.bin_type = BinType::REL,
        2 => header.bin_type = BinType::EXEC,
        3 => header.bin_type = BinType::DYN,
        4 => header.bin_type = BinType::CORE,
        0xFE00 => header.bin_type = BinType::LOOS,
        0xFEFF => header.bin_type = BinType::HIOS,
        0xFF00 => header.bin_type = BinType::LOPROC,
        0xFFFF => header.bin_type = BinType::HIPROC,
        _ => header.bin_type = BinType::NONE,
    }
    cursor += 0x02;
    // machine
    let mut machine_arr: &[u8] = &content[cursor..(cursor + 0x02)];
    let machine = match header.endian {
        Endian::Little => machine_arr.read_u16::<LittleEndian>().unwrap(),
        Endian::Big => machine_arr.read_u16::<BigEndian>().unwrap(),
        Endian::NONE => return Err("Endianness of the system not defined."),

    };
    match machine {
        0x03 => header.machine = Machine::X86,
        0x3E => header.machine = Machine::AMD64,
        _ =>  header.machine = Machine::NONE,
    }
    cursor += 0x02;

    // Another Version
    cursor += 0x04;

    // entry point
    let mut entry_point_arr: &[u8] = match header.class {
        Class::X32Bit | Class::X64Bit => &content[cursor..(cursor + width)],
        Class::NONE => return Err("Unknown binary type."),
    };

    header.entry_point = match header.endian {
        Endian::Little => {
            if header.class == Class::X64Bit {
                entry_point_arr.read_u64::<LittleEndian>().unwrap() 
            }
            else {
                entry_point_arr.read_u32::<LittleEndian>().unwrap() as u64
            }
        },
        Endian::Big => {
            if header.class == Class::X64Bit {
                entry_point_arr.read_u64::<BigEndian>().unwrap()
            } 
            else {
                entry_point_arr.read_u32::<BigEndian>().unwrap() as u64
            }
        },
        Endian::NONE => return Err("Endianness of the system not defined."),
    };
    cursor += width;

    // program header entry point
    let mut phdr_offset_arr: &[u8] = match header.class {
        Class::X32Bit | Class::X64Bit => &content[cursor..(cursor + width)],
        Class::NONE => return Err("Unknown binary type."),
    };

    header.phdr_offset = match header.endian {
        Endian::Little => {
            if header.class == Class::X64Bit {
                phdr_offset_arr.read_u64::<LittleEndian>().unwrap() 
            }
            else {
                phdr_offset_arr.read_u32::<LittleEndian>().unwrap() as u64
            }
        },
        Endian::Big => {
            if header.class == Class::X64Bit {
                phdr_offset_arr.read_u64::<BigEndian>().unwrap()
            } 
            else {
                phdr_offset_arr.read_u32::<BigEndian>().unwrap() as u64
            }
        },
        Endian::NONE => return Err("Endianness of the system not defined."),
    };
    cursor += width;

    // section header entry point
    let mut shdr_offset_arr: &[u8] = match header.class {
        Class::X32Bit | Class::X64Bit => &content[cursor..(cursor + width)],
        Class::NONE => return Err("Unknown binary type."),
    };

    header.shdr_offset = match header.endian {
        Endian::Little => {
            if header.class == Class::X64Bit {
                shdr_offset_arr.read_u64::<LittleEndian>().unwrap() 
            }
            else {
                shdr_offset_arr.read_u32::<LittleEndian>().unwrap() as u64
            }
        },
        Endian::Big => {
            if header.class == Class::X64Bit {
                shdr_offset_arr.read_u64::<BigEndian>().unwrap()
            } 
            else {
                shdr_offset_arr.read_u32::<BigEndian>().unwrap() as u64
            }
        },
        Endian::NONE => return Err("Endianness of the system not defined."),
    };
    cursor += width;

    // flags
    let mut flags_arr: &[u8] = &content[cursor..(cursor + 0x04)];
    header.flags = match header.endian {
        Endian::Little => flags_arr.read_u32::<LittleEndian>().unwrap(),
        Endian::Big => flags_arr.read_u32::<BigEndian>().unwrap(),
        Endian::NONE => return Err("Endianness of the system not defined."),

    };
    cursor += 0x04;

    // header size
    let mut hdr_sz_arr: &[u8] = &content[cursor..(cursor + 0x02)];
    header.hdr_sz = match header.endian {
        Endian::Little => hdr_sz_arr.read_u16::<LittleEndian>().unwrap(),
        Endian::Big => hdr_sz_arr.read_u16::<BigEndian>().unwrap(),
        Endian::NONE => return Err("Endianness of the system not defined."),

    };
    cursor += 0x02;
    
    dbg!(header);
    Ok(())
}
