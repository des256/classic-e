// E - GPU (Vulkan) - Pipeline
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

pub struct GraphicsPipeline {
    session: Rc<Session>,
    vk_pipeline_layout: VkPipelineLayout,
    vk_render_pass: VkRenderPass,
    vk_graphics_pipeline: VkPipeline,
}

impl GraphicsPipeline {

    pub fn new(session: &Rc<Session>,vertex_shader: &Rc<Shader>,fragment_shader: &Rc<Shader>) -> Option<Rc<GraphicsPipeline>> {

        let create_info = VkPipelineLayoutCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            setLayoutCount: 0,
            pSetLayouts: null_mut(),
            pushConstantRangeCount: 0,
            pPushConstantRanges: null_mut(),
        };
        let mut vk_pipeline_layout = MaybeUninit::uninit();
        match unsafe { vkCreatePipelineLayout(session.vk_device,&create_info,null_mut(),vk_pipeline_layout.as_mut_ptr()) } {
            VkResult_VK_SUCCESS => { },
            code => {
#[cfg(feature="debug_output")]
                println!("Unable to create Vulkan pipeline layout (error {})",code);
                return None;
            },
        }
        let vk_pipeline_layout = unsafe { vk_pipeline_layout.assume_init() };

        let attachment = VkAttachmentDescription {
            flags: 0,
            format: VkFormat_VK_FORMAT_B8G8R8A8_SRGB,
            samples: VkSampleCountFlagBits_VK_SAMPLE_COUNT_1_BIT,
            loadOp: VkAttachmentLoadOp_VK_ATTACHMENT_LOAD_OP_LOAD,
            storeOp: VkAttachmentStoreOp_VK_ATTACHMENT_STORE_OP_STORE,
            stencilLoadOp: VkAttachmentLoadOp_VK_ATTACHMENT_LOAD_OP_LOAD,
            stencilStoreOp: VkAttachmentStoreOp_VK_ATTACHMENT_STORE_OP_STORE,
            initialLayout: VkImageLayout_VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
            finalLayout: VkImageLayout_VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
        };
        let color_attachment_ref = VkAttachmentReference {
            attachment: 0,
            layout: VkImageLayout_VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
        };
        let subpass = VkSubpassDescription {
            flags: 0,
            pipelineBindPoint: VkPipelineBindPoint_VK_PIPELINE_BIND_POINT_GRAPHICS,
            inputAttachmentCount: 0,
            pInputAttachments: null_mut(),
            colorAttachmentCount: 1,
            pColorAttachments: &color_attachment_ref,
            pResolveAttachments: null_mut(),
            pDepthStencilAttachment: null_mut(),
            preserveAttachmentCount: 0,
            pPreserveAttachments: null_mut(),
        };
        let create_info = VkRenderPassCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            attachmentCount: 1,
            pAttachments: &attachment,
            subpassCount: 1,
            pSubpasses: &subpass,
            dependencyCount: 0,
            pDependencies: null_mut(),
        };
        let mut vk_render_pass = MaybeUninit::uninit();
        match unsafe { vkCreateRenderPass(session.vk_device,&create_info,null_mut(),vk_render_pass.as_mut_ptr()) } {
            VkResult_VK_SUCCESS => { },
            code => {
#[cfg(feature="debug_output")]
                println!("Unable to create Vulkan render pass (error {})",code);
                unsafe { vkDestroyPipelineLayout(session.vk_device,vk_pipeline_layout,null_mut()) };
                return None;
            }
        }
        let vk_render_pass = unsafe { vk_render_pass.assume_init() };

        let vertex_stage = VkPipelineShaderStageCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            stage: VkShaderStageFlagBits_VK_SHADER_STAGE_VERTEX_BIT,
            module: vertex_shader.vk_shader_module,
            pName: b"main".as_ptr() as *const i8,
            pSpecializationInfo: null_mut(),
        };

        let fragment_stage = VkPipelineShaderStageCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            stage: VkShaderStageFlagBits_VK_SHADER_STAGE_FRAGMENT_BIT,
            module: fragment_shader.vk_shader_module,
            pName: b"main".as_ptr() as *const i8,
            pSpecializationInfo: null_mut(),
        };

        let stages = vec![vertex_stage,fragment_stage];

        // array of:
        /*let binding = VkVertexInputBindingDescription {
            binding: 0,
            stride: 0,
            inputRate: VkVertexInputRate_VK_VERTEX_INPUT_RATE_VERTEX,
        };*/

        let vertex_input = VkPipelineVertexInputStateCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            vertexBindingDescriptionCount: 0,
            pVertexBindingDescriptions: null_mut(),
            vertexAttributeDescriptionCount: 0,
            pVertexAttributeDescriptions: null_mut(),
        };

        let input_assembly = VkPipelineInputAssemblyStateCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            topology: VkPrimitiveTopology_VK_PRIMITIVE_TOPOLOGY_POINT_LIST,
            primitiveRestartEnable: VK_FALSE,
        };

        let tesselation_state = VkPipelineTessellationStateCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            patchControlPoints: 1,
        };

        let viewport = VkViewport {
            x: 0.0,
            y: 0.0,
            width: 640.0,
            height: 480.0,
            minDepth: 0.0,
            maxDepth: 1.0,
        };

        let scissor = VkRect2D {
            offset: VkOffset2D {
                x: 0,
                y: 0,
            },
            extent: VkExtent2D {
                width: 640,
                height: 480,
            },
        };

        let viewport_state = VkPipelineViewportStateCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            viewportCount: 1,
            pViewports: &viewport,
            scissorCount: 1,
            pScissors: &scissor,
        };

        let rasterization_state = VkPipelineRasterizationStateCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            depthClampEnable: VK_TRUE,
            rasterizerDiscardEnable: VK_FALSE,
            polygonMode: VkPolygonMode_VK_POLYGON_MODE_FILL,
            cullMode: VkCullModeFlagBits_VK_CULL_MODE_FRONT_BIT,
            frontFace: VkFrontFace_VK_FRONT_FACE_CLOCKWISE,
            depthBiasEnable: VK_FALSE,
            depthBiasConstantFactor: 0.0,
            depthBiasClamp: 0.0,
            depthBiasSlopeFactor: 0.0,
            lineWidth: 1.0,
        };

        let multisample_state = VkPipelineMultisampleStateCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            rasterizationSamples: VkSampleCountFlagBits_VK_SAMPLE_COUNT_1_BIT,
            sampleShadingEnable: VK_FALSE,
            minSampleShading: 0.0,
            pSampleMask: null_mut(),
            alphaToCoverageEnable: VK_FALSE,
            alphaToOneEnable: VK_FALSE,
        };

        let depth_stencil_state = VkPipelineDepthStencilStateCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            depthTestEnable: VK_FALSE,
            depthWriteEnable: VK_FALSE,
            depthCompareOp: VkCompareOp_VK_COMPARE_OP_NEVER,
            depthBoundsTestEnable: VK_FALSE,
            stencilTestEnable: VK_FALSE,
            front: VkStencilOpState {
                failOp: VkStencilOp_VK_STENCIL_OP_KEEP,
                passOp: VkStencilOp_VK_STENCIL_OP_KEEP,
                depthFailOp: VkStencilOp_VK_STENCIL_OP_KEEP,
                compareOp: VkCompareOp_VK_COMPARE_OP_NEVER,
                compareMask: 0,
                writeMask: 0,
                reference: 0,
            },
            back: VkStencilOpState {
                failOp: VkStencilOp_VK_STENCIL_OP_KEEP,
                passOp: VkStencilOp_VK_STENCIL_OP_KEEP,
                depthFailOp: VkStencilOp_VK_STENCIL_OP_KEEP,
                compareOp: VkCompareOp_VK_COMPARE_OP_NEVER,
                compareMask: 0,
                writeMask: 0,
                reference: 0,
            },
            minDepthBounds: 0.0,
            maxDepthBounds: 1.0,
        };

        let color_blend_state = VkPipelineColorBlendStateCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            logicOpEnable: VK_FALSE,
            logicOp: VkLogicOp_VK_LOGIC_OP_SET,
            attachmentCount: 0,
            pAttachments: null_mut(),
            blendConstants: [0.0,0.0,0.0,0.0],
        };

        let dynamic_state = VkPipelineDynamicStateCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            dynamicStateCount: 0,
            pDynamicStates: null_mut(),
        };

        let create_info = VkGraphicsPipelineCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            stageCount: 2,
            pStages: stages.as_ptr(),
            pVertexInputState: &vertex_input,
            pInputAssemblyState: &input_assembly,
            pTessellationState: &tesselation_state,
            pViewportState: &viewport_state,
            pRasterizationState: &rasterization_state,
            pMultisampleState: &multisample_state,
            pDepthStencilState: &depth_stencil_state,
            pColorBlendState: &color_blend_state,
            pDynamicState: &dynamic_state,
            layout: vk_pipeline_layout,
            renderPass: vk_render_pass,
            subpass: 0,
            basePipelineHandle: null_mut(),
            basePipelineIndex: -1,
        };

        let mut vk_graphics_pipeline = MaybeUninit::uninit();
        match unsafe { vkCreateGraphicsPipelines(session.vk_device,null_mut(),1,&create_info,null_mut(),vk_graphics_pipeline.as_mut_ptr()) } {
            VkResult_VK_SUCCESS => { },
            code => {
#[cfg(feature="debug_output")]
                println!("Unable to create Vulkan graphics pipeline (error {})",code);
                unsafe { vkDestroyRenderPass(session.vk_device,vk_render_pass,null_mut()) };
                unsafe { vkDestroyPipelineLayout(session.vk_device,vk_pipeline_layout,null_mut()) };
                return None;
            },
        }
        let vk_graphics_pipeline = unsafe { vk_graphics_pipeline.assume_init() };

        Some(Rc::new(GraphicsPipeline {
            session: Rc::clone(session),
            vk_pipeline_layout: vk_pipeline_layout,
            vk_render_pass: vk_render_pass,
            vk_graphics_pipeline: vk_graphics_pipeline,
        }))
    }
}

impl Drop for GraphicsPipeline {

    fn drop(&mut self) {
        unsafe { vkDestroyPipeline(self.session.vk_device,self.vk_graphics_pipeline,null_mut()) };
        unsafe { vkDestroyRenderPass(self.session.vk_device,self.vk_render_pass,null_mut()) };
        unsafe { vkDestroyPipelineLayout(self.session.vk_device,self.vk_pipeline_layout,null_mut()) };
    }
}
