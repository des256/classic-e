// E - GPU (Vulkan) - Instance
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

pub struct GPU {
    pub(crate) system: Rc<System>,
    pub(crate) instance: VkInstance,
    pub(crate) physical_devices: Vec<VkPhysicalDevice>,
}

pub struct QueueFamily {
    pub graphics: bool,
    pub compute: bool,
    pub transfer: bool,
    pub max_queues: usize,
    // timestamp valid bits
    // minimum image transfer granularity
}

impl GPU {

    pub fn new(system: &Rc<System>) -> Result<Rc<GPU>,SystemError> {

        // create instance
        let app_info = VkApplicationInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: null_mut(),
            pApplicationName: null_mut(),
            applicationVersion: (1 << 22) as u32,
            pEngineName: null_mut(),
            engineVersion: (1 << 22) as u32,
            apiVersion: ((1 << 22) | (2 << 11)) as u32,
        };
        let create_info = VkInstanceCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pApplicationInfo: &app_info,
            enabledExtensionCount: 0,
            ppEnabledExtensionNames: null_mut(),
            enabledLayerCount: 0,
            flags: 0,
            pNext: null_mut(),
            ppEnabledLayerNames: null_mut(),
        };
        let mut instance = MaybeUninit::uninit();
        match unsafe { vkCreateInstance(&create_info,null_mut(),instance.as_mut_ptr()) } {
            VkResult_VK_SUCCESS => { },
            _ => { return Err(SystemError::Generic); },
        }
        let instance = unsafe { instance.assume_init() };

        // find physical devices
        let mut device_count = 0u32;
        unsafe { vkEnumeratePhysicalDevices(instance,&mut device_count,null_mut()) };
        if device_count == 0 {
            return Err(SystemError::Generic);
        }
        let mut devices = vec![null_mut() as VkPhysicalDevice; device_count as usize];
        unsafe { vkEnumeratePhysicalDevices(instance,&mut device_count,devices.as_mut_ptr()) };
        
        Ok(Rc::new(GPU {
            system: Rc::clone(system),
            instance: instance,
            physical_devices: devices,
        }))
    }

    pub fn enumerate_devices(&self) -> Vec<String> {

        // collect physical device names
        let mut device_names: Vec<String> = Vec::new();
        for device in &self.physical_devices {
            let mut properties = MaybeUninit::uninit();
            unsafe { vkGetPhysicalDeviceProperties(*device,properties.as_mut_ptr()) };
            let properties = unsafe { properties.assume_init() };
            let slice: &[u8] = unsafe { &*(&properties.deviceName as *const [i8] as *const [u8]) };
            let name = std::str::from_utf8(slice).unwrap();
            device_names.push(name.to_string());
        }
        device_names
    }

    pub fn enumerate_queue_families(&self,index: usize) -> Vec<QueueFamily> {

        if index >= self.physical_devices.len() as usize {
            return Vec::new();
        }
        let physical_device = self.physical_devices[index];

        // find queue families
        let mut queue_family_count = 0u32;
        unsafe { vkGetPhysicalDeviceQueueFamilyProperties(physical_device,&mut queue_family_count,null_mut()) };
        if queue_family_count == 0 {
            return Vec::new();
        }
        let mut families = vec![VkQueueFamilyProperties {
            queueFlags: 0,
            queueCount: 0,
            timestampValidBits: 0,
            minImageTransferGranularity: VkExtent3D {
                width: 0,
                height: 0,
                depth: 0,
            },
        }; queue_family_count as usize];
        unsafe { vkGetPhysicalDeviceQueueFamilyProperties(physical_device,&mut queue_family_count,families.as_mut_ptr()) };
        let mut queue_families: Vec<QueueFamily> = Vec::new();
        for family in families {
            queue_families.push(QueueFamily {
                graphics: (family.queueFlags & VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT) != 0,
                compute: (family.queueFlags & VkQueueFlagBits_VK_QUEUE_COMPUTE_BIT) != 0,
                transfer: (family.queueFlags & VkQueueFlagBits_VK_QUEUE_TRANSFER_BIT) != 0,
                max_queues: family.queueCount as usize,
            });
        }
        queue_families
    }
}

impl Drop for GPU {

    fn drop(&mut self) {
        unsafe { vkDestroyInstance(self.instance,null_mut()) };
    }
}
