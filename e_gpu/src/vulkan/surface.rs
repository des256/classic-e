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
    pub(crate) gpu: Rc<GPU>,
    pub(crate) surface: VkSurfaceKHR,
    pub(crate) devices: Vec<VkPhysicalDevice>,
}

impl Surface {

    pub fn new(gpu: &Rc<GPU>,window: &Window) -> Option<Surface> {

        let create_info = VkXcbSurfaceCreateInfoKHR {
            sType: VkStructureType_VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
            pNext: null_mut(),
            flags: 0,
            connection: window.system.connection as *mut xcb_connection_t,
            window: window.window,
        };
        let mut surface = MaybeUninit::uninit();
        let result = unsafe { vkCreateXcbSurfaceKHR(gpu.instance,&create_info,null_mut(),surface.as_mut_ptr()) };
        if result != VkResult_VK_SUCCESS {
            println!("result = {}",result);
            return None;            
        };
        let surface = unsafe { surface.assume_init() };

        for physical_device in &gpu.physical_devices {
            println!("device {}:",physical_device.name);
            for i in 0..physical_device.queue_families.len() {
                let mut support: VkBool32 = 0;
                unsafe { vkGetPhysicalDeviceSurfaceSupportKHR(physical_device.physical_device,i as u32,surface,&mut support) };
                println!("    queue family {}: {}",i,support);
            }
        }
        Some(Surface {
            gpu: Rc::clone(gpu),
            surface: surface,
            devices: Vec::new(),
        })
    }
}
