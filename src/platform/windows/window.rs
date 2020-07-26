// E - Window
// Desmond Germans, 2020

use {
    crate::*,
    std::{
        cell::Cell,
        rc::Rc
    },
};

use {
    std::ptr::null_mut,
    winapi::shared::{
        windef::{
            HWND,
            HDC,
            RECT,
        },
        minwindef::FALSE,
    },
    winapi::um::{
        winuser::{
            WS_OVERLAPPEDWINDOW,
            WS_EX_APPWINDOW,
            WS_EX_WINDOWEDGE,
            AdjustWindowRectEx,
            CreateWindowExW,
            WS_CLIPSIBLINGS,
            WS_CLIPCHILDREN,
            CW_USEDEFAULT,
            GetDC,
            ShowWindow,
            SW_SHOW,
            SetForegroundWindow,
            SetFocus,
            ReleaseDC,
            DestroyWindow,
        },
        wingdi::{
            SetPixelFormat,
            wglMakeCurrent,
        },
    },
};

/// OS window (for desktop environments).
pub struct Window {
    pub(crate) system: Rc<System>,
    pub size: Cell<Vec2<usize>>,
    pub(crate) hwnd: HWND,
    pub(crate) hdc: HDC,
}

impl Window {
    /// Create new OS window.
    /// # Arguments
    /// * `system` - System to create the window on.
    /// * `r` - Origin and size of the window.
    /// * `title` - Initial title of the window.
    /// # Returns
    /// * `Some(Window)` - The new window.
    /// * `None` - The window could not be created.
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
            system.hinstance,
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
        };
        unsafe { SetPixelFormat(hdc,system.pfid,&system.pfd) };
        unsafe { ShowWindow(hwnd,SW_SHOW) };
        unsafe { SetForegroundWindow(hwnd) };
        unsafe { SetFocus(hwnd) };
        Ok(window)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            wglMakeCurrent(self.system.hidden_hdc,self.system.hglrc);
            ReleaseDC(self.hwnd,self.hdc);
            DestroyWindow(self.hwnd);
        }
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
