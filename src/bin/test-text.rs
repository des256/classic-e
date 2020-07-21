// E - Text test
// Desmond Germans, 2020

use e::*;
use std::rc::Rc;
use std::cell::RefCell;

struct App {
    system: Rc<System>,
    ui: Rc<UI>,
    text: Text,
    running: bool,
}

impl Handler for App {
    fn handle(&mut self,event: Event) {
        match event {
            Event::Paint(size,r) => {
                let gc = GC::new(&self.ui).expect("what?");
                self.system.clear(vec4!(0.0,0.3,0.4,1.0));
                gc.set_size(vec2!(size.x as f32,size.y as f32));
                // TODO: set proper ppu in gc
                self.text.draw(&gc,rect!(0.0,0.0,size.x as f32,size.y as f32));
            },
            Event::Close => {
                self.running = false;
            },
            _ => { },
        }
    }
}

fn main() {
    {
        let system = Rc::new(match System::new() {
            Ok(system) => system,
            Err(_) => { panic!("Cannot open system."); },
        });

        let ui = Rc::new(match UI::new(&system) {
            Ok(ui) => ui,
            Err(_) => { panic!("Cannot initialize UI."); },
        });

        let text = Text::new(&ui,"Hello, World!");
        text.set_color(vec4!(1.0,0.5,0.0,1.0));

        let app = Rc::new(RefCell::new(App {
            system: Rc::clone(&system),
            ui: ui,
            text: text,
            running: true,
        }));

        system.create_window(
            rect!(50,50,640,360),
            "Test Window",
            app.clone()
        );

        while app.borrow().running {
            system.wait();
            system.pump();
        }
    }
}