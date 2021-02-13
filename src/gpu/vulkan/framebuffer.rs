// E - GPU (Vulkan) - Framebuffer
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

pub struct Framebuffer {
    pub imageview: Rc<ImageView>,
    pub graphics_pipeline: Rc<GraphicsPipeline>,
    pub size: Vec2<usize>,
#[doc(hidden)]
    pub(crate) vk_framebuffer: VkFramebuffer,
}

impl ImageView {

    pub fn create_framebuffer(self: &Rc<Self>,size: Vec2<usize>,graphics_pipeline: &Rc<GraphicsPipeline>) -> Option<Rc<Framebuffer>> {

        let info = VkFramebufferCreateInfo {
            sType: VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            renderPass: graphics_pipeline.vk_render_pass,
            attachmentCount: 1,
            pAttachments: &self.vk_imageview,
            width: size.x as u32,
            height: size.y as u32,
            layers: 1,
        };
        let mut vk_framebuffer = MaybeUninit::uninit();
        match unsafe { vkCreateFramebuffer(self.image.session.vk_device,&info,null_mut(),vk_framebuffer.as_mut_ptr()) } {
            VK_SUCCESS => { },
            code => {
        #[cfg(feature="debug_output")]
                println!("Unable to create Vulkan frame buffer (error {})",code);
                return None;
            }
        }
        Some(Rc::new(Framebuffer {
            imageview: Rc::clone(self),
            graphics_pipeline: Rc::clone(graphics_pipeline),
            size: size,
            vk_framebuffer: unsafe { vk_framebuffer.assume_init() },
        }))
    }
}

impl Drop for Framebuffer {

    fn drop(&mut self) {
        unsafe { vkDestroyFramebuffer(self.imageview.image.session.vk_device,self.vk_framebuffer,null_mut()) };
    }
}
