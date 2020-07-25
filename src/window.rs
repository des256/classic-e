// E - Window
// Desmond Germans, 2020

use {
    crate::*,
    std::{
        cell::Cell,
        rc::Rc
    },
};

#[cfg(target_os="linux")]
use {
    x11::{
        xlib::{
            XID,
            XSync,
            False,
        },
        glx::{
            glXMakeCurrent,
            glXSwapBuffers,
        },
    },
    xcb::xproto::*,
};

#[cfg(target_os="windows")]
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
            SwapBuffers,
        },
    },
};

/// OS window (for desktop environments).
pub struct Window {
    system: Rc<System>,
    pub size: Cell<Vec2<usize>>,
#[cfg(target_os="linux")]
    pub(crate) id: XID,
#[cfg(target_os="windows")]
    pub(crate) hwnd: HWND,
#[cfg(target_os="windows")]
    pub(crate) hdc: HDC,
}

impl Window {
    pub fn new(system: &Rc<System>,r: Rect<isize>,title: &str) -> Result<Window,SystemError> {
#[cfg(target_os="linux")]
        {
            let id = system.connection.generate_id() as XID;
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
                (CW_COLORMAP,system.colormap as u32),
            ];
            create_window(
                &system.connection,
                system.depth as u8,
                id as u32,
                system.rootwindow as u32,
                r.o.x as i16,r.o.y as i16,r.s.x as u16,r.s.y as u16,
                0,
                WINDOW_CLASS_INPUT_OUTPUT as u16,
                system.visualid as u32,
                &values
            );
            unsafe {
                map_window(&system.connection,id as u32);
                system.connection.flush();
                XSync(system.connection.get_raw_dpy(),False);
            }
            change_property(
                &system.connection,
                PROP_MODE_REPLACE as u8,
                id as u32,
                ATOM_WM_NAME,
                ATOM_STRING,
                8,
                title.as_bytes()
            );
            let protocol_set = [system.wm_delete_window];
            change_property(
                &system.connection,
                PROP_MODE_REPLACE as u8,
                id as u32,
                system.wm_protocols,
                ATOM_ATOM,
                32,
                &protocol_set
            );
            system.connection.flush();
            Ok(Window {
                system: Rc::clone(system),
                id: id,
                size: Cell::new(vec2!(r.s.x as usize,r.s.y as usize)),
            })
        }

#[cfg(target_os="windows")]
        {
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

    pub fn begin_paint(&self) {
        let size = self.size.get();
        unsafe {
#[cfg(target_os="linux")]
            glXMakeCurrent(self.system.connection.get_raw_dpy(),self.id,self.system.context);
#[cfg(target_os="windows")]
            wglMakeCurrent(self.hdc,self.system.hglrc);
            gl::Viewport(0,0,size.x as i32,size.y as i32);
            gl::Scissor(0,0,size.x as i32,size.y as i32);
        }
    }

    pub fn end_paint(&self) {
        unsafe {
            gl::Flush();
#[cfg(target_os="linux")]
            glXSwapBuffers(self.system.connection.get_raw_dpy(),self.id);
#[cfg(target_os="windows")]
            SwapBuffers(self.hdc);
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
#[cfg(target_os="linux")]
        {
            unsafe { glXMakeCurrent(self.system.connection.get_raw_dpy(),self.system.hidden_window,self.system.context); }
            unmap_window(&self.system.connection,self.id as u32);
            destroy_window(&self.system.connection,self.id as u32);
        }
#[cfg(target_os="windows")]
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
