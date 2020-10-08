// E - Text image
// Desmond Germans, 2020

use base::*;
use platform::*;
use gpu::*;
use ui::*;
use std::rc::Rc;

const FONT_DIR: &str = "/home/desmond/e/static/fonts";
const IMAGE_PATH: &str = "/home/desmond/e/static/images/world.png";

fn main() -> Result<(),SystemError> {
    let system = Rc::new(System::new()?);
    let graphics = Rc::new(Graphics::new(&system)?);
    let ui = Rc::new(UI::new(&system,&graphics,FONT_DIR)?);

    let mat = imageformats::load::<pixel::ARGB8>(IMAGE_PATH)?;

    let image = Rc::new(Image::new(&ui,mat)?);

    let id = ui.open_frame(rect!(50,50,640,350),"Image Test",&Rc::clone(&image));

    ui.run();

    ui.close(id);

    Ok(())
}
