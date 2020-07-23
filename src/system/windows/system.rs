// E - System - Windows - System
// Desmond Germans, 2020

use crate::*;
use winapi::shared::windef::*;
use winapi::shared::minwindef::*;
use winapi::um::winuser::*;
use winapi::um::wingdi::*;
use winapi::um::libloaderapi::*;
use std::ptr::null_mut;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use std::ffi::CString;
use std::ffi::c_void;
use std::mem::transmute;
use std::os::raw::c_int;
use std::rc::Rc;

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

pub struct System {
    pub(crate) hinstance: HINSTANCE,
    pub(crate) pfid: i32,
    pub(crate) pfd: PIXELFORMATDESCRIPTOR,
    pub(crate) hglrc: HGLRC,
    pub(crate) hidden_hdc: HDC,
    pub(crate) hidden_hwnd: HWND,
    pub(crate) opengl: OpenGL,
}

pub(crate) fn win32_string(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

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

impl System {
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

        let opengl = match OpenGL::new() {
            Ok(opengl) => opengl,
            Err(_) => { return Err(SystemError::Generic) },
        };

        Ok(System {
            hinstance: hinstance,
            pfid: pfid,
            pfd: pfd,
            hglrc: hglrc,
            hidden_hdc: hidden_hdc,
            hidden_hwnd: hidden_hwnd,
            opengl: opengl,
        })
    }

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
    
    pub fn wait(&self) {
        unsafe {
            WaitMessage();
        }
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
        unsafe {
            wglMakeCurrent(null_mut(),null_mut());
            wglDeleteContext(self.hglrc);
            ReleaseDC(self.hidden_hwnd,self.hidden_hdc);
            DestroyWindow(self.hidden_hwnd);
        }
    }
}

pub trait Pollable {
    type Result;
    fn do_poll(&self,system: &System) -> <Self as Pollable>::Result;
}

impl Pollable for Rc<Window> {
    type Result = Vec<Event>;
    fn do_poll(&self,system: &System) -> <Self as Pollable>::Result {
        let mut events: Vec<Event> = Vec::new();
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
        events
    }
}

impl Pollable for Vec<Rc<Window>> {
    type Result = Vec<(Rc<Window>,Event)>;
    fn do_poll(&self,system: &System) -> <Self as Pollable>::Result {
        let mut events: Vec<(Rc<Window>,Event)> = Vec::new();
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
        events
    }
}
