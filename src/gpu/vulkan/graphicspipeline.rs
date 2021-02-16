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
    pub pipeline_layout: Rc<PipelineLayout>,
    pub render_pass: Rc<RenderPass>,
    pub vertex_shader: Rc<Shader>,
    pub fragment_shader: Rc<Shader>,
#[doc(hidden)]
    pub(crate) vk_graphics_pipeline: VkPipeline,
}

pub enum Topology {
    Points,
    Lines,
    LineStrip,
    Triangles,
    TriangleStrip,
    TriangleFan,
    LinesAdjacency,
    LineStripAdjacency,
    TrianglesAdjacency,
    TriangleStripAdjacency,
    Patches,
}

pub enum PrimitiveRestart {
    Disabled,
    Enabled,
}

pub enum DepthClamp {
    Disabled,
    Enabled,
}

pub enum Discard {
    Disabled,
    Enabled,
}

pub enum PolygonMode {
    Fill,
    Line,
    Point,
}

pub enum CullMode {
    None,
    Front,
    Back,
    FrontAndBack,
}

pub enum FrontFace {
    CounterClockwise,
    Clockwise,
}

pub enum DepthBias {
    Disabled,
    Enabled { constant: f32,clamp: f32,slope: f32 },
}

pub enum SampleShading {
    Disabled,
    Enabled { min: f32 },
}

pub enum AlphaToCoverage {
    Disabled,
    Enabled,
}

pub enum AlphaToOne {
    Disabled,
    Enabled,
}

pub enum LogicOp {
    Disabled,
    Clear,
    And,
    AndReverse,
    Copy,
    AndInverted,
    NoOp,
    Xor,
    Or,
    Nor,
    Equivalent,
    Invert,
    OrReverse,
    CopyInverted,
    OrInverted,
    Nand,
    Set
}

pub enum ColorBlendFactor {
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    DstColor,
    OneMinusDstColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstAlpha,
    OneMinusDstAlpha,
    ConstColor,
    OneMinusConstColor,
    ConstAlpha,
    OneMinusConstAlpha,
    SrcAlphaSaturate,
}

pub enum AlphaBlendFactor {
    Zero,
    One,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstAlpha,
    OneMinusDstAlpha,
    ConstAlpha,
    OneMinusConstAlpha,
    SrcAlphaSaturate,
}

pub enum BlendOp {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
}

pub enum Blend {
    Disabled,
    Enabled {
        color_op: BlendOp,
        src_color: ColorBlendFactor,
        dst_color: ColorBlendFactor,
        alpha_op: BlendOp,
        src_alpha: AlphaBlendFactor,
        dst_alpha: AlphaBlendFactor,
    }
}

impl Session {

    pub fn create_graphics_pipeline(
        self: &Rc<Self>,
        pipeline_layout: &Rc<PipelineLayout>,
        render_pass: &Rc<RenderPass>,
        vertex_shader: &Rc<Shader>,
        fragment_shader: &Rc<Shader>,
        // TODO: vertex bindings
        topology: Topology,
        primitive_restart: PrimitiveRestart,
        // TODO: tesselation state
        viewport: Rect<f32>,
        depth_range: (f32,f32),
        scissor: Rect<i32>,
        depth_clamp: DepthClamp,
        discard: Discard,
        polygon_mode: PolygonMode,
        cull_mode: CullMode,
        front_face: FrontFace,
        depth_bias: DepthBias,
        line_width: f32,
        rasterization_samples: u32,
        sample_shading: SampleShading,
        // TODO: sample mask
        alpha_to_coverage: AlphaToCoverage,
        alpha_to_one: AlphaToOne,
        // TODO: depth/stencil
        logic_op: LogicOp,
        blend: Blend,  // TODO: multiple attachments
        write_mask: (bool,bool,bool,bool),  // TODO: multiple attachments

    ) -> Option<Rc<GraphicsPipeline>> {

        let depth_bias = match depth_bias {
            DepthBias::Disabled => (VK_FALSE,0.0f32,0.0f32,0.0f32),
            DepthBias::Enabled { constant,clamp,slope } => (VK_TRUE,constant,clamp,slope),
        };

        let sample_shading = match sample_shading {
            SampleShading::Disabled => (VK_FALSE,0.0f32),
            SampleShading::Enabled { min } => (VK_TRUE,min),
        };

        let blend = match blend {
            Blend::Disabled => (VK_FALSE,VK_BLEND_FACTOR_ONE,VK_BLEND_FACTOR_ZERO,VK_BLEND_OP_ADD,VK_BLEND_FACTOR_ONE,VK_BLEND_FACTOR_ZERO,VK_BLEND_OP_ADD),
            Blend::Enabled { color_op,src_color,dst_color,alpha_op,src_alpha,dst_alpha } => {
                let color_op = match color_op {
                    BlendOp::Add => VK_BLEND_OP_ADD,
                    BlendOp::Subtract => VK_BLEND_OP_SUBTRACT,
                    BlendOp::ReverseSubtract => VK_BLEND_OP_REVERSE_SUBTRACT,
                    BlendOp::Min => VK_BLEND_OP_MIN,
                    BlendOp::Max => VK_BLEND_OP_MAX,
                };
                let alpha_op = match alpha_op {
                    BlendOp::Add => VK_BLEND_OP_ADD,
                    BlendOp::Subtract => VK_BLEND_OP_SUBTRACT,
                    BlendOp::ReverseSubtract => VK_BLEND_OP_REVERSE_SUBTRACT,
                    BlendOp::Min => VK_BLEND_OP_MIN,
                    BlendOp::Max => VK_BLEND_OP_MAX,
                };
                let src_color = match src_color {
                    ColorBlendFactor::Zero => VK_BLEND_FACTOR_ZERO,
                    ColorBlendFactor::One => VK_BLEND_FACTOR_ONE,
                    ColorBlendFactor::SrcColor => VK_BLEND_FACTOR_SRC_COLOR,
                    ColorBlendFactor::OneMinusSrcColor => VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR,
                    ColorBlendFactor::DstColor => VK_BLEND_FACTOR_DST_COLOR,
                    ColorBlendFactor::OneMinusDstColor => VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR,
                    ColorBlendFactor::SrcAlpha => VK_BLEND_FACTOR_SRC_ALPHA,
                    ColorBlendFactor::OneMinusSrcAlpha => VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
                    ColorBlendFactor::DstAlpha => VK_BLEND_FACTOR_DST_ALPHA,
                    ColorBlendFactor::OneMinusDstAlpha => VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA,
                    ColorBlendFactor::ConstColor => VK_BLEND_FACTOR_CONSTANT_COLOR,
                    ColorBlendFactor::OneMinusConstColor => VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR,
                    ColorBlendFactor::ConstAlpha => VK_BLEND_FACTOR_CONSTANT_ALPHA,
                    ColorBlendFactor::OneMinusConstAlpha => VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA,
                    ColorBlendFactor::SrcAlphaSaturate => VK_BLEND_FACTOR_SRC_ALPHA_SATURATE,
                };
                let dst_color = match dst_color {
                    ColorBlendFactor::Zero => VK_BLEND_FACTOR_ZERO,
                    ColorBlendFactor::One => VK_BLEND_FACTOR_ONE,
                    ColorBlendFactor::SrcColor => VK_BLEND_FACTOR_SRC_COLOR,
                    ColorBlendFactor::OneMinusSrcColor => VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR,
                    ColorBlendFactor::DstColor => VK_BLEND_FACTOR_DST_COLOR,
                    ColorBlendFactor::OneMinusDstColor => VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR,
                    ColorBlendFactor::SrcAlpha => VK_BLEND_FACTOR_SRC_ALPHA,
                    ColorBlendFactor::OneMinusSrcAlpha => VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
                    ColorBlendFactor::DstAlpha => VK_BLEND_FACTOR_DST_ALPHA,
                    ColorBlendFactor::OneMinusDstAlpha => VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA,
                    ColorBlendFactor::ConstColor => VK_BLEND_FACTOR_CONSTANT_COLOR,
                    ColorBlendFactor::OneMinusConstColor => VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR,
                    ColorBlendFactor::ConstAlpha => VK_BLEND_FACTOR_CONSTANT_ALPHA,
                    ColorBlendFactor::OneMinusConstAlpha => VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA,
                    ColorBlendFactor::SrcAlphaSaturate => VK_BLEND_FACTOR_SRC_ALPHA_SATURATE,
                };
                let src_alpha = match src_alpha {
                    AlphaBlendFactor::Zero => VK_BLEND_FACTOR_ZERO,
                    AlphaBlendFactor::One => VK_BLEND_FACTOR_ONE,
                    AlphaBlendFactor::SrcAlpha => VK_BLEND_FACTOR_SRC_ALPHA,
                    AlphaBlendFactor::OneMinusSrcAlpha => VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
                    AlphaBlendFactor::DstAlpha => VK_BLEND_FACTOR_DST_ALPHA,
                    AlphaBlendFactor::OneMinusDstAlpha => VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA,
                    AlphaBlendFactor::ConstAlpha => VK_BLEND_FACTOR_CONSTANT_ALPHA,
                    AlphaBlendFactor::OneMinusConstAlpha => VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA,
                    AlphaBlendFactor::SrcAlphaSaturate => VK_BLEND_FACTOR_SRC_ALPHA_SATURATE,
                };
                let dst_alpha = match dst_alpha {
                    AlphaBlendFactor::Zero => VK_BLEND_FACTOR_ZERO,
                    AlphaBlendFactor::One => VK_BLEND_FACTOR_ONE,
                    AlphaBlendFactor::SrcAlpha => VK_BLEND_FACTOR_SRC_ALPHA,
                    AlphaBlendFactor::OneMinusSrcAlpha => VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
                    AlphaBlendFactor::DstAlpha => VK_BLEND_FACTOR_DST_ALPHA,
                    AlphaBlendFactor::OneMinusDstAlpha => VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA,
                    AlphaBlendFactor::ConstAlpha => VK_BLEND_FACTOR_CONSTANT_ALPHA,
                    AlphaBlendFactor::OneMinusConstAlpha => VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA,
                    AlphaBlendFactor::SrcAlphaSaturate => VK_BLEND_FACTOR_SRC_ALPHA_SATURATE,
                };
                (VK_TRUE,src_color,dst_color,color_op,src_alpha,dst_alpha,alpha_op)
            }
        };

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
                topology: match topology {
                    Topology::Points => VK_PRIMITIVE_TOPOLOGY_POINT_LIST,
                    Topology::Lines => VK_PRIMITIVE_TOPOLOGY_LINE_LIST,
                    Topology::LineStrip => VK_PRIMITIVE_TOPOLOGY_LINE_STRIP,
                    Topology::Triangles => VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
                    Topology::TriangleStrip => VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP,
                    Topology::TriangleFan => VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN,
                    Topology::LinesAdjacency => VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY,
                    Topology::LineStripAdjacency => VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY,
                    Topology::TrianglesAdjacency => VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY,
                    Topology::TriangleStripAdjacency => VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY,
                    Topology::Patches => VK_PRIMITIVE_TOPOLOGY_PATCH_LIST,
                },
                primitiveRestartEnable: match primitive_restart {
                    PrimitiveRestart::Enabled => VK_TRUE,
                    PrimitiveRestart::Disabled => VK_FALSE,
                },
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
                    x: viewport.o.x,
                    y: viewport.o.y,
                    width: viewport.s.x,
                    height: viewport.s.y,
                    minDepth: depth_range.0,
                    maxDepth: depth_range.1,
                },
                scissorCount: 1,
                pScissors: &VkRect2D {
                    offset: VkOffset2D {
                        x: scissor.o.x,
                        y: scissor.o.y,
                    },
                    extent: VkExtent2D {
                        width: scissor.s.x as u32,
                        height: scissor.s.y as u32,
                    },
                },
            },
            pRasterizationState: &VkPipelineRasterizationStateCreateInfo {
                sType: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
                pNext: null_mut(),
                flags: 0,
                depthClampEnable: match depth_clamp {
                    DepthClamp::Enabled => VK_TRUE,
                    DepthClamp::Disabled => VK_FALSE,
                },
                rasterizerDiscardEnable: match discard {
                    Discard::Enabled => VK_TRUE,
                    Discard::Disabled => VK_FALSE,
                },
                polygonMode: match polygon_mode {
                    PolygonMode::Fill => VK_POLYGON_MODE_FILL,
                    PolygonMode::Line => VK_POLYGON_MODE_LINE,
                    PolygonMode::Point => VK_POLYGON_MODE_POINT,
                },
                cullMode: match cull_mode {
                    CullMode::None => VK_CULL_MODE_NONE,
                    CullMode::Front => VK_CULL_MODE_FRONT_BIT,
                    CullMode::Back => VK_CULL_MODE_BACK_BIT,
                    CullMode::FrontAndBack => VK_CULL_MODE_FRONT_AND_BACK,
                },
                frontFace: match front_face {
                    FrontFace::CounterClockwise => VK_FRONT_FACE_COUNTER_CLOCKWISE,
                    FrontFace::Clockwise => VK_FRONT_FACE_CLOCKWISE,
                },
                depthBiasEnable: depth_bias.0,
                depthBiasConstantFactor: depth_bias.1,
                depthBiasClamp: depth_bias.2,
                depthBiasSlopeFactor: depth_bias.3,
                lineWidth: line_width,
            },
            pMultisampleState: &VkPipelineMultisampleStateCreateInfo {
                sType: VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
                pNext: null_mut(),
                flags: 0,
                rasterizationSamples: rasterization_samples,
                sampleShadingEnable: sample_shading.0,
                minSampleShading: sample_shading.1,
                pSampleMask: null_mut(),
                alphaToCoverageEnable: match alpha_to_coverage {
                    AlphaToCoverage::Disabled => VK_FALSE,
                    AlphaToCoverage::Enabled => VK_TRUE,
                },
                alphaToOneEnable: match alpha_to_one {
                    AlphaToOne::Disabled => VK_FALSE,
                    AlphaToOne::Enabled => VK_TRUE,
                },
            },
            pDepthStencilState: null_mut(),
            pColorBlendState: &VkPipelineColorBlendStateCreateInfo {
                sType: VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
                pNext: null_mut(),
                flags: 0,
                logicOpEnable: if let LogicOp::Disabled = logic_op { VK_FALSE } else { VK_TRUE },
                logicOp: match logic_op {
                    LogicOp::Disabled => VK_LOGIC_OP_COPY,
                    LogicOp::Clear => VK_LOGIC_OP_CLEAR,
                    LogicOp::And => VK_LOGIC_OP_AND,
                    LogicOp::AndReverse => VK_LOGIC_OP_AND_REVERSE,
                    LogicOp::Copy => VK_LOGIC_OP_COPY,
                    LogicOp::AndInverted => VK_LOGIC_OP_AND_INVERTED,
                    LogicOp::NoOp => VK_LOGIC_OP_NO_OP,
                    LogicOp::Xor => VK_LOGIC_OP_XOR,
                    LogicOp::Or => VK_LOGIC_OP_OR,
                    LogicOp::Nor => VK_LOGIC_OP_NOR,
                    LogicOp::Equivalent => VK_LOGIC_OP_EQUIVALENT,
                    LogicOp::Invert => VK_LOGIC_OP_INVERT,
                    LogicOp::OrReverse => VK_LOGIC_OP_OR_REVERSE,
                    LogicOp::CopyInverted => VK_LOGIC_OP_COPY_INVERTED,
                    LogicOp::OrInverted => VK_LOGIC_OP_OR_INVERTED,
                    LogicOp::Nand => VK_LOGIC_OP_NAND,
                    LogicOp::Set => VK_LOGIC_OP_SET,
                },
                attachmentCount: 1,
                pAttachments: &VkPipelineColorBlendAttachmentState {
                    blendEnable: blend.0,
                    srcColorBlendFactor: blend.1,
                    dstColorBlendFactor: blend.2,
                    colorBlendOp: blend.3,
                    srcAlphaBlendFactor: blend.4,
                    dstAlphaBlendFactor: blend.5,
                    alphaBlendOp: blend.6,
                    colorWriteMask: 
                        (if write_mask.0 { VK_COLOR_COMPONENT_R_BIT } else { 0 }) |
                        (if write_mask.1 { VK_COLOR_COMPONENT_G_BIT } else { 0 }) |
                        (if write_mask.2 { VK_COLOR_COMPONENT_B_BIT } else { 0 }) |
                        (if write_mask.3 { VK_COLOR_COMPONENT_A_BIT } else { 0 }),
                },
                blendConstants: [0.0,0.0,0.0,0.0],
            },
            pDynamicState: null_mut(),
            layout: pipeline_layout.vk_pipeline_layout,
            renderPass: render_pass.vk_render_pass,
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
                return None;
            },
        }        

        Some(Rc::new(GraphicsPipeline {
            session: Rc::clone(self),
            pipeline_layout: Rc::clone(pipeline_layout),
            render_pass: Rc::clone(render_pass),
            vertex_shader: Rc::clone(vertex_shader),
            fragment_shader: Rc::clone(fragment_shader),
            vk_graphics_pipeline: unsafe { vk_graphics_pipeline.assume_init() },
        }))
    }
}

impl Drop for GraphicsPipeline {

    fn drop(&mut self) {
        unsafe { vkDestroyPipeline(self.session.vk_device,self.vk_graphics_pipeline,null_mut()) };
    }
}
