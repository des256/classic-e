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
    sys_sys::*,
};

pub const KEY_UP: u8 = 111;
pub const KEY_DOWN: u8 = 116;
pub const KEY_LEFT: u8 = 113;
pub const KEY_RIGHT: u8 = 114;

/// Window.
pub struct Window {
    pub system: Rc<System>,
    #[doc(hidden)]
    pub window: xcb_window_t,
    #[doc(hidden)]
    pub r: Cell<Rect<i32>>,
    #[doc(hidden)]
    pub handler: RefCell<Option<Box<dyn Fn(Event)>>>,
}

impl Window {

    fn new(system: &Rc<System>,r: Rect<i32>,absolute: bool) -> Option<Rc<Window>> {

        let xcb_window = unsafe { xcb_generate_id(system.connection) };
        let values = xcb_create_window_value_list_t {
            background_pixmap: 0,
            background_pixel: 0,
            border_pixmap: 0,
            border_pixel: 0,
            bit_gravity: 0,
            win_gravity: 0,
            backing_store: 0,
            backing_planes: 0,
            backing_pixel: 0,
            override_redirect: if absolute { 1 } else { 0 },
            save_under: 0,
            event_mask: xcb_event_mask_t_XCB_EVENT_MASK_EXPOSURE
                | xcb_event_mask_t_XCB_EVENT_MASK_KEY_PRESS
                | xcb_event_mask_t_XCB_EVENT_MASK_KEY_RELEASE
                | xcb_event_mask_t_XCB_EVENT_MASK_BUTTON_PRESS
                | xcb_event_mask_t_XCB_EVENT_MASK_BUTTON_RELEASE
                | xcb_event_mask_t_XCB_EVENT_MASK_POINTER_MOTION
                | xcb_event_mask_t_XCB_EVENT_MASK_STRUCTURE_NOTIFY,
            do_not_propogate_mask: 0,
            colormap: unsafe { *system.screen }.default_colormap as u32,
            cursor: 0,
        };
        unsafe { xcb_create_window(
            system.connection,
            (*system.screen).root_depth as u8,
            xcb_window as u32,
            //if let Some(id) = parent { id as u32 } else { system.rootwindow as u32 },
            (*system.screen).root as u32,
            r.o.x as i16,r.o.y as i16,r.s.x as u16,r.s.y as u16,
            0,
            xcb_window_class_t_XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
            (*system.screen).root_visual as u32,
            xcb_cw_t_XCB_CW_EVENT_MASK | xcb_cw_t_XCB_CW_COLORMAP | xcb_cw_t_XCB_CW_OVERRIDE_REDIRECT,
            &values as *const xcb_create_window_value_list_t as *const std::os::raw::c_void
        ) };
        unsafe {
            xcb_map_window(system.connection,xcb_window as u32);
            xcb_flush(system.connection);
        }
        let window = Rc::new(Window {
            system: Rc::clone(system),
            window: xcb_window,
            r: Cell::new(r),
            handler: RefCell::new(None),
        });
        let encoded_pointer = Rc::as_ptr(&window) as u64;
        let encoded_pointer_dwords = [
            (encoded_pointer & 0x00000000FFFFFFFF) as u32,
            (encoded_pointer >> 32) as u32,
        ];
        let encoded_pointer_void = encoded_pointer_dwords.as_ptr() as *const std::os::raw::c_void;
        unsafe { xcb_change_property(
            system.connection,
            xcb_prop_mode_t_XCB_PROP_MODE_REPLACE as u8,
            xcb_window,
            system.e_window_pointer,
            xcb_atom_enum_t_XCB_ATOM_PRIMARY,
            32,
            2,
            encoded_pointer_void
        ) };
        Some(window)
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
    /// New frame window.
    pub fn new_frame(system: &Rc<System>,r: Rect<i32>,title: &str) -> Option<Rc<Window>> {
        let window = Window::new(system,r,false)?;
        let protocol_set = [system.wm_delete_window];
        let protocol_set_void = protocol_set.as_ptr() as *const std::os::raw::c_void;
        unsafe { xcb_change_property(
            system.connection,
            xcb_prop_mode_t_XCB_PROP_MODE_REPLACE as u8,
            window.window as u32,
            system.wm_protocols,
            xcb_atom_enum_t_XCB_ATOM_ATOM,
            32,
            1,
            protocol_set_void
        ) };
        unsafe { xcb_change_property(
            system.connection,
            xcb_prop_mode_t_XCB_PROP_MODE_REPLACE as u8,
            window.window as u32,
            xcb_atom_enum_t_XCB_ATOM_WM_NAME,
            xcb_atom_enum_t_XCB_ATOM_STRING,
            8,
            title.len() as u32,
            title.as_bytes().as_ptr() as *const std::os::raw::c_void
        ) };
        unsafe { xcb_flush(system.connection) };
        Some(window)
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
    /// New popup window.
    pub fn new_popup(system: &Rc<System>,r: Rect<i32>) -> Option<Rc<Window>> {
        let window = Window::new(system,r,true)?;
        let net_state = [system.wm_net_state_above];
        unsafe { xcb_change_property(
            system.connection,
            xcb_prop_mode_t_XCB_PROP_MODE_REPLACE as u8,
            window.window as u32,
            system.wm_net_state,
            xcb_atom_enum_t_XCB_ATOM_ATOM,
            32,
            1,
            net_state.as_ptr() as *const std::os::raw::c_void
        ) };
        let hints = [2u32,0,0,0,0];
        unsafe { xcb_change_property(
            system.connection,
            xcb_prop_mode_t_XCB_PROP_MODE_REPLACE as u8,
            window.window as u32,
            system.wm_motif_hints,
            xcb_atom_enum_t_XCB_ATOM_ATOM,
            32,
            5,
            hints.as_ptr() as *const std::os::raw::c_void
        ) };
        unsafe { xcb_flush(system.connection) };
        Some(window)
    }

    pub(crate) fn handle_event(&self,event: Event) {
        if let Event::Configure(r) = &event {
            // When resizing, X seems to return a rectangle with the initial
            // origin as specified during window creation. But when moving, X
            // seems to correctly update the origin coordinates.
            // Not sure what to make of this, but in order to get the actual
            // rectangle, this seems to work:
            let old_r = self.r.get();
            if r.s != old_r.s {
                self.r.set(rect!(old_r.o,r.s));
            }
            else {
                self.r.set(*r);
            }
        }
        if let Some(handler) = &*(self.handler).borrow() {
            (handler)(event);
        }
    }

    pub fn set_handler<T: Fn(Event) + 'static>(&self,handler: T) {
        *self.handler.borrow_mut() = Some(Box::new(handler));
    }

    pub fn clear_handler(&self) {
        *self.handler.borrow_mut() = None;
    }

    pub fn show(&self) {
        //println!("show {}",self.id);
        unsafe {
            xcb_map_window(self.system.connection,self.window as u32);
            xcb_flush(self.system.connection);
        }
    }

    pub fn hide(&self) {
        //println!("hide {}",self.id);
        unsafe {
            xcb_unmap_window(self.system.connection,self.window as u32);
            xcb_flush(self.system.connection);
        }
    }

    pub fn set_rect(&self,r: &Rect<i32>) {
        let values = xcb_configure_window_value_list_t {
            x: r.o.x as i32,
            y: r.o.y as i32,
            width: r.s.x as u32,
            height: r.s.y as u32,
            border_width: 0,
            sibling: 0,
            stack_mode: 0,
        };
        unsafe { xcb_configure_window(
            self.system.connection,
            self.window as u32,
            xcb_config_window_t_XCB_CONFIG_WINDOW_X as u16 |
                xcb_config_window_t_XCB_CONFIG_WINDOW_Y as u16 |
                xcb_config_window_t_XCB_CONFIG_WINDOW_WIDTH as u16 |
                xcb_config_window_t_XCB_CONFIG_WINDOW_HEIGHT as u16,
            &values as *const xcb_configure_window_value_list_t as *const std::os::raw::c_void
        ) };
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            xcb_unmap_window(self.system.connection,self.window as u32);
            xcb_destroy_window(self.system.connection,self.window as u32);
        }
    }
}
