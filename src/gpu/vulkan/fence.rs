// E - GPU (Vulkan) - Fence
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

pub struct Fence {
    pub session: Rc<Session>,
#[doc(hidden)]
    pub(crate) vk_fence: VkFence,
}

impl Session {

    pub fn create_fence(self: &Rc<Self>) -> Option<Rc<Fence>> {

        let info = VkFenceCreateInfo {
            sType: VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
            pNext: null_mut(),
            flags: VK_FENCE_CREATE_SIGNALED_BIT,
        };
        let mut vk_fence = MaybeUninit::uninit();
        match unsafe { vkCreateFence(self.vk_device,&info,null_mut(),vk_fence.as_mut_ptr()) } {
            VK_SUCCESS => { },
            code => {
#[cfg(feature="debug_output")]
                println!("Unable to create Vulkan fence (error {}).",code);
                return None;
            },
        }
        Some(Rc::new(Fence {
            session: Rc::clone(self),
            vk_fence: unsafe { vk_fence.assume_init() },
        }))
    }
}

impl Fence {
    pub fn wait(&self) {
        unsafe { vkWaitForFences(self.session.vk_device,1,&self.vk_fence,VK_TRUE,0xFFFFFFFFFFFFFFFF) };
    }

    pub fn reset(&self) {
        unsafe { vkResetFences(self.session.vk_device,1,&self.vk_fence) };
    }
}

impl Drop for Fence {
    fn drop(&mut self) {
        unsafe { vkDestroyFence(self.session.vk_device,self.vk_fence,null_mut()) };
    }
}
