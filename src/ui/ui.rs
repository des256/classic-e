// E - UI
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::RefCell,
};
use gl::types::{
    GLuint,
    GLvoid,
};

enum UIDelta {
    Skip,
    Draw,
}

pub struct UIWindow {
    window: Rc<Window>,                           // window
    widget: Rc<dyn ui::Widget>,                   // widget in this window
    delta: UIDelta,                               // what to do in order to validate
}

// TODO: the more tightly packed UIRect will come later
#[derive(Copy,Clone)]
#[repr(C)]
pub struct UIRect {
    pub(crate) r: Vec4<f32>,  // x, y, w, h
    pub(crate) t: Vec4<f32>,  // x, y, w, h
}

// TODO: this means that specifically only OpenGL is supported for now; solve this later
impl gpu::GLVertex for UIRect {
    fn bind() -> Vec<GLuint> {
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0,4,gl::FLOAT,gl::FALSE,32,0 as *const GLvoid);
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1,4,gl::FLOAT,gl::FALSE,32,16 as *const GLvoid);
        }
        vec![0,1]
    }

    fn len() -> isize {
        32
    }
}

/// UI subsystem.
pub struct UI {
    pub system: Rc<System>,
    pub graphics: Rc<gpu::Graphics>,
    pub flat_shader: gpu::Shader,
    pub alpha_shader: gpu::Shader,
    pub color_shader: gpu::Shader,
    pub proto_sans: Rc<ui::FontProto>,
    pub proto_serif: Rc<ui::FontProto>,
    pub proto_mono: Rc<ui::FontProto>,
    pub font: Rc<ui::Font>,
    pub windows: RefCell<Vec<UIWindow>>,
}

impl UI {
    /// Create new UI context.
    /// ## Arguments
    /// * `system` - System to create the UI context for.
    /// * `graphics` - GPU graphics context to use.
    pub fn new(system: &Rc<System>,graphics: &Rc<gpu::Graphics>,font_dir: &str) -> Result<UI,SystemError> {

        // generic vertex shader
        let vs = r#"
            #version 420 core
            
            layout(location = 0) in vec4 ir;
            layout(location = 1) in vec4 it;
            
            out Rect {
                vec4 t;
            } vs_out;
            
            void main() {
                gl_Position = ir;
                vs_out.t = it;
            }
        "#;

        // generic geometry shader
        let gs = r#"
            #version 420 core

            uniform vec2 canvas_size;
            
            layout(points) in;
            layout(triangle_strip,max_vertices = 4) out;

            in Rect {
                vec4 t;
            } gs_in[];

            out Vertex {
                vec2 t;
            } gs_out;

            void main() {

                vec4 r = gl_in[0].gl_Position;
                vec4 t = gs_in[0].t;

                vec4 pn = vec4(
                    -1.0 + 2.0 * r.x / canvas_size.x,
                    1.0 - 2.0 * r.y / canvas_size.y,
                    2.0 * r.z / canvas_size.x,
                    -2.0 * r.w / canvas_size.y
                );

                gl_Position = vec4(pn.x,pn.y,0.0,1.0);
                gs_out.t = vec2(t.x,t.y);
                EmitVertex();

                gl_Position = vec4(pn.x + pn.z,pn.y,0.0,1.0);
                gs_out.t = vec2(t.x + t.z,t.y);
                EmitVertex();

                gl_Position = vec4(pn.x,pn.y + pn.w,0.0,1.0);
                gs_out.t = vec2(t.x,t.y + t.w);
                EmitVertex();

                gl_Position = vec4(pn.x + pn.z,pn.y + pn.w,0.0,1.0);
                gs_out.t = vec2(t.x + t.z,t.y + t.w);
                EmitVertex();

                EndPrimitive();
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

        let flat_shader = gpu::Shader::new(&graphics,vs,Some(gs),flat_fs).expect("Unable to create flat shader.");
        let alpha_shader = gpu::Shader::new(&graphics,vs,Some(gs),alpha_fs).expect("Unable to create alpha shader.");
        let color_shader = gpu::Shader::new(&graphics,vs,Some(gs),color_fs).expect("Unable to create color shader.");

        let proto_sans = Rc::new(
            ui::FontProto::new(
                &graphics,
                &format!("{}/sans.fnt",font_dir)
            ).expect("Unable to load font")
        );
        let proto_serif = Rc::new(
            ui::FontProto::new(
                &graphics,
                &format!("{}/serif.fnt",font_dir)
            ).expect("Unable to load font")
        );
        let proto_mono = Rc::new(
            ui::FontProto::new(
                &graphics,
                &format!("{}/mono.fnt",font_dir)
            ).expect("Unable to load font")
        );

        // create default font
        let font = Rc::new(ui::Font::new(&proto_sans,16).expect("unable to load font"));

        Ok(UI {
            system: Rc::clone(system),
            graphics: Rc::clone(graphics),
            flat_shader: flat_shader,
            alpha_shader: alpha_shader,
            color_shader: color_shader,
            proto_sans: proto_sans,
            proto_serif: proto_serif,
            proto_mono: proto_mono,
            font: font,
            windows: RefCell::new(Vec::new()),
        })
    }

    /// Open new window.
    /// ## Arguments
    /// * `widget` - Widget to host.
    /// * `r` - Initial screen rectangle.
    /// * `title` - Window title.
    /// ## Returns
    /// * `false` - Window could not be created.
    /// * `true` - Window was created.
    pub fn open(&self,widget: &Rc<dyn ui::Widget>,r: Rect<isize>,title: &str) -> bool {
        let window = Rc::new(Window::new(&self.system,r,title).expect("unable to create window"));
        self.graphics.bind_target(&window);
        unsafe { (self.system.glx_swap_interval)(self.system.connection.get_raw_dpy(),window.id,0) };
        self.windows.borrow_mut().push(UIWindow {
            window: window,
            widget: Rc::clone(widget),
            delta: UIDelta::Draw,
        });
        true
    }

    /// Close window.
    /// ## Arguments
    /// * `widget` - Widget that is hosted there.
    pub fn close(&self,widget: &Rc<dyn ui::Widget>) {
        let len = self.windows.borrow().len();
        for i in 0..len {
            if Rc::ptr_eq(widget,&self.windows.borrow()[i].widget) {
                self.windows.borrow_mut().remove(i);
                break;
            }
        }
    }

    /// Start drawing session.
    pub fn begin_drawing(&self) -> Vec<ui::UIRect> {
        Vec::new()
    }

    /// (temporary) Draw rectangle.
    /// ## Arguments
    /// * `canvas_size` - Size of target canvas.
    /// * `r` - Rectangle to draw.
    /// * `color` - Color to draw rectangle in.
    /// * `blend_mode` - Blend mode for the rectangle.
    pub fn draw_rectangle<C: ColorParameter>(&self,canvas_size: Vec2<i32>,r: Rect<i32>,color: C,blend_mode: gpu::BlendMode) {
        let mut buffer: Vec<UIRect> = Vec::new();
        buffer.push(UIRect {
            r: vec4!(r.o.x as f32,r.o.y as f32,r.s.x as f32,r.s.y as f32),
            t: vec4!(0.0,0.0,0.0,0.0),
        });
        let vertexbuffer = gpu::VertexBuffer::<ui::UIRect>::new_from_vec(&self.graphics,buffer).expect("Unable to create vertexbuffer.");
        self.graphics.set_blend(blend_mode);
        self.graphics.bind_shader(&self.flat_shader);
        self.graphics.bind_vertexbuffer(&vertexbuffer);
        self.graphics.set_uniform("canvas_size",vec2!(canvas_size.x as f32,canvas_size.y as f32));
        self.graphics.set_uniform("color",color.into_vec4());
        self.graphics.draw_points(1);
    }

    /// Draw image.
    /// ## Arguments
    /// * `canvas_size` - Size of target canvas.
    /// * `r` - Rectangle to draw image in.
    /// * `texture` - Image to draw.
    /// * `color` - Color to multiply with the image.
    /// * `blend_mode` - Blend mode for the rectangle.
    pub fn draw_image<C: ColorParameter,T: gpu::GLFormat>(&self,canvas_size: Vec2<i32>,r: Rect<i32>,texture: &gpu::Texture2D<T>,color: C,blend_mode: gpu::BlendMode) {
        let mut buffer: Vec<UIRect> = Vec::new();
        buffer.push(UIRect {
            r: vec4!(r.o.x as f32,r.o.y as f32,r.s.x as f32,r.s.y as f32),
            t: vec4!(0.0,0.0,1.0,1.0),
        });
        let vertexbuffer = gpu::VertexBuffer::<ui::UIRect>::new_from_vec(&self.graphics,buffer).expect("Unable to create vertexbuffer.");
        self.graphics.set_blend(blend_mode);
        self.graphics.bind_shader(&self.flat_shader);
        self.graphics.bind_vertexbuffer(&vertexbuffer);
        self.graphics.bind_texture(0,texture);
        self.graphics.set_uniform("canvas_size",vec2!(canvas_size.x as f32,canvas_size.y as f32));
        self.graphics.set_uniform("color_texture",0);
        self.graphics.set_uniform("color",color.into_vec4());
        self.graphics.draw_points(1);
    }

    /// Draw text.
    /// ## Arguments
    /// * `canvas_size` - Size of target canvas.
    /// * `p` - Position to draw the text at.
    /// * `text` - Text to draw.
    /// * `color` - Color to multiply with the image.
    /// * `font` - font to use.
    pub fn draw_text<C: ColorParameter>(&self,canvas_size: Vec2<i32>,p: Vec2<i32>,text: &str,color: C,font: &ui::Font) {
        let mut buffer: Vec<UIRect> = Vec::new();
        for s in font.proto.sets.iter() {
            if s.font_size == font.font_size {
                let mut v = vec2!(p.x,p.y + (font.ratio * (s.y_bearing as f32)) as i32);
                for c in text.chars() {
                    let code = c as u32;
                    for ch in s.characters.iter() {
                        if ch.n == code {
                            buffer.push(ui::UIRect {
                                r: vec4!(
                                    (v.x + (font.ratio * (ch.bearing.x as f32)) as i32) as f32,
                                    (v.y - (font.ratio * (ch.bearing.y as f32)) as i32) as f32,
                                    ((font.ratio * (ch.r.s.x as f32)) as i32) as f32,
                                    ((font.ratio * (ch.r.s.y as f32)) as i32) as f32
                                ),
                                t: vec4!(
                                    (ch.r.o.x as f32) / (font.proto.texture.size.x as f32),
                                    (ch.r.o.y as f32) / (font.proto.texture.size.y as f32),
                                    (ch.r.s.x as f32) / (font.proto.texture.size.x as f32),
                                    (ch.r.s.y as f32) / (font.proto.texture.size.y as f32)
                                ),
                            });
                            v.x += (font.ratio * (ch.advance as f32)) as i32;
                            break;
                        }
                    }
                }
                break;
            }
        }
        let points = buffer.len();
        let vertexbuffer = gpu::VertexBuffer::<ui::UIRect>::new_from_vec(&self.graphics,buffer).expect("Unable to create vertexbuffer.");
        self.graphics.set_blend(gpu::BlendMode::Over);
        self.graphics.bind_shader(&self.alpha_shader);
        self.graphics.bind_vertexbuffer(&vertexbuffer);
        self.graphics.bind_texture(0,&font.proto.texture);
        self.graphics.set_uniform("canvas_size",vec2!(canvas_size.x as f32,canvas_size.y as f32));
        self.graphics.set_uniform("alpha_texture",0);
        self.graphics.set_uniform("color",color.into_vec4());
        self.graphics.draw_points(points as i32);
    }

    /// Run UI.
    /// 
    /// This runs the event loop and rebuilds and redraws the windows if needed.
    pub fn run(&self) {

        // keep on running
        let mut running = true;

        while running {

            // create list of windows for System::poll
            let mut winlist: Vec<Rc<Window>> = Vec::new();
            for w in self.windows.borrow().iter() {
                winlist.push(Rc::clone(&w.window));
            }

            // wait for an event to appear
            self.system.wait();

            // handle all captured events
            for winev in self.system.poll(&winlist) {

                let win = winev.0;
                let event = winev.1;
                let len = self.windows.borrow().len();
                for i in 0..len {
                    // borrow window for this event
                    let window = &mut self.windows.borrow_mut()[i];
                    if Rc::ptr_eq(&win,&window.window) {

                        match event {

                            // system wants to render this window
                            Event::Render => {
                                window.delta = UIDelta::Draw;
                            },

                            // system notifies that this window changed size
                            Event::Resize(s) => {
                                window.window.size.set(vec2!(s.x as usize,s.y as usize));
                                window.delta = UIDelta::Draw;
                            },

                            // user wants to close this window
                            Event::Close => {
                                running = false;  // TODO: closing other windows doesn't automatically mean end program; API user might want to ask something before actually quitting
                                window.delta = UIDelta::Skip;
                            },

                            // anything else should be handled by the hosted widget
                            _ => {
                                // handle the event
                                let window_size = window.window.size.get();
                                window.widget.handle(&event,rect!(0i32,0i32,window_size.x as i32,window_size.y as i32));
                                window.delta = UIDelta::Draw;  // always redraw for now
                            },
                        }        
                        break;
                    }
                }
            }
            for window in self.windows.borrow_mut().iter_mut() {

                // bind to this window
                self.graphics.bind_target(&window.window);

                // if widget tree needs to be rebuilt, rebuild it
                if let UIDelta::Draw = window.delta {
                    self.graphics.clear(0xFF001122);
                    let window_size = window.window.size.get();
                    window.widget.draw(vec2!(window_size.x as i32,window_size.y as i32),rect!(0i32,0i32,window_size.x as i32,window_size.y as i32));
                    self.graphics.flush();
                }
            }

            // swap buffers for all rendered windows
            for window in self.windows.borrow_mut().iter_mut() {

                // if redraw happened, present
                if let UIDelta::Draw = window.delta {
                    gpu::present(&self.system,&window.window);
                }
                
                window.delta = UIDelta::Skip;
            }
        }
    }
}
