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
