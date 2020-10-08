// E - UI - Draw
// Desmond Germans, 2020

use {
    crate::*,
    std::{
        cell::{
            Cell,
            RefCell,
        },
        rc::Rc,
    },
};

/// Draw context.
pub struct Draw {
    pub graphics: Rc<Graphics>,
    pub flat_shader: Shader,  // the shaders
    pub alpha_shader: Shader,
    pub color_shader: Shader,
    pub rect_vb: VertexBuffer<Vec2<f32>>,  // vertexbuffer containing fixed unit rectangle
    pub styles: RefCell<Styles>,  // fonts, colors, paddings, spacings, etc. for the style of the UI
    pub draw_ub: UniformBuffer<TexRect>,  // uniform buffer with actual rectangle specfications
    pub two_over_window_size: Cell<Vec2<f32>>,  // 2/w,2/h of the current window
    pub offset: Cell<Vec2<i32>>,  // drawing offset (TBD)
}

impl Draw {
    pub fn new(graphics: &Rc<Graphics>,font_path: &str) -> Result<Draw,SystemError> {

        // generic vertex shader
        let vs = r#"
            #version 420 core
            
            layout(location = 0) in vec2 i_p;
            
            struct TexRect {
                vec4 r;
                vec4 t;
            };

            layout(std140) uniform rect_block {
                TexRect rects[256];
            };
            uniform vec2 tows;

            out Vertex {
                vec2 t;
            } vs_out;
            
            void main() {
                vec4 r = rects[gl_InstanceID].r;
                vec4 t = rects[gl_InstanceID].t;
                gl_Position = vec4(
                    -1.0 + tows.x * (r.x + i_p.x * r.z),
                    1.0 - tows.y * (r.y + i_p.y * r.w),
                    0.0,
                    1.0
                );
                vs_out.t = vec2(
                    t.x + i_p.x * t.z,
                    t.y + i_p.y * t.w
                );
            }
        "#;

        // flat fragment shader
        let flat_fs = r#"
            #version 420 core

            uniform vec4 color;

            in Vertex {
                vec2 t;
            } fs_in;

            out vec4 o;

            void main() {
                o = color;    
            }
        "#;

        // alpha texture (font/icons) fragment shader
        let alpha_fs = r#"
            #version 420 core
            
            uniform vec4 color;
            uniform sampler2D alpha_texture;
            
            in Vertex {
                vec2 t;
            } fs_in;
            
            out vec4 o;
            
            void main() {
                float t = texture(alpha_texture,fs_in.t).x;
                o = vec4(color.xyz,t);
                //o = vec4(1.0,1.0,1.0,1.0);
            }
        "#;

        // color texture (images) fragment shader
        let color_fs = r#"
            #version 420 core

            uniform vec4 color;
            uniform sampler2D color_texture;
            
            in Vertex {
                vec2 t;
            } fs_in;
            
            out vec4 o;
            
            void main() {
                vec4 t = texture(color_texture,fs_in.t);
                o = color * t;
            }
        "#;

        let flat_shader = graphics.create_shader(vs,None,flat_fs)?;
        let alpha_shader = graphics.create_shader(vs,None,alpha_fs)?;
        let color_shader = graphics.create_shader(vs,None,color_fs)?;

        // create vertex buffer for one rectangle
        let rect_vb = graphics.create_vertexbuffer_from_vec::<Vec2<f32>>(vec![
            vec2!(0.0,0.0),
            vec2!(1.0,0.0),
            vec2!(1.0,1.0),
            vec2!(0.0,1.0)
        ]).expect("unable to create vertex buffer");

        // create default styles
        let styles = Styles::new_default(graphics,font_path)?;

        // create draw uniform buffer
        let draw_ub = graphics.create_uniformbuffer::<TexRect>()?;

        Ok(Draw {
            graphics: Rc::clone(&graphics),
            flat_shader: flat_shader,
            alpha_shader: alpha_shader,
            color_shader: color_shader,
            rect_vb: rect_vb,
            styles: RefCell::new(styles),
            draw_ub: draw_ub,
            two_over_window_size: Cell::new(vec2!(0.0,0.0)),
            offset: Cell::new(vec2!(0,0)),
        })
    }

    pub fn set_window_size(&self,size: Vec2<i32>) {
        self.two_over_window_size.set(vec2!(2.0 / (size.x() as f32),2.0 / (size.y() as f32)));
    }

    pub fn reset_offset(&self) {
        self.offset.set(vec2!(0i32,0i32));
    }

    pub fn delta_offset(&self,o: Vec2<i32>) {
        let old_offset = self.offset.get();
        self.offset.set(old_offset + o);
    }

    /// Draw rectangle.
    pub fn draw_rectangle<C: ColorParameter>(&self,r: Rect<i32>,color: C,blend_mode: BlendMode) {
        let ofs = self.offset.get();
        self.draw_ub.load(0,&vec![TexRect {
            r: rect!(
                (r.ox() + ofs.x()) as f32,
                (r.oy() + ofs.y()) as f32,
                r.sx() as f32,
                r.sy() as f32
            ),
            t: rect!(0.0,0.0,0.0,0.0),
        }]);
        self.graphics.set_blend(blend_mode);
        self.graphics.bind_shader(&self.flat_shader);
        self.graphics.bind_vertexbuffer(&self.rect_vb);
        self.graphics.bind_uniformbuffer(1,"rect_block",&self.draw_ub);
        let tows = self.two_over_window_size.get();
        self.graphics.set_uniform("tows",tows);
        self.graphics.set_uniform("color",color.as_vec4());
        self.graphics.draw_instanced_triangle_fan(4,1);
    }

    /// Draw texture.
    pub fn draw_texture<T: GPUTextureFormat>(&self,p: Vec2<i32>,texture: &Texture2D<T>,blend_mode: BlendMode) {
        let ofs = self.offset.get();
        self.draw_ub.load(0,&vec![TexRect {
            r: rect!(
                (p.x() + ofs.x()) as f32,
                (p.y() + ofs.y()) as f32,
                texture.size().x() as f32,
                texture.size().y() as f32
            ),
            t: rect!(0.0,0.0,1.0,1.0),
        }]);
        self.graphics.set_blend(blend_mode);
        self.graphics.bind_shader(&self.color_shader);
        self.graphics.bind_vertexbuffer(&self.rect_vb);
        self.graphics.bind_uniformbuffer(1,"rect_block",&self.draw_ub);
        let tows = self.two_over_window_size.get();
        self.graphics.set_uniform("tows",tows);
        self.graphics.set_uniform("color",vec4!(1.0,1.0,1.0,1.0));
        self.graphics.bind_texture(0,texture);
        self.graphics.set_uniform("color_texture",0);
        self.graphics.draw_instanced_triangle_fan(4,1);
    }

    /// Draw text.
    pub fn draw_text<C: ColorParameter>(&self,p: Vec2<i32>,text: &str,color: C,font: &Font) {
        let ofs = self.offset.get();
        let mut buffer: Vec<TexRect> = Vec::new();
        for s in font.proto.sets.iter() {
            if s.font_size == font.font_size {
                let mut v = vec2!(p.x() + ofs.x(),p.y() + ofs.y() + (font.ratio * (s.y_bearing as f32)) as i32);
                for c in text.chars() {
                    let code = c as u32;
                    for ch in s.characters.iter() {
                        if ch.n == code {
                            buffer.push(TexRect {
                                r: rect!(
                                    (v.x() + (font.ratio * (ch.bearing.x() as f32)) as i32) as f32,
                                    (v.y() - (font.ratio * (ch.bearing.y() as f32)) as i32) as f32,
                                    ((font.ratio * (ch.r.sx() as f32)) as i32) as f32,
                                    ((font.ratio * (ch.r.sy() as f32)) as i32) as f32
                                ),
                                t: rect!(
                                    (ch.r.ox() as f32) / (font.proto.texture.size().x() as f32),
                                    (ch.r.oy() as f32) / (font.proto.texture.size().y() as f32),
                                    (ch.r.sx() as f32) / (font.proto.texture.size().x() as f32),
                                    (ch.r.sy() as f32) / (font.proto.texture.size().y() as f32)
                                ),
                            });
                            /*buffer.push(TexRect {
                                r: rect!(
                                    (v.x() + (font.ratio * (ch.bearing.x() as f32)) as i32) as f32,
                                    (v.y() - (font.ratio * (ch.bearing.y() as f32)) as i32) as f32,
                                    ((font.ratio * (ch.r.sx() as f32)) as i32) as f32,
                                    ((font.ratio * (ch.r.sy() as f32)) as i32) as f32
                                ),
                                t: rect!(
                                    (ch.r.ox() as f32) / (font.proto.texture.size().x() as f32),
                                    (ch.r.oy() as f32) / (font.proto.texture.size().y() as f32),
                                    (ch.r.sx() as f32) / (font.proto.texture.size().x() as f32),
                                    (ch.r.sy() as f32) / (font.proto.texture.size().y() as f32)
                                ),
                            });*/
                            v.set_x(v.x() + (font.ratio * (ch.advance as f32)) as i32);
                            break;
                        }
                    }
                }
                break;
            }
        }
        self.draw_ub.load(0,&buffer);
        self.graphics.set_blend(BlendMode::Over);
        self.graphics.bind_shader(&self.alpha_shader);
        self.graphics.bind_vertexbuffer(&self.rect_vb);
        self.graphics.bind_uniformbuffer(1,"rect_block",&self.draw_ub);
        self.graphics.bind_texture(0,&font.proto.texture);
        self.graphics.set_uniform("alpha_texture",0);
        let tows = self.two_over_window_size.get();
        self.graphics.set_uniform("tows",tows);
        self.graphics.set_uniform("color",color.as_vec4());
        self.graphics.draw_instanced_triangle_fan(4,buffer.len() as i32);
    }
}