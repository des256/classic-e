# DEV NOTES

- Stacks and Grids don't have enough expressivity yet
- get icons from same texture resource, derive smaller textures from same texture resource in gpu part
- when scrollbars get too small, they need to auto-disable
- scrollbar tabs should have a minimum size
- add various types of padding to all widgets
- different mouse cursors updated by mousemove
- there are various instances where some default text is measured to estimate the size of a widget, this might not be appropriate
- taking Cell for everything might not be entirely necessary, play around with mutability
- use struct enums instead of tuple enums

## STILL TODO

- MenuBar and Menu
- actions with menus and toolbars
- states
- pickers: color, date, file, time
- Grid
- List and Tree
- Scroller
- ToolTip

## MENU

Microsoft menus seem to be activated by mouserelease instead of mousepress

the currently open menu captures the mouse in volatile mode

