// E - System
// Desmond Germans, 2020

/// System context.
pub struct System {
    anchor: Rc<bool>,
}

/// OS window (for desktop environments).
pub struct Window {
    pub r: Cell<Rect<i32>>,
}

impl System {
    /// Create new system context.
    /// ## Returns
    /// * `Ok(System)` - The new system context.
    /// * `Err(SystemError)` - The system context could not be created.
    pub fn new() -> Result<System,SystemError> {
        Err(SystemError::Generic)
    }

    /// Flush all pending window events.
    pub fn flush(&self) {
    }

    /// Wait until new window events appear.
    pub fn wait(&self) {
    }

    /// Release mouse pointer.
    pub fn release_mouse(&self) {
    }

    fn new_window(&self,r: Rect<i32>) -> Result<Rc<Window>,SystemError> {
        Err(SystemError::Generic)        
    }

    /// Create new framed window
    pub fn new_frame(&self,r: Rect<i32>,title: &str) -> Result<Rc<Window>,SystemError> {
        self.new_window(r)
    }

    /// Create new floating window
    pub fn new_popup(&self,r: Rect<i32>) -> Result<Rc<Window>,SystemError> {
        self.new_window(r)
    }
}

impl Drop for System {
    fn drop(&mut self) {
        // close all windows
    }
}

impl Window {
    /// Capture mouse pointer
    pub fn capture_mouse(&self) {
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        // close this window
    }
}
