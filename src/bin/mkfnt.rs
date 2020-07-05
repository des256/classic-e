// E - Make FNT file from TTF file
// Desmond Germans, 2020

use e::*;
use std::env;
use std::path::Path;
use std::process::Command;
use std::fs::File;
use std::io::prelude::*;

static CHARACTERS: &[(u32,u32)] = &[
    (0x0020,0x0080),  // ASCII
    (0x00A0,0x0100),  // Latin-1 Supplement
    (0x0100,0x0180),  // Latin Extended-A
    (0x0180,0x0250),  // Latin Extended-B
    //(0x0250,0x02B0),  // IPA Extensions
    //(0x02B0,0x0300),  // Spacing Modifier Letters
    //(0x0300,0x0370),  // Combining Diacritical Marks
    (0x0370,0x0400),  // Greek and Coptic
    (0x0400,0x0500),  // Cyrillic
    (0x0500,0x0530),  // Cyrillic Supplement
    //(0x0530,0x0590),  // Armenian
    //(0x0590,0x0600),  // Hebrew
    //(0x0600,0x0700),  // Arabic
    //(0x0700,0x0750),  // Syriac
    //(0x0750,0x0780),  // Arabic Supplement
    //(0x3040,0x30A0),  // Hiragana
    //(0x30A0,0x3100),  // Katakana
];

struct ImageCharacter {
    n: u32,
    image: Mat<ARGB8>,
    offset: Vec2<i32>,
    advance: i32,
}

struct EmptyCharacter {
    n: u32,
    offset: Vec2<i32>,
    advance: i32,
}

struct Character {
    n: u32,
    r: Rect<i32>,
    offset: Vec2<i32>,
    advance: i32,
}

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
    println!("    -s, --scale <scale>  Font size multiplier");
    println!("    -2, --pot            Output power-of-two texture instead of div-by-64");
    println!("    -t, --texcoord       Use f32 texture coordinates instead of u32 pixel coordinates");
    std::process::exit(-1);
}

fn exit_version() {
    println!("Make E Font Texture from TTF File");
    std::process::exit(-1);
}

fn find_empty(size: &Vec2<usize>,haystack: &Mat<ARGB8>,p: &mut Vec2<isize>) -> bool {
    for hy in 0..haystack.size.y - size.y as usize {
        for hx in 0..haystack.size.x - size.x as usize {
            let mut found = true;
            for y in 0..size.y {
                for x in 0..size.x {
                    if haystack.get(vec2!(hx + x,hy + y)) != ARGB8::zero() {
                        found = false;
                        break;
                    }
                }
                if !found {
                    break;
                }
            }
            if found {
                p.x = hx as isize;
                p.y = hy as isize;
                return true;
            }
        }
    }
    false
}

fn find_rect(needle: &Mat<ARGB8>,haystack: &Mat<ARGB8>,p: &mut Vec2<isize>) -> bool {
    for hy in 0..haystack.size.y - needle.size.y {
        for hx in 0..haystack.size.x - needle.size.x {
            let mut found = true;
            for y in 0..needle.size.y {
                for x in 0..needle.size.x {
                    if haystack.get(vec2!(hx + x,hy + y)) != needle.get(vec2!(x,y)) {
                        found = false;
                        break;
                    }
                }
                if !found {
                    break;
                }
            }
            if found {
                p.x = hx as isize;
                p.y = hy as isize;
                return true;
            }
        }
    }
    false
}

fn is_pot(v: usize) -> bool {
    (v == 1) || (v == 2) || (v == 4) || (v == 8) ||
    (v == 16) || (v == 32) || (v == 64) || (v == 128) ||
    (v == 256) || (v == 512) || (v == 1024) || (v == 2048) ||
    (v == 4096) || (v == 8192) || (v == 16384) || (v == 32768)
}

fn push_u32(buf: &mut Vec<u8>,v: u32) {
    buf.push((v & 255) as u8);
    buf.push(((v >> 8) & 255) as u8);
    buf.push(((v >> 16) & 255) as u8);
    buf.push((v >> 24) as u8);
}

fn push_i32(buf: &mut Vec<u8>,v: i32) {
    push_u32(buf,v as u32);
}

fn main() {
    // parse arguments
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
    let mut scale: i32 = 1;
    let mut options_pot = false;
    while let Some(arg) = args.next() {
        match &arg[..] {
            "-h" | "--help" => { exit_help(); },
            "-v" | "--version" => { exit_version(); },
            "-s" | "--scale" => {
                if let Some(value_string) = args.next() {
                    if let Ok(value) = value_string.parse::<i32>() {
                        scale = value;
                    }
                    else {
                        exit_help();
                    }
                }
            },
            "-2" | "--pot" => {
                options_pot = true;
            }
            _ => { exit_help(); },
        }
    }

    // standard parameters
    let gensize = vec2!(64 * scale,64 * scale);
    let translate = vec2!(16 * scale,16 * scale);
    let border = vec2!(2,2);
    println!("gensize {}x{}, translate {},{}; border {}x{}",gensize.x,gensize.y,translate.x,translate.y,border.x,border.y);

    // convert and load all characters
    let mut image_characters: Vec<ImageCharacter> = Vec::new();
    let mut empty_characters: Vec<EmptyCharacter> = Vec::new();
    for set in CHARACTERS.iter() {
        for n in set.0..set.1 {
            let cmd = format!("msdfgen msdf -printmetrics -scale {} -size {} {} -translate 16 16 -font {} {}",scale,gensize.x,gensize.y,infile.file_name().unwrap().to_str().unwrap(),n);
            let output = Command::new("sh").arg("-c").arg(cmd).output().expect("unable to run command");
            if output.status.code().expect("what?") != 0 {
                panic!("Error running msdfgen.");
            }
            let text = String::from_utf8(output.stdout).expect("what?");
            let chunks: Vec<&str> = text.split_whitespace().collect();
            if chunks[0] == "bounds" {
                let bx0 = chunks[2].trim().trim_matches(',').trim().parse::<f32>().expect("what?") * scale as f32;
                let by0 = chunks[3].trim().trim_matches(',').trim().parse::<f32>().expect("what?") * scale as f32;
                let bx1 = chunks[4].trim().trim_matches(',').trim().parse::<f32>().expect("what?") * scale as f32;
                let by1 = chunks[5].trim().trim_matches(',').trim().parse::<f32>().expect("what?") * scale as f32;
                let badv = chunks[8].trim().trim_matches(',').trim().parse::<f32>().expect("what?") * scale as f32;
                //println!("{:05}: {},{} - {},{}; {} (raw)",n,bx0,by0,bx1,by1,badv);
                let x0 = (bx0.floor() as i32) + translate.x;
                let y0 = (by0.floor() as i32) + translate.y;
                let x1 = (bx1.ceil() as i32) + translate.x;
                let y1 = (by1.ceil() as i32) + translate.y;
                let adv = badv as i32;
                //println!("       {},{} - {},{}; {} (pixel)",x0,y0,x1,y1,adv);
                let xr = x0 - border.x;
                let yr = y0 - border.y;
                let xs = (x1 - x0) + 2 * border.x;
                let ys = (y1 - y0) + 2 * border.y;
                //println!("       {},{} ({}x{}); {} (relative)",xr,yr,xs,ys,adv);
                let ox = translate.x - xr;
                let oy = translate.y - yr;
                //println!("       {},{} (offset)",ox,oy);
                println!("{:04X}: {},{} ({}x{}); {}; {},{}",n,xr,yr,xs,ys,adv,ox,oy);
                let mut file = File::open("output.png").expect("cannot open output.png");
                let mut buffer: Vec<u8> = Vec::new();
                file.read_to_end(&mut buffer).expect("unable to read file");
                let image = decode::<ARGB8>(&buffer).expect("unable to decode");
                let mut cutout = Mat::<ARGB8>::new(vec2!(xs as usize,ys as usize));
                for y in 0..ys {
                    for x in 0..xs {
                        cutout.set(vec2!(x as usize,y as usize),image.get(vec2!((xr + x) as usize,(gensize.y - yr - ys + y) as usize)));
                    }
                }
                image_characters.push(ImageCharacter {
                    n: n,
                    image: cutout,
                    offset: vec2!(ox,oy),
                    advance: adv,
                });
                //Command::new("sh").arg("-c").arg(format!("mv output.png {:04X}.png",n)).output().expect("unable to remove output.png");
            }
            else {
                let adv = chunks[2].trim().trim_matches(',').trim().parse::<f32>().expect("what?") as i32;
                empty_characters.push(EmptyCharacter {
                    n: n,
                    offset: vec2!(0,0),
                    advance: adv,
                });
            }
        }
    }

    Command::new("sh").arg("-c").arg(format!("rm output.png")).output().expect("unable to remove output.png");

    // sort image characters by surface
    image_characters.sort_by(|a,b| b.image.size.y.cmp(&a.image.size.y));

    for m in 2..64 {
        let tsize = m * 64;
        if !options_pot || is_pot(tsize) {
            println!("Trying to fit on {}x{} texture...",tsize,tsize);
            let mut image = Mat::<ARGB8>::new(vec2!(tsize,tsize));
            let mut characters: Vec<Character> = Vec::new();
            let mut everything_placed = true;
            for ch in image_characters.iter() {
                //println!("checking to see if {:04X} is already represented...",ch.n);
                let mut p = vec2!(0,0);
                if find_rect(&ch.image,&image,&mut p) {
                    let r = rect!(p.x as i32 + border.x,p.y as i32 + border.y,ch.image.size.x as i32 - 2 * border.x,ch.image.size.y as i32 - 2 * border.y);
                    let fr = rect!(r.o.x,tsize as i32 - r.o.y - r.s.y,r.s.x,r.s.y);
                    //println!("re-using pixels for {:04X}",ch.n);
                    //println!("    found at {},{}!",p.x,p.y);
                    characters.push(Character {
                        n: ch.n,
                        r: fr,
                        offset: ch.offset,
                        advance: ch.advance,
                    });
                }
                else {
                    //println!("    no, searching for empty space of {}x{} pixels...",ch.image.size.x,ch.image.size.y);
                    if find_empty(&ch.image.size,&image,&mut p) {
                        //println!("allocating pixels for {:04X}",ch.n);
                        //println!("        found at {},{}!",p.x,p.y);
                        //println!("        writing {:04X} into atlas...",ch.n);
                        for y in 0..ch.image.size.y {
                            for x in 0..ch.image.size.x {
                                image.set(vec2!(p.x as usize + x,p.y as usize + y),ch.image.get(vec2!(x,y)));
                            }
                        }
                        let r = rect!(p.x as i32 + border.x,p.y as i32 + border.y,ch.image.size.x as i32 - 2 * border.x,ch.image.size.y as i32 - 2 * border.y);
                        let fr = rect!(r.o.x,tsize as i32 - r.o.y - r.s.y,r.s.x,r.s.y);
                        characters.push(Character {
                            n: ch.n,
                            r: fr,
                            offset: ch.offset,
                            advance: ch.advance,
                        });
                    }
                    else {
                        everything_placed = false;
                        break;
                    }
                }
            }
            if everything_placed {
                for ch in empty_characters {
                    characters.push(Character {
                        n: ch.n,
                        r: rect!(0,0,0,0),
                        offset: ch.offset,
                        advance: ch.advance,
                    });
                }

                let mut file = File::create(outfile.file_name().unwrap().to_str().unwrap()).expect("cannot create file");
                let mut buffer: Vec<u8> = Vec::new();

                buffer.push(0x45);                       // header
                buffer.push(0x46);
                buffer.push(0x4E);
                buffer.push(0x84);
                buffer.push(0x30);
                buffer.push(0x30);
                buffer.push(0x31);
                buffer.push(0x00);
                push_u32(&mut buffer,scale as u32);  // scale
                push_u32(&mut buffer,characters.len() as u32);  // number of characters
                for ch in &characters {
                    push_u32(&mut buffer,ch.n);          // code point
                    push_i32(&mut buffer,ch.r.o.x);      // rect on the image
                    push_i32(&mut buffer,ch.r.o.y);
                    push_i32(&mut buffer,ch.r.s.x);
                    push_i32(&mut buffer,ch.r.s.y);
                    push_i32(&mut buffer,ch.offset.x);   // character origin offset
                    push_i32(&mut buffer,ch.offset.y);
                    push_i32(&mut buffer,ch.advance);    // advance to next character
                    //println!("{:04X}: {},{} ({}x{}); {},{}; {}",ch.n,ch.r.o.x,ch.r.o.y,ch.r.s.x,ch.r.s.y,ch.offset.x,ch.offset.y,ch.advance);
                }
                let image_buffer = bmp::encode::<ARGB8>(&image).expect("cannot encode");
                buffer.append(&mut image_buffer.clone());
                file.write_all(&buffer).expect("cannot write");

                //let mut file = File::create("debug.bmp").expect("what?");
                //file.write_all(&image_buffer).expect("what?");

                break;
            }
        }
    }
}