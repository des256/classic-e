// E - GPU (Vulkan) - Device
// Desmond Germans, 2020

use {
    crate::*,
    std::{
        ptr::null_mut,
        mem::MaybeUninit,
        rc::Rc,
    },
    vulkan_sys::*,
};

pub struct Session {
    pub(crate) gpu: Rc<GPU>,
    pub(crate) device: VkDevice,
}

impl Session {

    pub fn new(gpu: &Rc<GPU>,index: usize,queues: Vec<(usize,usize)>) -> Option<Rc<Session>> {

        if index >= gpu.physical_devices.len() as usize {
            return None;
        }
        let physical_device = gpu.physical_devices[index];

        let mut queue_create_infos = Vec::<VkDeviceQueueCreateInfo>::new();
        for queue in queues {
            let mut priorities = Vec::<f32>::new();
            for _i in 0..queue.1 {
                priorities.push(1.0);
            }
            queue_create_infos.push(VkDeviceQueueCreateInfo {
                sType: VkStructureType_VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
                pNext: null_mut(),
                flags: 0,
                queueFamilyIndex: queue.0 as u32,
                queueCount: queue.1 as u32,
                pQueuePriorities: priorities.as_mut_ptr(),
            });
        }

        let physical_device_features = VkPhysicalDeviceFeatures {
            robustBufferAccess: 0,
            fullDrawIndexUint32: 0,
            imageCubeArray: 0,
            independentBlend: 0,
            geometryShader: 0,
            tessellationShader: 0,
            sampleRateShading: 0,
            dualSrcBlend: 0,
            logicOp: 0,
            multiDrawIndirect: 0,
            drawIndirectFirstInstance: 0,
            depthClamp: 0,
            depthBiasClamp: 0,
            fillModeNonSolid: 0,
            depthBounds: 0,
            wideLines: 0,
            largePoints: 0,
            alphaToOne: 0,
            multiViewport: 0,
            samplerAnisotropy: 0,
            textureCompressionETC2: 0,
            textureCompressionASTC_LDR: 0,
            textureCompressionBC: 0,
            occlusionQueryPrecise: 0,
            pipelineStatisticsQuery: 0,
            vertexPipelineStoresAndAtomics: 0,
            fragmentStoresAndAtomics: 0,
            shaderTessellationAndGeometryPointSize: 0,
            shaderImageGatherExtended: 0,
            shaderStorageImageExtendedFormats: 0,
            shaderStorageImageMultisample: 0,
            shaderStorageImageReadWithoutFormat: 0,
            shaderStorageImageWriteWithoutFormat: 0,
            shaderUniformBufferArrayDynamicIndexing: 0,
            shaderSampledImageArrayDynamicIndexing: 0,
            shaderStorageBufferArrayDynamicIndexing: 0,
            shaderStorageImageArrayDynamicIndexing: 0,
            shaderClipDistance: 0,
            shaderCullDistance: 0,
            shaderFloat64: 0,
            shaderInt64: 0,
            shaderInt16: 0,
            shaderResourceResidency: 0,
            shaderResourceMinLod: 0,
            sparseBinding: 0,
            sparseResidencyBuffer: 0,
            sparseResidencyImage2D: 0,
            sparseResidencyImage3D: 0,
            sparseResidency2Samples: 0,
            sparseResidency4Samples: 0,
            sparseResidency8Samples: 0,
            sparseResidency16Samples: 0,
            sparseResidencyAliased: 0,
            variableMultisampleRate: 0,
            inheritedQueries: 0,
        };

        let create_info = VkDeviceCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            queueCreateInfoCount: queue_create_infos.len() as u32,
            pQueueCreateInfos: queue_create_infos.as_mut_ptr(),
            enabledLayerCount: 0, // validation layer stuff goes here
            ppEnabledLayerNames: null_mut(),
            enabledExtensionCount: 0,
            ppEnabledExtensionNames: null_mut(),
            pEnabledFeatures: &physical_device_features,
        };
        let mut device = MaybeUninit::uninit();
        match unsafe { vkCreateDevice(physical_device,&create_info,null_mut(),device.as_mut_ptr()) } {
            VkResult_VK_SUCCESS => { },
            _ => { return None; },
        }
        let device = unsafe { device.assume_init() };

        Some(Rc::new(Session {
            gpu: Rc::clone(gpu),
            device: device,
        }))
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe { vkDestroyDevice(self.device,null_mut()) };
    }
}
