# PRIORITY

- texture loading from file

# FOR LATER

- zoomable/pannable frame tree idea
- multiple window support in UI (syncing glXSwapBuffers seems to be difficult)
- setup sensible glXSwapInterval for each window
- capture mouse
- framebuffer size should be in i32
- keyboard control will be added later, first mouse

# OTHER WIDGET IDEAS

- Accordeon
- Breadcrumb
- Carousel
- DatePicker
- FilePicker
- MessageBox
- Pagination
- Splitter
- TimePicker
- Tree

# MOUSE CAPTURE

Widgets can capture mouse events from all over the screen. 
When a window receives an event, if the window wasn't already capturing, start capturing the mouse and simulate an "enter" event. When the mouse leaves the window area and none of the widgets is capturing, release the mouse and simulate a "leave" event.

# CORE WIDGETS

Core widgets have:

- Rc<> UI context reference
- Cell<Weak<>> parent reference
- A RefCell<Vec<Rc<>>> of children
- Cell<Option<Rc<>>> child node that currently captures the mouse
- Cell<Rect<i32>> for the rectangle in parent coordinates

implements ui::Widget