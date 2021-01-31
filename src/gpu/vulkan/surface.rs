// E - GPU (Vulkan) - Surface
// Desmond Germans, 2020

use {
    crate::*,
    std::{
        ptr::null_mut,
        mem::MaybeUninit,
        rc::Rc,
    },
    sys_sys::*,
};

pub struct Surface {
    pub gpu: Rc<GPU>,
    pub window: Rc<Window>,
    pub queue_families: Vec<usize>,
    pub(crate) vk_surface: VkSurfaceKHR,
}

impl Surface {

    pub fn new(gpu: &Rc<GPU>,window: &Rc<Window>) -> Option<Rc<Surface>> {

        let create_info = VkXcbSurfaceCreateInfoKHR {
            sType: VkStructureType_VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
            pNext: null_mut(),
            flags: 0,
            connection: window.system.connection as *mut xcb_connection_t,
            window: window.window,
        };
        let mut vk_surface = MaybeUninit::uninit();
        match unsafe { vkCreateXcbSurfaceKHR(gpu.system.vk_instance,&create_info,null_mut(),vk_surface.as_mut_ptr()) } {
            VkResult_VK_SUCCESS => { },
            code => {
#[cfg(feature="debug_output")]
                println!("Unable to create Vulkan XCB surface (error {})",code);
                return None;
            },
        }
        let vk_surface = unsafe { vk_surface.assume_init() };

        let mut queue_families = Vec::<usize>::new();
        for i in 0..gpu.queue_families.len() {
            let mut support: VkBool32 = 0;
            unsafe { vkGetPhysicalDeviceSurfaceSupportKHR(gpu.vk_physical_device,i as u32,vk_surface,&mut support) };
            if support != 0 {
                queue_families.push(i);
            }
        }

        Some(Rc::new(Surface {
            gpu: Rc::clone(gpu),
            window: Rc::clone(window),
            queue_families: queue_families,
            vk_surface: vk_surface,
        }))
    }
}

impl Drop for Surface {

    fn drop(&mut self) {
        unsafe { vkDestroySurfaceKHR(self.gpu.system.vk_instance,self.vk_surface,null_mut()) };
    }
}
