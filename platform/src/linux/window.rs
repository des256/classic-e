// E - Linux - Window
// Desmond Germans, 2020

#[doc(hidden)]
use {
    crate::*,
    std::{
        rc::Rc,
        cell::Cell,
        cell::RefCell,
    },
    x11::{
        xlib::{
            XID,
            XSync,
            False,
        },
        glx::glXMakeCurrent,
    },
    xcb::{
        xproto::{
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
            destroy_window,
            map_window,
            ATOM_WM_NAME,
            ATOM_STRING,
            unmap_window,
            ATOM_WINDOW,
        },
    },
};

pub const KEY_UP: u8 = 111;
pub const KEY_DOWN: u8 = 116;
pub const KEY_LEFT: u8 = 113;
pub const KEY_RIGHT: u8 = 114;

/// Window.
pub struct Window {
    system: Rc<System>,
    pub id: u64,
    pub r: Cell<Rect<i32>>,
    pub handler: RefCell<Option<Box<dyn Fn(Event)>>>,
}

impl Window {
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
            r.ox() as i16,r.oy() as i16,r.sx() as u16,r.sy() as u16,
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
            handler: RefCell::new(None),
        })
    }

    /// Open new frame window.
    ///
    /// A frame window is a window with a draggable frame and a titlebar.
    ///
    /// **Arguments**
    ///
    /// * `system` - System to open the frame window on.
    /// * `r` - Initial frame window rectangle.
    /// * `title` - Title of the frame window. 
    ///
    /// **Returns**
    ///
    /// New frame base window.
    pub fn new_frame(system: &Rc<System>,r: Rect<i32>,title: &str) -> Result<Window,SystemError> {
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

    /// Open new popup window.
    ///
    /// A popup window is a window with no title and no border. This is
    /// usually used for menus, tooltips, dropdowns, etc.
    ///
    /// **Arguments**
    ///
    /// * `system` - System to open the frame window on.
    /// * `r` - Initial frame window rectangle.
    ///
    /// **Returns**
    ///
    /// New popup base window.
    pub fn new_popup(system: &Rc<System>,parent_window: &Window,r: Rect<i32>) -> Result<Window,SystemError> {
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
        let transient_for = [parent_window.id];
        change_property(
            &system.connection,
            PROP_MODE_REPLACE as u8,
            window.id as u32,
            system.wm_transient_for,
            ATOM_WINDOW,
            32,
            &transient_for
        );
        system.connection.flush();
        Ok(window)
    }

    pub fn set_handler<T: Fn(Event) + 'static>(&self,handler: T) {
        *self.handler.borrow_mut() = Some(Box::new(handler));
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { glXMakeCurrent(self.system.connection.get_raw_dpy(),self.system.hidden_window,self.system.context); }
        unmap_window(&self.system.connection,self.id as u32);
        destroy_window(&self.system.connection,self.id as u32);    
    }
}
