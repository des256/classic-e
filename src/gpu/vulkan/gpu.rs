// E - GPU (Vulkan) - Instance
// Desmond Germans, 2020

use {
    crate::*,
    std::{
        ptr::null_mut,
        rc::Rc,
    },
    sys_sys::*,
};

pub struct QueueFamily {
    pub graphics: bool,
    pub compute: bool,
    pub transfer: bool,
    pub sparse: bool,
    pub max_queues: usize,
    // timestamp valid bits
    // minimum image transfer granularity
}

pub struct GPU {
    pub system: Rc<System>,
    pub queue_families: Vec<QueueFamily>,

    pub(crate) vk_physical_device: VkPhysicalDevice,
}

impl GPU {

    pub fn new(system: &Rc<System>,index: usize) -> Option<Rc<GPU>> {

        if index >= system.vk_physical_devices.len() {
#[cfg(feature="debug_output")]
            println!("GPU index {} out of range (0..{})",index,system.vk_physical_devices.len());
            return None;
        }

        let vk_physical_device = system.vk_physical_devices[index];

        let mut queue_families = Vec::<QueueFamily>::new();
        let mut count = 0u32;
        unsafe { vkGetPhysicalDeviceQueueFamilyProperties(vk_physical_device,&mut count,null_mut()) };
        if count > 0 {
            let mut families = vec![VkQueueFamilyProperties {
                queueFlags: 0,
                queueCount: 0,
                timestampValidBits: 0,
                minImageTransferGranularity: VkExtent3D {
                    width: 0,
                    height: 0,
                    depth: 0,
                },
            }; count as usize];
            unsafe { vkGetPhysicalDeviceQueueFamilyProperties(vk_physical_device,&mut count,families.as_mut_ptr()) };
            for family in families {
                queue_families.push(QueueFamily {
                    graphics: (family.queueFlags & VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT) != 0,
                    compute: (family.queueFlags & VkQueueFlagBits_VK_QUEUE_COMPUTE_BIT) != 0,
                    transfer: (family.queueFlags & VkQueueFlagBits_VK_QUEUE_TRANSFER_BIT) != 0,
                    sparse: (family.queueFlags & VkQueueFlagBits_VK_QUEUE_SPARSE_BINDING_BIT) != 0,
                    max_queues: family.queueCount as usize,
                });
            }
        }
    
        Some(Rc::new(GPU {
            system: Rc::clone(system),
            queue_families: queue_families,
            vk_physical_device: vk_physical_device,
        }))
    }
}
