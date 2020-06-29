// e utilities: turn FFT into font texture
// by Desmond Germans, 2019

extern crate json;

use std::{env,process::Command,path::Path,fs,fs::File,io::Write};

fn exit_help() {
    println!("Make E Font Texture from TTF File");
    println!();
    println!("USAGE:");
    println!("    mkfnt <infile> <outfile> [FLAGS] [OPTIONS]");
    println!();
    println!("FLAGS:");
    println!("    -h, --help     Prints help information");
    println!("    -v, --version  Prints version information");
    println!();
    println!("OPTIONS:");
    println!("    -d, --dimension <dim>   Resulting font texture dimension (512)");
    println!("    -s, --size <size>       Font size (40)");
    std::process::exit(-1);
}

fn exit_version() {
    println!("Make E Font Texture from TTF File");
    std::process::exit(-1);
}

fn main() {
    
    let mut args = env::args();
    let _command = args.next().unwrap();
    let infile_wrap = args.next();
    if infile_wrap == None {
        exit_help();
    }
    let infile_temp = String::from(infile_wrap.unwrap());
    let infile = Path::new(&infile_temp);
    let outfile_wrap = args.next();
    if outfile_wrap == None {
        exit_help();
    }
    let outfile_temp = String::from(outfile_wrap.unwrap());
    let outfile = Path::new(&outfile_temp);
    let mut dim: u32 = 512;
    let mut size: u32 = 40;
    while let Some(arg) = args.next() {
        match &arg[..] {
            "-h" | "--help" => { exit_help(); },
            "-v" | "--version" => { exit_version(); },
            "-d" | "--dim" => {
                if let Some(value_string) = args.next() {
                    if let Ok(value) = value_string.parse::<u32>() {
                        dim = value;
                    }
                    else {
                        exit_help();
                    }
                }
            },
            "-s" | "--size" => {
                if let Some(value_string) = args.next() {
                    if let Ok(value) = value_string.parse::<u32>() {
                        size = value;
                    }
                    else {
                        exit_help();
                    }
                }
            },
            _ => { exit_help(); },
        }
    }
    
    // run msdf-bmfont, which is totally a debian package that is already installed
    Command::new("sh").arg("-c").arg(format!("msdf-bmfont -f json -o temp.png -s {} -m {},{} {}",size,dim,dim,infile.file_name().unwrap().to_str().unwrap())).output().expect("what?");

    // read the json file it created
    let json_name = format!("{}.json",infile.file_stem().unwrap().to_str().unwrap());
    let json_text = fs::read_to_string(&json_name).unwrap();
    let json_parsed = json::parse(&json_text).unwrap();

    // open the created PNG file
    let img = image::open("temp.png").unwrap();
    let binary: Vec<u8> = img.raw_pixels();

    // append info describing the characters from the json file to the binary image data
    let mut header: Vec<u8> = vec!['G' as u8,'F' as u8,'N' as u8,'T' as u8,1,0,0,0];
    let common = &json_parsed["common"];
    let chars = json_parsed["chars"].members();
    let kernings = json_parsed["kernings"].members();
    header.extend_from_slice(&common["lineHeight"].as_i32().unwrap().to_le_bytes());
    header.extend_from_slice(&common["base"].as_i32().unwrap().to_le_bytes());
    header.extend_from_slice(&common["scaleW"].as_i32().unwrap().to_le_bytes());
    header.extend_from_slice(&common["scaleH"].as_i32().unwrap().to_le_bytes());
    header.extend_from_slice(&(chars.len() as i32).to_le_bytes());
    header.extend_from_slice(&(kernings.len() as i32).to_le_bytes());
    for c in chars {
        header.extend_from_slice(&c["id"].as_i32().unwrap().to_le_bytes());
        header.extend_from_slice(&c["index"].as_i32().unwrap().to_le_bytes());
        header.extend_from_slice(&c["width"].as_i32().unwrap().to_le_bytes());
        header.extend_from_slice(&c["height"].as_i32().unwrap().to_le_bytes());
        header.extend_from_slice(&c["xoffset"].as_i32().unwrap().to_le_bytes());
        header.extend_from_slice(&c["yoffset"].as_i32().unwrap().to_le_bytes());
        header.extend_from_slice(&c["xadvance"].as_i32().unwrap().to_le_bytes());
        header.extend_from_slice(&c["x"].as_i32().unwrap().to_le_bytes());
        header.extend_from_slice(&c["y"].as_i32().unwrap().to_le_bytes());
    }
    for k in kernings {
        header.extend_from_slice(&k["first"].as_i32().unwrap().to_le_bytes());
        header.extend_from_slice(&k["second"].as_i32().unwrap().to_le_bytes());
        header.extend_from_slice(&k["amount"].as_i32().unwrap().to_le_bytes());
    }

    // write all of that in a new binary file (the actual output file)
    let mut file = File::create(outfile.file_name().unwrap()).expect("what?");
    file.write_all(&header[..]).expect("what?");
    file.write_all(&binary[..]).expect("what?");

    // and delete the temporary stuff
    Command::new("sh").arg("-c").arg(format!("rm -rf {} temp.png",json_name)).output().expect("what?");
}
