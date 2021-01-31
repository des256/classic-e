#ifdef _GPU_VULKAN_
#include <vulkan/vulkan.h>
#endif

#ifdef _SYSTEM_LINUX_
#include <xcb/xcb.h>
#ifdef _GPU_VULKAN_
#include <vulkan/vulkan_xcb.h>
#endif
#endif

#ifdef _SYSTEM_WINDOWS_
#ifdef _GPU_VULKAN_
#include <vulkan/vulkan_win32.h>
#endif
#endif

#ifdef _SYSTEM_MACOS_
#endif

#ifdef _SYSTEM_ANDROID_
#endif

#ifdef _SYSTEM_IOS_
#endif
