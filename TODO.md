# TODO NOW

# TODO LATER

# TODO REFACTOR

- use either Option or Result to indicate result
- UI::set_current_window_size
- UI::invalidate

# DONE

- load image directly to `Mat` or even `Texture2D`
- refactor back to pre-Core days

# NOTES

- renamed the ambiguous windows to PlatformWindow and HandleEvent for the trait
- removed invalidate() entirely, always redraw after a bunch of events
- use UIState as kind of a DC from which the draw commands happen
- also make the offset implicit in UIState, remove from draw calls, use oldfashioned delta_offset!

## GENERAL WIDGET LAYOUT

state: Rc<UIState>,
r: Cell<Rect<i32>>,
hit: Cell<...Hit>,
contents
further state
positioning
styling
