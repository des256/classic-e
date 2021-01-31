// E - GPU (Vulkan) - SwapChain
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

pub struct SwapChain {
    pub session: Rc<Session>,
    pub surface: Rc<Surface>,
    pub(crate) vk_swapchain: VkSwapchainKHR,
    // TODO: references to the actual images
}

impl SwapChain {

    pub fn new(session: &Rc<Session>,surface: &Rc<Surface>) -> Option<SwapChain> {

        let mut capabilities = MaybeUninit::uninit();
        unsafe { vkGetPhysicalDeviceSurfaceCapabilitiesKHR(surface.gpu.vk_physical_device,surface.vk_surface,capabilities.as_mut_ptr()) };
        let capabilities = unsafe { capabilities.assume_init() };
        let mut extent = VkExtent2D { width: 0,height: 0 };
        if capabilities.currentExtent.width != 0xFFFFFFFF {
            extent = capabilities.currentExtent;
        }
        else {
            extent = VkExtent2D { width: surface.window.r.get().s.x as u32,height: surface.window.r.get().s.y as u32 };
            if extent.width < capabilities.minImageExtent.width {
                extent.width = capabilities.minImageExtent.width;
            }
            if extent.height < capabilities.minImageExtent.height {
                extent.height = capabilities.minImageExtent.height;
            }
            if extent.width > capabilities.maxImageExtent.width {
                extent.width = capabilities.maxImageExtent.width;
            }
            if extent.height > capabilities.maxImageExtent.height {
                extent.height = capabilities.maxImageExtent.height;
            }
        }
        println!("extent = {},{}",extent.width,extent.height);

        let mut image_count = capabilities.minImageCount + 1;
        if (capabilities.maxImageCount != 0) && (image_count > capabilities.maxImageCount) {
            image_count = capabilities.maxImageCount;
        }
        println!("image_count = {}",image_count);

        let mut count = 0u32;
        unsafe { vkGetPhysicalDeviceSurfaceFormatsKHR(surface.gpu.vk_physical_device,surface.vk_surface,&mut count,null_mut()) };
        if count == 0 {
#[cfg(feature="debug_output")]
            println!("No formats supported.");
            unsafe { vkDestroySurfaceKHR(surface.gpu.system.vk_instance,surface.vk_surface,null_mut()) };
            return None;
        }
        let mut formats = vec![VkSurfaceFormatKHR {
            format: 0,
            colorSpace: 0,
        }; count as usize];
        unsafe { vkGetPhysicalDeviceSurfaceFormatsKHR(surface.gpu.vk_physical_device,surface.vk_surface,&mut count,formats.as_mut_ptr()) };
        let mut format_supported = false;
        for i in 0..formats.len() {
            if (formats[i].format == VkFormat_VK_FORMAT_B8G8R8A8_SRGB) && 
               (formats[i].colorSpace == VkColorSpaceKHR_VK_COLOR_SPACE_SRGB_NONLINEAR_KHR) {
                format_supported = true;
            }
        }
        if !format_supported {
#[cfg(feature="debug_output")]
            println!("Format ARGB8 not supported.");
            unsafe { vkDestroySurfaceKHR(surface.gpu.system.vk_instance,surface.vk_surface,null_mut()) };
            return None;
        }
        println!("format = {}",VkFormat_VK_FORMAT_B8G8R8A8_SRGB);

        let mut count = 0u32;
        unsafe { vkGetPhysicalDeviceSurfacePresentModesKHR(surface.gpu.vk_physical_device,surface.vk_surface,&mut count,null_mut()) };
        if count == 0 {
#[cfg(feature="debug_output")]
            println!("No present modes supported.");
            unsafe { vkDestroySurfaceKHR(surface.gpu.system.vk_instance,surface.vk_surface,null_mut()) };
            return None;
        }
        let mut modes = vec![0 as VkPresentModeKHR; count as usize];
        unsafe { vkGetPhysicalDeviceSurfacePresentModesKHR(surface.gpu.vk_physical_device,surface.vk_surface,&mut count,modes.as_mut_ptr()) };
        let mut present_mode = VkPresentModeKHR_VK_PRESENT_MODE_FIFO_KHR;
        for mode in &modes {
            if (*mode == VkPresentModeKHR_VK_PRESENT_MODE_MAILBOX_KHR) {
                present_mode = *mode;
            }
        }
        println!("present_mode = {}",present_mode);

        let create_info = VkSwapchainCreateInfoKHR {
            sType: VkStructureType_VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
            pNext: null_mut(),
            flags: 0,
            surface: surface.vk_surface,
            minImageCount: image_count,
            imageFormat: VkFormat_VK_FORMAT_B8G8R8A8_SRGB,
            imageColorSpace: VkColorSpaceKHR_VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
            imageExtent: extent,
            imageArrayLayers: 1,
            imageUsage: VkImageUsageFlagBits_VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
            imageSharingMode: VkSharingMode_VK_SHARING_MODE_EXCLUSIVE,
            queueFamilyIndexCount: 0,
            pQueueFamilyIndices: null_mut(),
            preTransform: capabilities.currentTransform,
            compositeAlpha: VkCompositeAlphaFlagBitsKHR_VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
            presentMode: present_mode,
            clipped: VK_TRUE,
            oldSwapchain: null_mut(),
        };
        let mut vk_swapchain = MaybeUninit::uninit();
        match unsafe { vkCreateSwapchainKHR(session.vk_device,&create_info,null_mut(),vk_swapchain.as_mut_ptr()) } {
            VkResult_VK_SUCCESS => { },
            code => {
#[cfg(feature="debug_output")]
                println!("Unable to create swap chain (error {})",code);
                return None;
            },
        }
        let vk_swapchain = unsafe { vk_swapchain.assume_init() };

        Some(SwapChain {
            session: Rc::clone(session),
            surface: Rc::clone(surface),
            vk_swapchain: vk_swapchain,
        })
    }
}

impl Drop for SwapChain {
    fn drop(&mut self) {
        unsafe { vkDestroySwapchainKHR(self.session.vk_device,self.vk_swapchain,null_mut()) };
    }
}
