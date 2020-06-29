#[derive(Debug)]
pub struct Char {
    pub point: i32,
    pub w: f32,
    pub h: f32,
    pub dx: f32,
    pub dy: f32,
    pub adv: f32,
    pub u0: f32,
    pub v0: f32,
    pub us: f32,
    pub vs: f32,
}

#[derive(Debug)]
pub struct Kerning {
    pub a: i32,
    pub b: i32,
    pub n: f32,
}

#[derive(Debug)]
pub struct Font {
    pub page: u16,
    pub name: &'static str,
    pub fsize: i32,
    stretch_h: i32,
    space_x: i32,
    space_y: i32,
    pub line_height: i32,
    pub base: i32,
    pub scale_w: i32,
    pub scale_h: i32,
    pub chars: &'static [Char],
    pub kernings: &'static [Kerning],
}

impl Font {
    pub fn height(&self) -> f32 {
        (self.line_height as f32) / (self.fsize as f32)
    }

    pub fn advance(&self, text: &str) -> f32 {
        let mut total: f32 = 0.0;
        for ca in text.chars() {
            for a in self.chars {
                if a.point == ca as i32 {
                    total += a.adv;
                    break;
                }
            }
        }
        total
    }

    pub fn advance_kerning(&self, text: &str) -> f32 {
        let mut total: f32 = 0.0;
        let mut iter = text.chars().peekable();
        loop {
            match iter.next() {
                None => { break; },
                Some(ca) => {
                    for a in self.chars {
                        if a.point == ca as i32 {
                            total += a.adv;
                            match iter.peek() {
                                None => {},
                                Some(cb) => {
                                    for b in self.chars {
                                        if b.point == *cb as i32 {
                                            for k in self.kernings {
                                                if (k.a == a.point) && (k.b == b.point) {
                                                    total += k.n;
                                                }                                    
                                            }
                                            break;
                                        }
                                    }
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }
        total
    }
}

mod mono;
pub use mono::MONO;

mod pixel;
pub use pixel::PIXEL;

mod sans;
pub use sans::SANS;

mod sansi;
pub use sansi::SANSI;

mod serif;
pub use serif::SERIF;

mod serifi;
pub use serifi::SERIFI;
