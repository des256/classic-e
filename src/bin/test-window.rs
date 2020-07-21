// E - AppWindow test
// Desmond Germans, 2020

use e::*;
use std::sync::Arc;
use std::cell::RefCell;

struct App {
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
    let system = match System::new() {
        Ok(system) => system,
        Err(_) => { panic!("Cannot open system."); },
    };
    {
        let mut app = Arc::new(RefCell::new(App { running: true, }));
        let mut generic_app: Arc<RefCell<dyn Handler>> = app;
        let window = Window::new(
            &system,
            rect!(50,50,640,360),
            "Test Window",
            Arc::clone(&generic_app)
        );
        while app.borrow().running {
            system.wait();
            system.pump();
        }
    }
}
