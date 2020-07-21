# TODO

## Production

### Widgets

System provides windows and the event handling callback.
Widgets are statically linked constructions of Rust structs, roughly based on SwiftUI.

#### Window with Centering Label

Quick brainstorm... Taking SwiftUI, Win32 and Qt into the mix...

Widget = internal structure governing alignment, padding, etc.
    .padding(p)
    .align(a)

Text - single or multiline text
HStack - horizontal stack of widgets
VStack - vertical stack of widgets
Image - image
Button - button with many options
Toggle - checkbox or on/off switch
Stepper - up/down or left/right buttons
Slider - parameter slider
Progress - progress bar
Field - text input field with many options
List - scrollable list of widgets (maybe...)
DropList - combobox (maybe...)
Book - container for pages, accessible by row of tabs
Page - page for in a Book

### Image/Matrix Clash

It would be good if `Image` is used specifically for a widget, and `Mat` is used for any 2D array. Image loaders/savers use `Mat` as their format. The basic form should probably be `Mat`, not `Matrix`. Similar to `Vec` for 1D arrays.

It should be possible to use anything as contents of a `Mat`, also pixel formats and color designations.

Done. Now we have:

`Vec` = variable-sized array.
`Mat` = variable-sized 2D array.
`Vec2` = mathematical vector of 2 elements.
`Mat2x2` = mathematical matrix of 2x2 elements.
:

All 2D constructs are addressed with `Mat`, just like numpy+opencv+etc.

### f32/f64 RGB and RGBA

Bring back this kind of color specification.

No. Instead, made `Vec4<f32>` into a from-target for the colors, as well as `Vec4<u8>` (for the obvious), and `Vec4<f64>`.

## Style

### Rethink Vector Literals

`f32_2` or `usize_2` prove to be rather wordy to use. Few options:

- Use tuples for the obvious cases: `(4.0,5.0)` and `(1.0,1.0) + (3.0,8.0)`

    pro: looks very intuitive, minimal typing
    con: might become murky when other tuples are considered as well, leading to really strange bugs

- Use macros to shorten the creation of the vectors: `vec2!(4.0,5.0)`

    pro: matches WGSL and similar quite a bit
    con: -

- Remove vectors altogether, and use separate variables.

    pro: typing is easy and oldschool
    con: bad idea when considering bigger vectors

For now, let's go with using macros.

Also, use `rect!(ox,oy,sx,sy)` or `rect!(o,s)`.

Same for matrices, quaternions, etc.

### The Trouble with isize/usize

Probably better to use i32/u32 everywhere. And most likely i32 is enough. Except, when indexing, it needs to be usize..

### Using Rc<> it might be easier to refactor some ideas

Currently, events are passed via an enum to a closure, windows are owned by UI, and a Graphics object is used to load fonts, etc. Ideally, the UI object and the Graphics object are the same, and can be used ubiquitously. The closure can technically be a trait as well, one for each window.

### UI, GC, etc.

UI is the system that abstracts away user interface access to the machine. It deals with windows, graphics abstraction, events, etc. UI supplies textures, vertex buffers, draw commands, image loading, and many more things.

maybe: UI also provides a basic coordinate system abstraction that allows proper zooming/DPI/etc.

On top of UI, you can build AR/VR, widgets, games, etc. each requiring a slightly different approach to things.

For widgets we use a GC that abstracts away fonts, rectangles, icons, and other 2D primitives. All built on top of UI.

For games, we use Engine that abstracts away fonts, sprite sheets, map drawing and layers. All built on top of UI.

maybe: Fonts are more primitive, so they should be supported in UI instead.

UI is a more suitable name for widgets than the underlying layer, so the underlying layer should be called Windows? System? Video? System.

===

System abstracts the windows, graphics, fonts, events, textures, vertex buffers, draw commands, image loading, etc.

On top of System is UI for widgets, Engine for games, etc.

### Give Window struct to the user?

===

Initialization by functions that concatenate we'll look at later.