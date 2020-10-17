# DEV NOTES

- Stacks and Grids don't have enough expressivity yet
- get icons from same texture resource, derive smaller textures from same texture resource in gpu part
- when scrollbars get too small, they need to auto-disable
- scrollbar tabs should have a minimum size
- add various types of padding to all widgets
- add different mouse cursors updated by mousemove
- there are various instances where some default text is measured to estimate the size of a widget, this might not be appropriate
- taking Cell for everything might not be entirely necessary, play around with mutability
- use struct enums instead of tuple enums
- document all the code
- rustdoc
- basic examples
- simplify the mouse/capturing code
- timers
- separate UI resources from draw context
- design how buttons and menus trigger actions, action trait or closure
- Win32 implementation
- explore Cocoa/Metal
- explore web/WebGL, WASM interface; popup menus by divs over the canvas?
- potentially improve the graphics commands, update OpenGL API towards Vulkan?
- put constant dimensions in widget styles instead

## STILL TODO

- MenuBar and Menu
- in-menu toggles
- pickers: color, date, file, time
- messagebox
- Grid
- List and Tree
- Scroller
- ToolTip
- actions with menus and toolbars
