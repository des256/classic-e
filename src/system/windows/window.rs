// E - System - Windows - Window
// Desmond Germans, 2020

use crate::*;
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
use std::ffi::CString;
use std::ffi::c_void;
use std::mem::transmute;
use std::os::raw::c_int;
use std::cell::Cell;
use std::rc::Rc;

struct Window {
    system: Rc<System>,
    pub(crate) hwnd: HWND,
    pub(crate) hdc: HDC,
    pub size: Cell<Vec2<usize>>,
}

impl Window {
    pub fn new(system: &Rc<System>,r: Rect<isize>,title: &str) -> Result<Window,SystemError> {
        let window_style = WS_OVERLAPPEDWINDOW;
        let window_exstyle = WS_EX_APPWINDOW | WS_EX_WINDOWEDGE;
        let mut rc = RECT {
            left: r.o.x as i32,
            right: r.o.x as i32 + r.s.x as i32,
            top: r.o.y as i32,
            bottom: r.o.y as i32 + r.s.y as i32,
        };
        unsafe {
            AdjustWindowRectEx(
                &mut rc as *mut RECT,
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
        let window = Window {
            system: Rc::clone(system),
            hwnd: hwnd,
            hdc: hdc,
            size: Cell::new(vec2!(r.s.x as usize,r.s.y as usize)),
        }
        unsafe { SetPixelFormat(hdc,self.pfid,&self.pfd) };
        unsafe {
            SetWindowLongPtrW(
                hwnd,
                GWLP_USERDATA,
                system as *mut System as LONG_PTR
            )
        };
        unsafe { ShowWindow(hwnd,SW_SHOW) };
        unsafe { SetForegroundWindow(hwnd) };
        unsafe { SetFocus(hwnd) };
        Ok(window)
    }
}
