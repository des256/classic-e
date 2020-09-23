# E

After rummaging through several Rust projects on the Internet, trying to convince myself that I shouldn't do it all over again, I found nothing too helpful, or up to similar standards as my old trusty 25-year old... E.

# Overview

Regardless of how naive it sounds, E is supposed to function as a basic library for all supported platforms. The architecture is very simple.

At the root are the basic data structuring and mathematical tools. All submodules depend on this.

The submodule `platform` abstracts all platform-dependent stuff, like audio and desktop windowing (not UI widgets).

The submodule `gpu` abstracts away compute and graphics contexts that run on the GPU. This includes everything related to DirectX, OpenGL, Vulkan, etc. `gpu` needs `platform` to get access to the system.

The submodule `image` contains image format encoders and decoders.

The submodule `ui` defines interactive widgets that can be placed on windows from `platform`, using graphics from `gpu`.

## Data Structures

`Mat` are 2D arrays, just like `Vec` are 1D arrays.

`Ten` are 3D arrays, like `Vec` and `Mat`.

`pixel` defines various pixel formats as basic data types. Constructions like `Mat<pixel::ARGB8>` can be regarded as main memory images.

### Future

- Serialization/deserialization, but not with `Serde`.

## Mathematical Types

`Vec2`, `Vec3` and `Vec4` are 2D, 3D and 4D vectors in a mathematical sense. `Mat2x2`, `Mat3x3` and `Mat4x4` are matrices in a mathematical sense. Basic arithmetic operations are defined for these types, and they correspond directly to similar types inside shaders. Most likely, they are implemented using SIMD instructions.

`Vec3A` is different from `Vec3` in that it is aligned. It takes up 4x the base type (instead of 3), but is implemented as SIMD type. In contrast, `Vec3` is packed. Use `Vec3` for storage and `Vec3A` for calculations. The trait `From` is defined to convert one to the other, and math operations allow mixed types, biased towards `Vec3A`.

`Rect` describes a rectangle. A rectangle consists of an origin and a size.

`MultiVec2`, `MultiVec3` and `MultiVec4` are 2D, 3D and 4D multivectors (Geometric Algebra).

### Future

- Euler angles
- Quaternions
- Complex numbers
- Higher-dimensional vectors and matrices
- Optimization algorithms

## Platform Abstraction

### [2020/9]

In order to access the platform, create a `System` object. This abstracts away housekeeping of system resources. In order to open a desktop window (for Linux, Windows and MacOS), create a `BaseWindow` object. This represents an open window on the screen. To handle events, create an object that implements the `Window` trait (and owning a `BaseWindow`). Use `System::flush()` to handle pending windowing events with `Window::handle()`.

### Future

- Similar interface to panels on mobile phones, and the canvas on a webpage
- Touch/swipe/etc. events

## Compute and Graphics

To use the GPU for graphics, create a `gpu::Graphics` context. Bind the graphics object to a window by calling `Graphics::bind_target(T)` where `T` can be an object implementing the `Window` trait. As soon as the graphics pipeline is accessed, call `Graphics::present()` to swap the buffers.

### Future

- Compute functionality
- Modernize design towards Vulkan/DirectX12
- Singular shading language that could be deployed everywhere easily

## Image Formats

Supported image formats are BMP, PNG and JPEG.

### Future

- GIF
- TIFF
- TGA
- PBM
- WebP
- XBM

## UI Widgets

### [2020/9]

The `ui` module manages interaction and design of widgets via the `UI` context. The `UI` context manages everything necessary for the application (including platform resource access). Each window can host a widget hierarchy. Widgets are drawn using a `gpu::Graphics` context. To create a widgeted window, use `UI::open_frame()` or `UI::open_popup()`. These methods accept a `Rc<dyn Widget>` to the widget. Then, run the application with `UI.run()`.

### Future

- Book
- Breadcrumb
- Button
- DatePicker
- Field
- FilePicker
- HAccordeon
- HSplitter
- HStack
- Image
- List
- Menu
- MenuBar
- MessageBox
- Page
- Pagination
- Progress
- Scroller
- Slider
- Stepper
- Text
- TimePicker
- Toggle
- ToolBar
- Tree
- VAccordeon
- VSplitter
- VStack

## Future

- 3D Scene graph
- 3D Formats
- Video Formats
- Audio Formats
- AR
- VR
- Deep learning
- Networking
