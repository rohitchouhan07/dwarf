use clap::Parser;
use std::{error::Error, fs, process, str};

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
}
#[derive(Debug, Default)]
struct Header {
    class: &'static str,
    endian: &'static str,
    version: u8,
    abi: &'static str,
    bin_type: BinType,
    machine: &'static str,
    misc_version: u32,
    entry_point: u64,
    phdr_offset: u64,
    shdr_offset: u64,
    flags: u16,
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
    parse_header(content)?;
    Ok(())
}

fn parse_header(content: Vec<u8>) -> Result<(), &'static str> {
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

    // reading the class
    if content[4] == 1 {
        header.class = "32-bit";
    } else if content[4] == 2 {
        header.class = "64-bit";
    }

    // reading endianness
    if content[5] == 1 {
        header.endian = "Little";
    } else if content[5] == 2 {
        header.endian = "Big";
    }

    // read the version
    header.version = content[6];

    // OS abi
    if content[7] == 0 {
        header.abi = "SYS V";
    }

    // type of binary
    let bin_type: u16 = (content[16] + content[17] * 16) as u16;
    match bin_type {
        1 => header.bin_type = BinType::REL,
        2 => header.bin_type = BinType::EXEC,
        3 => header.bin_type = BinType::DYN,
        4 => header.bin_type = BinType::CORE,
        _ => header.bin_type = BinType::NONE,
    }

    // machine
    let machine: u16 = content[18] as u16 + content[19] as u16 * 16;
    if machine == 0x03 {
        header.machine = "x86";
    } else if machine == 0x3E {
        header.machine = "x86-64";
    }

    // entry point
    header.entry_point = content[24] as u64
        + content[25] as u64 * 16_u64.pow(1)
        + content[26] as u64 * 16_u64.pow(2)
        + content[27] as u64 * 16_u64.pow(3)
        + content[28] as u64 * 16_u64.pow(4)
        + content[29] as u64 * 16_u64.pow(5)
        + content[30] as u64 * 16_u64.pow(6)
        + content[31] as u64 * 16_u64.pow(7);

    // program header entry point
    header.phdr_offset = content[32] as u64
        + content[33] as u64 * 16_u64.pow(1)
        + content[34] as u64 * 16_u64.pow(2)
        + content[35] as u64 * 16_u64.pow(3)
        + content[36] as u64 * 16_u64.pow(4)
        + content[37] as u64 * 16_u64.pow(5)
        + content[38] as u64 * 16_u64.pow(6)
        + content[39] as u64 * 16_u64.pow(7);

    // section header entry point
    header.shdr_offset = content[40] as u64
        + content[41] as u64 * 16_u64.pow(1)
        + content[42] as u64 * 16_u64.pow(2)
        + content[43] as u64 * 16_u64.pow(3)
        + content[44] as u64 * 16_u64.pow(4)
        + content[45] as u64 * 16_u64.pow(5)
        + content[46] as u64 * 16_u64.pow(6)
        + content[47] as u64 * 16_u64.pow(7);

    // flags
    header.flags = content[48] as u16
    + content[49] as u16 * 16_u16.pow(1);

    // header size
    header.hdr_sz = content[50] as u16
    + content[51] as u16 * 16_u16.pow(1);

    println!("{}", content[25]);
    println!("{}", content[26]);
    dbg!(header);
    Ok(())
}
