// E - Button test
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

    let mut button = Button::new(&ui.state,"Click",&ui.state.font);
    button.padding = vec2!(40,20);

    let widget = Rc::new(button);
    ui.open_frame(rect!(50,50,640,350),"Button Test",&widget);

    ui.run();

    ui.close(&widget);
}
