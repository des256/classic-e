// E - GPU (Vulkan) - Queue
// Desmond Germans, 2020

use {
    crate::*,
    std::{
        mem::MaybeUninit,
        rc::Rc,
    },
    sys_sys::*,
};

pub struct Queue {
    pub session: Rc<Session>,
    pub(crate) vk_queue: VkQueue,
}

impl Queue {

    pub fn obtain(session: &Rc<Session>,family: usize,index: usize) -> Option<Queue> {
        let mut vk_queue = MaybeUninit::uninit();
        unsafe { vkGetDeviceQueue(session.vk_device,family as u32,index as u32,vk_queue.as_mut_ptr()) };
        let vk_queue = unsafe { vk_queue.assume_init() };

        Some(Queue {
            session: Rc::clone(&session),
            vk_queue: vk_queue,
        })
    }
}
