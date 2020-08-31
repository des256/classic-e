// E - Button test
// Desmond Germans, 2020

use e::*;
use std::rc::Rc;

fn main() {
    let system = Rc::new(System::new().expect("Cannot open system."));
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open GPU."));
    let ui = Rc::new(ui::UI::new(&system,&graphics,"static/fonts").expect("Cannot open UI."));

    let mut button = Box::new(ui::Button::new(&ui.anchor,"Click",&ui.anchor.font));
    button.padding = vec2!(40,20);

    let id = ui.open_frame(rect!(50,50,640,350),"Button Test",button);

    ui.run();

    ui.close_frame(id);
}
