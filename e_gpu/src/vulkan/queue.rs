// E - GPU (Vulkan) - Queue
// Desmond Germans, 2020

use {
    crate::*,
    std::{
        mem::MaybeUninit,
        rc::Rc,
    },
    vulkan_sys::*,
};

pub struct Queue {
    pub(crate) session: Rc<Session>,
    pub(crate) queue: VkQueue,
}

impl Queue {

    pub fn new(session: &Rc<Session>,family: usize,index: usize) -> Option<Queue> {
        let mut queue = MaybeUninit::uninit();
        unsafe { vkGetDeviceQueue(session.device,family as u32,index as u32,queue.as_mut_ptr()) };
        let queue = unsafe { queue.assume_init() };

        Some(Queue {
            session: Rc::clone(&session),
            queue: queue,
        })
    }
}
