// E - Text test
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
    let widget = Rc::new(RefCell::new(ui::Text::new(&ui.anchor,"Hello, World!",&ui.anchor.font)));
    ui.open_frame(rect!(50,50,640,360),"Text Test",&(widget as Rc<RefCell<dyn ui::Widget>>));
    ui.run();        
}
