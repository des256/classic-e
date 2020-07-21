System:
    Graphics:
        ideally, this should mimic the Vulkan API
    Compute:
    Window:

One way is for System to provide pump(T) method, where T: &Vec<Window> or &Window, and the result is a Vec<(&Window,Event)>, and then the main loop can do whatever it wants with it.