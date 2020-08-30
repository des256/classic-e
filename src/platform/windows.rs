// E - System
// Desmond Germans, 2020

#[doc(hidden)]
use {
    crate::*,
    std::{
        os::{
            raw::{
                c_void,
                c_int,
            },
            windows::ffi::OsStrExt,
        },
        ffi::{
            CString,
            OsStr,
        },
        mem::transmute,
        ptr::null_mut,
        rc::Rc,
        iter::once,
        cell::{
            Cell,
            RefCell,
        },
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
                DWORD,
                FALSE,
            },
        },
        um::{
            winuser::{
                WM_SIZE,
                WM_MOVE,
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
                WaitMessage,
                PeekMessageW,
                PM_REMOVE,
                TranslateMessage,
                DispatchMessageW,
                GetWindowLongPtrW,
                SetWindowLongPtrW,
                GWLP_USERDATA,
                ValidateRect,
                GetClientRect,
                WS_OVERLAPPEDWINDOW,
                WS_POPUPWINDOW,
                WS_EX_APPWINDOW,
                WS_EX_WINDOWEDGE,
                AdjustWindowRectEx,
                CW_USEDEFAULT,
                ShowWindow,
                SW_SHOW,
                SW_HIDE,
                SetForegroundWindow,
                SetFocus,
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

pub const KEY_UP: u8 = 38;
pub const KEY_DOWN: u8 = 40;
pub const KEY_LEFT: u8 = 37;
pub const KEY_RIGHT: u8 = 39;

const WGL_DRAW_TO_WINDOW_ARB: c_int = 0x2001;
const WGL_SUPPORT_OPENGL_ARB: c_int = 0x2010;
const WGL_DOUBLE_BUFFER_ARB: c_int = 0x2011;
const WGL_ACCELERATION_ARB: c_int = 0x2003;
const WGL_PIXEL_TYPE_ARB: c_int = 0x2013;
const WGL_COLOR_BITS_ARB: c_int = 0x2014;
const WGL_ALPHA_BITS_ARB: c_int = 0x201B;
const WGL_DEPTH_BITS_ARB: c_int = 0x2022;
const WGL_STENCIL_BITS_ARB: c_int = 0x2023;
const WGL_SAMPLE_BUFFERS_ARB: c_int = 0x2041;
const WGL_SAMPLES_ARB: c_int = 0x2042;
const WGL_TYPE_RGBA_ARB: c_int = 0x202B;
const WGL_FULL_ACCELERATION_ARB: c_int = 0x2027;
const WGL_CONTEXT_MAJOR_VERSION_ARB: c_int = 0x2091;
const WGL_CONTEXT_MINOR_VERSION_ARB: c_int = 0x2092;
const WGL_CONTEXT_PROFILE_MASK_ARB: c_int = 0x9126;
const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: c_int = 0x00000001;

type WglChoosePixelFormatARBProc = unsafe extern "C" fn(
    hdc: HDC,
    piAttribIList: *const c_int,
    pfAttribFList: *const FLOAT,
    nMaxFormats: UINT,
    piFormats: *mut c_int,
    nNumFormats: *mut UINT
) -> BOOL;

type WglCreateContextAttribsARBProc = unsafe extern "C" fn(
    hdc: HDC,
    hShareContext: HGLRC,
    attribList: *const c_int
) -> HGLRC;

type WglSwapIntervalEXT = unsafe extern "C" fn(
    interval: c_int
) -> BOOL;

pub(crate) fn win32_string(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

unsafe extern "system" fn win32_proc(
    hwnd: HWND,
    message: UINT,
    wparam: WPARAM,
    lparam: LPARAM
) -> LRESULT {
    let window = GetWindowLongPtrW(hwnd,GWLP_USERDATA) as *mut c_void;
    let window = match window.as_mut() {
        Some(window) => window,
        None => { return DefWindowProcW(hwnd,message,wparam,lparam); },
    };
    let window: &mut Window = transmute(window);
    window.wndproc(message,wparam,lparam)
}

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

pub(crate) struct SystemAnchor {
    pub(crate) hglrc: HGLRC,
    pub(crate) hidden_hdc: HDC,
}

pub struct System {
    pub(crate) anchor: Rc<SystemAnchor>,
    pub(crate) windows: RefCell<Vec<Rc<Window>>>,
    pub(crate) hinstance: HINSTANCE,
    pub(crate) pfid: i32,
    pub(crate) pfd: PIXELFORMATDESCRIPTOR,
    pub(crate) wgl_swap_interval: WglSwapIntervalEXT,
    pub(crate) hidden_hwnd: HWND,
}

impl System {
    /// Create new system context.
    /// ## Returns
    /// - * `Ok(System)` - New system context.
    /// - * `Err(SystemError)` - The system context could not be created.
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
        let wgl_swap_interval: WglSwapIntervalEXT = unsafe {
            transmute(
                load_function(opengl32_hinstance,"wglSwapIntervalEXT")
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
            anchor: Rc::new(SystemAnchor {
                hglrc: hglrc,
                hidden_hdc: hidden_hdc,
            }),
            windows: RefCell::new(Vec::new()),
            hinstance: hinstance,
            pfid: pfid,
            pfd: pfd,
            hidden_hwnd: hidden_hwnd,    
            wgl_swap_interval: wgl_swap_interval,
        })
    }

    pub fn hidden_hdc(&self) -> HDC {
        self.anchor.hidden_hdc
    }

    pub fn hglrc(&self) -> HGLRC {
        self.anchor.hglrc
    }

    /// Flush all pending window events.
    pub fn flush(&self) {
        for window in self.windows.borrow().iter() {
            unsafe {
                // pass pointer to closure reference, which is a pointer to pointer to closure
                SetWindowLongPtrW(
                    window.hwnd,
                    GWLP_USERDATA,
                    Rc::downgrade(&window).into_raw() as *const c_void as isize
                );
            }
        }
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
                DispatchMessageW(&msg);
            }
        }
    }

    /// Wait until new window events appear.
    pub fn wait(&self) {
        unsafe {
            WaitMessage();
        }
    }

    fn open_window(&self,r: Rect<i32>,title: &str,style: DWORD,exstyle: DWORD) -> Result<Rc<Window>,SystemError> {
        let mut rc = RECT {
            left: r.o.x,
            right: r.o.x + r.s.x,
            top: r.o.y,
            bottom: r.o.y + r.s.y,
        };
        unsafe {
            AdjustWindowRectEx(
                &mut rc as *mut RECT,
                style,
                FALSE,
                exstyle
            )
        };
        let hwnd = unsafe { CreateWindowExW(
            exstyle,
            win32_string("E").as_ptr(),
            win32_string(title).as_ptr(),
            WS_CLIPSIBLINGS | WS_CLIPCHILDREN | style,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            rc.right - rc.left,
            rc.bottom - rc.top,
            null_mut(),
            null_mut(),
            self.hinstance,
            null_mut())
        };
        if hwnd == null_mut() {
            return Err(SystemError::Generic);
        }
        let hdc = unsafe { GetDC(hwnd) };
        let window = Rc::new(Window {
            anchor: Rc::clone(&self.anchor),
            hwnd: hwnd,
            hdc: hdc,
            r: Cell::new(r),
            handler: RefCell::new(None),
        });
        unsafe { SetPixelFormat(hdc,self.pfid,&self.pfd) };
        unsafe { ShowWindow(hwnd,SW_SHOW) };
        unsafe { SetForegroundWindow(hwnd) };
        unsafe { SetFocus(hwnd) };
        self.windows.borrow_mut().push(Rc::clone(&window));
        Ok(window)
    }

    /// Create new framed window
    pub fn open_frame_window(&self,r: Rect<i32>,title: &str) -> Result<Rc<Window>,SystemError> {
        self.open_window(r,title,WS_OVERLAPPEDWINDOW,WS_EX_APPWINDOW | WS_EX_WINDOWEDGE)
    }

    /// Create new floating window
    pub fn open_popup_window(&self,r: Rect<i32>) -> Result<Rc<Window>,SystemError> {
        self.open_window(r,"",WS_POPUPWINDOW,0)
    }

    /// Close an open window
    pub fn close_window(&self,window: &Rc<Window>) {
        let len = self.windows.borrow().len();
        for i in 0..len {
            if Rc::ptr_eq(window,&self.windows.borrow()[i]) {
                unsafe { ShowWindow(window.hwnd,SW_HIDE) };
                self.windows.borrow_mut().remove(i);
                break;
            }
        }
    }

    /// Release mouse pointer.
    pub fn release_mouse(&self) {
        // TODO
    }
}

impl Drop for System {
    fn drop(&mut self) {
        unsafe {
            wglMakeCurrent(null_mut(),null_mut());
            wglDeleteContext(self.anchor.hglrc);
            ReleaseDC(self.hidden_hwnd,self.anchor.hidden_hdc);
            DestroyWindow(self.hidden_hwnd);
        }
    }
}

pub struct Window {
    pub(crate) anchor: Rc<SystemAnchor>,
    pub(crate) r: Cell<Rect<i32>>,
    pub(crate) hwnd: HWND,
    pub(crate) hdc: HDC,
    pub(crate) handler: RefCell<Option<Box<dyn Fn(Event)>>>,
}

impl Window {
    /// Connect handler.
    pub fn set_handler<F: Fn(Event) + 'static>(&self,handler: F) {
        *self.handler.borrow_mut() = Some(Box::new(handler));
    }

    /// Capture mouse pointer to specific window.
    pub fn capture_mouse(&self) {
        // TODO
    }

    fn wndproc(&mut self,message: UINT,wparam: WPARAM,lparam: LPARAM) -> LRESULT {
        if let Some(handler) = &*self.handler.borrow() {
            let wparam_hi = (wparam >> 16) as u16;
            let wparam_lo = (wparam & 0x0000FFFF) as u16;
            let lparam_hi = (lparam >> 16) as u16;
            let lparam_lo = (lparam & 0x0000FFFF) as u16;
            match message {
                WM_KEYDOWN => {
                    handler(Event::KeyPress(wparam_lo as u8));
                },
                WM_KEYUP => {
                    handler(Event::KeyRelease(wparam_lo as u8));
                },
                WM_LBUTTONDOWN => {
                    handler(Event::MousePress(vec2!(lparam_lo as i32,lparam_hi as i32),MouseButton::Left));
                },
                WM_LBUTTONUP => {
                    handler(Event::MouseRelease(vec2!(lparam_lo as i32,lparam_hi as i32),MouseButton::Left));
                },
                WM_MBUTTONDOWN => {
                    handler(Event::MousePress(vec2!(lparam_lo as i32,lparam_hi as i32),MouseButton::Middle));
                },
                WM_MBUTTONUP => {
                    handler(Event::MouseRelease(vec2!(lparam_lo as i32,lparam_hi as i32),MouseButton::Middle));
                },
                WM_RBUTTONDOWN => {
                    handler(Event::MousePress(vec2!(lparam_lo as i32,lparam_hi as i32),MouseButton::Right));
                },
                WM_RBUTTONUP => {
                    handler(Event::MouseRelease(vec2!(lparam_lo as i32,lparam_hi as i32),MouseButton::Right));
                },
                WM_MOUSEWHEEL => {
                    if wparam_hi >= 0x8000 {
                        handler(Event::MouseWheel(MouseWheel::Down));
                    } else {
                        handler(Event::MouseWheel(MouseWheel::Up));
                    }
                },
                WM_MOUSEMOVE => {
                    handler(Event::MouseMove(vec2!(lparam_lo as i32,lparam_hi as i32)));
                },
                WM_PAINT => {
                    let mut rc = RECT {
                        left: 0,
                        right: 0,
                        top: 0,
                        bottom: 0,
                    };
                    unsafe {
                        GetClientRect(self.hwnd,&mut rc);
                        ValidateRect(self.hwnd,&rc);
                    }
                    handler(Event::Render);
        
                    // NOTE: When resizing an overlapped window, MS Windows does not
                    // return from DispatchMessage, but calls WndProc with WM_SIZE and
                    // WM_PAINT internally; It assumes that WndProc can be used in
                    // that fashion. After a window is resized, all pushed events all
                    // of a sudden become available in one big chunk. This needs some
                    // looking in to later.
        
                    /*let mut paintstruct = PAINTSTRUCT {
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
                    BeginPaint(hwnd,&mut paintstruct);
                    EndPaint(hwnd,&paintstruct);*/
                },
                WM_SIZE => {
                    let mut r = self.r.get();
                    r.s = vec2!(lparam_lo as i32,lparam_hi as i32);
                    self.r.set(r);
                    handler(Event::Size(r.s));
                },
                WM_MOVE => {
                    let mut r = self.r.get();
                    r.o = vec2!(lparam_lo as i32,lparam_hi as i32);
                    self.r.set(r);
                    handler(Event::Move(r.o));
                },
                WM_CLOSE => {
                    handler(Event::Close);
                },
                _ => {
                    return unsafe { DefWindowProcW(self.hwnd,message,wparam,lparam) };
                },
            }
            0
        }
        else {
            unsafe { DefWindowProcW(self.hwnd,message,wparam,lparam) }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            wglMakeCurrent(self.anchor.hidden_hdc,self.anchor.hglrc);
            ReleaseDC(self.hwnd,self.hdc);
            DestroyWindow(self.hwnd);
        }
    }
}
