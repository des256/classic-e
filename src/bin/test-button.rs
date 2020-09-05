// E - Button test
// Desmond Germans, 2020

use e::*;
use std::rc::Rc;

fn main() {
    let system = Rc::new(System::new().expect("Cannot open system."));
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open GPU."));
    let mut ui = ui::UI::new(&system,&graphics,"static/fonts").expect("Cannot open UI.");

    let button = Rc::new(ui::Button::new(&ui.state,"Click",&ui.state.font));
    button.padding.set(vec2!(40,20));

    ui.open_frame(rect!(50,50,640,350),"Button Test",&Rc::clone(&button));

    ui.run();

    ui.close(&button);
}
