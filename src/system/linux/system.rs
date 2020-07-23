// E - System - Linux - System
// Desmond Germans, 2020

use crate::*;
use x11::{
    xlib::{
        XID,
        VisualID,
        Display,
        Bool,
        XFree,
        XSync,
        False,
        True,
    },
    glx::*,
    glx::arb::*,
};
use std::{
    os::{
        raw::{
            c_void,
            c_int,
        },
        unix::io::AsRawFd,
    },
    ffi::{
        CString,
        CStr,
    },
    mem::transmute,
    ptr::null_mut,
    rc::Rc,
};
use xcb::{
    base::{
        Connection,
        EventQueueOwner,
    },
    xproto::{
        intern_atom,
        create_colormap,
        COLORMAP_ALLOC_NONE,
        CW_EVENT_MASK,
        EVENT_MASK_EXPOSURE,
        EVENT_MASK_KEY_PRESS,
        EVENT_MASK_KEY_RELEASE,
        EVENT_MASK_BUTTON_PRESS,
        EVENT_MASK_BUTTON_RELEASE,
        EVENT_MASK_POINTER_MOTION,
        EVENT_MASK_STRUCTURE_NOTIFY,
        CW_COLORMAP,
        create_window,
        WINDOW_CLASS_INPUT_OUTPUT,
        change_property,
        PROP_MODE_REPLACE,
        ATOM_ATOM,
        ExposeEvent,
        KeyPressEvent,
        KeyReleaseEvent,
        ButtonPressEvent,
        ButtonReleaseEvent,
        MotionNotifyEvent,
        ConfigureNotifyEvent,
        ClientMessageEvent,
        destroy_window,
        EXPOSE,
        KEY_PRESS,
        KEY_RELEASE,
        BUTTON_PRESS,
        BUTTON_RELEASE,
        MOTION_NOTIFY,
        CONFIGURE_NOTIFY,
        CLIENT_MESSAGE,
    },
    cast_event,
    GenericEvent,
};
use libc::{
    epoll_create1,
    epoll_ctl,
    EPOLL_CTL_ADD,
    epoll_event,
    EPOLLIN,
    epoll_wait,
};

pub struct System {
    pub(crate) connection: Connection,
    pub(crate) hidden_window: XID,
    pub(crate) context: GLXContext,
    pub(crate) wm_delete_window: u32,
    pub(crate) rootwindow: XID,
    pub(crate) visualid: VisualID,
    pub(crate) depth: u8,
    pub(crate) wm_protocols: u32,
    pub(crate) colormap: XID,
    _wm_motif_hints: u32,
    _wm_transient_for: u32,
    _wm_net_type: u32,
    _wm_net_type_utility: u32,
    _wm_net_state: u32,
    _wm_net_state_above: u32,
    epfd: c_int,
    pub(crate) opengl: OpenGL,
}

type GlXCreateContextAttribsARBProc = unsafe extern "C" fn(
    dpy: *mut Display,
    fbc: GLXFBConfig,
    share_context: GLXContext,
    direct: Bool,
    attribs: *const c_int
) -> GLXContext;

fn load_function(name: &str) -> *mut c_void {
    let newname = CString::new(name).unwrap();
    let pointer: *mut c_void = unsafe {
        transmute(
            glXGetProcAddress(newname.as_ptr() as *const u8)
        )
    };
    if pointer.is_null() {
        panic!("(linux, ui+opengl) unable to access {}",name);
    }
    pointer
}

impl System {
    pub fn new() -> Result<System,SystemError> {
        let connection = match Connection::connect_with_xlib_display() {
            Ok((connection,_)) => connection,
            Err(_) => { return Err(SystemError::Generic); },
        };
        connection.set_event_queue_owner(EventQueueOwner::Xcb);
        let fd = connection.as_raw_fd();

        let (visual_screen,visualid,depth,fbconfig,glx_create_context_attribs) = {
            let mut glxmaj: c_int = 0;
            let mut glxmin: c_int = 0;
            unsafe {
                if glXQueryVersion(
                    connection.get_raw_dpy(),
                    &mut glxmaj as *mut c_int,
                    &mut glxmin as *mut c_int
                ) == 0 {
                    return Err(SystemError::Generic);
                }
            }
            if (glxmaj * 100 + glxmin) < 103 {
                return Err(SystemError::Generic);
            }
            let attribs = [
                GLX_X_RENDERABLE,  1,
                GLX_DRAWABLE_TYPE, GLX_WINDOW_BIT,
                GLX_RENDER_TYPE,   GLX_RGBA_BIT,
                GLX_X_VISUAL_TYPE, GLX_TRUE_COLOR,
                GLX_RED_SIZE,      8,
                GLX_GREEN_SIZE,    8,
                GLX_BLUE_SIZE,     8,
                GLX_ALPHA_SIZE,    8,
                GLX_DEPTH_SIZE,    24,
                GLX_STENCIL_SIZE,  8,
                GLX_DOUBLEBUFFER,  1,
                0,
            ];
            let mut fbcount: c_int = 0;
            let fbconfigs = unsafe {
                glXChooseFBConfig(
                    connection.get_raw_dpy(),
                    0,
                    attribs.as_ptr(),
                    &mut fbcount as *mut c_int
                )
            };
            if fbcount == 0 {
                return Err(SystemError::Generic);
            }
            let fbconfig = unsafe { *fbconfigs };
            unsafe { XFree(fbconfigs as *mut c_void); }
            let visual = unsafe { glXGetVisualFromFBConfig(connection.get_raw_dpy(),fbconfig) };
            let screen = unsafe { (*visual).screen };
            let visual_screen = connection.get_setup().roots().nth(screen as usize).unwrap();
            let depth = unsafe { (*visual).depth } as u8;
            let visualid = unsafe { (*visual).visualid };
            let extensions = unsafe {
                CStr::from_ptr(glXQueryExtensionsString(connection.get_raw_dpy(),screen))
            }.to_str().unwrap();
            let mut found = false;
            for extension in extensions.split(" ") {
                if extension == "GLX_ARB_create_context" {
                    found = true;
                    break;
                }
            }
            if !found {
                return Err(SystemError::Generic);
            }
            let glx_create_context_attribs: GlXCreateContextAttribsARBProc = unsafe {
                transmute(load_function("glXCreateContextAttribsARB"))
            };
            (visual_screen,visualid,depth,fbconfig,glx_create_context_attribs)
        };

        let protocols_com = intern_atom(&connection,false,"WM_PROTOCOLS");
        let delete_window_com = intern_atom(&connection,false,"WM_DELETE_WINDOW");
        let motif_hints_com = intern_atom(&connection,false,"_MOTIF_WM_HINTS");
        let transient_for_com = intern_atom(&connection,false,"WM_TRANSIENT_FOR");
        let net_type_com = intern_atom(&connection,false,"_NET_WM_TYPE");
        let net_type_utility_com = intern_atom(&connection,false,"_NET_WM_TYPE_UTILITY");
        let net_state_com = intern_atom(&connection,false,"_NET_WM_STATE");
        let net_state_above_com = intern_atom(&connection,false,"_NET_WM_STATE_ABOVE");
        let wm_protocols = match protocols_com.get_reply() {
            Ok(protocols) => protocols.atom(),
            Err(_) => { return Err(SystemError::Generic); },
        };
        let wm_delete_window = match delete_window_com.get_reply() {
            Ok(delete_window) => delete_window.atom(),
            Err(_) => { return Err(SystemError::Generic); },
        };
        let wm_motif_hints = match motif_hints_com.get_reply() {
            Ok(motif_hints) => motif_hints.atom(),
            Err(_) => { return Err(SystemError::Generic); },
        };
        let wm_transient_for = match transient_for_com.get_reply() {
            Ok(transient_for) => transient_for.atom(),
            Err(_) => { return Err(SystemError::Generic); },
        };
        let wm_net_type = match net_type_com.get_reply() {
            Ok(net_type) => net_type.atom(),
            Err(_) => { return Err(SystemError::Generic); },
        };
        let wm_net_type_utility = match net_type_utility_com.get_reply() {
            Ok(net_type_utility) => net_type_utility.atom(),
            Err(_) => { return Err(SystemError::Generic); },
        };
        let wm_net_state = match net_state_com.get_reply() {
            Ok(net_state) => net_state.atom(),
            Err(_) => { return Err(SystemError::Generic); },
        };
        let wm_net_state_above = match net_state_above_com.get_reply() {
            Ok(net_state_above) => net_state_above.atom(),
            Err(_) => { return Err(SystemError::Generic); },
        };
        
        let rootwindow = visual_screen.root() as XID;
        let hidden_window = connection.generate_id() as XID;
        let colormap = connection.generate_id() as XID;
        create_colormap(
            &connection,
            COLORMAP_ALLOC_NONE as u8,
            colormap as u32,
            rootwindow as u32,
            visualid as u32
        );
        let values = [
            (CW_EVENT_MASK,
                EVENT_MASK_EXPOSURE
                | EVENT_MASK_KEY_PRESS
                | EVENT_MASK_KEY_RELEASE
                | EVENT_MASK_BUTTON_PRESS
                | EVENT_MASK_BUTTON_RELEASE
                | EVENT_MASK_POINTER_MOTION
                | EVENT_MASK_STRUCTURE_NOTIFY
            ),
            (CW_COLORMAP,colormap as u32),
        ];
        create_window(
            &connection,
            depth,
            hidden_window as u32,
            rootwindow as u32,
            0,0,1,1,
            0,
            WINDOW_CLASS_INPUT_OUTPUT as u16,
            visualid as u32,
            &values
        );
        unsafe {
            connection.flush();
            XSync(connection.get_raw_dpy(),False);
        }

        let protocol_set = [wm_delete_window];
        change_property(&connection,PROP_MODE_REPLACE as u8,hidden_window as u32,wm_protocols,ATOM_ATOM,32,&protocol_set);

        let context = {
            let context_attribs: [c_int; 5] = [
                GLX_CONTEXT_MAJOR_VERSION_ARB as c_int,4,
                GLX_CONTEXT_MINOR_VERSION_ARB as c_int,5,
                0,
            ];
            let context = unsafe {
                glx_create_context_attribs(
                    connection.get_raw_dpy(),
                    fbconfig,
                    null_mut(),
                    True,
                    &context_attribs[0] as *const c_int
                )
            };
            connection.flush();
            unsafe { XSync(connection.get_raw_dpy(),False) };
            if context.is_null() {
                return Err(SystemError::Generic);
            }
            if unsafe { glXIsDirect(connection.get_raw_dpy(),context) } == 0 {
                return Err(SystemError::Generic);
            }
            unsafe { glXMakeCurrent(connection.get_raw_dpy(),hidden_window,context) };
            gl::load_with(|symbol| load_function(&symbol));
            context
        };

        let epfd = unsafe { epoll_create1(0) };
        let mut epe = [epoll_event { events: EPOLLIN as u32,u64: 0, }];
        unsafe { epoll_ctl(epfd,EPOLL_CTL_ADD,fd,epe.as_mut_ptr()) };

        let opengl = match OpenGL::new() {
            Ok(opengl) => opengl,
            Err(_) => { return Err(SystemError::Generic) },
        };

        Ok(System {
            connection: connection,
            hidden_window: hidden_window,
            context: context,
            wm_delete_window: wm_delete_window,
            rootwindow: rootwindow,
            visualid: visualid,
            depth: depth,
            wm_protocols: wm_protocols,
            colormap: colormap,
            _wm_motif_hints: wm_motif_hints,
            _wm_transient_for: wm_transient_for,
            _wm_net_type: wm_net_type,
            _wm_net_type_utility: wm_net_type_utility,
            _wm_net_state: wm_net_state,
            _wm_net_state_above: wm_net_state_above,
            epfd: epfd,
            opengl: opengl,
        })
    }

    fn parse_xevent(&self,xcb_event: GenericEvent) -> Option<(XID,Event)> {
        let r = xcb_event.response_type() & !0x80;
        match r {
            EXPOSE => {
                let expose: &ExposeEvent = unsafe { cast_event(&xcb_event) };
                let r = rect!(expose.x() as isize,expose.y() as isize,expose.width() as isize,expose.height() as isize);
                let id = expose.window() as XID;
                return Some((id,Event::Paint(r)));
            },
            KEY_PRESS => {
                let key_press: &KeyPressEvent = unsafe { cast_event(&xcb_event) };
                let k = key_press.detail() as u8;
                let id = key_press.event() as XID;
                return Some((id,Event::KeyPress(k)));
            },
            KEY_RELEASE => {
                let key_release: &KeyReleaseEvent = unsafe { cast_event(&xcb_event) };
                let k = key_release.detail() as u8;
                let id = key_release.event() as XID;
                return Some((id,Event::KeyRelease(k)));
            },
            BUTTON_PRESS => {
                let button_press: &ButtonPressEvent = unsafe { cast_event(&xcb_event) };
                let p = vec2!(button_press.event_x() as isize,button_press.event_y() as isize);
                let id = button_press.event() as XID;
                match button_press.detail() {
                    1 => { return Some((id,Event::MousePress(p,Mouse::Left))); },
                    2 => { return Some((id,Event::MousePress(p,Mouse::Middle))); },
                    3 => { return Some((id,Event::MousePress(p,Mouse::Right))); },
                    4 => { return Some((id,Event::MouseWheel(Wheel::Up))); },
                    5 => { return Some((id,Event::MouseWheel(Wheel::Down))); },
                    6 => { return Some((id,Event::MouseWheel(Wheel::Left))); },
                    7 => { return Some((id,Event::MouseWheel(Wheel::Right))); },
                    _ => { },
                }        
            },
            BUTTON_RELEASE => {
                let button_release: &ButtonReleaseEvent = unsafe { cast_event(&xcb_event) };
                let p = vec2!(button_release.event_x() as isize,button_release.event_y() as isize);
                let id = button_release.event() as XID;
                match button_release.detail() {
                    1 => { return Some((id,Event::MouseRelease(p,Mouse::Left))); },
                    2 => { return Some((id,Event::MouseRelease(p,Mouse::Middle))); },
                    3 => { return Some((id,Event::MouseRelease(p,Mouse::Right))); },
                    _ => { },
                }        
            },
            MOTION_NOTIFY => {
                let motion_notify: &MotionNotifyEvent = unsafe { cast_event(&xcb_event) };
                let p = vec2!(motion_notify.event_x() as isize,motion_notify.event_y() as isize);
                let id = motion_notify.event() as XID;
                return Some((id,Event::MouseMove(p)));
            },
            CONFIGURE_NOTIFY => {
                let configure_notify: &ConfigureNotifyEvent = unsafe { cast_event(&xcb_event) };
                let s = vec2!(configure_notify.width() as isize,configure_notify.height() as isize);
                let id = configure_notify.event() as XID;
                return Some((id,Event::Resize(s)));
            },
            CLIENT_MESSAGE => {
                let client_message : &ClientMessageEvent = unsafe { cast_event(&xcb_event) };
                let data = &client_message.data().data;
                let atom = (data[0] as u32) | ((data[1] as u32) << 8) | ((data[2] as u32) << 16) | ((data[3] as u32) << 24);
                if atom == self.wm_delete_window {
                    let id = client_message.window() as XID;
                    return Some((id,Event::Close));
                }
            },
            _ => { },
        }
        None
    }

    pub fn poll<T: Pollable>(&self,targets: &T) -> T::Result {
        targets.poll(&self)
    }
    
    pub fn wait(&self) {
        let mut epe = [epoll_event { events: EPOLLIN as u32,u64: 0, }];
        unsafe { epoll_wait(self.epfd,epe.as_mut_ptr(),1,-1) };
    }    

    pub fn clear<T>(&self,color: T) where Vec4<f32>: From<T> {
        self.opengl.clear(color);
    }

    pub fn draw_triangle_fan(&self,n: i32) {
        self.opengl.draw_triangle_fan(n);
    }

    pub fn draw_triangles(&self,n: i32) {
        self.opengl.draw_triangles(n);
    }

    pub fn set_blend(&self,mode: BlendMode) {
        self.opengl.set_blend(mode);
    }

    pub fn create_shader(&self,
        vertex_src: &str,
        geometry_src: Option<&str>,
        fragment_src: &str,
    ) -> Result<Shader,SystemError> {
        self.opengl.create_shader(vertex_src,geometry_src,fragment_src)
    }

    pub fn bind_shader(&self,shader: &Shader) {
        self.opengl.bind_shader(shader);
    }

    pub fn unbind_shader(&self) {
        self.opengl.unbind_shader();
    }

    pub fn set_uniform<T: OpenGLUniform>(&self,name: &str,value: T) {
        self.opengl.set_uniform(name,value);
    }

    pub fn create_vertexbuffer<T: Vertex>(&self,vertices: Vec<T>) -> Result<VertexBuffer<T>,SystemError> {
        self.opengl.create_vertexbuffer(vertices)
    }

    pub fn bind_vertexbuffer<T: Vertex>(&self,vertexbuffer: &VertexBuffer<T>) {
        self.opengl.bind_vertexbuffer(vertexbuffer);
    }

    pub fn unbind_vertexbuffer(&self) {
        self.opengl.unbind_vertexbuffer();
    }

    pub fn create_framebuffer(&self,size: Vec2<usize>) -> Result<Framebuffer,SystemError> {
        self.opengl.create_framebuffer(size)
    }

    pub fn bind_framebuffer(&self,framebuffer: &Framebuffer) {
        self.opengl.bind_framebuffer(framebuffer);
    }

    pub fn unbind_framebuffer(&self) {
        self.opengl.unbind_framebuffer();
    }

    pub fn bind_framebuffer_as_texture2d(&self,layer: usize,framebuffer: &Framebuffer) {
        self.opengl.bind_framebuffer_as_texture2d(layer,framebuffer);
    }

    pub fn create_texture2d<T: OpenGLFormat>(&self,image: &Mat<T>) -> Result<Texture2D<T>,SystemError> {
        self.opengl.create_texture2d(image)
    }

    pub fn bind_texture2d<T: OpenGLFormat>(&self,layer: usize,texture: &Texture2D<T>) {
        self.opengl.bind_texture2d(layer,texture);
    }

    pub fn unbind_texture2d(&self,layer: usize) {
        self.opengl.unbind_texture2d(layer);
    }
}

impl Drop for System {
    fn drop(&mut self) {
        unsafe { glXMakeCurrent(self.connection.get_raw_dpy(),0,null_mut()); }
        destroy_window(&self.connection,self.hidden_window as u32);
        unsafe { glXDestroyContext(self.connection.get_raw_dpy(),self.context); }
    }
}

pub trait Pollable {
    type Result;
    fn poll(&self,system: &System) -> <Self as Pollable>::Result;
}

impl Pollable for Rc<Window> {
    type Result = Vec<Event>;
    fn poll(&self,system: &System) -> <Self as Pollable>::Result {
        let mut events: Vec<Event> = Vec::new();
        while let Some(xcb_event) = system.connection.poll_for_event() {
            if let Some((xid,event)) = system.parse_xevent(xcb_event) {
                if xid == self.id {
                    events.push(event);
                }
            }
        }
        events
    }
}

impl Pollable for Vec<Rc<Window>> {
    type Result = Vec<(Rc<Window>,Event)>;
    fn poll(&self,system: &System) -> <Self as Pollable>::Result {
        let mut events: Vec<(Rc<Window>,Event)> = Vec::new();
        while let Some(xcb_event) = system.connection.poll_for_event() {
            if let Some((xid,event)) = system.parse_xevent(xcb_event) {
                for window in self.iter() {
                    if xid == window.id {
                        events.push((Rc::clone(window),event));
                        break;
                    }
                }
            }
        }
        events
    }
}
