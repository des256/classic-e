// E - Text test
// Desmond Germans, 2020

use e::*;
use std::rc::Rc;

fn main() {
    let system = Rc::new(System::new().expect("Cannot open system."));
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open GPU."));
    let ui = Rc::new(ui::UI::new(&system,&graphics,"static/fonts").expect("Cannot open UI."));

    let text = Box::new(ui::Text::new(&ui.anchor,"Hello, World!",&ui.anchor.font));

    let id = ui.open_frame(rect!(50,50,640,350),"Text Test",text);

    ui.run();

    ui.close_frame(id);
}
