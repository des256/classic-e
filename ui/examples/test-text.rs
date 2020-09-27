// E - Text test
// Desmond Germans, 2020

use base::*;
use platform::*;
use gpu::*;
use ui::*;
use std::rc::Rc;

fn main() {
    let system = Rc::new(System::new().expect("Cannot open system."));
    let graphics = Rc::new(Graphics::new(&system).expect("Cannot open GPU."));
    let mut ui = UI::new(&system,&graphics,"../static/fonts").expect("Cannot open UI.");

    let text = Rc::new(Text::new(&ui.state,"Hello, World!",&ui.state.font));

    ui.open_frame(rect!(50,50,640,350),"Text Test",&Rc::clone(&text));

    ui.run();

    ui.close(&text);
}
