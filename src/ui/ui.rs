// E - UI
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::Cell,
    cell::RefCell,
};

enum UIDelta {
    Skip,
    Draw,
}

pub struct UIWindow {
    window: Rc<Window>,
    widget: Rc<RefCell<dyn ui::Widget>>,
    delta: UIDelta,
}

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

/// UI subsystem.
pub struct UI {  // all system and graphics references live at least as long as UI
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
    current_window_size: Cell<Vec2<i32>>,
    capturing_index: Cell<Option<i32>>,
    rect_vb: gpu::VertexBuffer<Vec2<f32>>,
    draw_ub: gpu::UniformBuffer<UIRect>,
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
            current_window_size: Cell::new(vec2!(0,0)),
            capturing_index: Cell::new(None),
            rect_vb: rect_vb,
            draw_ub: draw_ub,
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
    pub fn open(&self,widget: &Rc<RefCell<dyn ui::Widget>>,r: Rect<i32>,title: &str) -> bool {
        let window = Rc::new(Window::new_framed(&self.system,r,title).expect("unable to create window"));
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
    pub fn close(&self,widget: &Rc<RefCell<dyn ui::Widget>>) {
        let len = self.windows.borrow().len();
        for i in 0..len {
            if Rc::ptr_eq(widget,&self.windows.borrow()[i].widget) {
                self.windows.borrow_mut().remove(i);
                break;
            }
        }
    }

    // Start drawing session.
    //pub fn begin_drawing(&self) -> Vec<ui::UIRect> {
    //    Vec::new()
    //}

    pub fn draw_rectangle<C: ColorParameter>(&self,r: Rect<i32>,color: C,blend_mode: gpu::BlendMode) {
        self.draw_ub.load(0,&vec![UIRect {
            r: vec4!(r.o.x as f32,r.o.y as f32,r.s.x as f32,r.s.y as f32),
            t: vec4!(0.0,0.0,0.0,0.0),
        }]);
        self.graphics.set_blend(blend_mode);
        self.graphics.bind_shader(&self.flat_shader);
        self.graphics.bind_vertexbuffer(&self.rect_vb);
        self.graphics.bind_uniformbuffer(1,"rect_block",&self.draw_ub);
        let window_size = self.current_window_size.get();
        self.graphics.set_uniform("tows",vec2!(2.0 / (window_size.x as f32),2.0 / (window_size.y as f32)));
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
        let window_size = self.current_window_size.get();
        self.graphics.set_uniform("tows",vec2!(2.0 / (window_size.x as f32),2.0 / (window_size.y as f32)));
        self.graphics.set_uniform("color",color.into_vec4());
        self.graphics.draw_instanced_triangle_fan(4,buffer.len() as i32);
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

                        window.delta = UIDelta::Draw;

                        match event {

                            // system wants to render this window
                            Event::Render => { },

                            // system notifies that this window moved/changed size
                            Event::Reconfigure(r) => {
                                window.window.r.set(r);
                                window.widget.borrow_mut().set_rect(rect!(vec2!(0,0),r.s));
                            },

                            // user wants to close this window
                            Event::Close => {
                                running = false;  // TODO: closing other windows doesn't automatically mean end program; API user might want to ask something before actually quitting
                            },

                            // anything else should be handled by the hosted widget
                            Event::KeyPress(_k) => {
                            },

                            Event::KeyRelease(_k) => {
                            },

                            Event::MousePress(_p,b) => {
                                if let ui::MouseResult::ProcessedCapture = window.widget.borrow_mut().handle_mouse_press(b) {
                                    match self.capturing_index.get() {
                                        Some(index) => {
                                            if index != i as i32 {
                                                self.system.capture_mouse(&window.window);
                                                self.capturing_index.set(Some(i as i32));
                                            }    
                                        },
                                        None => {
                                            self.system.capture_mouse(&window.window);
                                            self.capturing_index.set(Some(i as i32));
                                        },
                                    }
                                }
                                else {
                                    if let Some(_) = self.capturing_index.get() {
                                        self.system.release_mouse();
                                        self.capturing_index.set(None);    
                                    }
                                }
                            },

                            Event::MouseRelease(_,b) => {
                                if let ui::MouseResult::ProcessedCapture = window.widget.borrow_mut().handle_mouse_release(b) {
                                    match self.capturing_index.get() {
                                        Some(index) => {
                                            if index != i as i32 {
                                                self.system.capture_mouse(&window.window);
                                                self.capturing_index.set(Some(i as i32));
                                            }    
                                        },
                                        None => {
                                            self.system.capture_mouse(&window.window);
                                            self.capturing_index.set(Some(i as i32));
                                        },
                                    }
                                }
                                else {
                                    if let Some(_) = self.capturing_index.get() {
                                        self.system.release_mouse();
                                        self.capturing_index.set(None);    
                                    }
                                }
                            },

                            Event::MouseWheel(_w) => {
                            },

                            Event::MouseMove(p) => {
                                if let ui::MouseResult::ProcessedCapture = window.widget.borrow_mut().handle_mouse_move(p) {
                                    match self.capturing_index.get() {
                                        Some(index) => {
                                            if index != i as i32 {
                                                self.system.capture_mouse(&window.window);
                                                self.capturing_index.set(Some(i as i32));
                                            }    
                                        },
                                        None => {
                                            self.system.capture_mouse(&window.window);
                                            self.capturing_index.set(Some(i as i32));
                                        },
                                    }
                                }
                                else {
                                    if let Some(_) = self.capturing_index.get() {
                                        self.system.release_mouse();
                                        self.capturing_index.set(None);    
                                    }
                                }
                            },
                        }        
                        break;
                    }
                }
            }

            /*{
                let frames = self.frames.borrow();
                println!("frames:");
                for i in 0..9 {
                    println!("    {}: rect {} scroll {} pf {}",i,frames[i].r,frames[i].s,frames[i].pf)
                }
            }*/

            for window in self.windows.borrow_mut().iter_mut() {

                // bind to this window
                self.graphics.bind_target(&window.window);

                // if widget tree needs to be rebuilt, rebuild it
                if let UIDelta::Draw = window.delta {
                    let r = window.window.r.get();
                    self.current_window_size.set(r.s);  // so window_size in the shader corresponds to the actual window size
                    self.graphics.clear(0xFF001122);
                    let context = vec2!(0i32,0i32);
                    window.widget.borrow_mut().draw(context);
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
