// E - AppWindow test
// Desmond Germans, 2020

use base::*;
use platform::*;
use std::{
    rc::Rc,
    cell::Cell,
};

struct AppWindow {
    pub window: PlatformWindow,
    running: Cell<bool>,
}

impl HandleEvent for AppWindow {
    fn handle(&self,event: Event) {
        match event {
            Event::Close => {
                self.running.set(false);
            },
            _ => { },
        }
    }

    fn id(&self) -> u64 {
        self.window.id
    }
}

fn main() {

    // initialize system
    let system = Rc::new(System::new().expect("Cannot open system."));

    // create application window
    let appwindow = AppWindow {
        window: PlatformWindow::new_frame(&system,rect!(50,50,640,350),"Test Window"),
        running: Cell::new(true),
    };

    // run the show
    while appwindow.running.get() {
        system.wait();
        system.flush(&vec![&appwindow]);
    }
}
