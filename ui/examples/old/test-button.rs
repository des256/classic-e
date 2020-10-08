// E - Button test
// Desmond Germans, 2020

use base::*;
use platform::*;
use gpu::*;
use ui::*;
use std::rc::Rc;

const FONT_DIR: &str = "/home/desmond/e/static/fonts";

fn main() -> Result<(),SystemError> {
    let system = Rc::new(System::new()?);
    let graphics = Rc::new(Graphics::new(&system)?);
    let ui = Rc::new(UI::new(&system,&graphics,FONT_DIR)?);

    let mut button = Button::new(&ui,"Click")?;
    button.padding = vec2!(40,20);

    let widget = Rc::new(button);
    let id = ui.open_frame(rect!(50,50,640,350),"Button Test",&widget);

    ui.run();

    ui.close(id);

    Ok(())
}