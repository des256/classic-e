// e::engine::Layer
// by Desmond Germans, 2019

use canvas::{Canvas,Framebuffer};

/// graphics layer.
pub struct Layer<'a> {
    pub canvas: &'a Canvas,
    pub x0: f32,
    pub y0: f32,
    pub xs: f32,
    pub ys: f32,
    pub framebuffer: Framebuffer,
}

impl<'a> Layer<'a> {
    pub fn new(canvas: &'a Canvas, width: u32,height: u32,x0: f32,y0: f32,xs: f32,ys: f32) -> Layer<'a> {
        let framebuffer = Framebuffer::new(width,height);
        Layer {
            canvas: canvas,
            x0: x0,
            y0: y0,
            xs: xs,
            ys: ys,
            framebuffer: framebuffer,
        }
    }

    pub fn bind(&self) {
        self.framebuffer.bind();
    }

    /*pub fn clear(&mut self,r: f32,g: f32,b: f32,a: f32,d: f32) {
        unsafe {
            gl::ClearColor(r,g,b,a);
            gl::ClearDepth(d as f64);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }*/

   /*pub fn draw_text(&self,x: i32,y: i32,text: &str,font: &Font,sz: i32) {

        let mut chars: [f32; 4096] = [0.0; 4096];

        // font size multiplier
        let q = (sz as f32) / (font.base as f32);

        // character scale
        let csx = 1.0 / (self.framebuffer.width as f32);
        let csy = 1.0 / (self.framebuffer.height as f32);

        // font scale
        let fsx = 1.0 / (font.scale_w as f32);
        let fsy = 1.0 / (font.scale_h as f32);

        let mut curx = x as f32;

        let mut n = 0;
        let mut prev_ch: Option<&Char> = None;
        
        for c in text.chars() {

            let ch = match font.chars.iter().find(|&x| x.id == c as i32) {
                Some(ch) => { ch },
                None => { continue; },
            };

            // base point
            let bx = curx + q * (ch.xoffset as f32);
            let by = (y as f32) + q * (ch.yoffset as f32);

            let p0x = (bx as f32) * csx;
            let p0y = (by as f32) * csy;
            let psx = q * (ch.width as f32) * csx;
            let psy = q * (ch.height as f32) * csy;
            let t0x = (ch.x as f32) * fsx;
            let t0y = (ch.y as f32) * fsy;
            let tsx = (ch.width as f32) * fsx;
            let tsy = (ch.height as f32) * fsy;

            chars[n * 8] = p0x;
            chars[n * 8 + 1] = p0y;
            chars[n * 8 + 2] = psx;
            chars[n * 8 + 3] = psy;
            chars[n * 8 + 4] = t0x;
            chars[n * 8 + 5] = t0y;
            chars[n * 8 + 6] = tsx;
            chars[n * 8 + 7] = tsy;

            curx += q * (ch.xadvance as f32);

            match prev_ch {
                Some(pch) => {
                    match font.kernings.iter().find(|&x| (x.a == pch.id) && (x.b == ch.id)) {
                        Some(k) => { curx += q * (k.n as f32); },
                        None => { },
                    };
                },
                None => { },
            }

            prev_ch = Some(ch);
            
            n += 1;
        }

        unsafe {
            gl::UseProgram(self.video.text_shader.sp);
            gl::Uniform1i(uniform_location(self.video.text_shader.sp,"u_texture"),0);
            gl::Uniform4fv(uniform_location(self.video.text_shader.sp,"u_chars"),(n * 8) as i32,&chars[0] as *const f32);
            gl::Uniform4f(uniform_location(self.video.text_shader.sp,"u_color"),1.0,1.0,0.0,1.0);
            gl::BindTexture(gl::TEXTURE_2D,font.tex);
            gl::BindBuffer(gl::ARRAY_BUFFER,self.video.quad_vbo);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA,gl::ONE_MINUS_SRC_ALPHA);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0,2,gl::FLOAT,gl::FALSE,0,0 as *const c_void);
            gl::DrawArraysInstanced(gl::TRIANGLE_FAN,0,4,n as i32);
            gl::DisableVertexAttribArray(0);
            gl::Disable(gl::BLEND);
        }
    }*/

    /*pub fn draw_map(&self,map: &Map,x: f32,y: f32) {
        unsafe {
            gl::UseProgram(self.video.map_shader.sp);
            gl::Uniform1i(uniform_location(self.video.map_shader.sp,"u_texture"),0);
            gl::Uniform2f(uniform_location(self.video.map_shader.sp,"u_origin"),x,y);
            gl::BindTexture(gl::TEXTURE_2D,map.tex);
            gl::BindBuffer(gl::ARRAY_BUFFER,self.video.quad_vbo);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA,gl::ONE_MINUS_SRC_ALPHA);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0,2,gl::FLOAT,gl::FALSE,0,0 as *const c_void);
            gl::DrawArrays(gl::TRIANGLE_FAN,0,4);
            gl::DisableVertexAttribArray(0);
            gl::Disable(gl::BLEND);
        }
    }*/
}
