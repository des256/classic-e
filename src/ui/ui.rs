// E - UI
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::RefCell,
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

/// UI subsystem.
pub struct UI {
    pub system: Rc<System>,
    pub graphics: Rc<gpu::Graphics>,
    pub uber_shader: gpu::Shader,
    pub font_textures: Rc<gpu::Texture2DArray<pixel::R8>>,
    pub icons_textures: Rc<ui::Texture2DArrayAtlas<pixel::ARGB8>>,
    pub packed_textures: Rc<ui::Texture2DArrayAtlas<pixel::ARGB8>>,
    pub large_textures: Rc<ui::Texture2DArrayAtlas<pixel::ARGB8>>,
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

        // create uber shader
        let uber_vs = r#"
            #version 420 core

            layout(location = 0) in vec4 ir;
            layout(location = 1) in vec4 it;
            layout(location = 2) in uvec4 ifbdq;

            out Rect {
                vec4 t;
                flat uvec4 fbdq;
            } vs_out;

            void main() {
                gl_Position = ir;
                vs_out.t = it;
                vs_out.fbdq = ifbdq;
            }
        "#;
        let uber_gs = r#"
            #version 420 core

            uniform vec2 canvas_size;
            
            layout(points) in;
            layout(triangle_strip,max_vertices = 4) out;

            in Rect {
                vec4 t;
                flat uvec4 fbdq;
            } gs_in[];

            out Vertex {
                vec2 t;
                flat uvec4 fbdq;
            } gs_out;

            void main() {

                vec4 r = gl_in[0].gl_Position;
                vec4 t = gs_in[0].t;
                uvec4 fbdq = gs_in[0].fbdq;

                vec4 pn = vec4(
                    -1.0 + 2.0 * r.x / canvas_size.x,
                    1.0 - 2.0 * r.y / canvas_size.y,
                    2.0 * r.z / canvas_size.x,
                    -2.0 * r.w / canvas_size.y
                );

                gl_Position = vec4(pn.x,pn.y,0.0,1.0);
                gs_out.t = vec2(t.x,t.y);
                gs_out.fbdq = fbdq;
                EmitVertex();

                gl_Position = vec4(pn.x + pn.z,pn.y,0.0,1.0);
                gs_out.t = vec2(t.x + t.z,t.y);
                gs_out.fbdq = fbdq;
                EmitVertex();

                gl_Position = vec4(pn.x,pn.y + pn.w,0.0,1.0);
                gs_out.t = vec2(t.x,t.y + t.w);
                gs_out.fbdq = fbdq;
                EmitVertex();

                gl_Position = vec4(pn.x + pn.z,pn.y + pn.w,0.0,1.0);
                gs_out.t = vec2(t.x + t.z,t.y + t.w);
                gs_out.fbdq = fbdq;
                EmitVertex();

                EndPrimitive();
            }
        "#;

        let uber_fs = r#"
            #version 420 core

            uniform sampler2DArray alpha_textures;
            uniform sampler2DArray icons_textures;
            uniform sampler2DArray packed_textures;
            uniform sampler2DArray large_textures;

            in Vertex {
                vec2 t;
                flat uvec4 fbdq;
            } fs_in;

            out vec4 o;

            void main() {
                vec4 fc = unpackUnorm4x8(fs_in.fbdq.x).zyxw;
                vec4 bc = unpackUnorm4x8(fs_in.fbdq.y).zyxw;
                float d = float(fs_in.fbdq.z);
                uint qm = fs_in.fbdq.w >> 16;
                uint ql = uint(fs_in.fbdq.w) & uint(0xFFFF);
                vec4 t = vec4(0,0,0,0);
                switch(qm) {
                    case 0: t = texture(alpha_textures,vec3(fs_in.t.x,fs_in.t.y,ql)).xxxx; break;
                    case 1: t = texture(icons_textures,vec3(fs_in.t.x,fs_in.t.y,ql)); break;
                    case 2: t = texture(packed_textures,vec3(fs_in.t.x,fs_in.t.y,ql)); break;
                    case 3: t = texture(large_textures,vec3(fs_in.t.x,fs_in.t.y,ql)); break;
                }
                o = bc * (1.0 - t.w) + fc * t;
                //o = ta;
            }
        "#;
        let uber_shader = gpu::Shader::new(&graphics,uber_vs,Some(uber_gs),uber_fs).expect("what?");

        // create font atlas texture array
        let font_textures = Rc::new(gpu::Texture2DArray::<pixel::R8>::new(&graphics,vec3!(ui::FONT_TEXTURE_SIZE as usize,ui::FONT_TEXTURE_SIZE as usize,3)).expect("unable to allocate font texture atlas"));
        let proto_sans = Rc::new(ui::FontProto::new(&font_textures,&format!("{}/sans.fnt",font_dir),0).expect("Unable to load font"));
        let proto_serif = Rc::new(ui::FontProto::new(&font_textures,&format!("{}/serif.fnt",font_dir),1).expect("Unable to load font"));
        let proto_mono = Rc::new(ui::FontProto::new(&font_textures,&format!("{}/mono.fnt",font_dir),2).expect("Unable to load font"));

        // create default font
        let font = Rc::new(ui::Font::new(&proto_sans,16).expect("unable to load font"));

        // create texture array for icons (generally same size)
        let icons_textures = Rc::new(ui::Texture2DArrayAtlas::<pixel::ARGB8>::new(&graphics,vec2!(1024,1024)).expect("unable to allocate icon texture atlas"));

        // create texture array for packed images (all different)
        let packed_textures = Rc::new(ui::Texture2DArrayAtlas::<pixel::ARGB8>::new(&graphics,vec2!(1024,1024)).expect("unable to allocate packed texture atlas"));

        // create texture array for large textures (one per texture)
        let large_textures = Rc::new(ui::Texture2DArrayAtlas::<pixel::ARGB8>::new(&graphics,vec2!(4096,4096)).expect("unable to allocate icon texture atlas"));
        
        Ok(UI {
            system: Rc::clone(system),
            graphics: Rc::clone(graphics),
            uber_shader: uber_shader,
            font_textures: font_textures,
            icons_textures: icons_textures,
            packed_textures: packed_textures,
            large_textures: large_textures,
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
            //buffer: Vec::new(),
            //vertexbuffer: gpu::VertexBuffer::<ui::UIRect>::new(&self.graphics).expect("Unable to create vertexbuffer"),
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

    /// Finish drawing session.
    // canvas_size should not be here, maybe better in start_drawing
    pub fn end_drawing(&self,canvas_size: Vec2<i32>,buffer: Vec<ui::UIRect>,blend_mode: gpu::BlendMode) {
        let points = buffer.len();
        let vertexbuffer = gpu::VertexBuffer::<ui::UIRect>::new_from_vec(&self.graphics,buffer).expect("Unable to create vertexbuffer.");
        self.graphics.set_blend(blend_mode);
        self.graphics.bind_shader(&self.uber_shader);
        self.graphics.bind_texture(0,&*self.font_textures);
        self.graphics.bind_texture(1,&self.icons_textures.array);
        self.graphics.bind_texture(2,&self.packed_textures.array);
        self.graphics.bind_texture(3,&self.large_textures.array);
        self.graphics.set_uniform("alpha_textures",0);
        self.graphics.set_uniform("icons_textures",1);
        self.graphics.set_uniform("packed_textures",2);
        self.graphics.set_uniform("large_textures",3);
        self.graphics.bind_vertexbuffer(&vertexbuffer);
        self.graphics.set_uniform("canvas_size",vec2!(canvas_size.x as f32,canvas_size.y as f32));
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

pub trait UIRectFunctions {
    fn push_text(&mut self,p: Vec2<i32>,text: &str,font: &ui::Font,color: u32,back_color: u32);
    fn push_rect(&mut self,r: Rect<i32>,color: u32);
}

impl UIRectFunctions for Vec<ui::UIRect> {
    fn push_text(&mut self,p: Vec2<i32>,text: &str,font: &ui::Font,color: u32,back_color: u32) {
        font.build_text(self,p,text,0.0,color,back_color);
    }

    fn push_rect(&mut self,r: Rect<i32>,color: u32) {
        self.push(ui::UIRect {
            r: vec4!(r.o.x as f32,r.o.y as f32,r.s.x as f32,r.s.y as f32),
            t: vec4!(0.0,0.0,0.0,0.0),
            fbdq: vec4!(color,color,0,0x00000000),
        });
    }
}