# E

After rummaging through several Rust projects on the Internet, trying to
convince myself that I shouldn't do it all over again, I found nothing too
helpful, or up to similar standards as my old trusty 25-year old... E.

# Overview

Regardless of how naive it sounds, E is supposed to function as a basic library
for all supported platforms. The architecture is very simple.

At the root are the basic data structuring and mathematical tools. All
submodules depend on this.

The submodule `platform` abstracts all platform-dependent stuff, like audio and
desktop windowing (not UI widgets).

The submodule `gpu` abstracts away compute and graphics contexts that run on
the GPU. This includes everything related to DirectX, OpenGL, Vulkan, etc.
`gpu` needs `platform` to get access to the system.

The submodule `image` contains image format encoders and decoders.

The submodule `ui` defines interactive widgets that can be placed on windows
from `platform`, using graphics from `gpu`.

## Data Structures

`Mat` are 2D arrays, just like `Vec` are 1D arrays.

`Ten` are 3D arrays, like `Vec` and `Mat`.

`pixel` defines various pixel formats as basic data types. Constructions like
`Mat<pixel::ARGB8>` can be regarded as main memory images. The following pixel
formats are currently defined:

- pixel::R8
- pixel::R3G3B2
- pixel::ARGB2
- pixel::R5G6B5
- pixel::ARGB4
- pixel::A1RGB5
- pixel::RGB8
- pixel::ARGB8
- pixel::A2RGB10

### Future

- Serialization/deserialization, but not with `Serde`.

## Mathematical Types

`Complex` are complex numbers. `Quat` are quaternions.

`Vec2`, `Vec3`, `Vec3A` and `Vec4` are 2D, 3D and 4D vectors in a mathematical
sense. `Mat2x2`, `Mat3x3`, `Mat3x3A` and `Mat4x4` are matrices in a
mathematical sense. Basic arithmetic operations are defined for these types,
and they correspond directly to similar types inside shaders. `Vec2`, `Vec3A`
and `Vec4` are implemented on top of SIMD access. `Vec3` is not. `Mat2x2`,
`Mat3x3A` and `Mat4x4` are implemented on top of SIMD access. `Mat3x3` is not.
Use `Vec3` for storage and space-preservation. Use `Vec3A` for speed. The
trait `From` is defined to convert one to the other.

`Rect` describes a rectangle. A rectangle consists of an origin and a size.

`MultiVec2`, `MultiVec3` and `MultiVec4` are 2D, 3D and 4D multivectors.

`Quat` describes a quaternion.

### Future

- Euler angles (all 6 versions)
- Higher-dimensional vectors and matrices
- Optimization algorithms

## Platform Abstraction

In order to access the platform, create a `System` object. This abstracts away
housekeeping of system resources. In order to open a desktop window (for
Linux, Windows and MacOS), create a `Window` object via `Window::new_frame()`
or `Window::new_popup()`. This represents an open window on the screen. To
handle events, register a handler closure with `Window::set_handler`. Use
`System::wait()` to wait for events and `System::flush()` to distribute the
events to the windows via the closures.

### Future

- Similar interface to panels on mobile phones, and the canvas on a webpage
- Touch/swipe/etc. events

## Compute and Graphics

To use the GPU for graphics, create a `gpu::Graphics` context. Bind the
graphics object to a window by calling `Graphics::bind_target(T)` where `T`
can be an object implementing the `Window` trait. As soon as the graphics
pipeline is accessed, call `Graphics::present()` to swap the buffers.

### Future

- Compute functionality
- Modernize design towards Vulkan/DirectX12
- Singular shading language that could be deployed everywhere easily

## Image Formats

Supported image formats are BMP, PNG (only loading) and JPEG (only loading).

### Future

- GIF
- TIFF
- TGA
- PBM
- WebP
- XBM

## UI Widgets

The `ui` submodule builds on top of `System` and `Graphics` and provides
recursive widgets for each window, as well as management of all the windows
as they open and close during operation of the UI. Use
`UIWindow::new_frame()` and `UIWindow::new_popup()` to access the UI
resouces and use recursive widgets. Running the UI is done with `UI::run()`.
This takes care of the event handling loop. Drawing is done using the context
`Draw`, which specifies shaders and drawing mechanisms for UI widgets.

### Future

- Accordeon
- Book
- Button
- DatePicker
- Field
- FilePicker
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
- Splitter
- Stack
- Stepper
- Text
- TimePicker
- Toggle
- ToolBar
- Tree

## Later Future

- Some sort of scene graph library that can handle 3D graphics everywhere
- 3D Formats in that library
- Video Formats
- Audio Formats
- AR
- VR
- Deep learning
- Networking
