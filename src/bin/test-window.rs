// E - AppWindow test
// Desmond Germans, 2020

use e::UI;
use e::Event;
use e::isize_2;
use e::isize_r;
use std::rc::Rc;
use std::cell::RefCell;

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
        Event::Paint(r) => {
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
    let first_app = app.clone();
    ui.create_window(
        &isize_r::new(isize_2::new(50,50),isize_2::new(640,360)),
        "First Window",
        move |event| {
            let mut app = first_app.borrow_mut();
            handler("First Window",event,&mut *app);
        }
    );
    let second_app = app.clone();
    ui.create_window(
        &isize_r::new(isize_2::new(100,100),isize_2::new(640,360)),
        "Second Window",
        move |event| {
            let mut app = second_app.borrow_mut();
            handler("Second Window",event,&mut *app);
        }
    );
    while app.borrow().running {
        ui.wait();
        ui.pump();
    }
}