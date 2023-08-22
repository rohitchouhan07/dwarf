use clap::Parser;
use std::{error::Error, fs, process, str};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
pub mod header_enums;
use self::header_enums::{BinType, Class, Endian, Abi, Machine};
pub mod program_header;
use crate::program_header::parse;

struct CliArgs {
    file_path: String,
    program_header: bool,
}

fn parse_args() -> CliArgs {
    #[derive(Parser)]
    #[command(author, version, about, long_about = None)]
    struct Args {
        #[arg(long, short)]
        file_path: String,
        #[arg(long, short, action)]
        program_header: bool,
    }
    let args: Args = Args::parse();

    CliArgs {
        file_path: args.file_path,
        program_header: args.program_header
    } 
}

#[derive(Debug, Default)]
struct Header {
    class: Class,
    endian: Endian,
    version: u8,
    abi: Abi,
    abi_version: u8,
    bin_type: BinType,
    machine: Machine,
    misc_version: u32,
    entry_point: u64,
    phdr_offset: u64,
    shdr_offset: u64,
    flags: u32,
    hdr_sz: u16,
    phdr_entry_sz: u16,
    phdr_entries: u16,
    shdr_entry_sz: u16,
    shdr_entries: u16,
    shstr_idx: u16,
}

fn main() {
    // get the command line arguments like the binary file name
    let cli_args: CliArgs = parse_args();
    println!("The file to be parsed is: {0}", cli_args.file_path);

    // Now we need to read the binary ELF file
    if let Err(e) = run(cli_args) {
        println!("Error: {e}");
        process::exit(1);
    }
}

fn run(cli_args: CliArgs) -> Result<(), Box<dyn Error>> {
    let content: Vec<u8> = fs::read(cli_args.file_path)?;

    // we have the contents 
    // in a byte array, time to start parsing ELF header
    
    let mut header: Header = Header {
        ..Default::default()
    };
    
    parse_header(&content, &mut header)?;
    if cli_args.program_header == true {
        let mut entry: u16 = 0;
        let mut phdr_offset: u64 = header.phdr_offset;
        while entry < header.phdr_entries {
            program_header::parse(&content, phdr_offset,
                              header.class, header.endian)?;
            entry += 1;
            phdr_offset += header.phdr_entry_sz as u64;
        }
    }
    Ok(())
}

fn parse_header(content: &Vec<u8>,
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
