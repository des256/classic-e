// E - AppWindow test
// Desmond Germans, 2020

use e::*;
use std::{
    rc::Rc,
    cell::Cell,
};

struct AppWindow {
    pub core: BaseWindow,
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

    fn rect(&self) -> i32r {
        self.core.r.get()
    }

    fn set_rect(&self,r: i32r) {
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
        core: WindowCore::new_frame(&system,i32r::from_os(i32x2::from_xy(50,50),i32x2::from_xy(640,350)),"Test Window"),
        running: Cell::new(true),
    };

    // run the show
    while appwindow.running.get() {
        system.wait();
        system.flush(&vec![&appwindow]);
    }
}
