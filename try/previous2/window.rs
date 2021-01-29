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
            CW_OVERRIDE_REDIRECT,
            create_window,
            WINDOW_CLASS_INPUT_OUTPUT,
            change_property,
            change_property_checked,
            PROP_MODE_REPLACE,
            ATOM_ATOM,
            destroy_window,
            map_window,
            ATOM_WM_NAME,
            ATOM_STRING,
            unmap_window,
            ATOM_PRIMARY,
            //ATOM_WINDOW,
            configure_window,
            CONFIG_WINDOW_X,
            CONFIG_WINDOW_Y,
            CONFIG_WINDOW_WIDTH,
            CONFIG_WINDOW_HEIGHT,
        },
    },
};

pub const KEY_UP: u8 = 111;
pub const KEY_DOWN: u8 = 116;
pub const KEY_LEFT: u8 = 113;
pub const KEY_RIGHT: u8 = 114;

/// Window.
pub struct Window {
    pub system: Rc<System>,
    #[doc(hidden)]
    pub id: u64,
    #[doc(hidden)]
    pub r: Cell<Rect<i32>>,
    #[doc(hidden)]
    pub handler: RefCell<Option<Box<dyn Fn(Event)>>>,
}

impl Window {
    fn new(system: &Rc<System>,r: Rect<i32>,absolute: bool) -> Result<Rc<Window>,SystemError> {
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
            (CW_OVERRIDE_REDIRECT,if absolute { 1 } else { 0 }),
        ];
        create_window(
            &system.connection,
            system.depth as u8,
            id as u32,
            //if let Some(id) = parent { id as u32 } else { system.rootwindow as u32 },
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
        let window = Rc::new(Window {
            system: Rc::clone(system),
            id: id,
            r: Cell::new(r),
            handler: RefCell::new(None),
        });
        let encoded_pointer = Rc::as_ptr(&window) as u64;
        let encoded_pointer_dwords = [
            (encoded_pointer & 0x00000000FFFFFFFF) as u32,
            (encoded_pointer >> 32) as u32,
        ];
        match change_property_checked(
            &system.connection,
            PROP_MODE_REPLACE as u8,
            id as u32,
            system.e_window_pointer,
            ATOM_PRIMARY,
            32,
            &encoded_pointer_dwords
        ).request_check() {
            Ok(_) => { },
            Err(e) => { println!("response type {:?}, error code {:?}",e.response_type(),e.error_code()) },
        }
        Ok(window)
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
    pub fn new_frame(system: &Rc<System>,r: Rect<i32>,title: &str) -> Result<Rc<Window>,SystemError> {
        let window = Window::new(system,r,false)?;
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
    /// New popup window.
    pub fn new_popup(system: &Rc<System>,r: Rect<i32>) -> Result<Rc<Window>,SystemError> {
        let window = Window::new(system,r,true)?;
        /*let net_type = [system.wm_net_type_utility];
        change_property(
            &system.connection,
            PROP_MODE_REPLACE as u8,
            window.id as u32,
            system.wm_net_type,
            ATOM_ATOM,
            32,
            &net_type
        );*/
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
        /*let transient_for = [parent_window.id];
        change_property(
            &system.connection,
            PROP_MODE_REPLACE as u8,
            window.id as u32,
            system.wm_transient_for,
            ATOM_WINDOW,
            32,
            &transient_for
        );*/
        system.connection.flush();
        Ok(window)
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
        map_window(&self.system.connection,self.id as u32);
        self.system.connection.flush();
        //XSync(self.system.connection.get_raw_dpy(),False);
    }

    pub fn hide(&self) {
        //println!("hide {}",self.id);
        unmap_window(&self.system.connection,self.id as u32);
        self.system.connection.flush();
        //XSync(self.system.connection.get_raw_dpy(),False);
    }

    pub fn set_rect(&self,r: &Rect<i32>) {
        let values = [
            (CONFIG_WINDOW_X as u16,r.o.x as u32),
            (CONFIG_WINDOW_Y as u16,r.o.y as u32),
            (CONFIG_WINDOW_WIDTH as u16,r.s.x as u32),
            (CONFIG_WINDOW_HEIGHT as u16,r.s.y as u32),
        ];
        configure_window(&self.system.connection,self.id as u32,&values);
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        //self.system.windows.borrow_mut().remove(&self.id);
        unsafe { glXMakeCurrent(self.system.connection.get_raw_dpy(),self.system.hidden_window,self.system.context); }
        unmap_window(&self.system.connection,self.id as u32);
        destroy_window(&self.system.connection,self.id as u32);    
    }
}
