// E - GPU (Vulkan) - RenderPass
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

pub struct RenderPass {
    pub session: Rc<Session>,
    pub(crate) vk_render_pass: VkRenderPass,
}

pub enum LoadOp {
    Load,
    Clear,
    DontCare,
}

pub enum StoreOp {
    Store,
    DontCare,
}

pub enum ImageLayout {
    Undefined,
    General,
    Color,
    Depth,
    DepthRead,
    Stencil,
    StencilRead,
    DepthStencil,
    DepthReadStencil,
    DepthStencilRead,
    DepthReadStencilRead,
    ShaderRead,
    TransferSrc,
    TransferDst,
    Preinitialized,
    Present,
}

pub struct AttachmentDescription<'a> {
    pub image: &'a Image,
    pub samples: u32,
    pub load_op: LoadOp,
    pub store_op: StoreOp,
    pub stencil_load_op: LoadOp,
    pub stencil_store_op: StoreOp,
    pub initial_layout: ImageLayout,
    pub final_layout: ImageLayout,
}

pub enum PipelineBindPoint {
    Graphics,
    Compute,
}

pub struct AttachmentReference {
    index: usize,
    layout: ImageLayout,
}

pub struct SubpassDescription {
    bind_point: PipelineBindPoint,
    input_attachments: Vec<AttachmentReference>,
    color_attachments: Vec<AttachmentReference>,
    resolve_attachments: Vec<AttachmentReference>,
    depth_stencil_attachments: Vec<AttachmentReference>,
    preserve_attachments: Vec<AttachmentReference>,
}

pub enum PipelineStage {
    TopOfPipe,
    DrawIndirect,
    VertexInput,
    VertexShader,
    TesselationControlShader,
    TesselationEvaluationShader,
    GeometryShader,
    FragmentShader,
    EarlyFragmentTests,
    LateFragmentTests,
    ColorAttachmentOutput,
    ComputeShader,
    Transfer,
    BottomOfPipe,
    Host,
    AllGraphics,
    AllCommands,
}

impl Session {

    pub fn create_render_pass<'a>(self: &Rc<Self>,attachment_description: AttachmentDescription<'a>) -> Option<Rc<RenderPass>> {

        let info = VkRenderPassCreateInfo {
            sType: VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            attachmentCount: 1,
            pAttachments: &VkAttachmentDescription {
                flags: 0,
                format: attachment_description.image.vk_format,
                samples: attachment_description.samples,
                loadOp: match attachment_description.load_op {
                    LoadOp::Load => VK_ATTACHMENT_LOAD_OP_LOAD,
                    LoadOp::Clear => VK_ATTACHMENT_LOAD_OP_CLEAR,
                    LoadOp::DontCare => VK_ATTACHMENT_LOAD_OP_DONT_CARE,
                },
                storeOp: match attachment_description.store_op {
                    StoreOp::Store => VK_ATTACHMENT_STORE_OP_STORE,
                    StoreOp::DontCare => VK_ATTACHMENT_STORE_OP_DONT_CARE,
                },
                stencilLoadOp: match attachment_description.stencil_load_op {
                    LoadOp::Load => VK_ATTACHMENT_LOAD_OP_LOAD,
                    LoadOp::Clear => VK_ATTACHMENT_LOAD_OP_CLEAR,
                    LoadOp::DontCare => VK_ATTACHMENT_LOAD_OP_DONT_CARE,
                },
                stencilStoreOp: match attachment_description.stencil_store_op {
                    StoreOp::Store => VK_ATTACHMENT_STORE_OP_STORE,
                    StoreOp::DontCare => VK_ATTACHMENT_STORE_OP_DONT_CARE,
                },
                initialLayout: match attachment_description.initial_layout {
                    ImageLayout::Undefined => VK_IMAGE_LAYOUT_UNDEFINED,
                    ImageLayout::General => VK_IMAGE_LAYOUT_GENERAL,
                    ImageLayout::Color => VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
                    ImageLayout::Depth => VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL,
                    ImageLayout::DepthRead => VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL,
                    ImageLayout::Stencil => VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL,
                    ImageLayout::StencilRead => VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL,
                    ImageLayout::DepthStencil => VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                    ImageLayout::DepthReadStencil => VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL,
                    ImageLayout::DepthStencilRead => VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL,
                    ImageLayout::DepthReadStencilRead => VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL,
                    ImageLayout::ShaderRead => VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
                    ImageLayout::TransferSrc => VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                    ImageLayout::TransferDst => VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                    ImageLayout::Preinitialized => VK_IMAGE_LAYOUT_PREINITIALIZED,
                    ImageLayout::Present => VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
                },
                finalLayout: match attachment_description.final_layout {
                    ImageLayout::Undefined => VK_IMAGE_LAYOUT_UNDEFINED,
                    ImageLayout::General => VK_IMAGE_LAYOUT_GENERAL,
                    ImageLayout::Color => VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
                    ImageLayout::Depth => VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL,
                    ImageLayout::DepthRead => VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL,
                    ImageLayout::Stencil => VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL,
                    ImageLayout::StencilRead => VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL,
                    ImageLayout::DepthStencil => VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                    ImageLayout::DepthReadStencil => VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL,
                    ImageLayout::DepthStencilRead => VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL,
                    ImageLayout::DepthReadStencilRead => VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL,
                    ImageLayout::ShaderRead => VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
                    ImageLayout::TransferSrc => VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                    ImageLayout::TransferDst => VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                    ImageLayout::Preinitialized => VK_IMAGE_LAYOUT_PREINITIALIZED,
                    ImageLayout::Present => VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
                },
            },
            subpassCount: 1,
            pSubpasses: &VkSubpassDescription {
                flags: 0,
                pipelineBindPoint: VK_PIPELINE_BIND_POINT_GRAPHICS,
                inputAttachmentCount: 0,
                pInputAttachments: null_mut(),
                colorAttachmentCount: 1,
                pColorAttachments: &VkAttachmentReference {
                    attachment: 0,
                    layout: VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
                },
                pResolveAttachments: null_mut(),
                pDepthStencilAttachment: null_mut(),
                preserveAttachmentCount: 0,
                pPreserveAttachments: null_mut(),
            },
            dependencyCount: 1,
            pDependencies: &VkSubpassDependency {
                srcSubpass: VK_SUBPASS_EXTERNAL as u32,
                dstSubpass: 0,
                srcStageMask: VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
                dstStageMask: VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
                srcAccessMask: 0,
                dstAccessMask: VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
                dependencyFlags: 0,
            },
        };
        let mut vk_render_pass = MaybeUninit::uninit();
        match unsafe { vkCreateRenderPass(self.vk_device,&info,null_mut(),vk_render_pass.as_mut_ptr()) } {
            VK_SUCCESS => { },
            code => {
#[cfg(feature="debug_output")]
                println!("Unable to create Vulkan render pass (error {})",code);
                return None;
            }
        }

        Some(Rc::new(RenderPass {
            session: Rc::clone(self),
            vk_render_pass: unsafe { vk_render_pass.assume_init() },
        }))
    }
}

impl Drop for RenderPass {

    fn drop(&mut self) {
        unsafe { vkDestroyRenderPass(self.session.vk_device,self.vk_render_pass,null_mut()) };
    }
}
