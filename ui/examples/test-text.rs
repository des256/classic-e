// E - Text test
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
    let text = Rc::new(Text::new(&ui,"Hello, World!")?);
    let window = UIWindow::new_frame(&ui,rect!(50,50,640,350),"Text Test",&(text as Rc<dyn Widget>))?;
    ui.run();
    drop(window);
    Ok(())
}
