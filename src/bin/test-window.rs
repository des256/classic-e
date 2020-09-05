// E - AppWindow test
// Desmond Germans, 2020

use e::*;
use std::{
    rc::Rc,
    cell::Cell,
};

struct AppWindow {
    pub core: WindowCore,
    running: Cell<bool>,
}

impl Window for AppWindow {
    fn handle(&self,event: Event) {
        match event {
            Event::Close => {
                self.running.set(false);
            },
            _ => { },
        }
    }

    fn rect(&self) -> Rect<i32> {
        self.core.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.core.r.set(r);
    }

    fn id(&self) -> u64 {
        self.core.id
    }
}

fn main() {

    // initialize system
    let system = Rc::new(System::new().expect("Cannot open system."));

    // initialize graphics
    let _graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open graphics."));

    // create application window
    let appwindow = AppWindow {
        core: WindowCore::new_frame(&system,rect!(50,50,640,350),"Test Window"),
        running: Cell::new(true),
    };

    // run the show
    let windows = vec![appwindow];
    while windows[0].running.get() {
        system.wait();
        system.flush(&windows);
    }
}
