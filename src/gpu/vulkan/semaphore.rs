// E - GPU (Vulkan) - Semaphore
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

pub struct Semaphore {
    pub session: Rc<Session>,
    pub(crate) vk_semaphore: VkSemaphore,
}

impl Semaphore {
    pub fn new(session: &Rc<Session>) -> Option<Semaphore> {
        let info = VkSemaphoreCreateInfo {
            sType: VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
        };
        let mut vk_semaphore = MaybeUninit::uninit();
        match unsafe { vkCreateSemaphore(session.vk_device,&info,null_mut(),vk_semaphore.as_mut_ptr()) } {
            VK_SUCCESS => { },
            code => {
#[cfg(feature="debug_output")]
                println!("Unable to create semaphore (error {}).",code);
                return None;
            },
        }
        let vk_semaphore = unsafe { vk_semaphore.assume_init() };
        Some(Semaphore {
            session: Rc::clone(session),
            vk_semaphore: vk_semaphore,
        })
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        unsafe { vkDestroySemaphore(self.session.vk_device,self.vk_semaphore,null_mut()) };
    }
}
