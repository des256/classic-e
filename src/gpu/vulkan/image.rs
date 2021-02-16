// E - GPU (Vulkan) - Image
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

pub(crate) trait InternalFormat {
    const VK_FORMAT: VkFormat;
}

impl InternalFormat for pixel::R5G6B5UN { const VK_FORMAT: VkFormat = VK_FORMAT_R5G6B5_UNORM_PACK16; }
impl InternalFormat for pixel::A1RGB5UN { const VK_FORMAT: VkFormat = VK_FORMAT_A1R5G5B5_UNORM_PACK16; }
impl InternalFormat for pixel::R8UN { const VK_FORMAT: VkFormat = VK_FORMAT_R8_SRGB; }
impl InternalFormat for pixel::R8IN { const VK_FORMAT: VkFormat = VK_FORMAT_R8_SNORM; }
impl InternalFormat for pixel::R8U { const VK_FORMAT: VkFormat = VK_FORMAT_R8_USCALED; }
impl InternalFormat for pixel::R8I { const VK_FORMAT: VkFormat = VK_FORMAT_R8_SSCALED; }
impl InternalFormat for u8 { const VK_FORMAT: VkFormat = VK_FORMAT_R8_UINT; }
impl InternalFormat for i8 { const VK_FORMAT: VkFormat = VK_FORMAT_R8_SINT; }
impl InternalFormat for pixel::RG8UN { const VK_FORMAT: VkFormat = VK_FORMAT_R8G8_SRGB; }
impl InternalFormat for pixel::RG8IN { const VK_FORMAT: VkFormat = VK_FORMAT_R8G8_SNORM; }
impl InternalFormat for pixel::RG8U { const VK_FORMAT: VkFormat = VK_FORMAT_R8G8_USCALED; }
impl InternalFormat for pixel::RG8I { const VK_FORMAT: VkFormat = VK_FORMAT_R8G8_SSCALED; }
impl InternalFormat for Vec2<u8> { const VK_FORMAT: VkFormat = VK_FORMAT_R8G8_UINT; }
impl InternalFormat for Vec2<i8> { const VK_FORMAT: VkFormat = VK_FORMAT_R8G8_SINT; }
impl InternalFormat for pixel::RGB8UN { const VK_FORMAT: VkFormat = VK_FORMAT_R8G8B8_SRGB; }
impl InternalFormat for pixel::RGB8IN { const VK_FORMAT: VkFormat = VK_FORMAT_R8G8B8_SNORM; }
impl InternalFormat for pixel::RGB8U { const VK_FORMAT: VkFormat = VK_FORMAT_R8G8B8_USCALED; }
impl InternalFormat for pixel::RGB8I { const VK_FORMAT: VkFormat = VK_FORMAT_R8G8B8_SSCALED; }
impl InternalFormat for Vec3<u8> { const VK_FORMAT: VkFormat = VK_FORMAT_R8G8B8_UINT; }
impl InternalFormat for Vec3<i8> { const VK_FORMAT: VkFormat = VK_FORMAT_R8G8B8_SINT; }
impl InternalFormat for pixel::BGR8UN { const VK_FORMAT: VkFormat = VK_FORMAT_B8G8R8_SRGB; }
impl InternalFormat for pixel::BGR8IN { const VK_FORMAT: VkFormat = VK_FORMAT_B8G8R8_SNORM; }
impl InternalFormat for pixel::BGR8U { const VK_FORMAT: VkFormat = VK_FORMAT_B8G8R8_USCALED; }
impl InternalFormat for pixel::BGR8I { const VK_FORMAT: VkFormat = VK_FORMAT_B8G8R8_SSCALED; }
impl InternalFormat for pixel::RGBA8UN { const VK_FORMAT: VkFormat = VK_FORMAT_R8G8B8A8_SRGB; }
impl InternalFormat for pixel::RGBA8IN { const VK_FORMAT: VkFormat = VK_FORMAT_R8G8B8A8_SNORM; }
impl InternalFormat for pixel::RGBA8U { const VK_FORMAT: VkFormat = VK_FORMAT_R8G8B8A8_USCALED; }
impl InternalFormat for pixel::RGBA8I { const VK_FORMAT: VkFormat = VK_FORMAT_R8G8B8A8_SSCALED; }
impl InternalFormat for Vec4<u8> { const VK_FORMAT: VkFormat = VK_FORMAT_R8G8B8A8_UINT; }
impl InternalFormat for Vec4<i8> { const VK_FORMAT: VkFormat = VK_FORMAT_R8G8B8A8_SINT; }
impl InternalFormat for pixel::BGRA8UN { const VK_FORMAT: VkFormat = VK_FORMAT_B8G8R8A8_SRGB; }
impl InternalFormat for pixel::BGRA8IN { const VK_FORMAT: VkFormat = VK_FORMAT_B8G8R8A8_SNORM; }
impl InternalFormat for pixel::BGRA8U { const VK_FORMAT: VkFormat = VK_FORMAT_B8G8R8A8_USCALED; }
impl InternalFormat for pixel::BGRA8I { const VK_FORMAT: VkFormat = VK_FORMAT_B8G8R8A8_SSCALED; }
impl InternalFormat for pixel::ABGR8UN { const VK_FORMAT: VkFormat = VK_FORMAT_A8B8G8R8_SRGB_PACK32; }
impl InternalFormat for pixel::ABGR8IN { const VK_FORMAT: VkFormat = VK_FORMAT_A8B8G8R8_SNORM_PACK32; }
impl InternalFormat for pixel::ABGR8U { const VK_FORMAT: VkFormat = VK_FORMAT_A8B8G8R8_USCALED_PACK32; }
impl InternalFormat for pixel::ABGR8I { const VK_FORMAT: VkFormat = VK_FORMAT_A8B8G8R8_SSCALED_PACK32; }
impl InternalFormat for pixel::A2RGB10UN { const VK_FORMAT: VkFormat = VK_FORMAT_A2R10G10B10_UNORM_PACK32; }
impl InternalFormat for pixel::R16UN { const VK_FORMAT: VkFormat = VK_FORMAT_R16_UNORM; }
impl InternalFormat for pixel::R16IN { const VK_FORMAT: VkFormat = VK_FORMAT_R16_SNORM; }
impl InternalFormat for pixel::R16U { const VK_FORMAT: VkFormat = VK_FORMAT_R16_USCALED; }
impl InternalFormat for pixel::R16I { const VK_FORMAT: VkFormat = VK_FORMAT_R16_SSCALED; }
impl InternalFormat for u16 { const VK_FORMAT: VkFormat = VK_FORMAT_R16_UINT; }
impl InternalFormat for i16 { const VK_FORMAT: VkFormat = VK_FORMAT_R16_SINT; }
impl InternalFormat for pixel::RG16UN { const VK_FORMAT: VkFormat = VK_FORMAT_R16G16_UNORM; }
impl InternalFormat for pixel::RG16IN { const VK_FORMAT: VkFormat = VK_FORMAT_R16G16_SNORM; }
impl InternalFormat for pixel::RG16U { const VK_FORMAT: VkFormat = VK_FORMAT_R16G16_USCALED; }
impl InternalFormat for pixel::RG16I { const VK_FORMAT: VkFormat = VK_FORMAT_R16G16_SSCALED; }
impl InternalFormat for Vec2<u16> { const VK_FORMAT: VkFormat = VK_FORMAT_R16G16_UINT; }
impl InternalFormat for Vec2<i16> { const VK_FORMAT: VkFormat = VK_FORMAT_R16G16_SINT; }
impl InternalFormat for pixel::RGB16UN { const VK_FORMAT: VkFormat = VK_FORMAT_R16G16B16_UNORM; }
impl InternalFormat for pixel::RGB16IN { const VK_FORMAT: VkFormat = VK_FORMAT_R16G16B16_SNORM; }
impl InternalFormat for pixel::RGB16U { const VK_FORMAT: VkFormat = VK_FORMAT_R16G16B16_USCALED; }
impl InternalFormat for pixel::RGB16I { const VK_FORMAT: VkFormat = VK_FORMAT_R16G16B16_SSCALED; }
impl InternalFormat for Vec3<u16> { const VK_FORMAT: VkFormat = VK_FORMAT_R16G16B16_UINT; }
impl InternalFormat for Vec3<i16> { const VK_FORMAT: VkFormat = VK_FORMAT_R16G16B16_SINT; }
impl InternalFormat for pixel::RGBA16UN { const VK_FORMAT: VkFormat = VK_FORMAT_R16G16B16A16_UNORM; }
impl InternalFormat for pixel::RGBA16IN { const VK_FORMAT: VkFormat = VK_FORMAT_R16G16B16A16_SNORM; }
impl InternalFormat for pixel::RGBA16U { const VK_FORMAT: VkFormat = VK_FORMAT_R16G16B16A16_USCALED; }
impl InternalFormat for pixel::RGBA16I { const VK_FORMAT: VkFormat = VK_FORMAT_R16G16B16A16_SSCALED; }
impl InternalFormat for Vec4<u16> { const VK_FORMAT: VkFormat = VK_FORMAT_R16G16B16A16_UINT; }
impl InternalFormat for Vec4<i16> { const VK_FORMAT: VkFormat = VK_FORMAT_R16G16B16A16_SINT; }
impl InternalFormat for u32 { const VK_FORMAT: VkFormat = VK_FORMAT_R32_UINT; }
impl InternalFormat for i32 { const VK_FORMAT: VkFormat = VK_FORMAT_R32_SINT; }
impl InternalFormat for f32 { const VK_FORMAT: VkFormat = VK_FORMAT_R32_SFLOAT; }
impl InternalFormat for Vec2<u32> { const VK_FORMAT: VkFormat = VK_FORMAT_R32G32_UINT; }
impl InternalFormat for Vec2<i32> { const VK_FORMAT: VkFormat = VK_FORMAT_R32G32_SINT; }
impl InternalFormat for Vec2<f32> { const VK_FORMAT: VkFormat = VK_FORMAT_R32G32_SFLOAT; }
impl InternalFormat for Vec3<u32> { const VK_FORMAT: VkFormat = VK_FORMAT_R32G32B32_UINT; }
impl InternalFormat for Vec3<i32> { const VK_FORMAT: VkFormat = VK_FORMAT_R32G32B32_SINT; }
impl InternalFormat for Vec3<f32> { const VK_FORMAT: VkFormat = VK_FORMAT_R32G32B32_SFLOAT; }
impl InternalFormat for Vec4<u32> { const VK_FORMAT: VkFormat = VK_FORMAT_R32G32B32A32_UINT; }
impl InternalFormat for Vec4<i32> { const VK_FORMAT: VkFormat = VK_FORMAT_R32G32B32A32_SINT; }
impl InternalFormat for Vec4<f32> { const VK_FORMAT: VkFormat = VK_FORMAT_R32G32B32A32_SFLOAT; }
impl InternalFormat for u64 { const VK_FORMAT: VkFormat = VK_FORMAT_R64_UINT; }
impl InternalFormat for i64 { const VK_FORMAT: VkFormat = VK_FORMAT_R64_SINT; }
impl InternalFormat for f64 { const VK_FORMAT: VkFormat = VK_FORMAT_R64_SFLOAT; }
impl InternalFormat for Vec2<u64> { const VK_FORMAT: VkFormat = VK_FORMAT_R64G64_UINT; }
impl InternalFormat for Vec2<i64> { const VK_FORMAT: VkFormat = VK_FORMAT_R64G64_SINT; }
impl InternalFormat for Vec2<f64> { const VK_FORMAT: VkFormat = VK_FORMAT_R64G64_SFLOAT; }
impl InternalFormat for Vec3<u64> { const VK_FORMAT: VkFormat = VK_FORMAT_R64G64B64_UINT; }
impl InternalFormat for Vec3<i64> { const VK_FORMAT: VkFormat = VK_FORMAT_R64G64B64_SINT; }
impl InternalFormat for Vec3<f64> { const VK_FORMAT: VkFormat = VK_FORMAT_R64G64B64_SFLOAT; }
impl InternalFormat for Vec4<u64> { const VK_FORMAT: VkFormat = VK_FORMAT_R64G64B64A64_UINT; }
impl InternalFormat for Vec4<i64> { const VK_FORMAT: VkFormat = VK_FORMAT_R64G64B64A64_SINT; }
impl InternalFormat for Vec4<f64> { const VK_FORMAT: VkFormat = VK_FORMAT_R64G64B64A64_SFLOAT; }
impl InternalFormat for pixel::RG11B10F { const VK_FORMAT: VkFormat = VK_FORMAT_B10G11R11_UFLOAT_PACK32; }
impl InternalFormat for pixel::RGB9E5F { const VK_FORMAT: VkFormat = VK_FORMAT_E5B9G9R9_UFLOAT_PACK32; }

pub struct Image {
    pub session: Rc<Session>,
#[doc(hidden)]
    pub(crate) owned: bool,
#[doc(hidden)]
    pub(crate) vk_image: VkImage,
    #[doc(hidden)]
    pub(crate) vk_format: VkFormat,
}

// TODO: implement creation from session, like for texture mapping and such

impl SwapChain {

    pub fn get_images(&self) -> Vec<Rc<Image>> {

        let mut count = 0u32;
        match unsafe { vkGetSwapchainImagesKHR(self.session.vk_device,self.vk_swapchain,&mut count,null_mut()) } {
            VK_SUCCESS => { },
            code => {
#[cfg(feature="debug_output")]
                println!("Unable to get swap chain image count (error {})",code);
                return Vec::new();
            }
        }
        let mut vk_images = vec![null_mut() as VkImage; count as usize];
        match unsafe { vkGetSwapchainImagesKHR(self.session.vk_device,self.vk_swapchain,&mut count,vk_images.as_mut_ptr()) } {
            VK_SUCCESS => { },
            code => {
#[cfg(feature="debug_output")]
                println!("Unable to get swap chain images (error {})",code);
                unsafe { vkDestroySwapchainKHR(self.session.vk_device,self.vk_swapchain,null_mut()) };
                return Vec::new();
            },
        }
        let mut images = Vec::<Rc<Image>>::new();
        for vk_image in &vk_images {
            images.push(Rc::new(Image {
                session: Rc::clone(&self.session),
                owned: false,
                vk_image: *vk_image,
                vk_format: self.vk_format,
            }));
        }
        images
    }
}

impl Drop for Image {

    fn drop(&mut self) {
        if self.owned {
            unsafe { vkDestroyImage(self.session.vk_device,self.vk_image,null_mut()) };
        }
    }
}
