// E - UI - GC
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

/// UI drawing context.
pub struct DC {
    pub(crate) ui: Rc<ui::UI>,
    font: RefCell<Rc<ui::Font>>,
    color: Cell<Vec4<f32>>,
    ppu: Cell<Vec2<f32>>,  // pixels per unit
    size: Cell<Vec2<f32>>,  // viewport size (in units, not pixels)
}

const SCREEN: Vec2<f32> = Vec2 { x: 1.0,y: 1.0, };

impl DC {

    /// Create new drawing context.
    /// ## Arguments
    /// * `ui` - UI context to create this DC for.
    /// ## Returns
    /// * `Ok(DC)` - The new drawing context.
    /// * `Err(SystemError)` - The drawing context could not be created.
    pub fn new(ui: &Rc<ui::UI>) -> Result<ui::DC,SystemError> {
        Ok(DC {
            ui: Rc::clone(ui),
            font: RefCell::new(ui.get_font("arialn.fnt",vec2!(14.0,14.0),0.0).expect("cannot load font")),
            color: Cell::new(vec4!(1.0,1.0,1.0,1.0)),
            ppu: Cell::new(SCREEN),
            size: Cell::new(vec2!(1.0,1.0)),
        })
    }

    /// (temporary) Set DC window size.
    /// ## Arguments
    /// * `size` - New window size to use.
    pub fn set_size(&self,size: Vec2<f32>) {
        self.size.set(size);
    }

    /// (temporary) Set pixels per unit.
    /// 
    /// The UI uses "UI units" to define/align all widgets. The DC's PPU value
    /// indicates how many pixels fit inside one unit square.
    /// ## Arguments
    /// * `ppu` - New PPU specification.
    pub fn set_ppu(&self,ppu: Vec2<f32>) {
        self.ppu.set(ppu);
    }

    /// (temporary) Set current drawing color.
    /// ## Arguments
    /// * `color` - New color.
    pub fn set_color<T>(&self,color: T) where Vec4<f32>: From<T> {
        self.color.set(Vec4::<f32>::from(color));
    }

    /// (temporary) Set current drawing font.
    /// ## Arguments
    /// * `font` - New font.
    pub fn set_font(&self,font: Rc<ui::Font>) {
        *(self.font.borrow_mut()) = font;
    }

    /// (temporary) Draw text.
    /// 
    /// Draws the indicated text from the current font in the current color.
    /// ## Arguments
    /// * `p` - Coordinates of the start of the text baseline.
    /// * `text` - Text to draw.
    pub fn draw_text(&self,p: Vec2<f32>,text: &str) {
        let mut vertices: Vec<Vec4<f32>> = Vec::new();
        let mut lp = vec2!(p.x as f32,p.y as f32);
        let mut count = 0;
        let font = self.font.borrow();
        for c in text.chars() {
            if let Some(ch) = font.proto.find(c) {
                if (ch.r.s.x > 0) && (ch.r.s.y > 0) {
                    // size of the character, in GU
                    let sx = ui::FONT.x * font.size.x * (ch.r.s.x as f32) / (font.proto.scale as f32);
                    let sy = ui::FONT.y * font.size.y * (ch.r.s.y as f32) / (font.proto.scale as f32);

                    // bottom-left of the character, in GU
                    let ox = lp.x - ui::FONT.x * font.size.x * (ch.offset.x as f32) / (font.proto.scale as f32);
                    let oy = lp.y - sy + ui::FONT.y * font.size.y * (ch.offset.y as f32) / (font.proto.scale as f32);

                    // texture divisor
                    let tdx = 1.0 / (font.proto.texture.size.x as f32);
                    let tdy = 1.0 / (font.proto.texture.size.y as f32);

                    // texture coordinates
                    let tox = tdx * (ch.r.o.x as f32);
                    let toy = tdy * (ch.r.o.y as f32);
                    let tsx = tdx * (ch.r.s.x as f32);
                    let tsy = tdy * (ch.r.s.y as f32);

                    // add quad
                    vertices.push(vec4!(ox,oy + sy,tox,toy));
                    vertices.push(vec4!(ox + sx,oy + sy,tox + tsx,toy));
                    vertices.push(vec4!(ox + sx,oy,tox + tsx,toy + tsy));
                    vertices.push(vec4!(ox,oy + sy,tox,toy));
                    vertices.push(vec4!(ox + sx,oy,tox + tsx,toy + tsy));
                    vertices.push(vec4!(ox,oy,tox,toy + tsy));

                    count += 1;
                }
                lp.x += ui::FONT.x * font.size.x * (ch.advance as f32) / (font.proto.scale as f32) + ui::FONT.x * font.size.x * font.spacing;
            }
        }

        let vertexbuffer = gpu::VertexBuffer::new(&self.ui.graphics,vertices).expect("what?");
        self.ui.graphics.set_blend(gpu::BlendMode::Over);
        self.ui.graphics.bind_vertexbuffer(&vertexbuffer);
        self.ui.graphics.bind_shader(&self.ui.msdf_shader);
        self.ui.graphics.bind_texture(0,&font.proto.texture);
        self.ui.graphics.set_uniform("ppu",self.ppu.get());
        let canvas_size = self.size.get();
        self.ui.graphics.set_uniform("canvas_size",vec2!(canvas_size.x as f32,canvas_size.y as f32));
        self.ui.graphics.set_uniform("font_texture",0);
        self.ui.graphics.set_uniform("color",self.color.get());
        self.ui.graphics.set_uniform("sample_rad",0.5 / (font.proto.texture.size.x as f32));
        self.ui.graphics.draw_triangles(6 * count);
    }

    /// (temporary) Draw from texture.
    /// 
    /// Draws from the indicated texture.
    /// ## Arguments
    /// * `p` - Coordinates of the start of the text baseline.
    /// * `texture` - Texture to draw.
    pub fn draw(&self,p: Vec2<f32>,texture: &gpu::Texture2D::<pixel::ARGB8>) {
        self.ui.graphics.set_blend(gpu::BlendMode::Replace);
        self.ui.graphics.bind_vertexbuffer(&self.ui.quad_vertexbuffer);
        self.ui.graphics.bind_shader(&self.ui.texture_shader);
        self.ui.graphics.bind_texture(0,texture);
        self.ui.graphics.set_uniform("image_texture",0);
        let ppu = self.ppu.get();
        self.ui.graphics.set_uniform("ppu",ppu);
        let canvas_size = self.size.get();
        self.ui.graphics.set_uniform("canvas_size",vec2!(canvas_size.x as f32,canvas_size.y as f32));
        let src_size = vec2!(texture.size.x as f32,texture.size.y as f32);
        self.ui.graphics.set_uniform("src_size",src_size);
        let src = vec4!(0.0,0.0,texture.size.x as f32,texture.size.y as f32);
        self.ui.graphics.set_uniform("src",src);
        let dst = vec4!(p.x as f32,p.y as f32,texture.size.x as f32,texture.size.y as f32);
        self.ui.graphics.set_uniform("dst",dst);
        self.ui.graphics.draw_triangle_fan(4);
    }
}
