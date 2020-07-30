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
        },
        ffi::CString,
        mem::transmute,
        ptr::null_mut,
        rc::Rc,
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
            },
        },
        um::{
            winuser::{
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

pub(crate) fn win32_string(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

enum EventCollector<'a> {
    Single(&'a mut Vec<Event>),
    Multi(&'a Vec<Rc<Window>>,&'a mut Vec<(Rc<Window>,Event)>),
}

impl<'a> EventCollector<'a> {
    pub fn push(&mut self,hwnd: HWND,event: Event) {
        match self {
            EventCollector::Single(events) => {
                events.push(event);
            },
            EventCollector::Multi(windows,events) => {
                for window in windows.iter() {
                    if window.hwnd == hwnd {
                        events.push((Rc::clone(window),event));
                        break;
                    }
                }
            },
        }
    }
}

unsafe extern "system" fn win32_proc(
    hwnd: HWND,
    message: UINT,
    wparam: WPARAM,
    lparam: LPARAM
) -> LRESULT {
    let collector_ptr = GetWindowLongPtrW(hwnd,GWLP_USERDATA) as *mut EventCollector;
    let collector = match collector_ptr.as_mut() {
        Some(collector) => collector,
        None => { return DefWindowProcW(hwnd,message,wparam,lparam); },
    };
    let wparam_hi = (wparam >> 16) as u16;
    let wparam_lo = (wparam & 0x0000FFFF) as u16;
    let lparam_hi = (lparam >> 16) as u16;
    let lparam_lo = (lparam & 0x0000FFFF) as u16;
    match message {
        WM_KEYDOWN => {
            collector.push(hwnd,Event::KeyPress(wparam_lo as u8));
        },
        WM_KEYUP => {
            collector.push(hwnd,Event::KeyRelease(wparam_lo as u8));
        },
        WM_LBUTTONDOWN => {
            collector.push(hwnd,Event::MousePress(vec2!(lparam_lo as i16 as isize,lparam_hi as i16 as isize),Mouse::Left));
        },
        WM_LBUTTONUP => {
            collector.push(hwnd,Event::MouseRelease(vec2!(lparam_lo as i16 as isize,lparam_hi as i16 as isize),Mouse::Left));
        },
        WM_MBUTTONDOWN => {
            collector.push(hwnd,Event::MousePress(vec2!(lparam_lo as i16 as isize,lparam_hi as i16 as isize),Mouse::Middle));
        },
        WM_MBUTTONUP => {
            collector.push(hwnd,Event::MouseRelease(vec2!(lparam_lo as i16 as isize,lparam_hi as i16 as isize),Mouse::Middle));
        },
        WM_RBUTTONDOWN => {
            collector.push(hwnd,Event::MousePress(vec2!(lparam_lo as i16 as isize,lparam_hi as i16 as isize),Mouse::Right));
        },
        WM_RBUTTONUP => {
            collector.push(hwnd,Event::MouseRelease(vec2!(lparam_lo as i16 as isize,lparam_hi as i16 as isize),Mouse::Right));
        },
        WM_MOUSEWHEEL => {
            if wparam_hi >= 0x8000 {
                collector.push(hwnd,Event::MouseWheel(Wheel::Down));
            } else {
                collector.push(hwnd,Event::MouseWheel(Wheel::Up));
            }
        },
        WM_MOUSEMOVE => {
            collector.push(hwnd,Event::MouseMove(vec2!(lparam_lo as i16 as isize,lparam_hi as i16 as isize)));
        },
        WM_PAINT => {
            let mut rc = RECT {
                left: 0,
                right: 0,
                top: 0,
                bottom: 0,
            };
            GetClientRect(hwnd,&mut rc);
            ValidateRect(hwnd,&rc);
            collector.push(hwnd,Event::Render);
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
            collector.push(hwnd,Event::Resize(vec2!(lparam_lo as i16 as isize,lparam_hi as i16 as isize)));
        },
        WM_CLOSE => {
            collector.push(hwnd,Event::Close);
        },
        _ => {
            return DefWindowProcW(hwnd,message,wparam,lparam);
        },
    }
    0   
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

/// System context.
pub struct System {
    pub(crate) hinstance: HINSTANCE,
    pub(crate) pfid: i32,
    pub(crate) pfd: PIXELFORMATDESCRIPTOR,
    pub(crate) hglrc: HGLRC,
    pub(crate) hidden_hdc: HDC,
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

    /// Poll for events on one or more OS windows.
    /// 
    /// All events that are available for the window(s) are placed in a vector.
    /// If no events are available, the method returns immediately with an
    /// empty vector.
    /// ## Arguments
    /// * `targets` - Either a single window (`&Rc<Window>`) or a vector of
    /// windows (`&Vec<Rc<Window>>`).
    /// ## Returns
    /// Vector of events occurred on a single window (`Vec<Event>`) or on
    /// multiple windows (`Vec<(Rc<Window>,Event)>`).
    pub fn poll<T: Pollable>(&self,targets: &T) -> T::Result {
        targets.do_poll(&self)
    }
    
    /// Wait for events to arrive.
    /// 
    /// Blocks until an event arrives that can then be polled via the `poll`
    /// method.
    pub fn wait(&self) {
        unsafe {
            WaitMessage();
        }
    }
}

impl Drop for System {
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
    fn do_poll(&self,_system: &System) -> <Self as Pollable>::Result {
        let mut events: Vec<Event> = Vec::new();
        let mut collector = EventCollector::Single(&mut events);
        unsafe { SetWindowLongPtrW(self.hwnd,GWLP_USERDATA,&mut collector as *mut EventCollector as isize) };
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
        events
    }
}

#[doc(hidden)]
impl Pollable for Vec<Rc<Window>> {
    type Result = Vec<(Rc<Window>,Event)>;
    fn do_poll(&self,_system: &System) -> <Self as Pollable>::Result {
        let mut events: Vec<(Rc<Window>,Event)> = Vec::new();
        let mut collector = EventCollector::Multi(self,&mut events);
        for window in self.iter() {
            unsafe { SetWindowLongPtrW(window.hwnd,GWLP_USERDATA,&mut collector as *mut EventCollector as isize) };
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
        events
    }
}
