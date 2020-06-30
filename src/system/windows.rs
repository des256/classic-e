// E - System - Windows
// Desmond Germans, 2020

use winapi::shared::windef::*;
use winapi::shared::minwindef::*;
use winapi::shared::basetsd::*;
use winapi::um::winuser::*;
use winapi::um::wingdi::*;
use winapi::um::libloaderapi::*;
use std::ptr::null_mut;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use crate::Event;
use crate::Button;
use crate::Wheel;
use std::ffi::CString;
use std::ffi::c_void;
use std::mem::transmute;
use std::os::raw::c_int;
use crate::isize_2;
use crate::isize_r;
use crate::UIError;

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

struct Window<'a> {
    hwnd: HWND,
    hdc: HDC,
    handler: Box<dyn FnMut(Event) + 'a>,
}

pub struct UI<'a> {
    hinstance: HINSTANCE,
    pfid: i32,
    pfd: PIXELFORMATDESCRIPTOR,
    hglrc: HGLRC,
    hidden_hdc: HDC,
    hidden_hwnd: HWND,
    windows: Vec<Window<'a>>,
}

fn win32_string(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

unsafe extern "system" fn win32_proc(
    hwnd: HWND,
    message: UINT,
    wparam: WPARAM,
    lparam: LPARAM
) -> LRESULT {
    let ui_ptr = GetWindowLongPtrW(hwnd,GWLP_USERDATA);
    if ui_ptr == 0 {
        return DefWindowProcW(hwnd,message,wparam,lparam);
    }
    let ui = ui_ptr as *mut UI;
    (*ui).handle_event(hwnd,message,wparam,lparam)
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

impl<'a> UI<'a> {
    pub fn new() -> Result<UI<'a>,UIError> {
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
            return Err(UIError::Generic);
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
            return Err(UIError::Generic);
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
            return Err(UIError::Generic);
        }
        if unsafe { SetPixelFormat(fake_hdc,fake_pfdid,&fake_pfd) } == 0 {
            return Err(UIError::Generic);
        }
        let fake_hglrc = unsafe { wglCreateContext(fake_hdc) };
        if fake_hglrc == null_mut() {
            return Err(UIError::Generic);
        }
        if unsafe { wglMakeCurrent(fake_hdc,fake_hglrc) } == 0 {
            return Err(UIError::Generic);
        }
        let opengl32_hinstance = unsafe {
            LoadLibraryW(win32_string("opengl32.dll").as_ptr())
        };
        if opengl32_hinstance == null_mut() {
            return Err(UIError::Generic);
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
            return Err(UIError::Generic);
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
            return Err(UIError::Generic);
        }
        if numformats == 0 {
            return Err(UIError::Generic);
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
            return Err(UIError::Generic);
        }
        unsafe { wglMakeCurrent(null_mut(),null_mut()) };
        if unsafe { wglMakeCurrent(hidden_hdc,hglrc) } == 0 {
            return Err(UIError::Generic);
        }

        Ok(UI {
            hinstance: hinstance,
            pfid: pfid,
            pfd: pfd,
            hglrc: hglrc,
            hidden_hdc: hidden_hdc,
            hidden_hwnd: hidden_hwnd,
            windows: Vec::new(),        
        })
    }

    fn find_window(&mut self,hwnd: HWND) -> Option<&mut Window<'a>> {
        for window in &mut self.windows {
            if hwnd == window.hwnd {
                return Some(window);
            }
        }
        None
    }

    fn handle_event(&mut self,hwnd: HWND,message: UINT,wparam: WPARAM,lparam: LPARAM) -> LRESULT {

        let hglrc = self.hglrc;
        let hidden_hdc = self.hidden_hdc;
        let window = match self.find_window(hwnd) {
            Some(window) => window,
            None => { return unsafe { DefWindowProcW(hwnd,message,wparam,lparam) } },
        };
        let wparam_hi = (wparam >> 16) as u16;
        let wparam_lo = (wparam & 0x0000FFFF) as u16;
        let lparam_hi = (lparam >> 16) as u16;
        let lparam_lo = (lparam & 0x0000FFFF) as u16;
        match message {
            WM_KEYDOWN => {
                (window.handler)(Event::KeyPress(wparam_lo as u8));
            },
            WM_KEYUP => {
                (window.handler)(Event::KeyRelease(wparam_lo as u8));
            },
            WM_LBUTTONDOWN => {
                (window.handler)(Event::MousePress(isize_2::new(lparam_lo as i16 as isize,lparam_hi as i16 as isize),Button::Left));
            },
            WM_LBUTTONUP => {
                (window.handler)(Event::MouseRelease(isize_2::new(lparam_lo as i16 as isize,lparam_hi as i16 as isize),Button::Left));
            },
            WM_MBUTTONDOWN => {
                (window.handler)(Event::MousePress(isize_2::new(lparam_lo as i16 as isize,lparam_hi as i16 as isize),Button::Middle));
            },
            WM_MBUTTONUP => {
                (window.handler)(Event::MouseRelease(isize_2::new(lparam_lo as i16 as isize,lparam_hi as i16 as isize),Button::Middle));
            },
            WM_RBUTTONDOWN => {
                (window.handler)(Event::MousePress(isize_2::new(lparam_lo as i16 as isize,lparam_hi as i16 as isize),Button::Right));
            },
            WM_RBUTTONUP => {
                (window.handler)(Event::MouseRelease(isize_2::new(lparam_lo as i16 as isize,lparam_hi as i16 as isize),Button::Right));
            },
            WM_MOUSEWHEEL => {
                if wparam_hi >= 0x8000 {
                    (window.handler)(Event::MouseWheel(Wheel::Down));
                } else {
                    (window.handler)(Event::MouseWheel(Wheel::Up));
                }
            },
            WM_MOUSEMOVE => {
                (window.handler)(Event::MouseMove(isize_2::new(lparam_lo as i16 as isize,lparam_hi as i16 as isize)));
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
                unsafe { BeginPaint(hwnd,&mut paintstruct) };
                unsafe { wglMakeCurrent(window.hdc,hglrc) };
                (window.handler)(Event::Paint(isize_r::new(
                    isize_2::new(paintstruct.rcPaint.left as isize,paintstruct.rcPaint.top as isize),
                    isize_2::new(paintstruct.rcPaint.right as isize - paintstruct.rcPaint.left as isize,paintstruct.rcPaint.bottom as isize - paintstruct.rcPaint.top as isize)
                )));
                unsafe { wglMakeCurrent(hidden_hdc,hglrc) };
                unsafe { SwapBuffers(window.hdc) };
                unsafe { EndPaint(hwnd,&paintstruct) };
            },
            WM_SIZE => {
                (window.handler)(Event::Resize(isize_2::new(lparam_lo as i16 as isize,lparam_hi as i16 as isize)));
            },
            WM_CLOSE => {
                (window.handler)(Event::Close);
            },
            _ => {
                return unsafe { DefWindowProcW(hwnd,message,wparam,lparam) };
            },
        }
        0    
    }

    pub fn create_window(&mut self,r: &isize_r,title: &str,handler: impl FnMut(Event) + 'a) -> bool {
        let window_style = WS_OVERLAPPEDWINDOW;
        let window_exstyle = WS_EX_APPWINDOW | WS_EX_WINDOWEDGE;
        let mut rect = RECT {
            left: r.o.x as i32,
            right: r.o.x as i32 + r.s.x as i32,
            top: r.o.y as i32,
            bottom: r.o.y as i32 + r.s.y as i32,
        };
        unsafe {
            AdjustWindowRectEx(
                &mut rect as *mut RECT,
                window_style,
                FALSE,
                window_exstyle
            )
        };
        let hwnd = unsafe { CreateWindowExW(
            window_exstyle,
            win32_string("E").as_ptr(),
            win32_string(title).as_ptr(),
            WS_CLIPSIBLINGS | WS_CLIPCHILDREN | window_style,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            rect.right - rect.left,
            rect.bottom - rect.top,
            null_mut(),
            null_mut(),
            self.hinstance,
            null_mut())
        };
        if hwnd == null_mut() {
            return false;
        }
        let hdc = unsafe { GetDC(hwnd) };
        let window = Window {
            hwnd: hwnd,
            hdc: hdc,
            handler: Box::new(handler),
        };
        unsafe { SetPixelFormat(hdc,self.pfid,&self.pfd) };
        unsafe {
            SetWindowLongPtrW(
                hwnd,
                GWLP_USERDATA,
                self as *mut UI as LONG_PTR
            )
        };
        unsafe { ShowWindow(hwnd,SW_SHOW) };
        unsafe { SetForegroundWindow(hwnd) };
        unsafe { SetFocus(hwnd) };
        self.windows.push(window);
        true
    }

    pub fn pump(&mut self) {
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

    pub fn wait(&self) {
        unsafe {
            WaitMessage();
        }
    }
}

impl<'a> Drop for UI<'a> {
    fn drop(&mut self) {
        unsafe {
            wglMakeCurrent(null_mut(),null_mut());
            wglDeleteContext(self.hglrc);
            ReleaseDC(self.hidden_hwnd,self.hidden_hdc);
            DestroyWindow(self.hidden_hwnd);
        }
    }
}
