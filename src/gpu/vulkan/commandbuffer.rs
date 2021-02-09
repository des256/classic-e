// E - GPU (Vulkan) - CommandBuffer
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

pub struct CommandBuffer {
    pub session: Rc<Session>,
    pub queue_index: usize,
    pub(crate) vk_command_buffer: VkCommandBuffer,
}

impl CommandBuffer {
    pub(crate) fn new(session: &Rc<Session>,queue_index: usize,vk_command_buffer: VkCommandBuffer) -> Option<CommandBuffer> {
        Some(CommandBuffer {
            session: Rc::clone(session),
            queue_index: queue_index,
            vk_command_buffer: vk_command_buffer,
        })
    }

    pub fn begin(&self) -> bool {
        let info = VkCommandBufferBeginInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
            pNext: null_mut(),
            flags: 0,
            pInheritanceInfo: null_mut(),
        };
        match unsafe { vkBeginCommandBuffer(self.vk_command_buffer,&info) } {
            VkResult_VK_SUCCESS => { },
            code => {
#[cfg(feature="debug_output")]
                println!("Unable to begin Vulkan command buffer (error {})",code);
                return false;
            },
        }
        true
    }

    pub fn end(&self) -> bool {
        match unsafe { vkEndCommandBuffer(self.vk_command_buffer) } {
            VkResult_VK_SUCCESS => { },
            code => {
#[cfg(feature="debug_output")]
                println!("Unable to end Vulkan command buffer (error {})",code);
                return false;
            },
        }
        true
    }

    pub fn begin_render_pass(&self,render_pass: usize,swap_chain: &SwapChain,framebuffer_index: usize) {
        let clear_color = VkClearValue {
            color: VkClearColorValue {
                float32: [0.0,0.0,0.0,1.0]
            }
        };
        let info = VkRenderPassBeginInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
            pNext: null_mut(),
            renderPass: swap_chain.graphics_pipeline.vk_render_pass,
            framebuffer: swap_chain.vk_framebuffers[framebuffer_index],
            renderArea: VkRect2D { offset: VkOffset2D { x: 0,y: 0 },extent: VkExtent2D { width: swap_chain.extent.x as u32,height: swap_chain.extent.y as u32 } },
            clearValueCount: 1,
            pClearValues: &clear_color,
        };
        unsafe { vkCmdBeginRenderPass(self.vk_command_buffer,&info,VkSubpassContents_VK_SUBPASS_CONTENTS_INLINE) }
    }

    pub fn end_render_pass(&self) {
        unsafe { vkCmdEndRenderPass(self.vk_command_buffer) };
    }

    pub fn bind_pipeline(&self,pipeline: &GraphicsPipeline) {
        unsafe { vkCmdBindPipeline(self.vk_command_buffer,VkPipelineBindPoint_VK_PIPELINE_BIND_POINT_GRAPHICS,pipeline.vk_graphics_pipeline) };
    }

    pub fn draw(&self,vertex_count: usize,instance_count: usize,first_vertex: usize, first_instance: usize) {
        unsafe { vkCmdDraw(self.vk_command_buffer,vertex_count as u32,instance_count as u32,first_vertex as u32,first_instance as u32) };
    }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        unsafe { vkFreeCommandBuffers(self.session.vk_device,self.session.vk_command_pools[self.queue_index],1,&self.vk_command_buffer) };
    }
}
