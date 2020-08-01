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
    size: Cell<Vec2<f32>>,  // viewport size (in units, not pixels)
}

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
            font: RefCell::new(ui.get_font("arialn14.fnt").expect("cannot load font")),
            color: Cell::new(vec4!(1.0,1.0,1.0,1.0)),
            size: Cell::new(vec2!(1.0,1.0)),
        })
    }

    /// (temporary) Set DC window size.
    /// ## Arguments
    /// * `size` - New window size to use.
    pub fn set_size(&self,size: Vec2<f32>) {
        self.size.set(size);
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
    pub fn draw_text(&self,p: Vec2<i32>,text: &str) {
        //let mut vertices: Vec<Vec4<f32>> = Vec::new();
        let mut vertices: Vec<ui::Vertex> = Vec::new();
        let mut count = 0;
        let font = self.font.borrow();
        let color = self.color.get();
        let mut v = vec2!(p.x,p.y + font.y_bearing as i32);
        for c in text.chars() {
            if let Some(ch) = font.find(c) {
                let r = rect!(
                    v.x + ch.bearing.x as i32,
                    v.y - ch.bearing.y as i32,
                    ch.r.s.x as i32,
                    ch.r.s.y as i32
                );
                let tr = rect!(
                    (ch.r.o.x as f32) / (font.texture.size.x as f32),
                    (ch.r.o.y as f32) / (font.texture.size.y as f32),
                    (ch.r.s.x as f32) / (font.texture.size.x as f32),
                    (ch.r.s.y as f32) / (font.texture.size.y as f32)
                );
                let a = ui::Vertex {
                    pt: vec4!(r.o.x as f32,r.o.y as f32,tr.o.x,tr.o.y),
                    a: vec4!(0.0,0.0,0.0,0.0),
                    b: color,
                    mlfq: vec4!(1,0,0,0),
                };
                let b = ui::Vertex {
                    pt: vec4!(r.o.x as f32 + r.s.x as f32,r.o.y as f32,tr.o.x + tr.s.x,tr.o.y),
                    a: vec4!(0.0,0.0,0.0,0.0),
                    b: color,
                    mlfq: vec4!(1,0,0,0),
                };
                let c = ui::Vertex {
                    pt: vec4!(r.o.x as f32 + r.s.x as f32,r.o.y as f32 + r.s.y as f32,tr.o.x + tr.s.x,tr.o.y + tr.s.y),
                    a: vec4!(0.0,0.0,0.0,0.0),
                    b: color,
                    mlfq: vec4!(1,0,0,0),
                };
                let d = ui::Vertex {
                    pt: vec4!(r.o.x as f32,r.o.y as f32 + r.s.y as f32,tr.o.x,tr.o.y + tr.s.y),
                    a: vec4!(0.0,0.0,0.0,0.0),
                    b: color,
                    mlfq: vec4!(1,0,0,0),
                };
                vertices.push(a);
                vertices.push(b);
                vertices.push(c);
                vertices.push(a);
                vertices.push(c);
                vertices.push(d);
                count += 1;
                v.x += ch.advance as i32;
            }
        }

        let vertexbuffer = gpu::VertexBuffer::new(&self.ui.graphics,vertices).expect("what?");
        self.ui.graphics.bind_vertexbuffer(&vertexbuffer);
        self.ui.graphics.bind_shader(&self.ui.uber_shader);
        self.ui.graphics.bind_texture(0,&font.texture);
        self.ui.graphics.set_uniform("textures",0);
        let canvas_size = self.size.get();
        self.ui.graphics.set_uniform("canvas_size",vec2!(canvas_size.x as f32,canvas_size.y as f32));
        self.ui.graphics.draw_triangles(6 * count);
    }

    /// (temporary) Draw from texture.
    /// 
    /// Draws from the indicated texture.
    /// ## Arguments
    /// * `p` - Coordinates of the start of the text baseline.
    /// * `texture` - Texture to draw.
    pub fn draw_texture(&self,p: Vec2<i32>,texture: &gpu::Texture2D::<pixel::ARGB8>) {
        let mut vertices: Vec<ui::Vertex> = Vec::new();
        let a = ui::Vertex {
            pt: vec4!(p.x as f32,p.y as f32,0.0,0.0),
            a: vec4!(0.0,0.0,0.0,0.0),
            b: vec4!(1.0,1.0,1.0,1.0),
            mlfq: vec4!(1,0,0,0),
        };
        let b = ui::Vertex {
            pt: vec4!((p.x + texture.size.x as i32) as f32,p.y as f32,1.0,0.0),
            a: vec4!(0.0,0.0,0.0,0.0),
            b: vec4!(1.0,1.0,1.0,1.0),
            mlfq: vec4!(1,0,0,0),
        };
        let c = ui::Vertex {
            pt: vec4!((p.x + texture.size.x as i32) as f32,(p.y + texture.size.y as i32) as f32,1.0,1.0),
            a: vec4!(0.0,0.0,0.0,0.0),
            b: vec4!(1.0,1.0,1.0,1.0),
            mlfq: vec4!(1,0,0,0),
        };
        let d = ui::Vertex {
            pt: vec4!(p.x as f32,(p.y + texture.size.y as i32) as f32,0.0,1.0),
            a: vec4!(0.0,0.0,0.0,0.0),
            b: vec4!(1.0,1.0,1.0,1.0),
            mlfq: vec4!(1,0,0,0),
        };
        vertices.push(a);
        vertices.push(b);
        vertices.push(c);
        vertices.push(a);
        vertices.push(c);
        vertices.push(d);

        let vertexbuffer = gpu::VertexBuffer::new(&self.ui.graphics,vertices).expect("what?");
        self.ui.graphics.bind_vertexbuffer(&vertexbuffer);
        self.ui.graphics.bind_shader(&self.ui.uber_shader);
        self.ui.graphics.bind_texture(0,texture);
        self.ui.graphics.set_uniform("textures",0);
        let canvas_size = self.size.get();
        self.ui.graphics.set_uniform("canvas_size",vec2!(canvas_size.x as f32,canvas_size.y as f32));
        self.ui.graphics.draw_triangles(6);
    }

    /// (temporary) Draw rectangle.
    /// 
    /// Draws rectangle.
    /// ## Arguments
    /// * `r` - Rectangle to draw.
    pub fn draw_rect(&self,r: Rect<i32>) {
        let mut vertices: Vec<ui::Vertex> = Vec::new();
        let a = ui::Vertex {
            pt: vec4!(r.o.x as f32,r.o.y as f32,0.0,0.0),
            a: vec4!(0.0,0.0,0.0,0.0),
            b: vec4!(1.0,1.0,1.0,1.0),
            mlfq: vec4!(1,0,0,0),
        };
        let b = ui::Vertex {
            pt: vec4!((r.o.x + r.s.x) as f32,r.o.y as f32,0.0,0.0),
            a: vec4!(0.0,0.0,0.0,0.0),
            b: vec4!(1.0,1.0,1.0,1.0),
            mlfq: vec4!(1,0,0,0),
        };
        let c = ui::Vertex {
            pt: vec4!((r.o.x + r.s.x) as f32,(r.o.y + r.s.y) as f32,0.0,0.0),
            a: vec4!(0.0,0.0,0.0,0.0),
            b: vec4!(1.0,1.0,1.0,1.0),
            mlfq: vec4!(1,0,0,0),
        };
        let d = ui::Vertex {
            pt: vec4!(r.o.x as f32,(r.o.y + r.s.y) as f32,0.0,0.0),
            a: vec4!(0.0,0.0,0.0,0.0),
            b: vec4!(1.0,1.0,1.0,1.0),
            mlfq: vec4!(1,0,0,0),
        };
        vertices.push(a);
        vertices.push(b);
        vertices.push(c);
        vertices.push(a);
        vertices.push(c);
        vertices.push(d);

        let vertexbuffer = gpu::VertexBuffer::new(&self.ui.graphics,vertices).expect("what?");
        self.ui.graphics.bind_vertexbuffer(&vertexbuffer);
        self.ui.graphics.bind_shader(&self.ui.uber_shader);
        //self.ui.graphics.bind_texture(0,texture);
        self.ui.graphics.set_uniform("textures",0);
        let canvas_size = self.size.get();
        self.ui.graphics.set_uniform("canvas_size",vec2!(canvas_size.x as f32,canvas_size.y as f32));
        self.ui.graphics.draw_triangles(6);
    }
}
