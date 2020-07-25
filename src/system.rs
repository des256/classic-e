// E - System
// Desmond Germans, 2020

use {
    crate::*,
    std::{
        os::{
            raw::{
                c_void,
                c_int,
            },
        },
        ffi::CString,
        mem::transmute,
        ptr::null_mut,
        rc::Rc,
    },
};

#[cfg(target_os="linux")]
use {
    std::{
        os::unix::ui::AsRawFd,
        ffi::CStr,
    },
    x11::{
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
    },
    xcb::{
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
    },
    libc::{
        epoll_create1,
        epoll_ctl,
        EPOLL_CTL_ADD,
        epoll_event,
        EPOLLIN,
        epoll_wait,
    },
};
#[cfg(target_os="windows")]
use {
    std::{
        os::windows::ffi::OsStrExt,
        ffi::OsStr,
        iter::once,
    },
    winapi::{
        shared::{
            windef::{
                HDC,
                HGLRC,
                HWND,
                RECT,
                POINT,
            },
            minwindef::{
                UINT,
                LPARAM,
                WPARAM,
                HINSTANCE,
                LRESULT,
                FLOAT,
                BOOL,
                FALSE,
            },
        },
        um::{
            winuser::{
                PostMessageW,
                WM_USER,
                WM_SIZE,
                WM_CLOSE,
                WM_KEYDOWN,
                WM_KEYUP,
                WM_LBUTTONDOWN,
                WM_LBUTTONUP,
                WM_MBUTTONDOWN,
                WM_MBUTTONUP,
                WM_RBUTTONDOWN,
                WM_RBUTTONUP,
                WM_MOUSEWHEEL,
                WM_MOUSEMOVE,
                WM_PAINT,
                DefWindowProcW,
                WNDCLASSW,
                CS_VREDRAW,
                CS_HREDRAW,
                CS_OWNDC,
                LoadIconW,
                LoadCursorW,
                IDI_WINLOGO,
                IDC_ARROW,
                RegisterClassW,
                CreateWindowExW,
                WS_CLIPSIBLINGS,
                WS_CLIPCHILDREN,
                GetDC,
                ReleaseDC,
                DestroyWindow,
                MSG,
                BeginPaint,
                EndPaint,
                PAINTSTRUCT,
                WaitMessage,
                PeekMessageW,
                PM_REMOVE,
                TranslateMessage,
                DispatchMessageW,
            },
            wingdi::{
                wglGetProcAddress,
                PIXELFORMATDESCRIPTOR,
                PFD_DRAW_TO_WINDOW,
                PFD_SUPPORT_OPENGL,
                PFD_DOUBLEBUFFER,
                PFD_TYPE_RGBA,
                ChoosePixelFormat,
                SetPixelFormat,
                DescribePixelFormat,
                wglCreateContext,
                wglMakeCurrent,
                wglDeleteContext,
            },
            libloaderapi::{
                GetProcAddress,
                GetModuleHandleW,
                LoadLibraryW,
            },
        },
    },
};

#[cfg(target_os="linux")]
#[doc(hidden)]
type GlXCreateContextAttribsARBProc = unsafe extern "C" fn(
    dpy: *mut Display,
    fbc: GLXFBConfig,
    share_context: GLXContext,
    direct: Bool,
    attribs: *const c_int
) -> GLXContext;

#[cfg(target_os="linux")]
#[doc(hidden)]
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

#[cfg(target_os="windows")]
const WGL_DRAW_TO_WINDOW_ARB: c_int = 0x2001;
#[cfg(target_os="windows")]
const WGL_SUPPORT_OPENGL_ARB: c_int = 0x2010;
#[cfg(target_os="windows")]
const WGL_DOUBLE_BUFFER_ARB: c_int = 0x2011;
#[cfg(target_os="windows")]
const WGL_ACCELERATION_ARB: c_int = 0x2003;
#[cfg(target_os="windows")]
const WGL_PIXEL_TYPE_ARB: c_int = 0x2013;
#[cfg(target_os="windows")]
const WGL_COLOR_BITS_ARB: c_int = 0x2014;
#[cfg(target_os="windows")]
const WGL_ALPHA_BITS_ARB: c_int = 0x201B;
#[cfg(target_os="windows")]
const WGL_DEPTH_BITS_ARB: c_int = 0x2022;
#[cfg(target_os="windows")]
const WGL_STENCIL_BITS_ARB: c_int = 0x2023;
#[cfg(target_os="windows")]
const WGL_SAMPLE_BUFFERS_ARB: c_int = 0x2041;
#[cfg(target_os="windows")]
const WGL_SAMPLES_ARB: c_int = 0x2042;
#[cfg(target_os="windows")]
const WGL_TYPE_RGBA_ARB: c_int = 0x202B;
#[cfg(target_os="windows")]
const WGL_FULL_ACCELERATION_ARB: c_int = 0x2027;
#[cfg(target_os="windows")]
const WGL_CONTEXT_MAJOR_VERSION_ARB: c_int = 0x2091;
#[cfg(target_os="windows")]
const WGL_CONTEXT_MINOR_VERSION_ARB: c_int = 0x2092;
#[cfg(target_os="windows")]
const WGL_CONTEXT_PROFILE_MASK_ARB: c_int = 0x9126;
#[cfg(target_os="windows")]
const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: c_int = 0x00000001;

#[cfg(target_os="windows")]
type WglChoosePixelFormatARBProc = unsafe extern "C" fn(
    hdc: HDC,
    piAttribIList: *const c_int,
    pfAttribFList: *const FLOAT,
    nMaxFormats: UINT,
    piFormats: *mut c_int,
    nNumFormats: *mut UINT
) -> BOOL;

#[cfg(target_os="windows")]
type WglCreateContextAttribsARBProc = unsafe extern "C" fn(
    hdc: HDC,
    hShareContext: HGLRC,
    attribList: *const c_int
) -> HGLRC;

#[cfg(target_os="windows")]
pub(crate) fn win32_string(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

#[cfg(target_os="windows")]
unsafe extern "system" fn win32_proc(
    hwnd: HWND,
    message: UINT,
    wparam: WPARAM,
    lparam: LPARAM
) -> LRESULT {
    // HACK: WM_SIZE and WM_CLOSE appear not to be handled over the queue, so when the window proc gets these messages, send them back as WM_USER messages to the queue
    match message {
        WM_SIZE => {
            PostMessageW(hwnd,WM_USER,WM_SIZE as usize,lparam);
            0
        },
        WM_CLOSE => {
            PostMessageW(hwnd,WM_USER,WM_CLOSE as usize,0);
            0
        },
        _ => {
            DefWindowProcW(hwnd,message,wparam,lparam)
        }
    }
}

#[cfg(target_os="windows")]
fn load_function(hinstance: HINSTANCE,name: &str) -> *mut c_void {
    let newname = CString::new(name).unwrap();
    let mut pointer: *mut c_void = unsafe {
        transmute(
            wglGetProcAddress(newname.as_ptr() as *const i8)
        )
    };
    if pointer.is_null() {
        pointer = unsafe {
            transmute(
                GetProcAddress(hinstance,newname.as_ptr() as *const i8)
            )
        };
    }
    if pointer.is_null() {
        panic!("unable to access OpenGL function {}",name);
    }
    pointer
}

/// System context.
#[cfg(target_os="linux")]
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
}
#[cfg(target_os="windows")]
pub struct System {
    pub(crate) hinstance: HINSTANCE,
    pub(crate) pfid: i32,
    pub(crate) pfd: PIXELFORMATDESCRIPTOR,
    pub(crate) hglrc: HGLRC,
    pub(crate) hidden_hdc: HDC,
    pub(crate) hidden_hwnd: HWND,
}

impl System {
    #[cfg(target_os="linux")]
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
        })
    }

#[cfg(target_os="windows")]
    pub fn new() -> Result<System,SystemError> {
        let hinstance = unsafe { GetModuleHandleW(null_mut()) };
        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
            lpfnWndProc: Some(win32_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: unsafe { LoadIconW(null_mut(),IDI_WINLOGO) },
            hCursor: unsafe { LoadCursorW(null_mut(),IDC_ARROW) },
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
            lpszClassName: win32_string("E").as_ptr(),
        };
        if unsafe { RegisterClassW(&wc as *const WNDCLASSW) } == 0 {
            return Err(SystemError::Generic);
        }

        let fake_hwnd = unsafe {
            CreateWindowExW(
                0,
                win32_string("E").as_ptr(),
                win32_string("").as_ptr(),
                WS_CLIPSIBLINGS | WS_CLIPCHILDREN,
                0,
                0,
                1,
                1,
                null_mut(),
                null_mut(),
                hinstance,
                null_mut()
            )
        };
        if fake_hwnd == null_mut() {
            return Err(SystemError::Generic);
        }
        let fake_hdc = unsafe { GetDC(fake_hwnd) };
        let fake_pfd = PIXELFORMATDESCRIPTOR {
            nSize: 40,
            nVersion: 1,
            dwFlags:
                PFD_DRAW_TO_WINDOW |
                PFD_SUPPORT_OPENGL |
                PFD_DOUBLEBUFFER,
            iPixelType: PFD_TYPE_RGBA,
            cColorBits: 32,
            cRedBits: 0,
            cRedShift: 0,
            cGreenBits: 0,
            cGreenShift: 0,
            cBlueBits: 0,
            cBlueShift: 0,
            cAlphaBits: 8,
            cAlphaShift: 0,
            cAccumBits: 0,
            cAccumRedBits: 0,
            cAccumGreenBits: 0,
            cAccumBlueBits: 0,
            cAccumAlphaBits: 0,
            cDepthBits: 24,
            cStencilBits: 0,
            cAuxBuffers: 0,
            iLayerType: 0,
            bReserved: 0,
            dwLayerMask: 0,
            dwVisibleMask: 0,
            dwDamageMask: 0,
        };
        let fake_pfdid = unsafe { ChoosePixelFormat(fake_hdc,&fake_pfd) };
        if fake_pfdid == 0 {
            return Err(SystemError::Generic);
        }
        if unsafe { SetPixelFormat(fake_hdc,fake_pfdid,&fake_pfd) } == 0 {
            return Err(SystemError::Generic);
        }
        let fake_hglrc = unsafe { wglCreateContext(fake_hdc) };
        if fake_hglrc == null_mut() {
            return Err(SystemError::Generic);
        }
        if unsafe { wglMakeCurrent(fake_hdc,fake_hglrc) } == 0 {
            return Err(SystemError::Generic);
        }
        let opengl32_hinstance = unsafe {
            LoadLibraryW(win32_string("opengl32.dll").as_ptr())
        };
        if opengl32_hinstance == null_mut() {
            return Err(SystemError::Generic);
        }
        let wgl_choose_pixel_format: WglChoosePixelFormatARBProc = unsafe {
            transmute(
                load_function(opengl32_hinstance,"wglChoosePixelFormatARB")
            )
        };
        let wgl_create_context_attribs: WglCreateContextAttribsARBProc = unsafe {
            transmute(
                load_function(opengl32_hinstance,"wglCreateContextAttribsARB")
            )
        };
        gl::load_with(|s| load_function(opengl32_hinstance,s));
        unsafe { wglDeleteContext(fake_hglrc) };
        unsafe { ReleaseDC(fake_hwnd,fake_hdc) };
        unsafe { DestroyWindow(fake_hwnd) };
        let hidden_hwnd = unsafe {
            CreateWindowExW(
                0,
                win32_string("E").as_ptr(),
                win32_string("").as_ptr(),
                WS_CLIPSIBLINGS | WS_CLIPCHILDREN,
                0,
                0,
                1,
                1,
                null_mut(),
                null_mut(),
                hinstance,
                null_mut()
            )
        };
        if hidden_hwnd == null_mut() {
            return Err(SystemError::Generic);
        }
        let hidden_hdc = unsafe { GetDC(hidden_hwnd) };
        let pfattribs = [
            WGL_DRAW_TO_WINDOW_ARB,gl::TRUE as c_int,
            WGL_SUPPORT_OPENGL_ARB,gl::TRUE as c_int,
            WGL_DOUBLE_BUFFER_ARB,gl::TRUE as c_int,
            WGL_PIXEL_TYPE_ARB,WGL_TYPE_RGBA_ARB,
            WGL_ACCELERATION_ARB,WGL_FULL_ACCELERATION_ARB,
            WGL_COLOR_BITS_ARB,32,
            WGL_ALPHA_BITS_ARB,8,
            WGL_DEPTH_BITS_ARB,24,
            WGL_STENCIL_BITS_ARB,8,
            WGL_SAMPLE_BUFFERS_ARB,gl::TRUE as c_int,
            WGL_SAMPLES_ARB,1,
            0,
        ];
        let mut pfid = 0i32;
        let mut numformats: UINT = 0;
        if unsafe { wgl_choose_pixel_format(
            hidden_hdc,
            &pfattribs as *const c_int,
            null_mut(),
            1,
            &mut pfid,
            &mut numformats
        ) } == 0 {
            return Err(SystemError::Generic);
        }
        if numformats == 0 {
            return Err(SystemError::Generic);
        }
        let mut pfd = PIXELFORMATDESCRIPTOR {
            nSize: 40,
            nVersion: 1,
            dwFlags:
                PFD_DRAW_TO_WINDOW |
                PFD_SUPPORT_OPENGL |
                PFD_DOUBLEBUFFER,
            iPixelType: PFD_TYPE_RGBA,
            cColorBits: 32,
            cRedBits: 0,
            cRedShift: 0,
            cGreenBits: 0,
            cGreenShift: 0,
            cBlueBits: 0,
            cBlueShift: 0,
            cAlphaBits: 8,
            cAlphaShift: 0,
            cAccumBits: 0,
            cAccumRedBits: 0,
            cAccumGreenBits: 0,
            cAccumBlueBits: 0,
            cAccumAlphaBits: 0,
            cDepthBits: 24,
            cStencilBits: 0,
            cAuxBuffers: 0,
            iLayerType: 0,
            bReserved: 0,
            dwLayerMask: 0,
            dwVisibleMask: 0,
            dwDamageMask: 0,
        };
        unsafe { DescribePixelFormat(hidden_hdc,pfid,40,&mut pfd) };
        unsafe { SetPixelFormat(hidden_hdc,pfid,&pfd) };
        let ctxattribs = [
            WGL_CONTEXT_MAJOR_VERSION_ARB,4,
            WGL_CONTEXT_MINOR_VERSION_ARB,5,
            WGL_CONTEXT_PROFILE_MASK_ARB,WGL_CONTEXT_CORE_PROFILE_BIT_ARB,
            0,
        ];
        let hglrc = unsafe {
            wgl_create_context_attribs(
                hidden_hdc,
                null_mut(),
                &ctxattribs as *const c_int
            )
        };
        if hglrc == null_mut() {
            return Err(SystemError::Generic);
        }
        unsafe { wglMakeCurrent(null_mut(),null_mut()) };
        if unsafe { wglMakeCurrent(hidden_hdc,hglrc) } == 0 {
            return Err(SystemError::Generic);
        }

        Ok(System {
            hinstance: hinstance,
            pfid: pfid,
            pfd: pfd,
            hglrc: hglrc,
            hidden_hdc: hidden_hdc,
            hidden_hwnd: hidden_hwnd,
        })    
    }

#[cfg(target_os="linux")]
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

#[cfg(target_os="windows")]
    fn parse_message(&self,message: MSG) -> Option<(HWND,Event)> {
        let wparam_hi = (message.wParam >> 16) as u16;
        let wparam_lo = (message.wParam & 0x0000FFFF) as u16;
        let lparam_hi = (message.lParam >> 16) as u16;
        let lparam_lo = (message.lParam & 0x0000FFFF) as u16;
        match message.message {
            WM_KEYDOWN => {
                Some((message.hwnd,Event::KeyPress(wparam_lo as u8)))
            },
            WM_KEYUP => {
                Some((message.hwnd,Event::KeyRelease(wparam_lo as u8)))
            },
            WM_LBUTTONDOWN => {
                Some((message.hwnd,Event::MousePress(vec2!(lparam_lo as i16 as isize,lparam_hi as i16 as isize),Mouse::Left)))
            },
            WM_LBUTTONUP => {
                Some((message.hwnd,Event::MouseRelease(vec2!(lparam_lo as i16 as isize,lparam_hi as i16 as isize),Mouse::Left)))
            },
            WM_MBUTTONDOWN => {
                Some((message.hwnd,Event::MousePress(vec2!(lparam_lo as i16 as isize,lparam_hi as i16 as isize),Mouse::Middle)))
            },
            WM_MBUTTONUP => {
                Some((message.hwnd,Event::MouseRelease(vec2!(lparam_lo as i16 as isize,lparam_hi as i16 as isize),Mouse::Middle)))
            },
            WM_RBUTTONDOWN => {
                Some((message.hwnd,Event::MousePress(vec2!(lparam_lo as i16 as isize,lparam_hi as i16 as isize),Mouse::Right)))
            },
            WM_RBUTTONUP => {
                Some((message.hwnd,Event::MouseRelease(vec2!(lparam_lo as i16 as isize,lparam_hi as i16 as isize),Mouse::Right)))
            },
            WM_MOUSEWHEEL => {
                if wparam_hi >= 0x8000 {
                    Some((message.hwnd,Event::MouseWheel(Wheel::Down)))
                } else {
                    Some((message.hwnd,Event::MouseWheel(Wheel::Up)))
                }
            },
            WM_MOUSEMOVE => {
                Some((message.hwnd,Event::MouseMove(vec2!(lparam_lo as i16 as isize,lparam_hi as i16 as isize))))
            },
            WM_PAINT => {
                let mut paintstruct = PAINTSTRUCT {
                    hdc: null_mut(),
                    fErase: FALSE,
                    rcPaint: RECT {
                        left: 0,
                        right: 0,
                        top: 0,
                        bottom: 0,
                    },
                    fRestore: FALSE,
                    fIncUpdate: FALSE,
                    rgbReserved: [0; 32],
                };
                unsafe { BeginPaint(message.hwnd,&mut paintstruct) };
                unsafe { EndPaint(message.hwnd,&paintstruct) };
                Some((message.hwnd,Event::Paint(rect!(
                    paintstruct.rcPaint.left as isize,
                    paintstruct.rcPaint.top as isize,
                    (paintstruct.rcPaint.right - paintstruct.rcPaint.left) as isize,
                    (paintstruct.rcPaint.bottom - paintstruct.rcPaint.top) as isize
                ))))
            },
            WM_USER => {
                match message.wParam as u32 {
                    WM_SIZE => {
                        Some((message.hwnd,Event::Resize(vec2!(lparam_lo as i16 as isize,lparam_hi as i16 as isize))))
                    },
                    WM_CLOSE => {
                        Some((message.hwnd,Event::Close))
                    },
                    _ => {
                        None
                    }
                }
            },
            _ => {
                None
            },
        }   
    }

    pub fn poll<T: Pollable>(&self,targets: &T) -> T::Result {
        targets.do_poll(&self)
    }
    
#[cfg(target_os="linux")]
    pub fn wait(&self) {
        let mut epe = [epoll_event { events: EPOLLIN as u32,u64: 0, }];
        unsafe { epoll_wait(self.epfd,epe.as_mut_ptr(),1,-1) };
    }

#[cfg(target_os="windows")]
    pub fn wait(&self) {
        unsafe {
            WaitMessage();
        }
    }
}

impl Drop for System {
#[cfg(target_os="linux")]
    fn drop(&mut self) {
        unsafe { glXMakeCurrent(self.connection.get_raw_dpy(),0,null_mut()); }
        destroy_window(&self.connection,self.hidden_window as u32);
        unsafe { glXDestroyContext(self.connection.get_raw_dpy(),self.context); }
    }

#[cfg(target_os="windows")]
    fn drop(&mut self) {
        unsafe {
            wglMakeCurrent(null_mut(),null_mut());
            wglDeleteContext(self.hglrc);
            ReleaseDC(self.hidden_hwnd,self.hidden_hdc);
            DestroyWindow(self.hidden_hwnd);
        }
    }
}

#[doc(hidden)]
pub trait Pollable {
    type Result;
    fn do_poll(&self,system: &System) -> <Self as Pollable>::Result;
}

#[doc(hidden)]
impl Pollable for Rc<Window> {
    type Result = Vec<Event>;
    fn do_poll(&self,system: &System) -> <Self as Pollable>::Result {
        let mut events: Vec<Event> = Vec::new();
#[cfg(target_os="linux")]
        while let Some(xcb_event) = system.connection.poll_for_event() {
            if let Some((xid,event)) = system.parse_xevent(xcb_event) {
                if xid == self.id {
                    events.push(event);
                }
            }
        }
#[cfg(target_os="windows")]
        {
            let mut msg = MSG {
                hwnd: null_mut(),
                message: 0,
                wParam: 0,
                lParam: 0,
                time: 0,
                pt: POINT { x: 0,y: 0, },
            };
            unsafe {
                while PeekMessageW(&mut msg,null_mut(),0,0,PM_REMOVE) != 0 {
                    TranslateMessage(&msg);
                    if let Some((hwnd,event)) = system.parse_message(msg) {
                        if hwnd == self.hwnd {
                            events.push(event);
                        }
                    }
                    DispatchMessageW(&msg);
                }
            }
        }
        events
    }
}

#[doc(hidden)]
impl Pollable for Vec<Rc<Window>> {
    type Result = Vec<(Rc<Window>,Event)>;
    fn do_poll(&self,system: &System) -> <Self as Pollable>::Result {
        let mut events: Vec<(Rc<Window>,Event)> = Vec::new();
#[cfg(target_os="linux")]
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
#[cfg(target_os="windows")]
        {
            let mut msg = MSG {
                hwnd: null_mut(),
                message: 0,
                wParam: 0,
                lParam: 0,
                time: 0,
                pt: POINT { x: 0,y: 0, },
            };
            unsafe {
                while PeekMessageW(&mut msg,null_mut(),0,0,PM_REMOVE) != 0 {
                    TranslateMessage(&msg);
                    if let Some((hwnd,event)) = system.parse_message(msg) {
                        for window in self.iter() {
                            if hwnd == window.hwnd {
                                events.push((Rc::clone(window),event));
                                break;
                            }
                        }    
                    }
                    DispatchMessageW(&msg);
                }
            }    
        }        
        events
    }
}
