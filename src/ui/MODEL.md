# How to UI

There are 2 basic types of window connected to the system: App windows and Popup windows. App windows have a border, titlebar, can change size, maximize/minimize, etc. Popup windows have no border, and are generally hovering above app windows. Both window types have a rendering context, and the contents of the windows are rendered by a 3D rendering callback.

The windows are owned by the global UI object. Each window has an event closure that deals with the specifics of an event for that window. Events are run by `ui.pump()`.

This should hide Win32 and X11 issues.

