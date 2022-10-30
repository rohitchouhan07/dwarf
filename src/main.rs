use std::{fs, error::Error, process};
use clap::Parser;

fn parse_args() -> String {
    #[derive(Parser)]
    #[command(author, version, about, long_about = None)]
    struct Args {
        #[arg(long, short)]
        file_path: String,
    }
    let args:Args = Args::parse();
    args.file_path
}

// struct Header {
//     class: u8,
//     endian: u8,
//     version: u8,
//     abi: u8,
// }

fn main(){
    // get the command line arguments like the binary file name
    let binary_path: String = parse_args();
    println!("The binary to parse is: {binary_path}");

    // Now we need to read the binary ELF file
    if let Err(e) = run(binary_path) {
        println!("Error: {e}");
        process::exit(1);
    }
}

fn run(binary_path: String) -> Result<(), Box<dyn Error>>{
    let content:Vec<u8> = fs::read(binary_path)?;

    // we have the contents in a byte array, time to start parsing ELF header
    parse_header(content)?;
    Ok(())
}

fn parse_header(content:Vec<u8>) -> Result<(), &'static str> {
    // first we check whether it a valid ELF file or not
    const MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

    let magic: &[u8] = &content[0..4];
    if MAGIC != magic {
        return Err("Not an ELF binary");
    }
    println!("Valid ELF binary.");
    
    // now that we know the it is a valid ELF, we can move ahead
    if content[4] == 0x1 {
        println!("32-bit binary");
    }
    else {
        println!("64-bit binary");
    }

    if content[5] == 0x1 {
        println!("Little endian");
    }
    else {
        println!("Big endian");
    }

    println!("Version: {}", content[6]);

    if content[7] == 0x0 {
        println!("OS ABI: SysV");
    }
    else {
        println!("OS ABI: Unknown");
    }
    
    dbg!(&content[16]);
    Ok(())
}
