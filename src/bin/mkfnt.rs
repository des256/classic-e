// E - Make FNT file from TTF file
// Desmond Germans, 2020

extern crate freetype;

use e::*;
use std::env;
use std::path::Path;
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
    (0x3040,0x30A0),  // Hiragana
    (0x30A0,0x3100),  // Katakana
];

struct ImageCharacter {
    n: u32,
    image: Mat<u8>,
    bearing: Vec2<i32>,
    advance: i32,
}

struct Character {
    n: u32,
    r: Rect<i32>,
    bearing: Vec2<i32>,
    advance: i32,
}

#[derive(Clone,Copy)]
struct Mapped {
    used: bool,
    value: u8,
}

impl Zero for Mapped {
    fn zero() -> Mapped {
        Mapped {
            used: false,
            value: 0,
        }
    }
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
    println!("    -2,        --pot          Output power-of-two texture instead of div-by-64");
    println!("    -s <size>, --size <size>  Font size (default 28)");
    std::process::exit(-1);
}

fn exit_version() {
    println!("Make E Font Texture from TTF File");
    std::process::exit(-1);
}

fn find_empty(size: &Vec2<usize>,haystack: &Mat<Mapped>,p: &mut Vec2<isize>) -> bool {
    for hy in 0..haystack.size.y - size.y as usize {
        for hx in 0..haystack.size.x - size.x as usize {
            let mut found = true;
            for y in 0..size.y {
                for x in 0..size.x {
                    if haystack.get(vec2!(hx + x,hy + y)).used {
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

fn find_rect(needle: &Mat<u8>,haystack: &Mat<Mapped>,p: &mut Vec2<isize>) -> bool {
    for hy in 0..haystack.size.y - needle.size.y {
        for hx in 0..haystack.size.x - needle.size.x {
            let mut found = true;
            for y in 0..needle.size.y {
                for x in 0..needle.size.x {
                    if haystack.get(vec2!(hx + x,hy + y)).value != needle.get(vec2!(x,y)) {
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
    let mut fontsize: i32 = 28;
    let mut options_pot = false;
    while let Some(arg) = args.next() {
        match &arg[..] {
            "-h" | "--help" => { exit_help(); },
            "-v" | "--version" => { exit_version(); },
            "-s" | "--size" => {
                if let Some(value_string) = args.next() {
                    if let Ok(value) = value_string.parse::<i32>() {
                        fontsize = value;
                    }
                    else {
                        exit_help();
                    }
                }
            },
            "-2" | "--pot" => { options_pot = true; },
            _ => { exit_help(); },
        }
    }

    // initialize FreeType
    let ft = freetype::Library::init().unwrap();

    let face = ft.new_face(infile.file_name().unwrap().to_str().unwrap(),0).unwrap();
    face.set_char_size((fontsize * 64) as isize, 0, 50, 0).unwrap();  // TODO

    let padding: Vec2<i32> = vec2!(1,1);

    let mut min = 0i32;
    let mut max = 0i32;

    // convert and load all characters
    let mut image_characters: Vec<ImageCharacter> = Vec::new();
    for set in CHARACTERS.iter() {
        for n in set.0..set.1 {
            face.load_char(n as usize,freetype::face::LoadFlag::RENDER).unwrap();
            let glyph = face.glyph();
            let bitmap = glyph.bitmap();
            let width = bitmap.width();
            let height = bitmap.rows();
            let buffer = bitmap.buffer();
            let metrics = glyph.metrics();
            let bx = metrics.horiBearingX >> 6;
            let by = metrics.horiBearingY >> 6;
            let a = metrics.horiAdvance >> 6;
            println!("{:04X}: {}x{}, bearing {},{}, advance {}",n,width,height,bx,by,a);
            let mut cutout = Mat::<u8>::new(vec2!(
                (width + 2 * padding.x) as usize,
                (height + 2 * padding.y) as usize)
            );
            for y in 0..height {
                for x in 0..width {
                    let b = buffer[(y * width + x) as usize];
                    cutout.set(vec2!((x + padding.x) as usize,(y + padding.y) as usize),b);
                }
            }
            image_characters.push(ImageCharacter {
                n: n,
                image: cutout,
                bearing: vec2!(bx as i32,by as i32),
                advance: a as i32,
            });
            if -(by as i32) < min {
                min = -(by as i32);
            }
            if -(by as i32) + (height as i32) > max {
                max = -(by as i32) + (height as i32);
            }
        }
    }

    //Command::new("sh").arg("-c").arg(format!("rm output.png")).output().expect("unable to remove output.png");

    // sort image characters by height
    image_characters.sort_by(|a,b| b.image.size.y.cmp(&a.image.size.y));

    for m in 2..64 {
        let tsize = m * 64;
        if !options_pot || is_pot(tsize) {
            println!("Trying to fit on {}x{} texture...",tsize,tsize);
            let mut image = Mat::<Mapped>::new(vec2!(tsize,tsize));
            let mut characters: Vec<Character> = Vec::new();
            let mut everything_placed = true;
            for ch in image_characters.iter() {
                //println!("checking to see if {:04X} is already represented...",ch.n);
                let mut p = vec2!(0,0);
                if find_rect(&ch.image,&image,&mut p) {
                    let r = rect!(
                        p.x as i32 + padding.x,
                        p.y as i32 + padding.y,
                        ch.image.size.x as i32 - 2 * padding.x,
                        ch.image.size.y as i32 - 2 * padding.y
                    );
                    //println!("re-using pixels for {:04X}",ch.n);
                    //println!("    found at {},{}!",p.x,p.y);
                    characters.push(Character {
                        n: ch.n,
                        r: r,
                        bearing: ch.bearing,
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
                                image.set(vec2!(p.x as usize + x,p.y as usize + y),Mapped { used: true,value: ch.image.get(vec2!(x,y)), });
                            }
                        }
                        let r = rect!(
                            p.x as i32 + padding.x,
                            p.y as i32 + padding.y,
                            ch.image.size.x as i32 - 2 * padding.x,
                            ch.image.size.y as i32 - 2 * padding.y
                        );
                        characters.push(Character {
                            n: ch.n,
                            r: r,
                            bearing: ch.bearing,
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
                let mut file = File::create(outfile.file_name().unwrap().to_str().unwrap()).expect("cannot create file");
                let mut buffer: Vec<u8> = Vec::new();

                buffer.push(0x45);
                buffer.push(0x46);
                buffer.push(0x4E);
                buffer.push(0x54);
                buffer.push(0x30);
                buffer.push(0x30);
                buffer.push(0x32);
                buffer.push(0x00);
                push_i32(&mut buffer,max - min);  // font height
                push_i32(&mut buffer,-min);  // font Y bearing
                push_u32(&mut buffer,image.size.x as u32);  // texture atlas width
                push_u32(&mut buffer,image.size.y as u32);  // texture atlas height
                push_u32(&mut buffer,characters.len() as u32);  // number of characters
                for ch in &characters {
                    push_u32(&mut buffer,ch.n);
                    push_i32(&mut buffer,ch.r.o.x);
                    push_i32(&mut buffer,ch.r.o.y);
                    push_i32(&mut buffer,ch.r.s.x);
                    push_i32(&mut buffer,ch.r.s.y);
                    push_i32(&mut buffer,ch.bearing.x);
                    push_i32(&mut buffer,ch.bearing.y);
                    push_i32(&mut buffer,ch.advance);
                    //println!("{:04X}: {},{} ({}x{}); {},{}; {}",ch.n,ch.r.o.x,ch.r.o.y,ch.r.s.x,ch.r.s.y,ch.offset.x,ch.offset.y,ch.advance);
                }
                for y in 0..image.size.y {
                    for x in 0..image.size.x {
                        buffer.push(image.get(vec2!(x,y)).value);
                    }
                }
                file.write_all(&buffer).expect("cannot write");

                let mut file = File::create("debug.bmp").expect("what?");
                let mut debug_image = Mat::<pixel::ARGB8>::new(image.size);
                for y in 0..image.size.y {
                    for x in 0..image.size.x {
                        let b = image.get(vec2!(x,y)).value as u32;
                        let d = 0xFF000000 | (b << 16) | (b << 8) | b;
                        let p = pixel::ARGB8::from(d);
                        debug_image.set(vec2!(x,y),p);
                    }
                }
                let debug_buffer = image::bmp::encode(&debug_image).expect("what?");
                file.write_all(&debug_buffer).expect("what?");

                break;
            }
        }
    }
}