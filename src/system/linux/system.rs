// E - Linux - System
// Desmond Germans, 2020

#[doc(hidden)]
use {
    crate::*,
    std::{
        os::{
            raw::{
                c_int,
            },
        },
        ptr::null_mut,
        mem::MaybeUninit,
        rc::Rc,
    },
    sys_sys::*,
    libc::{
        epoll_create1,
        epoll_ctl,
        EPOLL_CTL_ADD,
        epoll_event,
        EPOLLIN,
        epoll_wait,
    },
};

/// Main system context.
pub struct System {
    #[doc(hidden)]
    pub connection: *mut xcb_connection_t,
    pub screen: *const xcb_screen_t,
    pub(crate) wm_protocols: u32,
    pub(crate) wm_delete_window: u32,
    pub(crate) wm_motif_hints: u32,
    _wm_transient_for: u32,
    _wm_net_type: u32,
    _wm_net_type_utility: u32,
    _wm_net_type_dropdown_menu: u32,
    pub(crate) wm_net_state: u32,
    pub(crate) wm_net_state_above: u32,
    pub(crate) e_window_pointer: u32,
    epfd: c_int,
#[cfg(feature="gpu_vulkan")]
    pub(crate) vk_instance: VkInstance,
    #[cfg(feature="gpu_vulkan")]
    pub(crate) vk_physical_devices: Vec<VkPhysicalDevice>,
}

fn xcb_intern_atom_(connection: *mut xcb_connection_t,name: &str) -> xcb_intern_atom_cookie_t {
    let i8_name = unsafe { std::mem::transmute::<_,&[i8]>(name.as_bytes()) };
    unsafe { xcb_intern_atom(connection,false as u8,name.len() as u16,i8_name.as_ptr()) }
}

impl System {
    /// Create new system context.
    /// 
    /// **Returns**
    /// 
    /// * `Ok(System)` - The new system context.
    /// * `Err(SystemError)` - The system context could not be created.
    pub fn new() -> Option<Rc<System>> {

        let connection = unsafe { xcb_connect(null_mut(),null_mut()) };
        if connection == null_mut() {
#[cfg(feature="debug_output")]
            println!("Unable to connect to X server.");
            return None;
        }

        let fd = unsafe { xcb_get_file_descriptor(connection) };

        let protocols_cookie = xcb_intern_atom_(connection,"WM_PROTOCOLS");
        let delete_window_cookie = xcb_intern_atom_(connection,"WM_DELETE_WINDOW");
        let motif_hints_cookie = xcb_intern_atom_(connection,"_MOTIF_WM_HINTS");
        let transient_for_cookie = xcb_intern_atom_(connection,"WM_TRANSIENT_FOR");
        let net_type_cookie = xcb_intern_atom_(connection,"_NET_WM_TYPE");
        let net_type_utility_cookie = xcb_intern_atom_(connection,"_NET_WM_TYPE_UTILITY");
        let net_type_dropdown_menu_cookie = xcb_intern_atom_(connection,"_NET_WM_TYPE_DROPDOWN_MENU");
        let net_state_cookie = xcb_intern_atom_(connection,"_NET_WM_STATE");
        let net_state_above_cookie = xcb_intern_atom_(connection,"_NET_WM_STATE_ABOVE");
        let e_window_pointer_cookie = xcb_intern_atom_(connection,"E_WINDOW_POINTER");

        let wm_protocols = unsafe { (*xcb_intern_atom_reply(connection,protocols_cookie,null_mut())).atom };
        let wm_delete_window = unsafe { (*xcb_intern_atom_reply(connection,delete_window_cookie,null_mut())).atom };
        let wm_motif_hints = unsafe { (*xcb_intern_atom_reply(connection,motif_hints_cookie,null_mut())).atom };
        let wm_transient_for = unsafe { (*xcb_intern_atom_reply(connection,transient_for_cookie,null_mut())).atom };
        let wm_net_type = unsafe { (*xcb_intern_atom_reply(connection,net_type_cookie,null_mut())).atom };
        let wm_net_type_utility = unsafe { (*xcb_intern_atom_reply(connection,net_type_utility_cookie,null_mut())).atom };
        let wm_net_type_dropdown_menu = unsafe { (*xcb_intern_atom_reply(connection,net_type_dropdown_menu_cookie,null_mut())).atom };
        let wm_net_state = unsafe { (*xcb_intern_atom_reply(connection,net_state_cookie,null_mut())).atom };
        let wm_net_state_above = unsafe { (*xcb_intern_atom_reply(connection,net_state_above_cookie,null_mut())).atom };
        let e_window_pointer = unsafe { (*xcb_intern_atom_reply(connection,e_window_pointer_cookie,null_mut())).atom };

        let setup = unsafe { xcb_get_setup(connection) };
        if setup == null_mut() {
#[cfg(feature="debug_output")]
            println!("Unable to obtain X server setup.");
            return None;
        }
        let screen = unsafe { xcb_setup_roots_iterator(setup).data };
        if screen == null_mut() {
#[cfg(feature="debug_output")]
            println!("Unable to obtain X root screen.");
            return None;
        }

        let epfd = unsafe { epoll_create1(0) };
        let mut epe = [epoll_event { events: EPOLLIN as u32,u64: 0, }];
        unsafe { epoll_ctl(epfd,EPOLL_CTL_ADD,fd,epe.as_mut_ptr()) };

#[cfg(feature="gpu_vulkan")]
        let vk_instance = {
            let app_info = VkApplicationInfo {
                sType: VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
                pNext: null_mut(),
                pApplicationName: null_mut(),
                applicationVersion: (1 << 22) as u32,
                pEngineName: null_mut(),
                engineVersion: (1 << 22) as u32,
                apiVersion: ((1 << 22) | (2 << 11)) as u32,
            };
            let extension_names = [
                VK_KHR_SURFACE_EXTENSION_NAME.as_ptr(),
                VK_KHR_XCB_SURFACE_EXTENSION_NAME.as_ptr(),
                VK_EXT_DEBUG_REPORT_EXTENSION_NAME.as_ptr(),
            ];
            let layer_names = [
                b"VK_LAYER_KHRONOS_validation\0",
            ];
            let create_info = VkInstanceCreateInfo {
                sType: VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
                pApplicationInfo: &app_info,
                enabledExtensionCount: extension_names.len() as u32,
                ppEnabledExtensionNames: extension_names.as_ptr() as *const *const i8,
                enabledLayerCount: layer_names.len() as u32,
                flags: 0,
                pNext: null_mut(),
                ppEnabledLayerNames: layer_names.as_ptr() as *const *const i8,
            };
            let mut instance = MaybeUninit::uninit();
            match unsafe { vkCreateInstance(&create_info,null_mut(),instance.as_mut_ptr()) } {
                VkResult_VK_SUCCESS => { },
                code => {
#[cfg(feature="debug_output")]
                    println!("Unable to create Vulkan instance (error {}).",code);
                    return None;
                },
            }
            unsafe { instance.assume_init() }
        };

#[cfg(feature="gpu_vulkan")]
        let vk_physical_devices = {
            // find physical devices
            let mut count = 0u32;
            unsafe { vkEnumeratePhysicalDevices(vk_instance,&mut count,null_mut()) };
            if count == 0 {
#[cfg(feature="debug_output")]
                println!("Unable to find Vulkan physical devices.");
                return None;
            }
            let mut devices = vec![null_mut() as VkPhysicalDevice; count as usize];
            unsafe { vkEnumeratePhysicalDevices(vk_instance,&mut count,devices.as_mut_ptr()) };
            devices
        };

        Some(Rc::new(System {
            connection: connection,
            screen: screen,
            wm_protocols: wm_protocols,
            wm_delete_window: wm_delete_window,
            wm_motif_hints: wm_motif_hints,
            _wm_transient_for: wm_transient_for,
            _wm_net_type: wm_net_type,
            _wm_net_type_utility: wm_net_type_utility,
            _wm_net_type_dropdown_menu: wm_net_type_dropdown_menu,
            wm_net_state: wm_net_state,
            wm_net_state_above: wm_net_state_above,
            e_window_pointer: e_window_pointer,
            epfd: epfd,
#[cfg(feature="gpu_vulkan")]
            vk_instance: vk_instance,
#[cfg(feature="gpu_vulkan")]
            vk_physical_devices: vk_physical_devices,
        }))
    }

    pub fn enumerate_gpu_names(&self) -> Vec<String> {
        let mut gpu_names = Vec::<String>::new();
#[cfg(feature="gpu_vulkan")]
        for device in &self.vk_physical_devices {
            let mut properties = MaybeUninit::uninit();
            unsafe { vkGetPhysicalDeviceProperties(*device,properties.as_mut_ptr()) };
            let properties = unsafe { properties.assume_init() };
            let slice: &[u8] = unsafe { &*(&properties.deviceName as *const [i8] as *const [u8]) };
            let name = std::str::from_utf8(slice).unwrap();
            gpu_names.push(name.to_string());
        }
        gpu_names
    }

    // translate X11 events to Event events that can be handled by the windows
    fn translate_event(&self,xcb_event: *mut xcb_generic_event_t) -> Option<(xcb_window_t,Event)> {
        match (unsafe { *xcb_event }.response_type & 0x7F) as u32 {
            XCB_EXPOSE => {
                let expose = xcb_event as *const xcb_expose_event_t;
                //let expose = unsafe { std::mem::transmute::<_,xcb_expose_event_t>(xcb_event) };
                //let r = rect!(expose.x as isize,expose.y as isize,expose.width() as isize,expose.height() as isize);
                let xcb_window = unsafe { *expose }.window;
                return Some((xcb_window,Event::Render));
            },
            XCB_KEY_PRESS => {
                let key_press = xcb_event as *const xcb_key_press_event_t;
                let xcb_window = unsafe { *key_press }.event;
                return Some((xcb_window,Event::KeyPress(unsafe { *key_press }.detail as u8)));
            },
            XCB_KEY_RELEASE => {
                let key_release = xcb_event as *const xcb_key_release_event_t;
                let xcb_window = unsafe { *key_release }.event;
                return Some((xcb_window,Event::KeyRelease(unsafe { *key_release }.detail as u8)));
            },
            XCB_BUTTON_PRESS => {
                let button_press = xcb_event as *const xcb_button_press_event_t;
                let p = vec2!(unsafe { *button_press }.event_x as i32,unsafe { *button_press }.event_y as i32);
                let xcb_window = unsafe { *button_press }.event;
                match unsafe { *button_press }.detail {
                    1 => { return Some((xcb_window,Event::MousePress(p,MouseButton::Left))); },
                    2 => { return Some((xcb_window,Event::MousePress(p,MouseButton::Middle))); },
                    3 => { return Some((xcb_window,Event::MousePress(p,MouseButton::Right))); },
                    4 => { return Some((xcb_window,Event::MouseWheel(MouseWheel::Up))); },
                    5 => { return Some((xcb_window,Event::MouseWheel(MouseWheel::Down))); },
                    6 => { return Some((xcb_window,Event::MouseWheel(MouseWheel::Left))); },
                    7 => { return Some((xcb_window,Event::MouseWheel(MouseWheel::Right))); },
                    _ => { },
                }        
            },
            XCB_BUTTON_RELEASE => {
                let button_release = xcb_event as *const xcb_button_release_event_t;
                let p = vec2!(
                    unsafe { *button_release }.event_x as i32,
                    unsafe { *button_release }.event_y as i32
                );
                let xcb_window = unsafe { *button_release }.event;
                match unsafe { *button_release }.detail {
                    1 => { return Some((xcb_window,Event::MouseRelease(p,MouseButton::Left))); },
                    2 => { return Some((xcb_window,Event::MouseRelease(p,MouseButton::Middle))); },
                    3 => { return Some((xcb_window,Event::MouseRelease(p,MouseButton::Right))); },
                    _ => { },
                }        
            },
            XCB_MOTION_NOTIFY => {
                let motion_notify = xcb_event as *const xcb_motion_notify_event_t;
                let p = vec2!(
                    unsafe { *motion_notify }.event_x as i32,
                    unsafe { *motion_notify }.event_y as i32
                );
                let xcb_window = unsafe { *motion_notify }.event;
                return Some((xcb_window,Event::MouseMove(p)));
            },
            XCB_CONFIGURE_NOTIFY => {
                let configure_notify = xcb_event as *const xcb_configure_notify_event_t;
                let r = rect!(
                    unsafe { *configure_notify }.x as i32,
                    unsafe { *configure_notify }.y as i32,
                    unsafe { *configure_notify }.width as i32,
                    unsafe { *configure_notify }.height as i32
                );
                let xcb_window = unsafe { *configure_notify }.window;
                return Some((xcb_window,Event::Configure(r)));
            },
            XCB_CLIENT_MESSAGE => {
                let client_message = xcb_event as *const xcb_client_message_event_t;
                let atom = unsafe { (*client_message).data.data32[0] };
                if atom == self.wm_delete_window {
                    let xcb_window = unsafe { *client_message }.window;
                    return Some((xcb_window,Event::Close));
                }
            },
            _ => { },
        }
        None
    }

    /// Flush all pending window events.
    /// 
    /// This processes each pending event from the system's event queue by
    /// calling `handle` on the associated handlers.
    pub fn flush(&self) {
        let mut event = unsafe { xcb_poll_for_event(self.connection) };
        while event != null_mut() {
            if let Some((xcb_window,event)) = self.translate_event(event) {
                let cookie = unsafe { xcb_get_property(self.connection,0u8,xcb_window,self.e_window_pointer,xcb_atom_enum_t_XCB_ATOM_ANY,0,2) };
                let reply = unsafe { xcb_get_property_reply(self.connection,cookie,null_mut()) };  // assuming no errors
                let encoded_pointer = unsafe { xcb_get_property_value(reply) };
                let window_pointer = encoded_pointer as *const Window;
                unsafe { (*window_pointer).handle_event(event); }
            }
            event = unsafe { xcb_poll_for_event(self.connection) };
        }
    }

    /// Wait until new events are available on the system's event queue.
    pub fn wait(&self) {
        let mut epe = [epoll_event { events: EPOLLIN as u32,u64: 0, }];
        unsafe { epoll_wait(self.epfd,epe.as_mut_ptr(),1,-1) };
    }

    /// Capture mouse pointer.
    /// 
    /// After this, all mouse events are sent to the indicated window, even if
    /// they occur outside the window's range.
    /// 
    /// **Arguments**
    /// 
    /// * `id` - Unique ID of the window.
    pub fn capture_mouse(&self,_id: u64) {
        /*println!("XGrabPointer");
        grab_pointer(
            &self.connection,
            false,
            id as u32,
            (EVENT_MASK_BUTTON_PRESS | EVENT_MASK_BUTTON_RELEASE| EVENT_MASK_POINTER_MOTION) as u16,
            GRAB_MODE_ASYNC as u8,
            GRAB_MODE_ASYNC as u8,
            WINDOW_NONE,
            CURSOR_NONE,
            TIME_CURRENT_TIME
        );*/
    }
    
    /// Release the mouse pointer.
    /// 
    /// Events are sent to all windows again.
    pub fn release_mouse(&self) {
        //println!("XUngrabPointer");
        //ungrab_pointer(&self.connection,TIME_CURRENT_TIME);
    }

    /// Set current mouse cursor shape.
    ///
    /// **Arguments**
    /// * `id` - Unique ID of the window.
    /// * `n` - Mouse cursor shape.
    pub fn set_mousecursor(&self,_id: u64,_n: usize) {
        //let values = [(CW_CURSOR,self.cursors[n])];
        //change_window_attributes(&self.connection,id as u32,&values);
    }
}

impl Drop for System {
    fn drop(&mut self) {
#[cfg(feature="gpu_vulkan")]
        unsafe { vkDestroyInstance(self.vk_instance,null_mut()) };
        // TODO: close connection
    }
}
