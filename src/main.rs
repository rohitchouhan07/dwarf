use clap::Parser;
use std::{error::Error, fs, process};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
pub mod elf_header;
use crate::elf_header::header_enums::{Class, Endian};
use crate::elf_header::Header;
pub mod program_header;
pub mod section_header;

struct CliArgs {
    file_path: String,
    program_header: bool,
    section_header: bool,
}

fn parse_args() -> CliArgs {
    #[derive(Parser)]
    #[command(author, version, about, long_about = None)]
    struct Args {
        #[arg(long, short)]
        file_path: String,
        #[arg(long, short, action)]
        program_header: bool,
        #[arg(long, short, action)]
        section_header: bool,
    }
    let args: Args = Args::parse();

    CliArgs {
        file_path: args.file_path,
        program_header: args.program_header,
        section_header: args.section_header,
    } 
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
    
    elf_header::parse(&content, &mut header)?;
    
    if cli_args.program_header == true {
        
        let mut phdr_offset: u64 = header.phdr_offset;
        for _ in 0..header.phdr_entries {
            program_header::parse(&content, phdr_offset,
                              header.class, header.endian)?;
            phdr_offset += header.phdr_entry_sz as u64;
        }
    }

    if cli_args.section_header == true {
        
        let mut shdr_offset: u64 = header.shdr_offset;
        let strtab_header_start: u64 = header.shdr_offset
                            + ((header.shstr_idx) as u64
                            * header.shdr_entry_sz as u64);
        let strtab_start: u64;
        if header.class == Class::X32Bit {
            if header.endian == Endian::Little {
                let mut buff: &[u8];
                buff = &content[(strtab_header_start + 0x10) as usize
                                ..(strtab_header_start + 0x14)
                                as usize];
                strtab_start = buff.read_u32::<LittleEndian>().unwrap() as u64;
            }
            else {
                let mut buff: &[u8];
                buff = &content[(strtab_header_start + 0x10) as usize
                                ..(strtab_header_start + 0x14)
                                as usize];
                strtab_start = buff.read_u32::<BigEndian>().unwrap() as u64;
            }
        }
        else {
             if header.endian == Endian::Little {
                let mut buff: &[u8];
                buff = &content[(strtab_header_start + 0x18) as usize
                                ..(strtab_header_start + 0x20)
                                as usize];
                strtab_start = buff.read_u64::<LittleEndian>().unwrap();
            }
            else {
                let mut buff: &[u8];
                buff = &content[(strtab_header_start + 0x10) as usize
                                ..(strtab_header_start + 0x14)
                                as usize];
                strtab_start = buff.read_u64::<BigEndian>().unwrap();
            }           
        }
        println!("{strtab_start}");
        for _ in 0..header.shdr_entries { 
            section_header::parse(&content, shdr_offset,
                          header.class, header.endian,
                          strtab_start)?;     
            shdr_offset += header.shdr_entry_sz as u64;
        }
    }
    
    Ok(())
}

