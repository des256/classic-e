// E - Window
// Desmond Germans, 2020

use {
    crate::*,
    std::{
        cell::Cell,
        rc::Rc,
    },
    x11::{
        xlib::{
            XID,
            XSync,
            False,
        },
        glx::glXMakeCurrent,
    },
    xcb::xproto::{
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
        map_window,
        change_property,
        PROP_MODE_REPLACE,
        ATOM_WM_NAME,
        ATOM_STRING,
        ATOM_ATOM,
        unmap_window,
        destroy_window,
    }
};

/// OS window (for desktop environments).
pub struct Window {
    system: Rc<System>,
    pub r: Cell<Rect<i32>>,
    pub(crate) id: XID,
}

impl Window {
    /// Create new OS window.
    /// ## Arguments
    /// * `system` - System to create the window on.
    /// * `r` - Origin and size of the window.
    /// * `title` - Initial title of the window.
    /// ## Returns
    /// * `Ok(Window)` - The new window.
    /// * `Err(SystemError)` - The window could not be created.
    fn new(system: &Rc<System>,r: Rect<i32>) -> Result<Window,SystemError> {
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
        Ok(Window {
            system: Rc::clone(system),
            id: id,
            r: Cell::new(r),
        })
    }

    pub fn new_framed(system: &Rc<System>,r: Rect<i32>,title: &str) -> Result<Window,SystemError> {
        let window = Window::new(system,r)?;
        let protocol_set = [system.wm_delete_window];
        change_property(
            &system.connection,
            PROP_MODE_REPLACE as u8,
            window.id as u32,
            system.wm_protocols,
            ATOM_ATOM,
            32,
            &protocol_set
        );        
        change_property(
            &system.connection,
            PROP_MODE_REPLACE as u8,
            window.id as u32,
            ATOM_WM_NAME,
            ATOM_STRING,
            8,
            title.as_bytes()
        );
        system.connection.flush();
        Ok(window)
    }

    pub fn new_floating(system: &Rc<System>,r: Rect<i32>) -> Result<Window,SystemError> {
        let window = Window::new(system,r)?;
        let net_type = [system.wm_net_type_utility];
        change_property(
            &system.connection,
            PROP_MODE_REPLACE as u8,
            window.id as u32,
            system.wm_net_type,
            ATOM_ATOM,
            32,
            &net_type
        );
        let net_state = [system.wm_net_state_above];
        change_property(
            &system.connection,
            PROP_MODE_REPLACE as u8,
            window.id as u32,
            system.wm_net_state,
            ATOM_ATOM,
            32,
            &net_state
        );
        let hints = [2u32,0,0,0,0];
        change_property(
            &system.connection,
            PROP_MODE_REPLACE as u8,
            window.id as u32,
            system.wm_motif_hints,
            ATOM_ATOM,
            32,
            &hints
        );
        system.connection.flush();
        Ok(window)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { glXMakeCurrent(self.system.connection.get_raw_dpy(),self.system.hidden_window,self.system.context); }
        unmap_window(&self.system.connection,self.id as u32);
        destroy_window(&self.system.connection,self.id as u32);
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
