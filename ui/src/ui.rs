// E - UI
// Desmond Germans, 2020

use crate::*;

use std::{
    rc::Rc,
    cell::{
        Cell,
        RefCell,
    },
};

// TODO: the more tightly packed UIRect will come later
#[derive(Copy,Clone)]
#[repr(C)]
#[doc(hidden)]
pub struct UIRect {
    pub(crate) r: Vec4<f32>,  // rectangle: x, y, w, h
    pub(crate) t: Vec4<f32>,  // texture coordinates: x, y, w, h
}

// this is locks the system into OpenGL, fix later
impl GLUniform for UIRect {
    fn len() -> isize {
        32
    }
}

#[doc(hidden)]
pub struct DrawContext {
    pub r: Rect<i32>,
}

pub struct UIState {
    pub system: Rc<System>,  // system reference
    pub graphics: Rc<Graphics>,  // graphics reference
    pub flat_shader: Shader,  // the shaders
    pub alpha_shader: Shader,
    pub color_shader: Shader,
    pub rect_vb: VertexBuffer<Vec2<f32>>,  // vertexbuffer containing fixed unit rectangle
    pub draw_ub: UniformBuffer<UIRect>,  // uniform buffer with actual rectangle specfications
    pub styles: RefCell<Styles>,  // fonts, colors, paddings, spacings, etc. for the style of the UI
    pub running: Cell<bool>,  // whether or not the UI is running
    pub two_over_current_window_size: Cell<Vec2<f32>>,  // 2/w,2/h of the current window
    pub current_capturing_id: Cell<Option<u64>>,  // window that is currently capturing the mouse (TBD)
    pub offset: Cell<Vec2<i32>>,  // drawing offset (TBD)
}

impl UIState {
    pub fn new(system: &Rc<System>,graphics: &Rc<Graphics>,font_path: &str) -> Result<UIState,SystemError> {

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

        let flat_shader = Shader::new(graphics,vs,None,flat_fs).expect("Unable to create flat shader.");
        let alpha_shader = Shader::new(graphics,vs,None,alpha_fs).expect("Unable to create alpha shader.");
        let color_shader = Shader::new(graphics,vs,None,color_fs).expect("Unable to create color shader.");

        // create vertex buffer for one rectangle
        let rect_vb = VertexBuffer::<Vec2<f32>>::new_from_vec(graphics,vec![
            vec2!(0.0,0.0),
            vec2!(1.0,0.0),
            vec2!(1.0,1.0),
            vec2!(0.0,1.0)
        ]).expect("unable to create vertex buffer");

        // create draw uniform buffer
        let draw_ub = UniformBuffer::<UIRect>::new(graphics).expect("unable to create uniform buffer");

        // create default styles
        let styles = Styles::new_default(graphics,font_path)?;

        Ok(UIState {
            system: Rc::clone(system),
            graphics: Rc::clone(graphics),
            flat_shader: flat_shader,
            alpha_shader: alpha_shader,
            color_shader: color_shader,
            rect_vb: rect_vb,
            draw_ub: draw_ub,
            styles: RefCell::new(styles),
            running: Cell::new(true),
            two_over_current_window_size: Cell::new(Vec2::<f32>::zero()),
            current_capturing_id: Cell::new(None),
            offset: Cell::new(vec2!(0i32,0i32)),
        })
    }

    pub fn set_current_window_size(&self,size: Vec2<i32>) {
        self.two_over_current_window_size.set(vec2!(2.0 / (size.x() as f32),2.0 / (size.y() as f32)));
    }

    pub fn delta_offset(&self,o: Vec2<i32>) {
        let old_offset = self.offset.get();
        self.offset.set(old_offset + o);
    }

    /// Draw rectangle.
    pub fn draw_rectangle<C: ColorParameter>(&self,r: Rect<i32>,color: C,blend_mode: BlendMode) {
        let ofs = self.offset.get();
        self.draw_ub.load(0,&vec![UIRect {
            r: vec4!((r.ox() + ofs.x()) as f32,(r.oy() + ofs.y()) as f32,r.sx() as f32,r.sy() as f32),
            t: Vec4::<f32>::zero(),
        }]);
        self.graphics.set_blend(blend_mode);
        self.graphics.bind_shader(&self.flat_shader);
        self.graphics.bind_vertexbuffer(&self.rect_vb);
        self.graphics.bind_uniformbuffer(1,"rect_block",&self.draw_ub);
        let tows = self.two_over_current_window_size.get();
        self.graphics.set_uniform("tows",tows);
        self.graphics.set_uniform("color",color.as_vec4());
        self.graphics.draw_instanced_triangle_fan(4,1);
    }

    /// Draw texture.
    pub fn draw_texture<T: GPUDataFormat>(&self,p: Vec2<i32>,texture: &Texture2D<T>,blend_mode: BlendMode) {
        let ofs = self.offset.get();
        self.draw_ub.load(0,&vec![UIRect {
            r: vec4!((p.x() + ofs.x()) as f32,(p.y() + ofs.y()) as f32,texture.size().x() as f32,texture.size().y() as f32),
            t: vec4!(0.0,0.0,1.0,1.0),
        }]);
        self.graphics.set_blend(blend_mode);
        self.graphics.bind_shader(&self.color_shader);
        self.graphics.bind_vertexbuffer(&self.rect_vb);
        self.graphics.bind_uniformbuffer(1,"rect_block",&self.draw_ub);
        let tows = self.two_over_current_window_size.get();
        self.graphics.set_uniform("tows",tows);
        self.graphics.set_uniform("color",vec4!(1.0,1.0,1.0,1.0));
        self.graphics.bind_texture(0,texture);
        self.graphics.set_uniform("color_texture",0);
        self.graphics.draw_instanced_triangle_fan(4,1);
    }

    /// Draw text.
    pub fn draw_text<C: ColorParameter>(&self,p: Vec2<i32>,text: &str,color: C,font: &Font) {
        let ofs = self.offset.get();
        let mut buffer: Vec<UIRect> = Vec::new();
        for s in font.proto.sets.iter() {
            if s.font_size == font.font_size {
                let mut v = vec2!(p.x() + ofs.x(),p.y() + ofs.y() + (font.ratio * (s.y_bearing as f32)) as i32);
                for c in text.chars() {
                    let code = c as u32;
                    for ch in s.characters.iter() {
                        if ch.n == code {
                            buffer.push(UIRect {
                                r: vec4!(
                                    (v.x() + (font.ratio * (ch.bearing.x() as f32)) as i32) as f32,
                                    (v.y() - (font.ratio * (ch.bearing.y() as f32)) as i32) as f32,
                                    ((font.ratio * (ch.r.sx() as f32)) as i32) as f32,
                                    ((font.ratio * (ch.r.sy() as f32)) as i32) as f32
                                ),
                                t: vec4!(
                                    (ch.r.ox() as f32) / (font.proto.texture.size().x() as f32),
                                    (ch.r.oy() as f32) / (font.proto.texture.size().y() as f32),
                                    (ch.r.sx() as f32) / (font.proto.texture.size().x() as f32),
                                    (ch.r.sy() as f32) / (font.proto.texture.size().y() as f32)
                                ),
                            });
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
        let tows = self.two_over_current_window_size.get();
        self.graphics.set_uniform("tows",tows);
        self.graphics.set_uniform("color",color.as_vec4());
        self.graphics.draw_instanced_triangle_fan(4,buffer.len() as i32);
    }
}

pub struct WidgetWindow {
    pub state: Rc<UIState>,
    pub window: PlatformWindow,
    pub widget: Rc<dyn Widget>,
}

/// UI engine.
pub struct UI {
    pub state: Rc<UIState>,
    pub windows: Vec<WidgetWindow>,
}

impl UI {
    pub fn new(system: &Rc<System>,graphics: &Rc<Graphics>,font_dir: &str) -> Result<UI,SystemError> {
        let state = Rc::new(UIState::new(system,graphics,font_dir)?);
        Ok(UI {
            state: state,
            windows: Vec::new(),
        })
    }

    /// Open frame with a widget.
    /// 
    /// **Arguments**
    /// * `r` - Rectangle of the frame.
    /// * `title` - Title of the frame.
    /// * `widget` - The widget interface to run here.
    /// 
    /// **Returns**
    /// Unique ID for this frame.
    pub fn open_frame<T: Widget + 'static>(&mut self,r: Rect<i32>,title: &str,widget: &Rc<T>) {
        let window = PlatformWindow::new_frame(&self.state.system,r,title);
        let widget = Rc::clone(widget);
        widget.set_rect(Rect::<i32>::new_os(Vec2::<i32>::zero(),r.s()));
        self.windows.push(WidgetWindow {
            state: Rc::clone(&self.state),
            window: window,
            widget: widget,
        });
    }

    pub fn close<T: Widget + 'static>(&mut self,_widget: &Rc<T>) {
        // TODO: find WidgetWindow and remove it from self.windows
    }

    /// Run UI.
    /// 
    /// This runs the event loop and rebuilds and redraws the windows if
    /// needed.
    pub fn run(&self) {
        self.state.running.set(true);
        while self.state.running.get() {
            self.state.system.wait();
            let mut windows: Vec<&WidgetWindow> = Vec::new();
            for window in self.windows.iter() {
                windows.push(&window);
            }
            self.state.system.flush(&windows);
            for window in windows {
                self.state.graphics.bind_target(window);
                self.state.graphics.clear(0xFF001122);
                self.state.set_current_window_size(window.widget.rect().s());
                self.state.offset.set(vec2!(0i32,0i32));
                window.widget.draw();
                self.state.graphics.flush();
                self.state.graphics.present(window.id());
            }
        }
    }
}

impl HandleEvent for WidgetWindow {
    fn handle(&self,event: Event) {
        match event {

            Event::Render => {
                // redrawing is always happening anyway, so ignore this
            },

            Event::Configure(r) => {
                self.widget.set_rect(rect!(vec2!(0i32,0i32),r.s()));
            }

            Event::Close => {
                self.state.running.set(false);  // TODO: closing other windows doesn't automatically mean end program; API user might want to ask something before actually quitting
            },

            Event::KeyPress(_k) => {
            },

            Event::KeyRelease(_k) => {
            },

            Event::MousePress(p,b) => {
                self.widget.handle_mouse_press(p,b);
            },

            Event::MouseRelease(p,b) => {
                self.widget.handle_mouse_release(p,b);
            },

            Event::MouseWheel(w) => {
                self.widget.handle_mouse_wheel(w);
            },

            Event::MouseMove(p) => {
                self.widget.handle_mouse_move(p);
            },
        }
    }

    fn id(&self) -> u64 {
        self.window.id
    }
}
