// E - System - Linux
// Desmond Germans, 2020

use crate::*;
use crate::prelude::*;
use x11::xlib::*;
use std::os::raw::c_void;
use std::os::raw::c_int;
use std::ffi::CString;
use std::mem::transmute;
use xcb::base::Connection;
use xcb::base::EventQueueOwner;
use x11::glx::*;
use std::ffi::CStr;
use xcb::xproto::*;
use x11::glx::arb::*;
use std::ptr::null_mut;
use xcb::cast_event;
use xcb::GenericEvent;
use libc::epoll_create1;
use libc::epoll_ctl;
use libc::EPOLL_CTL_ADD;
use libc::epoll_event;
use libc::EPOLLIN;
use std::os::unix::io::AsRawFd;
use libc::epoll_wait;
use std::cell::Cell;
use std::rc::Rc;

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

struct Window<'a> {
    window: XID,
    size: Cell<Vec2<usize>>,
    handler: Box<dyn Fn(Event) + 'a>,
}

pub struct UI<'a> {
    connection: Connection,
    hidden_window: XID,
    context: GLXContext,
    wm_delete_window: u32,
    rootwindow: XID,
    visualid: VisualID,
    depth: u8,
    wm_protocols: u32,
    colormap: XID,
    _wm_motif_hints: u32,
    _wm_transient_for: u32,
    _wm_net_type: u32,
    _wm_net_type_utility: u32,
    _wm_net_state: u32,
    _wm_net_state_above: u32,
    windows: Vec<Window<'a>>,
    epfd: c_int,
    graphics: Rc<Graphics>,
}

impl<'a> UI<'a> {
    pub fn new() -> Result<UI<'a>,UIError> {

        let connection = match Connection::connect_with_xlib_display() {
            Ok((connection,_)) => connection,
            Err(_) => { return Err(UIError::Generic); },
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
                    return Err(UIError::Generic);
                }
            }
            if (glxmaj * 100 + glxmin) < 103 {
                return Err(UIError::Generic);
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
                return Err(UIError::Generic);
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
                return Err(UIError::Generic);
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
            Err(_) => { return Err(UIError::Generic); },
        };
        let wm_delete_window = match delete_window_com.get_reply() {
            Ok(delete_window) => delete_window.atom(),
            Err(_) => { return Err(UIError::Generic); },
        };
        let wm_motif_hints = match motif_hints_com.get_reply() {
            Ok(motif_hints) => motif_hints.atom(),
            Err(_) => { return Err(UIError::Generic); },
        };
        let wm_transient_for = match transient_for_com.get_reply() {
            Ok(transient_for) => transient_for.atom(),
            Err(_) => { return Err(UIError::Generic); },
        };
        let wm_net_type = match net_type_com.get_reply() {
            Ok(net_type) => net_type.atom(),
            Err(_) => { return Err(UIError::Generic); },
        };
        let wm_net_type_utility = match net_type_utility_com.get_reply() {
            Ok(net_type_utility) => net_type_utility.atom(),
            Err(_) => { return Err(UIError::Generic); },
        };
        let wm_net_state = match net_state_com.get_reply() {
            Ok(net_state) => net_state.atom(),
            Err(_) => { return Err(UIError::Generic); },
        };
        let wm_net_state_above = match net_state_above_com.get_reply() {
            Ok(net_state_above) => net_state_above.atom(),
            Err(_) => { return Err(UIError::Generic); },
        };
        
        let rootwindow = visual_screen.root() as XID;
        let window = connection.generate_id() as XID;
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
            window as u32,
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
        change_property(&connection,PROP_MODE_REPLACE as u8,window as u32,wm_protocols,ATOM_ATOM,32,&protocol_set);

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
                return Err(UIError::Generic);
            }
            if unsafe { glXIsDirect(connection.get_raw_dpy(),context) } == 0 {
                return Err(UIError::Generic);
            }
            unsafe { glXMakeCurrent(connection.get_raw_dpy(),window,context) };
            gl::load_with(|symbol| load_function(&symbol));
            context
        };

        let epfd = unsafe { epoll_create1(0) };
        let mut epe = [epoll_event { events: EPOLLIN as u32,u64: 0, }];
        unsafe { epoll_ctl(epfd,EPOLL_CTL_ADD,fd,epe.as_mut_ptr()) };

        Ok(UI {
            connection: connection,
            hidden_window: window,
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
            windows: Vec::new(),
            epfd: epfd,
            graphics: Rc::new(Graphics::new()),
        })
    }

    pub fn create_window(&mut self,r: Rect<isize>,title: &str,handler: impl Fn(Event) + 'a) -> bool {
        let window = self.connection.generate_id() as XID;
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
            (CW_COLORMAP,self.colormap as u32),
        ];
        create_window(
            &self.connection,
            self.depth as u8,
            window as u32,
            self.rootwindow as u32,
            r.o.x as i16,r.o.y as i16,r.s.x as u16,r.s.y as u16,
            0,
            WINDOW_CLASS_INPUT_OUTPUT as u16,
            self.visualid as u32,
            &values
        );
        unsafe {
            map_window(&self.connection,window as u32);
            self.connection.flush();
            XSync(self.connection.get_raw_dpy(),False);
        }
        change_property(
            &self.connection,
            PROP_MODE_REPLACE as u8,
            window as u32,
            ATOM_WM_NAME,
            ATOM_STRING,
            8,
            title.as_bytes()
        );
        let protocol_set = [self.wm_delete_window];
        change_property(
            &self.connection,
            PROP_MODE_REPLACE as u8,
            window as u32,
            self.wm_protocols,
            ATOM_ATOM,
            32,
            &protocol_set
        );
        self.connection.flush();
        self.windows.push(
            Window {
                window: window,
                size: Cell::new(vec2!(r.s.x as usize,r.s.y as usize)),
                handler: Box::new(handler),
            }
        );
        true
    }

    fn handle_event(&mut self,xcb_event: GenericEvent) {
        let r = xcb_event.response_type() & !0x80;
        match r {
            EXPOSE => {
                let expose: &ExposeEvent = unsafe { cast_event(&xcb_event) };
                //let r = rect!(expose.x() as isize,expose.y() as isize,expose.width() as isize,expose.height() as isize);
                let id = expose.window() as XID;
                for window in &mut self.windows {
                    if window.window == id {
                        unsafe { glXMakeCurrent(self.connection.get_raw_dpy(),window.window,self.context) };
                        let size = window.size.get();
                        unsafe { gl::Viewport(0,0,size.x as i32,size.y as i32) };
                        unsafe { gl::Scissor(0,0,size.x as i32,size.y as i32) };
                        self.graphics.set_window_size(size);
                        let space = self.graphics.get_space();
                        (window.handler)(Event::Paint(Rc::clone(&self.graphics),space));
                        unsafe { gl::Flush() };
                        unsafe { glXSwapBuffers(self.connection.get_raw_dpy(),window.window) };
                    }
                }
            },
            KEY_PRESS => {
                let key_press: &KeyPressEvent = unsafe { cast_event(&xcb_event) };
                let k = key_press.detail() as u8;
                let id = key_press.event() as XID;
                for window in &mut self.windows {
                    if window.window == id {
                        (window.handler)(Event::KeyPress(k));
                    }
                }
            },
            KEY_RELEASE => {
                let key_release: &KeyReleaseEvent = unsafe { cast_event(&xcb_event) };
                let k = key_release.detail() as u8;
                let id = key_release.event() as XID;
                for window in &mut self.windows {
                    if window.window == id {
                        (window.handler)(Event::KeyRelease(k));
                    }
                }
            },
            BUTTON_PRESS => {
                let button_press: &ButtonPressEvent = unsafe { cast_event(&xcb_event) };
                let p = vec2!(button_press.event_x() as isize,button_press.event_y() as isize);
                let id = button_press.event() as XID;
                for window in &mut self.windows {
                    if window.window == id {
                        match button_press.detail() {
                            1 => { (window.handler)(Event::MousePress(p,Mouse::Left)); },
                            2 => { (window.handler)(Event::MousePress(p,Mouse::Middle)); },
                            3 => { (window.handler)(Event::MousePress(p,Mouse::Right)); },
                            4 => { (window.handler)(Event::MouseWheel(Wheel::Up)); },
                            5 => { (window.handler)(Event::MouseWheel(Wheel::Down)); },
                            6 => { (window.handler)(Event::MouseWheel(Wheel::Left)); },
                            7 => { (window.handler)(Event::MouseWheel(Wheel::Right)); },
                            _ => { },
                        }
                        break;
                    }
                }
            },
            BUTTON_RELEASE => {
                let button_release: &ButtonReleaseEvent = unsafe { cast_event(&xcb_event) };
                let p = vec2!(button_release.event_x() as isize,button_release.event_y() as isize);
                let id = button_release.event() as XID;
                for window in &mut self.windows {
                    if window.window == id {
                        match button_release.detail() {
                            1 => { (window.handler)(Event::MouseRelease(p,Mouse::Left)); },
                            2 => { (window.handler)(Event::MouseRelease(p,Mouse::Middle)); },
                            3 => { (window.handler)(Event::MouseRelease(p,Mouse::Right)); },
                            _ => { },
                        }
                        break;
                    }
                }
            },
            MOTION_NOTIFY => {
                let motion_notify: &MotionNotifyEvent = unsafe { cast_event(&xcb_event) };
                let p = vec2!(motion_notify.event_x() as isize,motion_notify.event_y() as isize);
                let id = motion_notify.event() as XID;
                for window in &mut self.windows {
                    if window.window == id {
                        (window.handler)(Event::MouseMove(p));
                    }
                }
            },
            CONFIGURE_NOTIFY => {
                let configure_notify: &ConfigureNotifyEvent = unsafe { cast_event(&xcb_event) };
                let s = vec2!(configure_notify.width() as isize,configure_notify.height() as isize);
                let id = configure_notify.event() as XID;
                for window in &mut self.windows {
                    if window.window == id {
                        window.size.set(vec2!(s.x as usize,s.y as usize));
                        (window.handler)(Event::Resize(s));
                    }
                }
            },
            CLIENT_MESSAGE => {
                let client_message : &ClientMessageEvent = unsafe { cast_event(&xcb_event) };
                let data = &client_message.data().data;
                let atom = (data[0] as u32) | ((data[1] as u32) << 8) | ((data[2] as u32) << 16) | ((data[3] as u32) << 24);
                let id = client_message.window() as XID;
                if atom == self.wm_delete_window {
                    for window in &mut self.windows {
                        if window.window == id {
                            (window.handler)(Event::Close);
                        }
                    }
                }
            },
            _ => { },
        }
    }

    pub fn pump(&mut self) {
        while let Some(xcb_event) = self.connection.poll_for_event() {
            self.handle_event(xcb_event);
        }
    }

    pub fn wait(&self) {
        let mut epe = [epoll_event { events: EPOLLIN as u32,u64: 0, }];
        unsafe { epoll_wait(self.epfd,epe.as_mut_ptr(),1,-1) };
    }

    pub fn graphics(&self) -> Rc<Graphics> {
        Rc::clone(&self.graphics)
    }
}

impl<'a> Drop for UI<'a> {
    fn drop(&mut self) {
        unsafe { glXMakeCurrent(self.connection.get_raw_dpy(),0,null_mut()); }
        for window in &self.windows {
            unmap_window(&self.connection,window.window as u32);
            destroy_window(&self.connection,window.window as u32);
        }
        destroy_window(&self.connection,self.hidden_window as u32);
        unsafe { glXDestroyContext(self.connection.get_raw_dpy(),self.context); }
    }
}

/*impl<'a> PopupWindow<'a> {
    pub fn new(ui: &'a UI,r: &isize_r,owner: &AppWindow) -> PopupWindow<'a> {
        let window = create_window_base(ui,r);
        let net_type = [ui.wm_net_type_utility];
        change_property(&ui.connection,PROP_MODE_REPLACE as u8,window as u32,ui.wm_net_type,ATOM_ATOM,32,&net_type);
        let net_state = [ui.wm_net_state_above];
        change_property(&ui.connection,PROP_MODE_REPLACE as u8,window as u32,ui.wm_net_state,ATOM_ATOM,32,&net_state);
        let hints = [2u32,0,0,0,0];
        change_property(&ui.connection,PROP_MODE_REPLACE as u8,window as u32,ui.wm_motif_hints,ATOM_ATOM,32,&hints);
        let transient = [owner.window as u32];
        change_property(&ui.connection,PROP_MODE_REPLACE as u8,window as u32,ui.wm_transient_for,ATOM_ATOM,32,&transient);
        ui.connection.flush();
        PopupWindow {
            ui: ui,
            window: window,
        }
    }
}*/