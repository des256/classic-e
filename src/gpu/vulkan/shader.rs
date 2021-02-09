// E - GPU (Vulkan) - Shader
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

pub struct Shader {
    pub session: Rc<Session>,
    pub(crate) vk_shader_module: VkShaderModule,
}

impl Shader {

    pub fn new(session: &Rc<Session>,code: &[u8]) -> Option<Rc<Shader>> {

        let create_info = VkShaderModuleCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            codeSize: code.len() as u64,
            pCode: code.as_ptr() as *const u32,
        };

        let mut vk_shader_module = MaybeUninit::uninit();
        match unsafe { vkCreateShaderModule(session.vk_device,&create_info,null_mut(),vk_shader_module.as_mut_ptr()) } {
            VkResult_VK_SUCCESS => { },
            code => {
#[cfg(feature="debug_output")]
                println!("Unable to create Vulkan shader module (error {})",code);
                return None;
            },
        }
        let vk_shader_module = unsafe { vk_shader_module.assume_init() };

        Some(Rc::new(Shader {
            session: Rc::clone(session),
            vk_shader_module: vk_shader_module,
        }))
    }
}

impl Drop for Shader {

    fn drop(&mut self) {
        unsafe { vkDestroyShaderModule(self.session.vk_device,self.vk_shader_module,null_mut()) };
    }
}
