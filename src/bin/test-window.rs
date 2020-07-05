// E - AppWindow test
// Desmond Germans, 2020

use e::*;
use std::rc::Rc;
use std::cell::RefCell;

struct App {
    system: Rc<System>,
    running: bool,
}

impl Handler for App {
    fn handle(&mut self,event: Event) {
        match event {
            Event::KeyPress(k) => {
                println!("KeyPress {}",k);
            },
            Event::KeyRelease(k) => {
                println!("KeyRelease {}",k);
            },
            Event::MousePress(p,b) => {
                println!("MousePress {},{}",p,b);
            },
            Event::MouseRelease(p,b) => {
                println!("MouseRelease {},{}",p,b);
            },
            Event::MouseMove(p) => {
                println!("MouseMove {}",p);
            },
            Event::MouseWheel(b) => {
                println!("MouseWheel {}",b);
            },
            Event::Resize(s) => {
                println!("Resize {}",s);
            },
            Event::Paint(s,r) => {
                println!("Paint {}; {}",s,r);
            },
            Event::Close => {
                println!("Close");
                self.running = false;
            },
        }
    }
}

fn main() {
    let system = Rc::new(match System::new() {
        Ok(system) => system,
        Err(_) => { panic!("Cannot open system."); },
    });

    let app = Rc::new(RefCell::new(App {
        system: Rc::clone(&system),
        running: true,
    }));

    // create the window
    system.create_window(
        rect!(50,50,640,360),
        "Test Window",
        app.clone()
    );

    // event loop
    while app.borrow().running {
        system.wait();
        system.pump();
    }
}
