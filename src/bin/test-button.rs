// E - Button test
// Desmond Germans, 2020

use e::*;
//use e::ui::Widget;
use std::rc::Rc;

fn main() {

    // initialize system
    let system = Rc::new(System::new().expect("Cannot open system."));

    // initialize graphics context
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open GPU."));

    // initialize UI
    let ui = Rc::new(ui::UI::new(&system,&graphics,"static/fonts").expect("Cannot open UI."));

    // create button widget
    let widget = Rc::new(ui::Button::new(&ui,"Click",&ui.font).expect("Cannot create button."));

    // open window to host the text widget
    ui.open(&(widget as Rc<dyn ui::Widget>),rect!(50,50,640,360),"Test Window");

    // run UI loop
    ui.run();
}
