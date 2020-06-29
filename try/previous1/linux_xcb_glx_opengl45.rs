// e::canvas::Canvas for Linux/GLX/OpenGL45
// by Desmond Germans, 2019

use std::{ffi::{CStr,CString},os::raw::{c_int,c_void},ptr::null_mut};
#[doc(no_inline)]
extern crate x11;
use x11::{glx,xlib::*};
#[doc(no_inline)]
extern crate gl;
use gl::types::GLuint;

use math::Vec2;
use crate::Rect;

const GLX_CONTEXT_MAJOR_VERSION_ARB: u32 = 0x2091;
const GLX_CONTEXT_MINOR_VERSION_ARB: u32 = 0x2092;

type GlXCreateContextAttribsARBProc = unsafe extern "C" fn(dpy: *mut Display,fbc: glx::GLXFBConfig,share_context: glx::GLXContext,direct: Bool,attribs: *const c_int) -> glx::GLXContext;

fn load_function(name: &str) -> *mut c_void {
    let newname = CString::new(name).unwrap();
    let pointer: *mut c_void = unsafe { std::mem::transmute(glx::glXGetProcAddress(newname.as_ptr() as *const u8)) };
    if pointer.is_null() { panic!("Canvas: unable to access {}", name); }
    pointer
}

pub fn uniform_location(sp: GLuint,name: &str) -> i32 {
    let s = CString::new(name).expect("CString error??");
    let res = unsafe { gl::GetUniformLocation(sp,s.as_ptr()) };
    if res < 0 {
        panic!("Canvas: \"{}\" uniform not found (result: {})",name,res);
    }
    res
}

struct SubCanvas {
    rect: Rect,
    aspect: f32,
    draw: Option<fn(&Rect)>,
    rect_changed: Option<fn(&Rect)>,

    window: u32,
}

pub struct Canvas {
    rect: Rect,
    aspect: f32,
    draw: Option<fn(&Rect)>,
    rect_changed: Option<fn(&Rect)>,
    subs: Vec<SubCanvas>,

    connection: xcb::Connection,
    visual: *const XVisualInfo,
    window: u32,
    wm_motif_hints: u32,
    wm_protocols: u32,
    wm_delete_window: u32,
    wm_transient_for: u32,
    glx_create_context_attribs: GlXCreateContextAttribsARBProc,
    fbconfig: glx::GLXFBConfig,
    context: glx::GLXContext,
    current_window: u32,
}

impl Canvas {
    pub fn new(width: usize,height: usize,aspect: f32,title: &str) -> Canvas {
        let (connection, _screen_number) = xcb::Connection::connect_with_xlib_display().unwrap();
        connection.set_event_queue_owner(xcb::EventQueueOwner::Xcb);
        let mut glxmaj: c_int = 0;
        let mut glxmin: c_int = 0;
        unsafe { if glx::glXQueryVersion(connection.get_raw_dpy(),&mut glxmaj as *mut c_int,&mut glxmin as *mut c_int) == 0 { panic!("Canvas: unable to get glX version"); } }
        if (glxmaj * 100 + glxmin) < 103 { panic!("Canvas: glX version {}.{} needs to be at least 1.3",glxmaj,glxmin); }
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
        if fbcount == 0 { panic!("Canvas: unable to find framebuffer config"); }
        let fbconfig = unsafe { *fbconfigs };
        unsafe { XFree(fbconfigs as *mut c_void) };
        let visual = unsafe { glx::glXGetVisualFromFBConfig(connection.get_raw_dpy(), fbconfig) };
        let screen = unsafe { (*visual).screen };
        let visual_screen = connection.get_setup().roots().nth(screen as usize).unwrap();
        let depth = unsafe { (*visual).depth };
        let visualid = unsafe { (*visual).visualid };
        let window = connection.generate_id();
        let colormap = connection.generate_id();
        xcb::create_colormap(&connection,xcb::COLORMAP_ALLOC_NONE as u8,colormap,visual_screen.root(),visualid as u32);
        let values = [
            (xcb::CW_BACK_PIXEL,visual_screen.white_pixel()),
            (xcb::CW_BORDER_PIXEL,visual_screen.black_pixel()),
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
        xcb::create_window(&connection,depth as u8,window,visual_screen.root(),0,0,width as u16,height as u16,0,xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,visualid as u32,&values);
        xcb::change_property(&connection,xcb::PROP_MODE_REPLACE as u8,window,xcb::ATOM_WM_NAME,xcb::ATOM_STRING,8,title.as_bytes());
        let (wm_motif_hints,wm_protocols,wm_delete_window,wm_transient_for) = {
            let motif_hints_com = xcb::intern_atom(&connection,false,"_MOTIF_WM_HINTS");
            let protocols_com = xcb::intern_atom(&connection,false,"WM_PROTOCOLS");
            let delete_window_com = xcb::intern_atom(&connection,false,"WM_DELETE_WINDOW");
            let wm_transient_for_com = xcb::intern_atom(&connection,false,"WM_TRANSIENT_FOR");
            let motif_hints = match motif_hints_com.get_reply() {
                Ok(motif_hints) => motif_hints.atom(),
                Err(_) => panic!("Canvas: unable to access _MOTIF_WM_HINTS"),
            };
            let protocols = match protocols_com.get_reply() {
                Ok(protocols) => protocols.atom(),
                Err(_) => panic!("Canvas: unable to access WM_PROTOCOLS"),
            };
            let delete_window = match delete_window_com.get_reply() {
                Ok(delete_window) => delete_window.atom(),
                Err(_) => panic!("Canvas: unable to access WM_DELETE_WINDOW"),
            };
            let wm_transient_for = match wm_transient_for_com.get_reply() {
                Ok(wm_transient_for) => wm_transient_for.atom(),
                Err(_) => panic!("Canvas: unable to access WM_TRANSIENT_FOR"),
            };
            (motif_hints,protocols,delete_window,wm_transient_for)
        };
        let protocol_set = [wm_delete_window];
        xcb::change_property(&connection,xcb::PROP_MODE_REPLACE as u8,window,wm_protocols,xcb::ATOM_ATOM,32,&protocol_set);
        xcb::map_window(&connection, window);
        connection.flush();
        unsafe { XSync(connection.get_raw_dpy(),False) };
        let extensions = unsafe { CStr::from_ptr(glx::glXQueryExtensionsString(connection.get_raw_dpy(),screen)) }.to_str().unwrap();
        let mut found = false;
        for extension in extensions.split(" ") {
            if extension == "GLX_ARB_create_context" {
                found = true;
                break;
            }
        }
        if !found { panic!("Canvas: unable to access GLX_ARB_create_context"); }
        let glx_create_context_attribs: GlXCreateContextAttribsARBProc = unsafe { std::mem::transmute(load_function("glXCreateContextAttribsARB")) };
        let context_attribs: [c_int; 5] = [
            GLX_CONTEXT_MAJOR_VERSION_ARB as c_int, 4,
            GLX_CONTEXT_MINOR_VERSION_ARB as c_int, 5,
            0,
        ];
        let context = unsafe { glx_create_context_attribs(connection.get_raw_dpy(),fbconfig,std::ptr::null_mut(),True,&context_attribs[0] as *const c_int) };
        connection.flush();
        unsafe { XSync(connection.get_raw_dpy(), False) };
        if context.is_null() { panic!("Canvas: unable to open OpenGL context"); }
        if unsafe { glx::glXIsDirect(connection.get_raw_dpy(),context) } == 0 { panic!("Canvas: OpenGL context is indirect"); }
        unsafe { glx::glXMakeCurrent(connection.get_raw_dpy(),window as XID,context) };
        gl::load_with(|symbol| load_function(&symbol));
        Canvas {
            rect: Rect::new(0,0,width,height),
            aspect: aspect,
            draw: None,
            rect_changed: None,
            subs: Vec::new(),

            connection: connection,
            visual: visual,
            window: window,
            wm_motif_hints: wm_motif_hints,
            wm_protocols: wm_protocols,
            wm_delete_window: wm_delete_window,
            wm_transient_for: wm_transient_for,
            glx_create_context_attribs: glx_create_context_attribs,
            fbconfig: fbconfig,
            context: context,
            current_window: window,
        }
    }

    pub fn new_sub(&mut self,width: usize,height: usize,aspect: f32) -> usize {
        let screen = unsafe { (*self.visual).screen };
        let visual_screen = self.connection.get_setup().roots().nth(screen as usize).unwrap();
        let depth = unsafe { (*self.visual).depth };
        let visualid = unsafe { (*self.visual).visualid };
        let window = self.connection.generate_id();
        let colormap = self.connection.generate_id();
        xcb::create_colormap(&self.connection,xcb::COLORMAP_ALLOC_NONE as u8,colormap,visual_screen.root(),visualid as u32);
        let values = [
            (xcb::CW_BACK_PIXEL,visual_screen.white_pixel()),
            (xcb::CW_BORDER_PIXEL,visual_screen.black_pixel()),
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
            (xcb::CW_OVERRIDE_REDIRECT,1),
        ];
        xcb::create_window(&self.connection,depth as u8,window,visual_screen.root(),0,0,width as u16,height as u16,0,xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,visualid as u32,&values);
        let hints = [2u32,0,0,0,0];
        xcb::change_property(&self.connection,xcb::PROP_MODE_REPLACE as u8,window,self.wm_motif_hints,xcb::ATOM_ATOM,32,&hints);
        let transient = [self.window];
        xcb::change_property(&self.connection,xcb::PROP_MODE_REPLACE as u8,window,self.wm_transient_for,xcb::ATOM_ATOM,32,&transient);
        xcb::map_window(&self.connection, window);
        let context_attribs: [c_int; 5] = [
            GLX_CONTEXT_MAJOR_VERSION_ARB as c_int, 4,
            GLX_CONTEXT_MINOR_VERSION_ARB as c_int, 5,
            0,
        ];
        self.connection.flush();
        let sub = SubCanvas {
            rect: Rect::new(0,0,width,height),
            aspect: aspect,

            draw: None,
            rect_changed: None,

            window: window,
        };
        let id = self.subs.len();
        self.subs.push(sub);
        id
    }

    pub fn on_draw(&mut self,draw: fn(&Rect)) {
        self.draw = Some(draw);
    }

    pub fn on_rect_changed(&mut self,rect_changed: fn(&Rect)) {
        self.rect_changed = Some(rect_changed);
    }

    pub fn on_sub_draw(&mut self,id: usize,draw: fn(&Rect)) {
        self.subs[id].draw = Some(draw);
    }

    pub fn on_sub_rect_changed(&mut self,id: usize,rect_changed: fn(&Rect)) {
        self.subs[id].rect_changed = Some(rect_changed);
    }

    pub fn set_rect(&mut self,rect: &Rect) {
        self.rect = *rect;
        let values = [(xcb::CONFIG_WINDOW_X as u16,rect.orig.x as u32),(xcb::CONFIG_WINDOW_Y as u16,rect.orig.y as u32),(xcb::CONFIG_WINDOW_WIDTH as u16,rect.size.x as u32),(xcb::CONFIG_WINDOW_HEIGHT as u16,rect.size.y as u32)];
        xcb::configure_window(&self.connection,self.window,&values[0..]);
    }

    pub fn set_aspect(&mut self,aspect: f32) {
        self.aspect = aspect;
    }

    pub fn set_sub_rect(&mut self,id: usize,rect: &Rect) {
        self.subs[id].rect = *rect;
        let values = [(xcb::CONFIG_WINDOW_X as u16,rect.orig.x as u32),(xcb::CONFIG_WINDOW_Y as u16,rect.orig.y as u32),(xcb::CONFIG_WINDOW_WIDTH as u16,rect.size.x as u32),(xcb::CONFIG_WINDOW_HEIGHT as u16,rect.size.y as u32)];
        xcb::configure_window(&self.connection,self.subs[id].window,&values[0..]);        
    }

    pub fn set_sub_aspect(&mut self,id: usize,aspect: f32) {
        self.subs[id].aspect = aspect;
    }

    pub fn handle_events(&mut self) -> bool {
        loop {
            let event = self.connection.poll_for_event();
            match event {
                None => { break; }
                Some(event) => {
                    let r = event.response_type() & !0x80;
                    match r {
                        xcb::EXPOSE => {
                            let expose: &xcb::ExposeEvent = unsafe { xcb::cast_event(&event) };
                            let rect = Rect { orig: Vec2 { x: expose.x() as usize,y: expose.y() as usize }, size: Vec2 { x: expose.width() as usize,y: expose.height() as usize } };
                            for id in 0..self.subs.len() {
                                if expose.window() == self.subs[id].window {
                                    if self.current_window != self.subs[id].window {
                                        let result = unsafe { glx::glXMakeCurrent(self.connection.get_raw_dpy(),self.subs[id].window as XID,self.context) };
                                        self.current_window = self.subs[id].window;
                                    }
                                    unsafe {
                                        gl::ClearColor(0.1,0.2,0.4,1.0);
                                        gl::Clear(gl::COLOR_BUFFER_BIT);
                                    }
                                    if let Some(draw) = self.subs[id].draw {
                                        draw(&rect);
                                    }
                                    break;
                                }
                            }
                            if expose.window() == self.window {
                                if self.current_window != self.window {
                                    let result = unsafe { glx::glXMakeCurrent(self.connection.get_raw_dpy(),self.window as XID,self.context) };
                                    self.current_window = self.window;
                                }
                                unsafe {
                                    gl::ClearColor(0.4,0.2,0.1,1.0);
                                    gl::Clear(gl::COLOR_BUFFER_BIT);
                                }
                                if let Some(draw) = self.draw {
                                    draw(&rect);
                                }
                            }
                            unsafe {
                                gl::Flush();
                                glx::glXSwapBuffers(self.connection.get_raw_dpy(),self.current_window as XID);
                            }
                        },
                        xcb::KEY_PRESS => {
                            let key_press : &xcb::KeyPressEvent = unsafe { xcb::cast_event(&event) };
                            println!("key '{}' pressed", key_press.detail());
                        },
                        xcb::KEY_RELEASE => {
                            let key_release : &xcb::KeyReleaseEvent = unsafe { xcb::cast_event(&event) };
                            println!("key '{}' released", key_release.detail());
                        },
                        xcb::CONFIGURE_NOTIFY => {
                            let configure_notify : &xcb::ConfigureNotifyEvent = unsafe { xcb::cast_event(&event) };
                            let rect = Rect { orig: Vec2 { x: configure_notify.x() as usize,y: configure_notify.y() as usize }, size: Vec2 { x: configure_notify.width() as usize,y: configure_notify.height() as usize } };
                            for id in 0..self.subs.len() {
                                if configure_notify.window() == self.subs[id].window {
                                    self.subs[id].rect = rect;
                                    if let Some(rect_changed) = self.subs[id].rect_changed {
                                        rect_changed(&rect);
                                    }
                                    break;
                                }
                            }
                            if configure_notify.window() == self.window {
                                self.rect = rect;
                                if let Some(rect_changed) = self.rect_changed {
                                    rect_changed(&rect);
                                }
                            }
                        },
                        xcb::CLIENT_MESSAGE => {
                            let client_message : &xcb::ClientMessageEvent = unsafe { xcb::cast_event(&event) };
                            let data = &client_message.data().data;
                            let atom = (data[0] as u32) | ((data[1] as u32) << 8) | ((data[2] as u32) << 16) | ((data[3] as u32) << 24);
                            if atom == self.wm_delete_window {
                                return false;
                            }
                        },
                        _ => { },
                    }
                }
            }
        }
        true
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        unsafe { glx::glXMakeCurrent(self.connection.get_raw_dpy(),0,null_mut()); }
        for id in 0..self.subs.len() {
            xcb::unmap_window(&self.connection,self.subs[id].window);
            xcb::destroy_window(&self.connection,self.subs[id].window);
        }
        xcb::unmap_window(&self.connection,self.window);
        xcb::destroy_window(&self.connection,self.window);
        unsafe { glx::glXDestroyContext(self.connection.get_raw_dpy(),self.context) };
    }
}
