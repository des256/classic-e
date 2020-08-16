# TODO

* all regular integers should be i32, also in font format

## Redesign UI

The entire UI is a combination of vertexbuffers and views. Vertexbuffers contain vertices, each responsible for one rectangle of color and/or texture, belonging to a specific frame. A frame is a node in a tree, defining a rectangle with offset/zoom for its contents. The entire UI can be one single vertexbuffer, or any combination. The frames are uniforms.

### Vertex Format

4 64-bit words:

* `rect` (`gl_Position`) - The rectangle, pos `x` (i16),`y` (i16), size `z` (i16) x `w` (i16).
* `texc` - The texture coordinates, pos `x` (i16),`y` (i16), size `z` (i16) x `w` (i16).
* `cols` - colors.
* `dmt` - `d` = depth (f32), `m` = render mode (u8), `t` = texture layer (u16), 0 = reserved (u8).

Development steps:

1. `rect`
2. `rect` + `cols`
3. `rect` + `texc` + `cols`
4. all

### Texture Atlases

* `0` - Monochrome alpha fonts at 1024x1024.
* `1` - Fixed icons at 1024x1024.
* `2` - Packed images at 1024x1024.
* `3` - Large images at 4096x4096.

### Render Modes

b + t a; where t = t0, t1, t2 or t3
(1 - t.a) b + t a; where t = t0, t1, t2 or t3

modes:
0..7 = t = t0..t7

### Vertex Shader

#### With Geometry Shader Available

Rendering UI-related things is done by drawing `GL_POINTS` with `glDrawArrays`. One point for each rectangle. The vertex shader only passes down the points to the geometry shader.

#### Without Geometry Shader Available

Rendering without geometry shader means drawing `GL_TRIANGLE_FAN`s with `glDrawArraysInstanced`. The instance number points to the buffer containing the rectangle specs (same as the other case), and the rectangle is 4 points like so:

* 0,0
* 1,0
* 1,1
* 0,1

### Geometry Shader

The geometry shader takes each passed rectangle and outputs it as 4 vertices that describe the rectangle. This means referencing the frame tree to properly convert the corner coordinates, as well as preparing clipping data on each output vertex. During development:

1. only one frame for all rectangles
2. multiple frames, but unrelated
3. frames are in a tree
4. each frame can be offset
5. each frame can zoom

Quite possibly a good idea to start without geometry shaders, and add those later.

Also good to add a `build` function to the `Widget` trait. This will add rectangles to the buffer.

Maybe good to separate structure from state. So also maintain a state list containing `fcol` and `bcol` entries, indexed from the points. This only helps with color changes. There would be one state entry for each widget/context. 256 is not enough.

So next to vertices, we have a buffer with frames and one with states. Quick research about uniform buffers shows that GL_IMG_UNIFORM_BUFFER_OBJECT is only supported for 3% of the devices. OpenGL ES 3.0 is only supported on a few devices. In short, it might be best to stick to a simpler buffer management. After `build`, the widget knows which points are theirs and how to change state.

So `build` generates the rectangles (the text, the icons, the rectangles, etc.) into a new points list, other widget functions would be allowed to adjust the colors of specific rectangles.

A frame looks like 4 32-bit words:

* `rect` - The frame's origin `x` (i16),`y` (i16), and size `z` (i16) x `w` (i16).
* `flag` - The parent `x` (u16) and flags `y` (u16), and reserved `zw` (32).
* `zoom` - The frame's zoom factor `x` (f32) x `y` (f32).
* `ofs` - The frame's offset `x` (f32), `y` (f32).

The flags are:

* `0x0001` - Frame is visible.

### Fragment Shader

The fragment shader kills the fragment if it shouldn't exist (like depth test), and otherwise just blasts the rectangle in the specified mode.

Each draw call (or vertexbuffer) can have different blending modes. This means that an entire subtree of widgets can potentially be transparent, additive, fading in/out, etc.

## Steps

* Finish official atlassed texture functionality in UI
* Index textures with usize