// E - AppWindow test
// Desmond Germans, 2020

use e::UI;
use e::Event;
use std::rc::Rc;
use std::cell::RefCell;
use e::vec2;
use e::prelude::*;
use e::rect;

struct App {
    running: bool,
}

fn handler(name: &str,event: Event,app: &mut App) {
    match event {
        Event::KeyPress(k) => {
            println!("{}: KeyPress {}",name,k);
        },
        Event::KeyRelease(k) => {
            println!("{}: KeyRelease {}",name,k);
        },
        Event::MousePress(p,b) => {
            println!("{}: MousePress {},{}",name,p,b);
        },
        Event::MouseRelease(p,b) => {
            println!("{}: MouseRelease {},{}",name,p,b);
        },
        Event::MouseMove(p) => {
            println!("{}: MouseMove {}",name,p);
        },
        Event::MouseWheel(b) => {
            println!("{}: MouseWheel {}",name,b);
        },
        Event::Resize(s) => {
            println!("{}: Resize {}",name,s);
        },
        Event::Paint(_,r) => {
            println!("{}: Paint {}",name,r);
        },
        Event::Close => {
            println!("{}: Close",name);
            app.running = false;
        }
    }
}

fn main() {
    let app = Rc::new(RefCell::new(App { running: true, }));
    let mut ui = match UI::new() {
        Ok(ui) => ui,
        Err(_) => { panic!("Cannot open UI."); },
    };
    let cloned_app = app.clone();
    ui.create_window(
        rect!(50,50,640,360),
        "Test Window",
        move |event| {
            let mut app = cloned_app.borrow_mut();
            handler("Test Window",event,&mut *app);
        }
    );
    while app.borrow().running {
        ui.wait();
        ui.pump();
    }
}