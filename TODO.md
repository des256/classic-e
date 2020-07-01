# TODO

## Production

## Style

### Rethink Vectors

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

### Same for the other types

Use `rect!(ox,oy,sx,sy)` or `rect!(o,s)`.

Same should happen with Quaternion, etc.