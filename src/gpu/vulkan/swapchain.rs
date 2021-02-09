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
    pub graphics_pipeline: Rc<GraphicsPipeline>,
    pub surface: Rc<Surface>,
    pub extent: Vec2<usize>,
    pub queue_index: usize,
    pub command_buffers: Vec<CommandBuffer>,
    pub(crate) vk_swapchain: VkSwapchainKHR,
    pub(crate) vk_images: Vec<VkImage>,
    pub(crate) vk_imageviews: Vec<VkImageView>,
    pub(crate) vk_framebuffers: Vec<VkFramebuffer>,
}

impl SwapChain {

    pub fn new(session: &Rc<Session>,graphics_pipeline: &Rc<GraphicsPipeline>,surface: &Rc<Surface>,queue_index: usize) -> Option<SwapChain> {

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
        let mut image_count = 0u32;
        match unsafe { vkGetSwapchainImagesKHR(session.vk_device,vk_swapchain,&mut image_count,null_mut()) } {
            VkResult_VK_SUCCESS => { },
            code => {
#[cfg(feature="debug_output")]
                println!("Unable to get swap chain image count (error {})",code);
                unsafe { vkDestroySwapchainKHR(session.vk_device,vk_swapchain,null_mut()) };
                return None;
            }
        }
        println!("swap chain images: {}",image_count);
        let mut vk_images = vec![null_mut() as VkImage; image_count as usize];
        match unsafe { vkGetSwapchainImagesKHR(session.vk_device,vk_swapchain,&mut image_count,vk_images.as_mut_ptr()) } {
            VkResult_VK_SUCCESS => { },
            code => {
#[cfg(feature="debug_output")]
                println!("Unable to get swap chain images (error {})",code);
                unsafe { vkDestroySwapchainKHR(session.vk_device,vk_swapchain,null_mut()) };
                return None;
            },
        }
        let mut vk_imageviews = vec![null_mut() as VkImageView; image_count as usize];
        let mut vk_framebuffers = vec![null_mut() as VkFramebuffer; image_count as usize];
        let mut command_buffers = Vec::<CommandBuffer>::new();
        for i in 0..image_count as usize {
            let create_info = VkImageViewCreateInfo {
                sType: VkStructureType_VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
                pNext: null_mut(),
                flags: 0,
                image: vk_images[i],
                viewType: VkImageViewType_VK_IMAGE_VIEW_TYPE_2D,
                format: VkFormat_VK_FORMAT_B8G8R8A8_SRGB,
                components: VkComponentMapping {
                    r: VkComponentSwizzle_VK_COMPONENT_SWIZZLE_IDENTITY,
                    g: VkComponentSwizzle_VK_COMPONENT_SWIZZLE_IDENTITY,
                    b: VkComponentSwizzle_VK_COMPONENT_SWIZZLE_IDENTITY,
                    a: VkComponentSwizzle_VK_COMPONENT_SWIZZLE_IDENTITY,
                },
                subresourceRange: VkImageSubresourceRange {
                    aspectMask: VkImageAspectFlagBits_VK_IMAGE_ASPECT_COLOR_BIT,
                    baseMipLevel: 0,
                    levelCount: 1,
                    baseArrayLayer: 0,
                    layerCount: 1,
                },            
            };
            match unsafe { vkCreateImageView(session.vk_device,&create_info,null_mut(),&mut vk_imageviews[i]) } {
                VkResult_VK_SUCCESS => { },
                code => {
#[cfg(feature="debug_output")]
                    println!("Unable to create Vulkan image view (error {})",code);
                    for k in 0..i {
                        unsafe { vkDestroyFramebuffer(session.vk_device,vk_framebuffers[k],null_mut()) };
                        unsafe { vkDestroyImageView(session.vk_device,vk_imageviews[k],null_mut()) };
                    }
                    unsafe { vkDestroySwapchainKHR(session.vk_device,vk_swapchain,null_mut()) };
                    return None;
                },
            }
            let create_info = VkFramebufferCreateInfo {
                sType: VkStructureType_VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
                pNext: null_mut(),
                flags: 0,
                renderPass: graphics_pipeline.vk_render_pass,
                attachmentCount: 1,
                pAttachments: &vk_imageviews[i],
                width: extent.width,
                height: extent.height,
                layers: 1,        
            };
            match unsafe { vkCreateFramebuffer(session.vk_device,&create_info,null_mut(),&mut vk_framebuffers[i]) } {
                VkResult_VK_SUCCESS => { },
                code => {
#[cfg(feature="debug_output")]
                    println!("Unable to create Vulkan frame buffer (error {})",code);
                    for k in 0..i {
                        unsafe { vkDestroyFramebuffer(session.vk_device,vk_framebuffers[k],null_mut()) };
                        unsafe { vkDestroyImageView(session.vk_device,vk_imageviews[k],null_mut()) };
                    }
                    unsafe { vkDestroyImageView(session.vk_device,vk_imageviews[i],null_mut()) };
                    unsafe { vkDestroySwapchainKHR(session.vk_device,vk_swapchain,null_mut()) };
                    return None;
                }
            }

            let alloc_info = VkCommandBufferAllocateInfo {
                sType: VkStructureType_VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
                pNext: null_mut(),
                commandPool: session.vk_command_pools[queue_index],
                level: VkCommandBufferLevel_VK_COMMAND_BUFFER_LEVEL_PRIMARY,
                commandBufferCount: 1,
            };
            let mut vk_command_buffer = null_mut() as VkCommandBuffer;
            match unsafe { vkAllocateCommandBuffers(session.vk_device,&alloc_info,&mut vk_command_buffer) } {
                VkResult_VK_SUCCESS => { },
                code => {
#[cfg(feature="debug_output")]
                    println!("Unable to create Vulkan command buffer (error {})",code);
                    for k in 0..i {
                        unsafe { vkDestroyFramebuffer(session.vk_device,vk_framebuffers[k],null_mut()) };
                        unsafe { vkDestroyImageView(session.vk_device,vk_imageviews[k],null_mut()) };
                    }
                    unsafe { vkDestroyFramebuffer(session.vk_device,vk_framebuffers[i],null_mut()) };
                    unsafe { vkDestroyImageView(session.vk_device,vk_imageviews[i],null_mut()) };
                    unsafe { vkDestroySwapchainKHR(session.vk_device,vk_swapchain,null_mut()) };
                    return None;
                }
            }
            command_buffers.push(CommandBuffer::new(session,queue_index,vk_command_buffer).unwrap());
        }

        Some(SwapChain {
            session: Rc::clone(session),
            graphics_pipeline: Rc::clone(graphics_pipeline),
            surface: Rc::clone(surface),
            extent: vec2![extent.width as usize,extent.height as usize],
            queue_index: queue_index,
            command_buffers: command_buffers,
            vk_swapchain: vk_swapchain,
            vk_images: vk_images,
            vk_imageviews: vk_imageviews,
            vk_framebuffers: vk_framebuffers,
        })
    }

    pub fn get_command_buffer(&self,index: usize) -> &CommandBuffer {
        &self.command_buffers[index]
    }
}

impl Drop for SwapChain {
    fn drop(&mut self) {
        for vk_framebuffer in &self.vk_framebuffers {
            unsafe { vkDestroyFramebuffer(self.session.vk_device,*vk_framebuffer,null_mut()) };
        }
        for vk_imageview in &self.vk_imageviews {
            unsafe { vkDestroyImageView(self.session.vk_device,*vk_imageview,null_mut()) };
        }
        unsafe { vkDestroySwapchainKHR(self.session.vk_device,self.vk_swapchain,null_mut()) };
    }
}
