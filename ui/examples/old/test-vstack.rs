// E - VStack test
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

    let text1 = Box::new(Text::new(&ui,"This")?);
    let text2 = Box::new(Text::new(&ui,"is a vertical")?);
    let text3 = Box::new(Text::new(&ui,"stack with")?);
    let text4 = Box::new(Text::new(&ui,"a bunch of")?);
    let text5 = Box::new(Text::new(&ui,"texts that just align")?);
    let text6 = Box::new(Text::new(&ui,"nicely.")?);
    let text7 = Box::new(Text::new(&ui,"Almost before we knew it, we had left the ground.")?);

    let vstack = Rc::new(VStack::new_from_vec(&ui,vec![text1,text2,text3,text4,text5,text6,text7])?);
    vstack.halign.set(HAlignment::Center);

    let id = ui.open_frame(rect!(50,50,640,350),"VStack Test",&vstack);

    ui.run();

    ui.close(id);

    Ok(())
}
