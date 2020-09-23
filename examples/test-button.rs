// E - Button test
// Desmond Germans, 2020

use e::*;
use std::rc::Rc;

fn main() {
    let system = Rc::new(System::new().expect("Cannot open system."));
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open GPU."));
    let mut ui = ui::UI::new(&system,&graphics,"static/fonts").expect("Cannot open UI.");

    let mut button = ui::Button::new(&ui.state,"Click",&ui.state.font);
    button.padding = Vec2::<i32>::new(40,20);

    let widget = Rc::new(button);
    ui.open_frame(Rect::<i32>::new(50,50,640,350),"Button Test",&widget);

    ui.run();

    ui.close(&widget);
}
