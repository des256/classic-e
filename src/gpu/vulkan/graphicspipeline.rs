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
    pub session: Rc<Session>,
    pub(crate) vk_pipeline_layout: VkPipelineLayout,
    pub(crate) vk_render_pass: VkRenderPass,
    pub(crate) vk_graphics_pipeline: VkPipeline,
}

impl Session {

    pub fn create_graphics_pipeline(self: &Rc<Self>,vertex_shader: &Rc<Shader>,fragment_shader: &Rc<Shader>) -> Option<Rc<GraphicsPipeline>> {

        // create pipeline layout
        let vk_pipeline_layout = {
            let info = VkPipelineLayoutCreateInfo {
                sType: VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
                pNext: null_mut(),
                flags: 0,
                setLayoutCount: 0,
                pSetLayouts: null_mut(),
                pushConstantRangeCount: 0,
                pPushConstantRanges: null_mut(),
            };
            let mut vk_pipeline_layout = MaybeUninit::uninit();
            match unsafe { vkCreatePipelineLayout(self.vk_device,&info,null_mut(),vk_pipeline_layout.as_mut_ptr()) } {
                VK_SUCCESS => { },
                code => {
#[cfg(feature="debug_output")]
                    println!("Unable to create Vulkan pipeline layout (error {})",code);
                    return None;
                },
            }
            unsafe { vk_pipeline_layout.assume_init() }
        };

        // create render pass
        let vk_render_pass = {
            let info = VkRenderPassCreateInfo {
                sType: VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
                pNext: null_mut(),
                flags: 0,
                attachmentCount: 1,
                pAttachments: &VkAttachmentDescription {
                    flags: 0,
                    format: VK_FORMAT_B8G8R8A8_SRGB,
                    samples: VK_SAMPLE_COUNT_1_BIT,
                    loadOp: VK_ATTACHMENT_LOAD_OP_CLEAR,
                    storeOp: VK_ATTACHMENT_STORE_OP_STORE,
                    stencilLoadOp: VK_ATTACHMENT_LOAD_OP_DONT_CARE,
                    stencilStoreOp: VK_ATTACHMENT_STORE_OP_DONT_CARE,
                    initialLayout: VK_IMAGE_LAYOUT_UNDEFINED,
                    finalLayout: VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
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
                    unsafe { vkDestroyPipelineLayout(self.vk_device,vk_pipeline_layout,null_mut()) };
                    return None;
                }
            }
            unsafe { vk_render_pass.assume_init() }
        };

        // create pipeline
        let vk_graphics_pipeline = {

            let create_info = VkGraphicsPipelineCreateInfo {
                sType: VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
                pNext: null_mut(),
                flags: 0,
                stageCount: 2,
                pStages: [
                    VkPipelineShaderStageCreateInfo {
                        sType: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                        pNext: null_mut(),
                        flags: 0,
                        stage: VK_SHADER_STAGE_VERTEX_BIT,
                        module: vertex_shader.vk_shader_module,
                        pName: b"main\0".as_ptr() as *const i8,
                        pSpecializationInfo: null_mut(),
                    },
                    VkPipelineShaderStageCreateInfo {
                        sType: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                        pNext: null_mut(),
                        flags: 0,
                        stage: VK_SHADER_STAGE_FRAGMENT_BIT,
                        module: fragment_shader.vk_shader_module,
                        pName: b"main\0".as_ptr() as *const i8,
                        pSpecializationInfo: null_mut(),
                    }
                ].as_ptr(),
                pVertexInputState: &VkPipelineVertexInputStateCreateInfo {
                    sType: VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
                    pNext: null_mut(),
                    flags: 0,
                    vertexBindingDescriptionCount: 0,
                    pVertexBindingDescriptions: null_mut(),
                    vertexAttributeDescriptionCount: 0,
                    pVertexAttributeDescriptions: null_mut(),
                },
                pInputAssemblyState: &VkPipelineInputAssemblyStateCreateInfo {
                    sType: VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
                    pNext: null_mut(),
                    flags: 0,
                    topology: VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
                    primitiveRestartEnable: VK_FALSE,
                },
                pTessellationState: &VkPipelineTessellationStateCreateInfo {
                    sType: VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO,
                    pNext: null_mut(),
                    flags: 0,
                    patchControlPoints: 1,
                },
                pViewportState: &VkPipelineViewportStateCreateInfo {
                    sType: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
                    pNext: null_mut(),
                    flags: 0,
                    viewportCount: 1,
                    pViewports: &VkViewport {
                        x: 0.0,
                        y: 0.0,
                        width: 640.0,
                        height: 480.0,
                        minDepth: 0.0,
                        maxDepth: 1.0,
                    },
                    scissorCount: 1,
                    pScissors: &VkRect2D {
                        offset: VkOffset2D {
                            x: 0,
                            y: 0,
                        },
                        extent: VkExtent2D {
                            width: 640,
                            height: 480,
                        },
                    },
                },
                pRasterizationState: &VkPipelineRasterizationStateCreateInfo {
                    sType: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
                    pNext: null_mut(),
                    flags: 0,
                    depthClampEnable: VK_FALSE,
                    rasterizerDiscardEnable: VK_FALSE,
                    polygonMode: VK_POLYGON_MODE_FILL,
                    cullMode: VK_CULL_MODE_BACK_BIT,
                    frontFace: VK_FRONT_FACE_CLOCKWISE,
                    depthBiasEnable: VK_FALSE,
                    depthBiasConstantFactor: 0.0,
                    depthBiasClamp: 0.0,
                    depthBiasSlopeFactor: 0.0,
                    lineWidth: 1.0,
                },
                pMultisampleState: &VkPipelineMultisampleStateCreateInfo {
                    sType: VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
                    pNext: null_mut(),
                    flags: 0,
                    rasterizationSamples: VK_SAMPLE_COUNT_1_BIT,
                    sampleShadingEnable: VK_FALSE,
                    minSampleShading: 1.0,
                    pSampleMask: null_mut(),
                    alphaToCoverageEnable: VK_FALSE,
                    alphaToOneEnable: VK_FALSE,
                },
                pDepthStencilState: null_mut(),
                pColorBlendState: &VkPipelineColorBlendStateCreateInfo {
                    sType: VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
                    pNext: null_mut(),
                    flags: 0,
                    logicOpEnable: VK_FALSE,
                    logicOp: VK_LOGIC_OP_COPY,
                    attachmentCount: 1,
                    pAttachments: &VkPipelineColorBlendAttachmentState {
                        blendEnable: VK_FALSE,
                        srcColorBlendFactor: VK_BLEND_FACTOR_ONE,
                        dstColorBlendFactor: VK_BLEND_FACTOR_ZERO,
                        colorBlendOp: VK_BLEND_OP_ADD,
                        srcAlphaBlendFactor: VK_BLEND_FACTOR_ONE,
                        dstAlphaBlendFactor: VK_BLEND_FACTOR_ZERO,
                        alphaBlendOp: VK_BLEND_OP_ADD,
                        colorWriteMask: VK_COLOR_COMPONENT_R_BIT |
                            VK_COLOR_COMPONENT_G_BIT |
                            VK_COLOR_COMPONENT_B_BIT |
                            VK_COLOR_COMPONENT_A_BIT,
                    },
                    blendConstants: [0.0,0.0,0.0,0.0],
                },
                pDynamicState: null_mut(),
                layout: vk_pipeline_layout,
                renderPass: vk_render_pass,
                subpass: 0,
                basePipelineHandle: null_mut(),
                basePipelineIndex: -1,
            };
            let mut vk_graphics_pipeline = MaybeUninit::uninit();
            match unsafe { vkCreateGraphicsPipelines(self.vk_device,null_mut(),1,&create_info,null_mut(),vk_graphics_pipeline.as_mut_ptr()) } {
                VK_SUCCESS => { },
                code => {
#[cfg(feature="debug_output")]
                    println!("Unable to create Vulkan graphics pipeline (error {})",code);
                    unsafe { vkDestroyRenderPass(self.vk_device,vk_render_pass,null_mut()) };
                    unsafe { vkDestroyPipelineLayout(self.vk_device,vk_pipeline_layout,null_mut()) };
                    return None;
                },
            }
            unsafe { vk_graphics_pipeline.assume_init() }
        };

        Some(Rc::new(GraphicsPipeline {
            session: Rc::clone(self),
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
