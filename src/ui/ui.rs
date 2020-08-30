// E - UI
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::Cell,
    cell::RefCell,
};

// TODO: the more tightly packed UIRect will come later
#[derive(Copy,Clone)]
#[repr(C)]
pub struct UIRect {
    pub(crate) r: Vec4<f32>,  // rectangle: x, y, w, h
    pub(crate) t: Vec4<f32>,  // texture coordinates: x, y, w, h
}

// this is locks the system into OpenGL, fix later
impl gpu::GLUniform for UIRect {
    fn len() -> isize {
        32
    }
}

pub struct DrawContext {
    pub r: Rect<i32>,
}

pub struct UIAnchor {
    pub graphics: Rc<gpu::Graphics>,
    pub flat_shader: gpu::Shader,
    pub alpha_shader: gpu::Shader,
    pub color_shader: gpu::Shader,
    pub proto_sans: Rc<ui::FontProto>,
    pub proto_serif: Rc<ui::FontProto>,
    pub proto_mono: Rc<ui::FontProto>,
    pub font: Rc<ui::Font>,
    rect_vb: gpu::VertexBuffer<Vec2<f32>>,
    draw_ub: gpu::UniformBuffer<UIRect>,
    pub running: Cell<bool>,
    pub two_over_current_window_size: Cell<Vec2<f32>>,
}

impl UIAnchor {
    pub fn new(_system: &Rc<System>,graphics: &Rc<gpu::Graphics>,font_dir: &str) -> Result<UIAnchor,SystemError> {

        // generic vertex shader
        let vs = r#"
            #version 420 core
            
            layout(location = 0) in vec2 i_p;
            
            struct UIRect {
                vec4 r;
                vec4 t;
            };

            layout(std140) uniform rect_block {
                UIRect rects[256];
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

        let flat_shader = gpu::Shader::new(graphics,vs,None,flat_fs).expect("Unable to create flat shader.");
        let alpha_shader = gpu::Shader::new(graphics,vs,None,alpha_fs).expect("Unable to create alpha shader.");
        let color_shader = gpu::Shader::new(graphics,vs,None,color_fs).expect("Unable to create color shader.");

        let proto_sans = Rc::new(
            ui::FontProto::new(
                graphics,
                &format!("{}/sans.fnt",font_dir)
            ).expect("Unable to load font")
        );
        let proto_serif = Rc::new(
            ui::FontProto::new(
                graphics,
                &format!("{}/serif.fnt",font_dir)
            ).expect("Unable to load font")
        );
        let proto_mono = Rc::new(
            ui::FontProto::new(
                graphics,
                &format!("{}/mono.fnt",font_dir)
            ).expect("Unable to load font")
        );

        // create default font
        let font = Rc::new(ui::Font::new(&proto_sans,16).expect("unable to load font"));

        // create vertex buffer for one rectangle
        let rect_vb = gpu::VertexBuffer::<Vec2<f32>>::new_from_vec(graphics,vec![
            vec2!(0.0,0.0),
            vec2!(1.0,0.0),
            vec2!(1.0,1.0),
            vec2!(0.0,1.0)
        ]).expect("unable to create vertex buffer");

        // create draw uniform buffer
        let draw_ub = gpu::UniformBuffer::<UIRect>::new(graphics).expect("unable to create uniform buffer");

        Ok(UIAnchor {
            graphics: Rc::clone(graphics),
            flat_shader: flat_shader,
            alpha_shader: alpha_shader,
            color_shader: color_shader,
            proto_sans: proto_sans,
            proto_serif: proto_serif,
            proto_mono: proto_mono,
            font: font,
            rect_vb: rect_vb,
            draw_ub: draw_ub,  
            running: Cell::new(true),
            two_over_current_window_size: Cell::new(vec2!(0.0,0.0)),      
        })
    }

    pub fn set_current_window_size(&self,size: Vec2<i32>) {
        self.two_over_current_window_size.set(vec2!(2.0 / (size.x as f32),2.0 / (size.y as f32)));
    }

    pub fn draw_rectangle<C: ColorParameter>(&self,r: Rect<i32>,color: C,blend_mode: gpu::BlendMode) {
        self.draw_ub.load(0,&vec![UIRect {
            r: vec4!(r.o.x as f32,r.o.y as f32,r.s.x as f32,r.s.y as f32),
            t: vec4!(0.0,0.0,0.0,0.0),
        }]);
        self.graphics.set_blend(blend_mode);
        self.graphics.bind_shader(&self.flat_shader);
        self.graphics.bind_vertexbuffer(&self.rect_vb);
        self.graphics.bind_uniformbuffer(1,"rect_block",&self.draw_ub);
        let tows = self.two_over_current_window_size.get();
        self.graphics.set_uniform("tows",tows);
        self.graphics.set_uniform("color",color.into_vec4());
        self.graphics.draw_instanced_triangle_fan(4,1);
    }

    pub fn draw_text<C: ColorParameter>(&self,p: Vec2<i32>,text: &str,color: C,font: &ui::Font) {
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
        self.draw_ub.load(0,&buffer);
        self.graphics.set_blend(gpu::BlendMode::Over);
        self.graphics.bind_shader(&self.alpha_shader);
        self.graphics.bind_vertexbuffer(&self.rect_vb);
        self.graphics.bind_uniformbuffer(1,"rect_block",&self.draw_ub);
        self.graphics.bind_texture(0,&font.proto.texture);
        self.graphics.set_uniform("alpha_texture",0);
        let tows = self.two_over_current_window_size.get();
        self.graphics.set_uniform("tows",tows);
        self.graphics.set_uniform("color",color.into_vec4());
        self.graphics.draw_instanced_triangle_fan(4,buffer.len() as i32);
    }
}

/// UI subsystem.
pub struct UI {  // all system and graphics references live at least as long as UI
    pub system: Rc<System>,
    pub anchor: Rc<UIAnchor>,
    pub uiwindows: RefCell<Vec<Rc<UIWindow>>>,
}

impl UI {
    /// Create new UI context.
    /// ## Arguments
    /// * `system` - System to create the UI context for.
    /// * `graphics` - GPU graphics context to use.
    pub fn new(system: &Rc<System>,graphics: &Rc<gpu::Graphics>,font_dir: &str) -> Result<UI,SystemError> {
        Ok(UI {
            system: Rc::clone(system),
            anchor: Rc::new(UIAnchor::new(system,graphics,font_dir)?),
            uiwindows: RefCell::new(Vec::new()),
        })
    }

    /// Open new window.
    /// ## Arguments
    /// * `widget` - Widget to host.
    /// * `r` - Initial screen rectangle.
    /// * `title` - Window title.
    pub fn open_frame(&self,r: Rect<i32>,title: &str,widget: &Rc<RefCell<dyn ui::Widget>>) {
        let window = self.system.open_frame_window(r,title).expect("Unable to create frame.");
        self.anchor.graphics.bind_target(&window);
        widget.borrow_mut().set_rect(rect!(vec2!(0,0),r.s));
        let uiwindow = Rc::new(UIWindow {
            anchor: Rc::clone(&self.anchor),
            window: Rc::clone(&window),
            widget: Rc::clone(widget),
        });
        let closure_uiwindow = Rc::clone(&uiwindow);
        window.set_handler(move |event| closure_uiwindow.handler(event));
        //unsafe { (self.system.glx_swap_interval)(self.system.connection.get_raw_dpy(),window.id,0) };
        self.uiwindows.borrow_mut().push(uiwindow);
    }

    /// Close window.
    /// ## Arguments
    /// * `widget` - Widget that is hosted there.
    pub fn close(&self,widget: &Rc<RefCell<dyn ui::Widget>>) {
        let len = self.uiwindows.borrow().len();
        for i in 0..len {
            if Rc::ptr_eq(widget,&self.uiwindows.borrow()[i].widget) {
                self.uiwindows.borrow_mut().remove(i);
                self.system.close_window(&self.uiwindows.borrow()[i].window);
                break;
            }
        }
    }

    // Start drawing session.
    //pub fn begin_drawing(&self) -> Vec<ui::UIRect> {
    //    Vec::new()
    //}

    /// Run UI.
    /// 
    /// This runs the event loop and rebuilds and redraws the windows if needed.
    pub fn run(&self) {
        self.anchor.running.set(true);
        while self.anchor.running.get() {
            self.system.wait();
            self.system.flush();
        }
    }
}

pub struct UIWindow {
    anchor: Rc<UIAnchor>,
    window: Rc<Window>,
    widget: Rc<RefCell<dyn ui::Widget>>,
}

impl UIWindow {
    fn handler(&self,event: Event) {
        match event {
            Event::Render => {
                self.anchor.graphics.bind_target(&self.window);   
                self.anchor.graphics.clear(0xFF001122);
                let context = vec2!(0i32,0i32);
                self.anchor.set_current_window_size(self.window.r.get().s);
                self.widget.borrow_mut().draw(context);
                self.anchor.graphics.flush();
                self.anchor.graphics.present(&self.window);
            },

            Event::Size(s) => {
                self.widget.borrow_mut().set_rect(rect!(vec2!(0,0),s));
            },

            Event::Move(_o) => {
            },

            Event::Close => {
                self.anchor.running.set(false);  // TODO: closing other windows doesn't automatically mean end program; API user might want to ask something before actually quitting
            },

            Event::KeyPress(_k) => {
            },

            Event::KeyRelease(_k) => {
            },

            Event::MousePress(_p,b) => {
                // TODO: handle capturing
                self.widget.borrow_mut().handle_mouse_press(b);
            },

            Event::MouseRelease(_,b) => {
                // TODO: handle capturing
                self.widget.borrow_mut().handle_mouse_release(b);
            },

            Event::MouseWheel(_w) => {
            },

            Event::MouseMove(p) => {
                // TODO: handle capturing
                self.widget.borrow_mut().handle_mouse_move(p);
            },
        }        
    }
}