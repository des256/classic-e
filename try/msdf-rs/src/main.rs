extern crate roxmltree;
use std::env;
use std::fs;
use std::io::Read;

#[derive(Debug)]
struct Char {
    id: i32,
    index: i32,
    character: char,
    width: i32,
    height: i32,
    xoffset: i32,
    yoffset: i32,
    xadvance: i32,
    chnl: i32,
    x: i32,
    y: i32,
    page: i32,
}

#[derive(Debug)]
struct Kerning {
    first: i32,
    second: i32,
    amount: i32,
}

#[derive(Debug)]
struct VariableFont {
    face: String,
    size: i32,
    bold: bool,
    italic: bool,
    charset: String,
    unicode: bool,
    stretch_h: i32,
    smooth: bool,
    aa: bool,
    padx0: i32,
    pady0: i32,
    padx1: i32,
    pady1: i32,
    spacex: i32,
    spacey: i32,
    line_height: i32,
    base: i32,
    scale_w: i32,
    scale_h: i32,
    pages: Vec<String>,
    packed: bool,
    alpha: bool,
    red: bool,
    green: bool,
    blue: bool,
    field_type: String,
    distance_range: i32,
    chars: Vec<Char>,
    kernings: Vec<Kerning>,
}

fn load_file(path: &str) -> String {
    let mut file = fs::File::open(path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

pub fn main() {
    let args = env::args().collect::<Vec<String>>();
    let text = load_file(&args[1]);
    let doc = roxmltree::Document::parse(&text).unwrap();
    let mut font = VariableFont {
        face: String::from(""),
        size: 0,
        bold: false,
        italic: false,
        charset: String::from(""),
        unicode: false,
        stretch_h: 0,
        smooth: false,
        aa: false,
        padx0: 0,
        pady0: 0,
        padx1: 0,
        pady1: 0,
        spacex: 0,
        spacey: 0,
        line_height: 0,
        base: 0,
        scale_w: 0,
        scale_h: 0,
        pages: Vec::new(),
        packed: false,
        alpha: false,
        red: false,
        green: false,
        blue: false,
        field_type: String::from(""),
        distance_range: 0,
        chars: Vec::new(),
        kernings: Vec::new(),
    };
    let root = doc.root_element();
    if root.tag_name().name() != "font" {
        panic!("unknown XML element {}", root.tag_name().name());
    }
    for child in root.children() {
        if child.is_element() {
            if child.tag_name().name() == "info" {
                for attrib in child.attributes() {
                    if attrib.name() == "face" {
                        font.face = attrib.value().to_string();
                    } else if attrib.name() == "size" {
                        font.size = attrib.value().parse().unwrap();
                    } else if attrib.name() == "bold" {
                        if attrib.value() != "0" {
                            font.bold = true;
                        }
                    } else if attrib.name() == "italic" {
                        if attrib.value() != "0" {
                            font.italic = true;
                        }
                    } else if attrib.name() == "charset" {
                        font.charset = attrib.value().to_string();
                    } else if attrib.name() == "unicode" {
                        if attrib.value() != "0" {
                            font.unicode = true;
                        }
                    } else if attrib.name() == "stretchH" {
                        font.stretch_h = attrib.value().parse().unwrap();
                    } else if attrib.name() == "smooth" {
                        if attrib.value() != "0" {
                            font.smooth = true;
                        }
                    } else if attrib.name() == "aa" {
                        if attrib.value() != "0" {
                            font.aa = true;
                        }
                    } else if attrib.name() == "padding" {
                        let paddings: Vec<&str> = attrib.value().split(",").collect();
                        font.padx0 = paddings[0].parse().unwrap();
                        font.pady0 = paddings[1].parse().unwrap();
                        font.padx1 = paddings[2].parse().unwrap();
                        font.pady1 = paddings[3].parse().unwrap();
                    } else if attrib.name() == "spacing" {
                        let spacings: Vec<&str> = attrib.value().split(",").collect();
                        font.spacex = spacings[0].parse().unwrap();
                        font.spacey = spacings[1].parse().unwrap();
                    } else {
                        panic!("unknown attribute in info: {}", attrib.name());
                    }
                }
            } else if child.tag_name().name() == "common" {
                for attrib in child.attributes() {
                    if attrib.name() == "lineHeight" {
                        font.line_height = attrib.value().parse().unwrap();
                    } else if attrib.name() == "base" {
                        font.base = attrib.value().parse().unwrap();
                    } else if attrib.name() == "scaleW" {
                        font.scale_w = attrib.value().parse().unwrap();
                    } else if attrib.name() == "scaleH" {
                        font.scale_h = attrib.value().parse().unwrap();
                    } else if attrib.name() == "pages" {
                        // infer number of pages from the actual pages element
                    } else if attrib.name() == "packed" {
                        if attrib.value() != "0" {
                            font.packed = true;
                        }
                    } else if attrib.name() == "alphaChnl" {
                        if attrib.value() != "0" {
                            font.alpha = true;
                        }
                    } else if attrib.name() == "redChnl" {
                        if attrib.value() != "0" {
                            font.red = true;
                        }
                    } else if attrib.name() == "greenChnl" {
                        if attrib.value() != "0" {
                            font.green = true;
                        }
                    } else if attrib.name() == "blueChnl" {
                        if attrib.value() != "0" {
                            font.blue = true;
                        }
                    } else {
                        panic!("unknown attribute in common: {}", attrib.name());
                    }
                }
            } else if child.tag_name().name() == "pages" {
                for element in child.children() {
                    if element.is_element() {
                        if element.tag_name().name() != "page" {
                            panic!(
                                "page expected in pages, instead of {}",
                                element.tag_name().name()
                            );
                        }
                        let mut id: i32 = 0;
                        let mut name: &str = "";
                        for attrib in element.attributes() {
                            if attrib.name() == "id" {
                                id = attrib.value().parse().unwrap();
                            } else if attrib.name() == "file" {
                                name = attrib.value();
                            } else {
                                panic!("unknown attribute in page: {}", attrib.name());
                            }
                        }
                        font.pages.push(name.to_string());
                    }
                }
            } else if child.tag_name().name() == "distanceField" {
                for attrib in child.attributes() {
                    if attrib.name() == "fieldType" {
                        font.field_type = attrib.value().to_string();
                    } else if attrib.name() == "distanceRange" {
                        font.distance_range = attrib.value().parse().unwrap();
                    } else {
                        panic!("unknown attribute in distanceField: {}", attrib.name());
                    }
                }
            } else if child.tag_name().name() == "chars" {
                for element in child.children() {
                    if element.is_element() {
                        if element.tag_name().name() != "char" {
                            panic!(
                                "char expected in chars, instead of {}",
                                element.tag_name().name()
                            );
                        }
                        let mut character = Char {
                            id: 0,
                            index: 0,
                            character: ' ',
                            width: 0,
                            height: 0,
                            xoffset: 0,
                            yoffset: 0,
                            xadvance: 0,
                            chnl: 0,
                            x: 0,
                            y: 0,
                            page: 0,
                        };
                        for attrib in element.attributes() {
                            if attrib.name() == "id" {
                                character.id = attrib.value().parse().unwrap();
                            } else if attrib.name() == "index" {
                                character.index = attrib.value().parse().unwrap();
                            } else if attrib.name() == "char" {
                                character.character = attrib.value().chars().next().unwrap();
                            } else if attrib.name() == "width" {
                                character.width = attrib.value().parse().unwrap();
                            } else if attrib.name() == "height" {
                                character.height = attrib.value().parse().unwrap();
                            } else if attrib.name() == "xoffset" {
                                character.xoffset = attrib.value().parse().unwrap();
                            } else if attrib.name() == "yoffset" {
                                character.yoffset = attrib.value().parse().unwrap();
                            } else if attrib.name() == "xadvance" {
                                character.xadvance = attrib.value().parse().unwrap();
                            } else if attrib.name() == "chnl" {
                                character.chnl = attrib.value().parse().unwrap();
                            } else if attrib.name() == "x" {
                                character.x = attrib.value().parse().unwrap();
                            } else if attrib.name() == "y" {
                                character.y = attrib.value().parse().unwrap();
                            } else if attrib.name() == "page" {
                                character.page = attrib.value().parse().unwrap();
                            } else {
                                panic!("unknown attribute in char: {}", attrib.name());
                            }
                        }
                        font.chars.push(character);
                    }
                }
            } else if child.tag_name().name() == "kernings" {
                for element in child.children() {
                    if element.is_element() {
                        if element.tag_name().name() != "kerning" {
                            panic!(
                                "kerning expected in kernings, instead of {}",
                                element.tag_name().name()
                            );
                        }
                        let mut kerning = Kerning {
                            first: 0,
                            second: 0,
                            amount: 0,
                        };
                        for attrib in element.attributes() {
                            if attrib.name() == "first" {
                                kerning.first = attrib.value().parse().unwrap();
                            } else if attrib.name() == "second" {
                                kerning.second = attrib.value().parse().unwrap();
                            } else if attrib.name() == "amount" {
                                kerning.amount = attrib.value().parse().unwrap();
                            } else {
                                panic!("unknown attribute in kerning: {}", attrib.name());
                            }
                        }
                        font.kernings.push(kerning);
                    }
                }
            } else {
                panic!("unknown XML element {}", child.tag_name().name());
            }
        }
    }
    println!("pub const FONTNAME: Font = Font {{");
    println!("    fsize: {},", font.size);
    println!("    stretch_h: {},", font.stretch_h);
    println!("    space_x: {},", font.spacex);
    println!("    space_y: {},", font.spacey);
    println!("    line_height: {},", font.line_height);
    println!("    base: {},", font.base);
    println!("    scale_w: {},", font.scale_w);
    println!("    scale_h: {},", font.scale_h);
    println!("    chars: &[");
    for c in font.chars {
        println!("        Char {{ point: {}, w: {}f32, h: {}f32, dx: {}f32, dy: {}f32, adv: {}f32, u0: {}f32, v0: {}f32, us: {}f32, vs: {}f32 }},",
            c.id,
            (c.width as f32) / (font.size as f32),
            (c.height as f32) / (font.size as f32),
            (c.xoffset as f32) / (font.size as f32),
            (c.yoffset as f32) / (font.size as f32),
            (c.xadvance as f32) / (font.size as f32),
            (c.x as f32) / (font.scale_w as f32),
            (c.y as f32) / (font.scale_h as f32),
            (c.width as f32) / (font.scale_w as f32),
            (c.height as f32) / (font.scale_h as f32));
    }
    println!("    ],");
    println!("    kernings: &[");
    for k in font.kernings {
        println!(
            "        Kerning {{ a: {}, b: {}, n: {}f32 }},",
            k.first,
            k.second,
            (k.amount as f32) / (font.size as f32),
        );
    }
    println!("    ],");
    println!("}};")
}
