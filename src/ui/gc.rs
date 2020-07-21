// E - UI - GC
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

pub struct GC<'a> {
    pub(crate) ui: &'a UI<'a>,
    font: RefCell<Rc<Font>>,
    color: Cell<Vec4<f32>>,
    ppu: Cell<Vec2<f32>>,  // pixels per unit
    size: Cell<Vec2<f32>>,  // viewport size (in units, not pixels)
}

const SCREEN: Vec2<f32> = Vec2 { x: 1.0,y: 1.0, };

impl<'a> GC<'a> {
    pub fn new(ui: &'a UI<'a>) -> Result<GC<'a>,SystemError> {
        Ok(GC {
            ui: ui,
            font: RefCell::new(ui.get_font("arialn.fnt",vec2!(14.0,14.0),0.0).expect("cannot load font")),
            color: Cell::new(vec4!(1.0,1.0,1.0,1.0)),
            ppu: Cell::new(SCREEN),
            size: Cell::new(vec2!(1.0,1.0)),
        })
    }

    pub fn set_size(&self,size: Vec2<f32>) {
        self.size.set(size);
    }

    pub fn set_ppu(&self,ppu: Vec2<f32>) {
        self.ppu.set(ppu);
    }

    pub fn set_color<T>(&self,color: T) where Vec4<f32>: From<T> {
        self.color.set(Vec4::<f32>::from(color));
    }

    pub fn set_font(&self,font: Rc<Font>) {
        *(self.font.borrow_mut()) = font;
    }

    pub fn draw_text(&self,p: Vec2<f32>,text: &str) {
        let mut vertices: Vec<Vec4<f32>> = Vec::new();
        let mut lp = vec2!(p.x as f32,p.y as f32);
        let mut count = 0;
        let font = self.font.borrow();
        for c in text.chars() {
            if let Some(ch) = font.proto.find(c) {
                if (ch.r.s.x > 0) && (ch.r.s.y > 0) {
                    // bottom-left of the character, in GU
                    let ox = lp.x - FONT.x * font.size.x * (ch.offset.x as f32) / (font.proto.scale as f32);
                    let oy = lp.y - FONT.y * font.size.y * (ch.offset.y as f32) / (font.proto.scale as f32);

                    // size of the character, in GU
                    let sx = FONT.x * font.size.x * (ch.r.s.x as f32) / (font.proto.scale as f32);
                    let sy = FONT.y * font.size.y * (ch.r.s.y as f32) / (font.proto.scale as f32);

                    // texture divisor
                    let tdx = 1.0 / (font.proto.texture.size.x as f32);
                    let tdy = 1.0 / (font.proto.texture.size.y as f32);

                    // texture coordinates
                    let tox = tdx * (ch.r.o.x as f32);
                    let toy = tdy * (ch.r.o.y as f32);
                    let tsx = tdx * (ch.r.s.x as f32);
                    let tsy = tdy * (ch.r.s.y as f32);

                    // add quad
                    vertices.push(vec4!(ox,oy,tox,toy));
                    vertices.push(vec4!(ox + sx,oy,tox + tsx,toy));
                    vertices.push(vec4!(ox + sx,oy + sy,tox + tsx,toy + tsy));
                    vertices.push(vec4!(ox,oy,tox,toy));
                    vertices.push(vec4!(ox + sx,oy + sy,tox + tsx,toy + tsy));
                    vertices.push(vec4!(ox,oy + sy,tox,toy + tsy));

                    // advance
                    lp.x += FONT.x * font.size.x * (ch.advance as f32) / (font.proto.scale as f32) + FONT.x * font.size.x * font.spacing;

                    count += 1;
                }
                else {
                    // only advance
                    lp.x += 2.0 * FONT.x * font.size.x * (ch.advance as f32) / (font.proto.scale as f32) + FONT.x * font.size.x * font.spacing;  // the choice for double spacing is arbitrary
                }
            }
        }

        let vertexbuffer = self.ui.system.create_vertexbuffer(vertices).expect("what?");
        self.ui.system.bind_vertexbuffer(&vertexbuffer);
        self.ui.system.bind_shader(&self.ui.msdf_shader);
        self.ui.system.bind_texture2d(0,&font.proto.texture);
        self.ui.system.set_uniform("ppu",self.ppu.get());
        self.ui.system.set_uniform("size",self.size.get());
        self.ui.system.set_uniform("font_texture",0);
        self.ui.system.set_uniform("color",self.color.get());
        self.ui.system.draw_triangles(6 * count);
        self.ui.system.unbind_vertexbuffer();
        self.ui.system.unbind_shader();
        self.ui.system.unbind_texture2d(0);
    }
}
