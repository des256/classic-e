// e UI (Linux/XCB/glX)
// by Desmond Germans, 2019

use std::{ffi::{CStr,CString},os::raw::{c_int,c_void},ptr::null_mut};
#[doc(no_inline)]
extern crate x11;
use x11::{glx,xlib::*};
#[doc(no_inline)]
extern crate gl;
use gl::types::GLuint;
use crate::*;

const GLX_CONTEXT_MAJOR_VERSION_ARB: u32 = 0x2091;
const GLX_CONTEXT_MINOR_VERSION_ARB: u32 = 0x2092;

type GlXCreateContextAttribsARBProc = unsafe extern "C" fn(dpy: *mut Display,fbc: glx::GLXFBConfig,share_context: glx::GLXContext,direct: Bool,attribs: *const c_int) -> glx::GLXContext;

fn load_function(name: &str) -> *mut c_void {
    let newname = CString::new(name).unwrap();
    let pointer: *mut c_void = unsafe { std::mem::transmute(glx::glXGetProcAddress(newname.as_ptr() as *const u8)) };
    if pointer.is_null() { panic!("Canvas: unable to access {}", name); }
    pointer
}

struct Window {
    allocated: bool,
    xcb_id: u32,
    frames: Vec<UIFrame>,
    quads: usize,
    ubo: GLuint,
    vao: GLuint,
    redraw: bool,
}

pub struct WindowEvent {
    pub id: usize,
    pub event: Event,
}

pub struct UI {
    connection: xcb::Connection,
    //visual: *const XVisualInfo,
    //screen: i32,
    depth: i32,
    visualid: u64,
    wm_motif_hints: u32,
    wm_protocols: u32,
    wm_delete_window: u32,
    wm_transient_for: u32,
    //glx_create_context_attribs: GlXCreateContextAttribsARBProc,
    //fbconfig: glx::GLXFBConfig,
    colormap: u32,
    rootwindow: u32,
    hidden_window: u32,
    context: glx::GLXContext,
    shader: Shader,
    pub(crate) font: crate::Font,
    texture_array: Texture2DArray,
    windows: Vec<Window>,
}

impl<'a> UI {
    pub fn new() -> UI {

        // open connection
        let (connection,_screen_number) = xcb::Connection::connect_with_xlib_display().unwrap();
        connection.set_event_queue_owner(xcb::EventQueueOwner::Xcb);

        // check of glX is useful
        let mut glxmaj: c_int = 0;
        let mut glxmin: c_int = 0;
        unsafe { if glx::glXQueryVersion(connection.get_raw_dpy(),&mut glxmaj as *mut c_int,&mut glxmin as *mut c_int) == 0 { panic!("UI: unable to get glX version"); } }
        if (glxmaj * 100 + glxmin) < 103 { panic!("UI: glX version {}.{} needs to be at least 1.3",glxmaj,glxmin); }

        // choose appropriate framebuffer configuration
        let attribs = [
            glx::GLX_X_RENDERABLE,  1,
            glx::GLX_DRAWABLE_TYPE, glx::GLX_WINDOW_BIT,
            glx::GLX_RENDER_TYPE,   glx::GLX_RGBA_BIT,
            glx::GLX_X_VISUAL_TYPE, glx::GLX_TRUE_COLOR,
            glx::GLX_RED_SIZE,      8,
            glx::GLX_GREEN_SIZE,    8,
            glx::GLX_BLUE_SIZE,     8,
            glx::GLX_ALPHA_SIZE,    8,
            glx::GLX_DEPTH_SIZE,    24,
            glx::GLX_STENCIL_SIZE,  8,
            glx::GLX_DOUBLEBUFFER,  1,
            0,
        ];
        let mut fbcount: c_int = 0;
        let fbconfigs = unsafe { glx::glXChooseFBConfig(connection.get_raw_dpy(),0,attribs.as_ptr(),&mut fbcount as *mut c_int) };
        if fbcount == 0 { panic!("UI: unable to find framebuffer config"); }
        let fbconfig = unsafe { *fbconfigs };
        unsafe { XFree(fbconfigs as *mut c_void) };
        let visual = unsafe { glx::glXGetVisualFromFBConfig(connection.get_raw_dpy(), fbconfig) };
        let screen = unsafe { (*visual).screen };
        let visual_screen = connection.get_setup().roots().nth(screen as usize).unwrap();
        let depth = unsafe { (*visual).depth };
        let visualid = unsafe { (*visual).visualid };

        // verify needed extensions
        let extensions = unsafe { CStr::from_ptr(glx::glXQueryExtensionsString(connection.get_raw_dpy(),screen)) }.to_str().unwrap();
        let mut found = false;
        for extension in extensions.split(" ") {
            if extension == "GLX_ARB_create_context" {
                found = true;
                break;
            }
        }
        if !found { panic!("UI: unable to access GLX_ARB_create_context"); }
        let glx_create_context_attribs: GlXCreateContextAttribsARBProc = unsafe { std::mem::transmute(load_function("glXCreateContextAttribsARB")) };

        // load configuration atoms
        let (wm_motif_hints,wm_protocols,wm_delete_window,wm_transient_for) = {
            let motif_hints_com = xcb::intern_atom(&connection,false,"_MOTIF_WM_HINTS");
            let protocols_com = xcb::intern_atom(&connection,false,"WM_PROTOCOLS");
            let delete_window_com = xcb::intern_atom(&connection,false,"WM_DELETE_WINDOW");
            let wm_transient_for_com = xcb::intern_atom(&connection,false,"WM_TRANSIENT_FOR");
            let motif_hints = match motif_hints_com.get_reply() {
                Ok(motif_hints) => motif_hints.atom(),
                Err(_) => panic!("UI: unable to access _MOTIF_WM_HINTS"),
            };
            let protocols = match protocols_com.get_reply() {
                Ok(protocols) => protocols.atom(),
                Err(_) => panic!("UI: unable to access WM_PROTOCOLS"),
            };
            let delete_window = match delete_window_com.get_reply() {
                Ok(delete_window) => delete_window.atom(),
                Err(_) => panic!("UI: unable to access WM_DELETE_WINDOW"),
            };
            let wm_transient_for = match wm_transient_for_com.get_reply() {
                Ok(wm_transient_for) => wm_transient_for.atom(),
                Err(_) => panic!("UI: unable to access WM_TRANSIENT_FOR"),
            };
            (motif_hints,protocols,delete_window,wm_transient_for)
        };

        // create invisible window
        let rootwindow = visual_screen.root();
        let hidden_window = connection.generate_id();
        let colormap = connection.generate_id();
        xcb::create_colormap(&connection,xcb::COLORMAP_ALLOC_NONE as u8,colormap,rootwindow,visualid as u32);
        let values = [
            (xcb::CW_EVENT_MASK,
                xcb::EVENT_MASK_EXPOSURE
                | xcb::EVENT_MASK_KEY_PRESS
                | xcb::EVENT_MASK_KEY_RELEASE
                | xcb::EVENT_MASK_BUTTON_PRESS
                | xcb::EVENT_MASK_BUTTON_RELEASE
                | xcb::EVENT_MASK_POINTER_MOTION
                | xcb::EVENT_MASK_STRUCTURE_NOTIFY
            ),
            (xcb::CW_COLORMAP,colormap),
        ];
        xcb::create_window(&connection,depth as u8,hidden_window,rootwindow,0,0,1,1,0,xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,visualid as u32,&values);
        //xcb::map_window(&connection,hidden_window);
        connection.flush();
        unsafe { XSync(connection.get_raw_dpy(),False) };

        // create glX context
        let context_attribs: [c_int; 5] = [
            GLX_CONTEXT_MAJOR_VERSION_ARB as c_int, 4,
            GLX_CONTEXT_MINOR_VERSION_ARB as c_int, 5,
            0,
        ];
        let context = unsafe { glx_create_context_attribs(connection.get_raw_dpy(),fbconfig,std::ptr::null_mut(),True,&context_attribs[0] as *const c_int) };
        connection.flush();
        unsafe { XSync(connection.get_raw_dpy(), False) };
        if context.is_null() { panic!("UI: unable to open OpenGL context"); }
        if unsafe { glx::glXIsDirect(connection.get_raw_dpy(),context) } == 0 { panic!("UI: OpenGL context is indirect"); }
        unsafe { glx::glXMakeCurrent(connection.get_raw_dpy(),hidden_window as XID,context) };

        // load OpenGL symbols
        gl::load_with(|symbol| load_function(&symbol));

        // create UI shader
        let vs = r#"
            #version 420 core

            layout(location = 0) in vec4 i_geo;    // x0,y0,xs,ys
            layout(location = 1) in vec4 i_tex;    // u0,v0,us,vs
            layout(location = 2) in vec4 i_col;    // b,g,r,a
            layout(location = 3) in uvec4 i_fmlf;  // f,m,l,f

            out Quad {
                vec4 tex;    // u0,v0,us,vs
                vec4 col;    // r,g,b,a
                uvec4 fmlf;  // f,m,l,f
            } vs_out;

            void main() {
                gl_Position = i_geo;
                vs_out.tex = i_tex;
                vs_out.col = i_col.zyxw;
                vs_out.fmlf = i_fmlf;
            }
        "#;
        let gs = r#"
            #version 420 core

            struct Frame
            {
                vec4 geometry;  // frame geometry, relative to parent
                vec4 scroll;    // scrolling offset and scale
                uvec4 fpuu;     // flags,parent,-,-
            };

            layout(std140,binding = 0) uniform uniform_block
            {
                Frame u_frames[256];  // this might not be enough...
            };

            layout(points) in;
            layout(triangle_strip, max_vertices = 4) out;

            in Quad {
                vec4 tex;    // u0,v0,us,vs
                vec4 col;    // r,g,b,a
                uvec4 fmlf;  // f,m,l,f
            } gs_in[];

            out Vertex {
                vec2 tex;       // u,v
                vec4 col;       // r,g,b,a
                vec4 clip;      // x - cx0,y - cy0,cx1 - x,cy1 - y
                flat uvec2 ml;  // m,l
            } gs_out;

            void main() {
                uint f = gs_in[0].fmlf.x;  // the current frame
                vec4 p = gl_in[0].gl_Position;  // quad is expressed in local coordinates
                vec4 c = u_frames[f].geometry;  // clipping rectangle is expressed in parent coordinates

                // discard if quad not allocated, frame not allocated or visible, or clipping rectangle empty
                //if (((gs_in[0].fmlf.w & 0x0001) != 0x0001) || ((u_frames[f].fpuu.x & 0x00000003) != 0x00000003) || ((c.z <= 0.0) || (c.w <= 0.0)))
                //    return;

                while(f != 0) {

                    // transform quad to parent coordinates
                    //p.xy *= u_frames[f].scroll.zw;
                    //p.zw *= u_frames[f].scroll.zw;
                    //p.xy += u_frames[f].scroll.xy;
                    p.xy += u_frames[f].geometry.xy;

                    // discard if entire quad is clipped
                    //if ((p.x > c.x + c.z) || (p.x + p.z < c.x) || (p.y > c.y + c.w) || (p.y + p.w < c.y))
                    //    return;

                    // switch to parent
                    f = u_frames[f].fpuu.y;

                    // discard if frame not allocated or not visible
                    //if ((u_frames[f].fpuu.x & 0x00000003) != 0x00000003)
                    //    return;

                    // transform clipping rectangle to parent coordinates
                    //c.xy *= u_frames[f].scroll.zw;
                    //c.zw *= u_frames[f].scroll.zw;
                    //c.xy += u_frames[f].scroll.xy;
                    c.xy += u_frames[f].geometry.xy;

                    // merge clipping rectangle into f
                    c.z = min(c.x + c.z,u_frames[f].geometry.z) - c.x;
                    c.w = min(c.y + c.w,u_frames[f].geometry.w) - c.y;
                    c.x = max(c.x,u_frames[f].geometry.x);
                    c.y = max(c.y,u_frames[f].geometry.y);

                    // discard if clipping rectangle empty
                    //if ((c.z <= 0.0) || (c.w <= 0.0))
                    //    return;
                }

                // transform quad to window coordinates as well
                //p.xy *= u_frames[0].scroll.zw;
                //p.zw *= u_frames[0].scroll.zw;
                //p.xy += u_frames[0].scroll.xy;

                // discard if clipping rectangle empty, or entire quad is clipped
                //if (((c.z <= 0.0) || (c.w <= 0.0)) || ((p.x > c.x + c.z) || (p.x + p.z < c.x) || (p.y > c.y + c.w) || (p.y + p.w < c.y)))
                //    return;

                // convert quad to normalized coordinates
                vec4 pn = vec4(
                    -1.0 + 2.0 * p.x / u_frames[0].geometry.z,
                    1.0 - 2.0 * p.y / u_frames[0].geometry.w,
                    -1.0 + 2.0 * (p.x + p.z) / u_frames[0].geometry.z,
                    1.0 - 2.0 * (p.y + p.w) / u_frames[0].geometry.w
                );

                vec4 tex = gs_in[0].tex;
                vec4 col = gs_in[0].col;
                uvec2 ml = gs_in[0].fmlf.yz;

                // emit the quad
                gl_Position = vec4(pn.x,pn.y,0.0,1.0);
                gs_out.clip = vec4(p.x - c.x,p.y - c.y,c.x + c.z - p.x,c.y + c.w - p.y);
                gs_out.tex = vec2(tex.x,tex.y);
                gs_out.col = col;
                gs_out.ml = ml;
                EmitVertex();

                gl_Position = vec4(pn.z,pn.y,0.0,1.0);
                gs_out.clip = vec4(p.x + p.z - c.x,p.y - c.y,c.x + c.z - p.x - p.z,c.y + c.w - p.y);
                gs_out.tex = vec2(tex.x + tex.z,tex.y);
                gs_out.col = col;
                gs_out.ml = ml;
                EmitVertex();

                gl_Position = vec4(pn.x,pn.w,0.0,1.0);
                gs_out.clip = vec4(p.x - c.x,p.y + p.w - c.y,c.x + c.z - p.x,c.y + c.w - p.y - p.w);
                gs_out.tex = vec2(tex.x,tex.y + tex.w);
                gs_out.col = col;
                gs_out.ml = ml;
                EmitVertex();

                gl_Position = vec4(pn.z,pn.w,0.0,1.0);
                gs_out.clip = vec4(p.x + p.z - c.x,p.y + p.w - c.y,c.x + c.z - p.x - p.z,c.y + c.w - p.y - p.w);
                gs_out.tex = vec2(tex.x + tex.z,tex.y + tex.w);
                gs_out.col = col;
                gs_out.ml = ml;
                EmitVertex();

                EndPrimitive();
            }
        "#;
        let fs = r#"
            #version 420 core

            in Vertex {
                vec2 tex;       // u,v
                vec4 col;       // r,g,b,a
                vec4 clip;      // x - cx0,y - cy0,cx1 - x,cy1 - y
                flat uvec2 ml;  // m,l
            } fs_in;

            uniform sampler2DArray sampler;

            out vec4 FragColor;

            float msdf(vec4 t) {
                vec2 unit = (4.0 / textureSize(sampler,0)).xy;
                float dist = max(min(t.r,t.g),min(max(t.r,t.g),t.b)) - 0.5;
                dist *= dot(unit,0.5 / fwidth(fs_in.tex));
                return clamp(dist + 0.5,0.0,1.0);
            }

            void main() {
                // discard if clipped
                // TODO: the geometry shader can potentially already clip the quad, so it will not be necessary to discard any fragments
                //if ((fs_in.clip.x < 0.0) || (fs_in.clip.y < 0.0) || (fs_in.clip.z < 0.0) || (fs_in.clip.w < 0.0))
                //    discard;

                // get color and texture values
                vec4 c = fs_in.col;
                vec4 t = texture(sampler,vec3(fs_in.tex,fs_in.ml.y));

                // render
                switch(fs_in.ml.x) {

                    // color
                    case 0: FragColor = c; break;

                    // texture
                    case 1: FragColor = t; break;

                    // color * texture
                    case 2: FragColor = c * t; break;

                    // msdf
                    case 3: FragColor = c * msdf(t); break;
                }
            }
        "#;
        let shader = Shader::new(vs,Some(gs),fs);

        let font = crate::Font::new("../../../../static/fonts/robotoi.fnt");
        let mut texture_array = Texture2DArray::new(1,font.image.size);
        texture_array.upload_image(0,isize_2::new(0,0),&font.image);

        UI {
            connection: connection,
            //visual: visual,
            //screen: screen,
            depth: depth,
            visualid: visualid,
            wm_motif_hints: wm_motif_hints,
            wm_protocols: wm_protocols,
            wm_delete_window: wm_delete_window,
            wm_transient_for: wm_transient_for,
            //glx_create_context_attribs: glx_create_context_attribs,
            //fbconfig: fbconfig,
            rootwindow: rootwindow,
            hidden_window: hidden_window,
            colormap: colormap,
            context: context,
            shader: shader,
            font: font,
            texture_array: texture_array,
            windows: Vec::new(),
        }
    }

    pub fn next_event(&mut self) -> Option<WindowEvent> {
        for i in 0..self.windows.len() {
            if self.windows[i].allocated && self.windows[i].redraw {
                let geometry = &self.windows[i].frames[0].geometry();
                let rect = isize_r::new(geometry.o.x as isize,geometry.o.y as isize,geometry.s.x as isize,geometry.s.y as isize);
                self.paint_window(i,&rect);
                self.windows[i].redraw = false;
            }
        }
        while let Some(xcb_event) = self.connection.poll_for_event() {
            let r = xcb_event.response_type() & !0x80;
            match r {
                xcb::EXPOSE => {
                    let expose: &xcb::ExposeEvent = unsafe { xcb::cast_event(&xcb_event) };
                    let rect = isize_r {
                        o: isize_2 { x: expose.x() as isize,y: expose.y() as isize, },
                        s: isize_2 { x: expose.width() as isize,y: expose.height() as isize, },
                    };
                    for i in 0..self.windows.len() {
                        if self.windows[i].allocated && (self.windows[i].xcb_id == expose.window()) {                        
                            self.paint_window(i,&rect);
                            break;
                        }
                    }
                },
                xcb::KEY_PRESS => {
                    let key_press: &xcb::KeyPressEvent = unsafe { xcb::cast_event(&xcb_event) };
                    for i in 0..self.windows.len() {
                        if self.windows[i].allocated && (self.windows[i].xcb_id == key_press.event()) {
                            return Some(WindowEvent { id: i, event: Event::KeyPress(key_press.detail()) });
                        }
                    }
                },
                xcb::KEY_RELEASE => {
                    let key_release: &xcb::KeyReleaseEvent = unsafe { xcb::cast_event(&xcb_event) };
                    for i in 0..self.windows.len() {
                        if self.windows[i].allocated && (self.windows[i].xcb_id == key_release.event()) {
                            return Some(WindowEvent { id: i, event: Event::KeyRelease(key_release.detail()) });
                        }
                    }
                },
                xcb::BUTTON_PRESS => {
                    let button_press: &xcb::ButtonPressEvent = unsafe { xcb::cast_event(&xcb_event) };
                    for i in 0..self.windows.len() {
                        if self.windows[i].allocated && (self.windows[i].xcb_id == button_press.event()) {
                            return Some(WindowEvent { id: i, event: Event::MousePress(isize_2::new(button_press.event_x() as isize,button_press.event_y() as isize),button_press.state() as u8) });
                        }
                    }
                },
                xcb::BUTTON_RELEASE => {
                    let button_release: &xcb::ButtonReleaseEvent = unsafe { xcb::cast_event(&xcb_event) };
                    for i in 0..self.windows.len() {
                        if self.windows[i].allocated && (self.windows[i].xcb_id == button_release.event()) {
                            return Some(WindowEvent { id: i, event: Event::MouseRelease(isize_2::new(button_release.event_x() as isize,button_release.event_y() as isize),button_release.state() as u8) });
                        }
                    }
                },
                xcb::MOTION_NOTIFY => {
                    let motion_notify: &xcb::MotionNotifyEvent = unsafe { xcb::cast_event(&xcb_event) };
                    for i in 0..self.windows.len() {
                        if self.windows[i].allocated && (self.windows[i].xcb_id == motion_notify.event()) {
                            return Some(WindowEvent { id: i, event: Event::MouseMove(isize_2::new(motion_notify.event_x() as isize,motion_notify.event_y() as isize)) });
                        }
                    }
                },
                xcb::CONFIGURE_NOTIFY => {
                    let configure_notify: &xcb::ConfigureNotifyEvent = unsafe { xcb::cast_event(&xcb_event) };
                    let rect = isize_r {
                        o: isize_2 { x: configure_notify.x() as isize,y: configure_notify.y() as isize, },
                        s: isize_2 { x: configure_notify.width() as isize,y: configure_notify.height() as isize, },
                    };
                    for i in 0..self.windows.len() {
                        if self.windows[i].allocated && (self.windows[i].xcb_id == configure_notify.window()) {
                            self.windows[i].frames[0].set_geometry(f32_r::new(rect.o.x as f32,rect.o.y as f32,rect.s.x as f32,rect.s.y as f32));
                            self.upload_frames(i);
                            return Some(WindowEvent { id: i, event: Event::Geometry(rect) });
                        }
                    }
                },
                xcb::CLIENT_MESSAGE => {
                    let client_message : &xcb::ClientMessageEvent = unsafe { xcb::cast_event(&xcb_event) };
                    let data = &client_message.data().data;
                    let atom = (data[0] as u32) | ((data[1] as u32) << 8) | ((data[2] as u32) << 16) | ((data[3] as u32) << 24);
                    if atom == self.wm_delete_window {
                        for i in 0..self.windows.len() {
                            if self.windows[i].allocated && (self.windows[i].xcb_id == client_message.window()) {
                                return Some(WindowEvent { id: i, event: Event::Close });
                            }
                        }
                    }
                },
                _ => { },
            }
        }
        None
    }

    fn create_window(&mut self,rect: isize_r,o_r: u32) -> usize {
        let xcb_id = self.connection.generate_id();
        let values = [
            (xcb::CW_EVENT_MASK,
                xcb::EVENT_MASK_EXPOSURE
                | xcb::EVENT_MASK_KEY_PRESS
                | xcb::EVENT_MASK_KEY_RELEASE
                | xcb::EVENT_MASK_BUTTON_PRESS
                | xcb::EVENT_MASK_BUTTON_RELEASE
                | xcb::EVENT_MASK_POINTER_MOTION
                | xcb::EVENT_MASK_STRUCTURE_NOTIFY
            ),
            (xcb::CW_COLORMAP,self.colormap),
            (xcb::CW_OVERRIDE_REDIRECT,o_r),
        ];
        xcb::create_window(&self.connection,self.depth as u8,xcb_id,self.rootwindow,rect.o.x as i16,rect.o.y as i16,rect.s.x as u16,rect.s.y as u16,0,xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,self.visualid as u32,&values);
        xcb::map_window(&self.connection,xcb_id);
        self.connection.flush();
        let mut frames: Vec<UIFrame> = Vec::new();
        let mut frame = UIFrame::new();
        frame.set_geometry(f32_r::new(rect.o.x as f32,rect.o.y as f32,rect.s.x as f32,rect.s.y as f32));
        frame.set_allocated(true);
        frame.set_visible(true);
        frames.push(frame);
        let mut ubo: GLuint = 0;
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1,&mut ubo);
            gl::BindBuffer(gl::UNIFORM_BUFFER,ubo);
            gl::BufferData(gl::UNIFORM_BUFFER,(UIFRAME_SIZE * frames.len()) as isize,frames.as_ptr() as *const UIFrame as *const c_void,gl::DYNAMIC_DRAW);
            gl::GenVertexArrays(1,&mut vao);
            gl::BindVertexArray(vao);
            gl::GenBuffers(1,&mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER,vbo);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0,4,gl::FLOAT,gl::FALSE,UIQUAD_SIZE as i32,0 as *const c_void);
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1,4,gl::FLOAT,gl::FALSE,UIQUAD_SIZE as i32,16 as *const c_void);
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(2,4,gl::UNSIGNED_BYTE,gl::TRUE,UIQUAD_SIZE as i32,32 as *const c_void);
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribIPointer(3,4,gl::UNSIGNED_SHORT,UIQUAD_SIZE as i32,36 as *const c_void);
            gl::BufferData(gl::ARRAY_BUFFER,0,0 as *const c_void,gl::DYNAMIC_DRAW);
        }
        let window = Window {
            allocated: true,
            xcb_id: xcb_id,
            frames: frames,
            quads: 0,
            ubo: ubo,
            vao: vao,
            redraw: false,
        };
        for i in 0..self.windows.len() {
            if self.windows[i].allocated == false {
                self.windows[i] = window;
                return i;
            }
        }
        self.windows.push(window);
        self.windows.len() - 1
    }

    pub fn create_app_window(&mut self,rect: isize_r,title: &str) -> usize {
        let id = self.create_window(rect,0);
        xcb::change_property(&self.connection,xcb::PROP_MODE_REPLACE as u8,self.windows[id].xcb_id,xcb::ATOM_WM_NAME,xcb::ATOM_STRING,8,title.as_bytes());
        let protocol_set = [self.wm_delete_window];
        xcb::change_property(&self.connection,xcb::PROP_MODE_REPLACE as u8,self.windows[id].xcb_id,self.wm_protocols,xcb::ATOM_ATOM,32,&protocol_set);
        self.connection.flush();
        id
    }

    pub fn create_popup_window(&mut self,rect: isize_r,owner_id: usize) -> usize {
        let id = self.create_window(rect,1);
        let hints = [2u32,0,0,0,0];
        xcb::change_property(&self.connection,xcb::PROP_MODE_REPLACE as u8,self.windows[id].xcb_id,self.wm_motif_hints,xcb::ATOM_ATOM,32,&hints);
        if (owner_id < self.windows.len()) && self.windows[owner_id].allocated {
            let transient = [self.windows[owner_id].xcb_id];
            xcb::change_property(&self.connection,xcb::PROP_MODE_REPLACE as u8,self.windows[id].xcb_id,self.wm_transient_for,xcb::ATOM_ATOM,32,&transient);
        }
        self.connection.flush();
        id
    }

    pub fn destroy_window(&mut self,id: usize) {
        if (id < self.windows.len()) && self.windows[id].allocated {
            unsafe {
                gl::DeleteBuffers(1,&self.windows[id].ubo);
                gl::DeleteVertexArrays(1,&self.windows[id].vao);
                glx::glXMakeCurrent(self.connection.get_raw_dpy(),self.hidden_window as XID,self.context);
            }
            xcb::unmap_window(&self.connection,self.windows[id].xcb_id);
            xcb::destroy_window(&self.connection,self.windows[id].xcb_id);
            self.windows[id].allocated = false;
        }
    }

    pub fn invalidate(&mut self,wid: usize) {
        if (wid < self.windows.len()) && self.windows[wid].allocated {
            self.windows[wid].redraw = true;
        }        
    }

    fn paint_window(&mut self,wid: usize,rect: &isize_r) {
        let window = &self.windows[wid];
        println!("paint_window, quads = {}",window.quads);
        unsafe {
            glx::glXMakeCurrent(self.connection.get_raw_dpy(),window.xcb_id as u64,self.context);
            gl::Viewport(rect.o.x as i32,rect.o.y as i32,rect.s.x as i32,rect.s.y as i32);
            gl::Scissor(rect.o.x as i32,rect.o.y as i32,rect.s.x as i32,rect.s.y as i32);
            gl::ClearColor(0.1,0.1,0.1,1.0);  // for debugging
            gl::ClearDepth(0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            if window.quads > 0 {
                gl::Enable(gl::BLEND);
                gl::BlendEquationSeparate(gl::FUNC_ADD,gl::FUNC_ADD);
                gl::BlendFuncSeparate(gl::SRC_ALPHA,gl::ONE_MINUS_SRC_ALPHA,gl::ONE,gl::ONE_MINUS_SRC_ALPHA);
                self.shader.bind();
                self.texture_array.bind();
                gl::BindBufferBase(gl::UNIFORM_BUFFER,0,window.ubo);
                gl::BindVertexArray(window.vao);
                gl::DrawArrays(gl::POINTS,0,window.quads as i32);
                gl::Disable(gl::BLEND);
            }
            gl::Flush();
            glx::glXSwapBuffers(self.connection.get_raw_dpy(),window.xcb_id as u64);
        }
    }

    pub fn alloc_frame(&mut self,wid: usize) -> Option<usize> {
        if (wid < self.windows.len()) && self.windows[wid].allocated {
            for i in 0..self.windows[wid].frames.len() {
                if !self.windows[wid].frames[i].allocated() {
                    self.windows[wid].frames[i].set_allocated(true);
                    return Some(i);
                }
            }
            let i = self.windows[wid].frames.len();
            if i >= MAX_UIFRAMES {
                return None;
            }
            self.windows[wid].frames.push(UIFrame::new());
            self.windows[wid].frames[i].set_allocated(true);
            return Some(i);
        }
        None
    }

    pub fn free_frame(&mut self,wid: usize,fid: usize) {
        if (wid < self.windows.len()) && self.windows[wid].allocated && (fid < self.windows[wid].frames.len()) && self.windows[wid].frames[fid].allocated() {
            self.windows[wid].frames[fid].set_allocated(false);
            let mut highest_i = self.windows[wid].frames.len() - 1;
            while (highest_i > 0) && !self.windows[wid].frames[highest_i].allocated() {
                highest_i -= 1;
            }
            if highest_i + 1 != self.windows[wid].frames.len() {
                self.windows[wid].frames.truncate(highest_i + 1);
            }
        }
    }

    pub fn frame(&self,wid: usize,fid: usize) -> Option<UIFrame> {
        if (wid < self.windows.len()) && self.windows[wid].allocated && (fid < self.windows[wid].frames.len()) && self.windows[wid].frames[fid].allocated() {
            Some(self.windows[wid].frames[fid])
        }
        else {
            None
        }
    }

    pub fn set_frame(&mut self,wid: usize,fid: usize,frame: &UIFrame) {
        if (wid < self.windows.len()) && self.windows[wid].allocated && (fid < self.windows[wid].frames.len()) && self.windows[wid].frames[fid].allocated() {
            self.windows[wid].frames[fid] = *frame;
        }
    }

    pub fn upload_frames(&self,wid: usize) {
        if (wid < self.windows.len()) && self.windows[wid].allocated {
            unsafe {
                println!("uploading {} frames:",self.windows[wid].frames.len());
                for i in 0..self.windows[wid].frames.len() {
                    println!("    {}: {}",i,self.windows[wid].frames[i]);
                }
                gl::BindBuffer(gl::UNIFORM_BUFFER,self.windows[wid].ubo);
                gl::BufferData(gl::UNIFORM_BUFFER,(self.windows[wid].frames.len() * UIFRAME_SIZE) as isize,self.windows[wid].frames.as_ptr() as *const UIFrame as *const c_void,gl::DYNAMIC_DRAW);
            }
        }
    }

    pub fn upload_quads(&mut self,wid: usize,quads: &Vec<UIQuad>) {
        if (wid < self.windows.len()) && self.windows[wid].allocated {
            let window = &mut self.windows[wid];
            window.quads = quads.len();
            unsafe {
                gl::BindVertexArray(self.windows[wid].vao);
                gl::BufferData(gl::ARRAY_BUFFER,(quads.len() * UIQUAD_SIZE) as isize,quads.as_ptr() as *const UIQuad as *const c_void,gl::DYNAMIC_DRAW);
            }
        }
    }

    pub fn globalize(&self,wid: usize,fid: usize,p: &mut f32_2) {
        if (wid < self.windows.len()) && self.windows[wid].allocated {
            let mut cfid = fid;
            while (cfid != 0) && (cfid < self.windows[wid].frames.len()) && self.windows[wid].frames[cfid].allocated() {
                self.windows[wid].frames[cfid].globalize(p);
                cfid = self.windows[wid].frames[cfid].parent();
            }
            self.windows[wid].frames[0].globalize(p);
        }
    }

    pub fn localize(&self,wid: usize,fid: usize,p: &mut f32_2) {
        if (wid < self.windows.len()) && self.windows[wid].allocated {            
            let mut cfid = 0;
            while cfid != fid {
                self.windows[wid].frames[cfid].localize(p);
                let mut id = fid;
                while self.windows[wid].frames[id].parent() != cfid {
                    id = self.windows[wid].frames[id].parent();
                }
                cfid = id;
            }
            self.windows[wid].frames[cfid].localize(p);
        }
    }
}

impl Drop for UI {
    fn drop(&mut self) {
        unsafe { glx::glXMakeCurrent(self.connection.get_raw_dpy(),0,null_mut()); }
        for i in 0..self.windows.len() {
            if self.windows[i].allocated {
                self.destroy_window(i);
            }
        }
        xcb::unmap_window(&self.connection,self.hidden_window);
        xcb::destroy_window(&self.connection,self.hidden_window);
        unsafe { glx::glXDestroyContext(self.connection.get_raw_dpy(),self.context); }
    }
}
