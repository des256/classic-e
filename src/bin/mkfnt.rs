// E - Make FNT file from TTF file
// Desmond Germans, 2020

use std::env;
use std::path::Path;
use std::process::Command;
use e::i32_2;
use e::Image;
use e::ARGB8;
use std::fs::File;
use std::io::prelude::*;
use e::decode;
use e::bmp;
use e::usize_2;
use e::Zero;
use e::isize_r;
use e::i32_r;
use e::isize_2;
use e::Pixel;

static CHARACTERS: [char; 95] = [
    'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
    'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
    '0','1','2','3','4','5','6','7','8','9',
    ' ','!','"','#','$','%','&','\'','(',')','*','+',',','-','.','/',':',';','<','=','>','?','@','[','\\',']','^','_','`','{','|','}','~',
];

static TSIZES: [usize_2; 5] = [
    usize_2 { x: 64,y: 64, },
    usize_2 { x: 128,y: 128, },
    usize_2 { x: 256,y: 256, },
    usize_2 { x: 512,y: 512, },
    usize_2 { x: 1024,y: 1024, },
];

struct ImageCharacter {
    n: u32,
    image: Image<ARGB8>,
    offset: i32_2,
    advance: i32,
}

struct EmptyCharacter {
    n: u32,
    offset: i32_2,
    advance: i32,
}

struct Character {
    n: u32,
    r: i32_r,
    offset: i32_2,
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
    std::process::exit(-1);
}

fn exit_version() {
    println!("Make E Font Texture from TTF File");
    std::process::exit(-1);
}

pub fn rect_empty(image: &Image<ARGB8>,r: isize_r) -> bool {
    for wy in 0..r.s.y {
        let y = r.o.y + wy;
        if (y < 0) || (y >= image.size.y as isize) {
            return false;
        }
        for wx in 0..r.s.x {
            let x = r.o.x + wx;
            if (x < 0) || (x >= image.size.x as isize) {
                return false;
            }
            if image.pixel(usize_2 { x: x as usize,y: y as usize, }) != ARGB8::new_rgb(0,0,0) {
                return false;
            }
        }
    }
    true
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
            _ => { exit_help(); },
        }
    }

    // standard parameters
    let gensize = i32_2 { x: 48 * scale,y: 48 * scale, };
    let translate = i32_2 { x: 8 * scale,y: 8 * scale, };
    let border = i32_2 { x: 2,y: 2, };
    println!("gensize {}x{}, translate {},{}; border {}x{}",gensize.x,gensize.y,translate.x,translate.y,border.x,border.y);

    // convert and load all characters
    let mut image_characters: Vec<ImageCharacter> = Vec::new();
    let mut empty_characters: Vec<EmptyCharacter> = Vec::new();
    for c in CHARACTERS.iter() {
        let n = *c as u32;
        let cmd = format!("msdfgen msdf -printmetrics -scale {} -size {} {} -translate 8 8 -font {} {}",scale,gensize.x,gensize.y,infile.file_name().unwrap().to_str().unwrap(),n);
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
            println!("{:05}: {},{} - {},{}; {} (msdfgen)",n,bx0,by0,bx1,by1,badv);
            let px0 = (bx0.floor() as i32) + translate.x;
            let py0 = (by0.floor() as i32) + translate.y;
            let px1 = (bx1.ceil() as i32) + translate.x;
            let py1 = (by1.ceil() as i32) + translate.y;
            let padv = badv as i32;
            println!("       {},{} - {},{}; {} (pixel msdfgen)",px0,py0,px1,py1,padv);
            let x0 = px0;
            let y0 = gensize.y - 1 - py1;
            let x1 = px1;
            let y1 = gensize.y - 1 - py0;
            let adv = padv;
            println!("       {},{} - {},{}; {} (pixel)",x0,y0,x1,y1,adv);
            let xr = x0 - border.x;
            let yr = y0 - border.y;
            let xs = (x1 - x0) + 2 * border.x;
            let ys = (y1 - y0) + 2 * border.y;
            println!("       {},{} ({}x{}); {} (relative pixel)",xr,yr,xs,ys,adv);
            let ox = translate.x - xr;
            let oy = gensize.y - 1 - translate.y - yr;
            println!("       {},{} (offset)",ox,oy);
            let mut file = File::open("output.png").expect("cannot open output.png");
            let mut buffer: Vec<u8> = Vec::new();
            file.read_to_end(&mut buffer).expect("unable to read file");
            let image = decode::<ARGB8>(&buffer).expect("unable to decode");
            let mut cutout = Image::<ARGB8>::new(usize_2 { x: xs as usize,y: ys as usize, });
            for y in 0..ys {
                for x in 0..xs {
                    cutout.set_pixel(usize_2 { x: x as usize,y: y as usize, },image.pixel(usize_2 { x: (xr + x) as usize,y: (yr + y) as usize, }));
                }
            }
            image_characters.push(ImageCharacter {
                n: n,
                image: cutout,
                offset: i32_2 { x: ox,y: oy, },
                advance: adv,
            });
            //Command::new("sh").arg("-c").arg(format!("mv output.png {:05}.png",n)).output().expect("unable to remove output.png");
        }
        else {
            println!("{:?}",chunks);
            let adv = chunks[2].trim().trim_matches(',').trim().parse::<f32>().expect("what?") as i32;
            empty_characters.push(EmptyCharacter {
                n: n,
                offset: i32_2 { x: 0,y: 0, },
                advance: adv,
            });
        }
    }

    Command::new("sh").arg("-c").arg(format!("rm output.png")).output().expect("unable to remove output.png");

    // sort image characters by surface
    image_characters.sort_by(|a,b| b.image.size.y.cmp(&a.image.size.y));

    for tsize in TSIZES.iter() {
        let mut image = Image::<ARGB8>::new(*tsize);
        for y in 0..image.size.y {
            for x in 0..image.size.x {
                image.set_pixel(usize_2 { x: x as usize,y: y as usize, },ARGB8::new_rgb(0,0,0));
            }
        }
        let mut characters: Vec<Character> = Vec::new();
        let mut wx = 0;
        let mut wy = 0;
        let mut everything_placed = true;
        for ch in image_characters.iter() {
            let mut searching = true;
            while searching {
                if rect_empty(
                    &image,
                    isize_r {
                        o: isize_2 { x: wx as isize,y: wy as isize, },
                        s: isize_2 { x: ch.image.size.x as isize,y: ch.image.size.y as isize, },
                    }) {
                    println!("room for {:05} at {},{}",ch.n,wx,wy);
                    for y in 0..ch.image.size.y {
                        for x in 0..ch.image.size.x {
                            image.set_pixel(usize_2 { x: wx + x,y: wy + y, },ch.image.pixel(usize_2 { x: x,y: y, }));
                        }
                    }
                    characters.push(Character {
                        n: ch.n,
                        r: i32_r {
                            o: i32_2 { x: wx as i32 + border.x,y: wy as i32 + border.y + 1, },  // that + 1 is magic, seems to be a rounding artifact
                            s: i32_2 { x: ch.image.size.x as i32 - 2 * border.x,y: ch.image.size.y as i32 - 2 * border.y, }, },
                        offset: ch.offset,
                        advance: ch.advance,
                    });
                    searching = false;
                }
                wy += 1;
                if wy >= image.size.y {
                    wy = 0;
                    wx += 1;
                    if wx >= image.size.x {
                        println!("ran out of space");
                        break;
                    }
                }
            }
            if searching {
                println!("font does not fit on {}x{} texture",tsize.x,tsize.y);
                everything_placed = false;
                break;
            }
        }
        if everything_placed {
            let mut file = File::create(outfile.file_name().unwrap().to_str().unwrap()).expect("cannot create file");
            let buffer = bmp::encode::<ARGB8>(&image).expect("cannot encode");
            file.write_all(&buffer).expect("cannot write");
            for ch in characters {
                println!("{:05}: {},{} ({}x{}); {},{}; {}",ch.n,ch.r.o.x,ch.r.o.y,ch.r.s.x,ch.r.s.y,ch.offset.x,ch.offset.y,ch.advance);
            }
            break;
        }
    }
}