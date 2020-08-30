// E - Button test
// Desmond Germans, 2020

use e::*;
use std::{
    rc::Rc,
    cell::RefCell,
};

fn main() {
    let system = Rc::new(System::new().expect("Cannot open system."));
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open GPU."));
    let ui = Rc::new(ui::UI::new(&system,&graphics,"static/fonts").expect("Cannot open UI."));

    let button = Rc::new(RefCell::new(ui::Button::new(&ui.anchor,"Click",&ui.anchor.font)));
    button.borrow_mut().padding = vec2!(40,20);

    ui.open_frame(rect!(50,50,640,360),"Button Test",&(button as Rc<RefCell<dyn ui::Widget>>));

    ui.run();        
}
