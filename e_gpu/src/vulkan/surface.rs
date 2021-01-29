// E - GPU (Vulkan) - Surface
// Desmond Germans, 2020

use {
    crate::*,
    std::{
        ptr::null_mut,
        mem::MaybeUninit,
        rc::Rc,
    },
    vulkan_sys::*,
};

pub struct Surface {
    pub(crate) session: Rc<Session>,
    pub(crate) surface: VkSurfaceKHR,
}

impl Surface {

    pub fn new(session: &Rc<Session>,window: &Window) -> Option<Surface> {
        let create_info = VkXcbSurfaceCreateInfoKHR {
            sType: VkStructureType_VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
            pNext: null_mut(),
            flags: 0,
            connection: window.system.connection as *mut xcb_connection_t,
            window: window.window,
        };
        let mut surface = MaybeUninit::uninit();
        unsafe { vkCreateXcbSurfaceKHR(session.gpu.instance,&create_info,null_mut(),surface.as_mut_ptr()) };
        let surface = unsafe { surface.assume_init() };
        Some(Surface {
            session: Rc::clone(session),
            surface: surface,
        })
    }
}