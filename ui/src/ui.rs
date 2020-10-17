// E - UI
// Desmond Germans, 2020

// The UI manages the various open windows of the interface.

use crate::*;
use std::{
    cell::{
        Cell,
        RefCell,
    },
    rc::Rc,
};

pub struct UIWindow {
    pub ui: Rc<UI>,
    pub window: Rc<Window>,
    pub widget: Rc<dyn Widget>,
    pub capturing: Cell<bool>,
}

/// UI context.
pub struct UI {
    pub system: Rc<System>,
    pub graphics: Rc<Graphics>,
    pub flat_shader: Shader,  // the shaders
    pub alpha_shader: Shader,
    pub color_shader: Shader,
    pub rect_vb: VertexBuffer<Vec2<f32>>,  // vertexbuffer containing fixed unit rectangle
    pub draw_ub: UniformBuffer<TexRect>,  // uniform buffer with actual rectangle specfications
    pub two_over_window_size: Cell<Vec2<f32>>,  // 2/w,2/h of the current window
    pub offset: Cell<Vec2<i32>>,  // drawing offset (TBD)
    pub proto_sans: Rc<FontProto>,
    pub proto_serif: Rc<FontProto>,
    pub proto_mono: Rc<FontProto>,
    pub font: Rc<Font>,
    pub uiwindows: RefCell<Vec<Rc<UIWindow>>>,
    pub new_uiwindows: RefCell<Vec<Rc<UIWindow>>>,
    pub drop_uiwindows: RefCell<Vec<usize>>,
    pub current_capturing_id: Cell<Option<u64>>,  // window that is currently capturing the mouse (TBD)
    pub running: Cell<bool>,
}

impl UI {
    pub fn new(system: &Rc<System>,graphics: &Rc<Graphics>,font_path: &str) -> Result<Rc<UI>,SystemError> {

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

        // create draw uniform buffer
        let draw_ub = graphics.create_uniformbuffer::<TexRect>()?;

        // font prototypes
        let proto_sans = Rc::new(
            FontProto::new(
                graphics,
                &format!("{}/sans.fnt",font_path)
            )?
        );
        let proto_serif = Rc::new(
            FontProto::new(
                graphics,
                &format!("{}/serif.fnt",font_path)
            )?
        );
        let proto_mono = Rc::new(
            FontProto::new(
                graphics,
                &format!("{}/mono.fnt",font_path)
            )?
        );

        // the main font for now
        let font = Rc::new(Font::new(&proto_sans,16)?);

        Ok(Rc::new(UI {
            system: Rc::clone(system),
            graphics: Rc::clone(graphics),
            flat_shader: flat_shader,
            alpha_shader: alpha_shader,
            color_shader: color_shader,
            rect_vb: rect_vb,
            draw_ub: draw_ub,
            two_over_window_size: Cell::new(vec2!(0.0,0.0)),
            offset: Cell::new(vec2!(0,0)),
            proto_sans: proto_sans,
            proto_serif: proto_serif,
            proto_mono: proto_mono,
            font: font,
            uiwindows: RefCell::new(Vec::new()),
            new_uiwindows: RefCell::new(Vec::new()),
            drop_uiwindows: RefCell::new(Vec::new()),
            current_capturing_id: Cell::new(None),
            running: Cell::new(true),
        }))
    }

    pub fn terminate(&self) {
        self.running.set(false);
    }

    pub fn run(&self) {
        self.running.set(true);
        while self.running.get() {

            // wait for events
            self.system.wait();

            // process the events
            self.system.flush();

            // redraw all windows
            for uiwindow in self.uiwindows.borrow().iter() {
                uiwindow.draw_widget();
            }

            // start updating uiwindows
            let mut uiwindows = self.uiwindows.borrow_mut();

            // remove dropped windows
            self.drop_uiwindows.borrow_mut().sort_unstable();
            for index in self.drop_uiwindows.borrow().iter().rev() {
                uiwindows.remove(*index);
            }
            self.drop_uiwindows.borrow_mut().clear();

            // append new windows
            uiwindows.append(&mut self.new_uiwindows.borrow_mut());
        }
    }

    pub fn set_window_size(&self,size: Vec2<i32>) {
        self.two_over_window_size.set(vec2!(2.0 / (size.x as f32),2.0 / (size.y as f32)));
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
                (r.o.x + ofs.x) as f32,
                (r.o.y + ofs.y) as f32,
                r.s.x as f32,
                r.s.y as f32
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
                (p.x + ofs.x) as f32,
                (p.y + ofs.y) as f32,
                texture.size().x as f32,
                texture.size().y as f32
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
                let mut v = vec2!(p.x + ofs.x,p.y + ofs.y + (font.ratio * (s.y_bearing as f32)) as i32);
                for c in text.chars() {
                    let code = c as u32;
                    for ch in s.characters.iter() {
                        if ch.n == code {
                            buffer.push(TexRect {
                                r: rect!(
                                    (v.x + (font.ratio * (ch.bearing.x as f32)) as i32) as f32,
                                    (v.y - (font.ratio * (ch.bearing.y as f32)) as i32) as f32,
                                    ((font.ratio * (ch.r.s.x as f32)) as i32) as f32,
                                    ((font.ratio * (ch.r.s.y as f32)) as i32) as f32
                                ),
                                t: rect!(
                                    (ch.r.o.x as f32) / (font.proto.texture.size().x as f32),
                                    (ch.r.o.y as f32) / (font.proto.texture.size().y as f32),
                                    (ch.r.s.x as f32) / (font.proto.texture.size().x as f32),
                                    (ch.r.s.y as f32) / (font.proto.texture.size().y as f32)
                                ),
                            });
                            /*buffer.push(TexRect {
                                r: rect!(
                                    (v.x + (font.ratio * (ch.bearing.x as f32)) as i32) as f32,
                                    (v.y - (font.ratio * (ch.bearing.y as f32)) as i32) as f32,
                                    ((font.ratio * (ch.r.sx() as f32)) as i32) as f32,
                                    ((font.ratio * (ch.r.sy() as f32)) as i32) as f32
                                ),
                                t: rect!(
                                    (ch.r.ox() as f32) / (font.proto.texture.size().x as f32),
                                    (ch.r.oy() as f32) / (font.proto.texture.size().y as f32),
                                    (ch.r.sx() as f32) / (font.proto.texture.size().x as f32),
                                    (ch.r.sy() as f32) / (font.proto.texture.size().y as f32)
                                ),
                            });*/
                            v.x += (font.ratio * (ch.advance as f32)) as i32;
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

impl UIWindow {
    fn register(self) -> Rc<UIWindow> {
        let rced_self = Rc::new(self);
        let handler_rced_self = Rc::clone(&rced_self);
        rced_self.window.set_handler(move |platform_event| UIWindow::dispatch_platform_event(&handler_rced_self,platform_event));
        rced_self.widget.set_rect(rced_self.window.r.get());
        rced_self.ui.new_uiwindows.borrow_mut().push(Rc::clone(&rced_self));
        rced_self
    }

    pub fn new_frame(ui: &Rc<UI>,r: Rect<i32>,title: &str,widget: Rc<dyn Widget>) -> Result<Rc<UIWindow>,SystemError> {
        Ok(UIWindow {
            ui: Rc::clone(&ui),
            window: Window::new_frame(&ui.system,r,title)?,
            widget: widget,
            capturing: Cell::new(false),
        }.register())
    }

    pub fn new_popup(ui: &Rc<UI>,r: Rect<i32>,widget: Rc<dyn Widget>) -> Result<Rc<UIWindow>,SystemError> {
        Ok(UIWindow {
            ui: Rc::clone(&ui),
            window: Window::new_popup(&ui.system,r)?,
            widget: widget,
            capturing: Cell::new(false),
        }.register())
    }

    pub fn close(&self) {
        self.window.clear_handler();
        let uiwindows = self.ui.uiwindows.borrow();
        for i in 0..uiwindows.len() {
            let cur_uiwindow = &uiwindows[i];
            if Rc::as_ptr(cur_uiwindow) == self as *const UIWindow {
                self.ui.drop_uiwindows.borrow_mut().push(i);
                break;
            }
        }
    }
    
    pub fn configure(&self,r: Rect<i32>) {
        self.window.configure(&r);
    }

    pub fn show(&self) {
        self.window.show();
    }

    pub fn hide(&self) {
        self.window.hide();
    }

    fn dispatch_platform_event(window: &Rc<UIWindow>,platform_event: platform::Event) {
        let mut new_capturing = false;
        match platform_event {
            platform::Event::KeyPress(k) => {
                window.widget.keypress(&window.ui,window,k);
            },
            platform::Event::KeyRelease(k) => {
                window.widget.keyrelease(&window.ui,window,k);
            },
            platform::Event::MousePress(p,b) => {
                new_capturing = window.widget.mousepress(&window.ui,&window,p,b);
            },
            platform::Event::MouseRelease(p,b) => {
                new_capturing = window.widget.mouserelease(&window.ui,&window,p,b);
            },
            platform::Event::MouseWheel(w) => {
                new_capturing = window.widget.mousewheel(&window.ui,&window,w);
            },
            platform::Event::MouseMove(p) => {
                new_capturing = window.widget.mousemove(&window.ui,&window,p);
            },
            platform::Event::Configure(r) => {
                window.widget.set_rect(r);
            },
            platform::Event::Render => {
#[cfg(target_os="linux")]
                {
                    // In X11, window resizing is done in the main loop, so just
                    // collect the need to rerender in the dirty variable, and
                    // handle the actual rendering in UI::run()
                }
#[cfg(target_os="windows")]
                {
                    // In Windows, window resizing uses its own loop, so
                    // DispatchMessage() doesn't return until the window is
                    // resized, but WndProc() gets called to redraw the window
                    // anyway, handle the drawing here
                    window.draw_widget();
                }
            },
            platform::Event::Close => {
                // TODO
            },
        }

        // capture the mouse in this window
        if new_capturing != window.capturing.get() {
            if new_capturing {
                window.ui.system.capture_mouse(window.window.id);
            }
            else {
                window.ui.system.release_mouse();
            }
            window.capturing.set(new_capturing);
        }
    }

    fn draw_widget(&self) {

        // select and clear window to draw in
        self.ui.graphics.bind_target(&self.window);
        self.ui.graphics.clear(0xFF001122);

        // prepare draw context
        self.ui.set_window_size(self.window.r.get().s);
        self.ui.reset_offset();

        // draw the widget hierarchy
        self.widget.draw();

        // and flush and present the window
        self.ui.graphics.flush();
        self.ui.graphics.present(self.window.id);
    }
}
