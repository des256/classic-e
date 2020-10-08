// E - HStack test
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

    // create texts
    let text1 = Box::new(Text::new(&ui,"Sans 16")?);
    let text2 = Box::new(Text::new(&ui,"Sans 32")?);
    let text3 = Box::new(Text::new(&ui,"Serif 16")?);
    let text4 = Box::new(Text::new(&ui,"Serif 32")?);
    let text5 = Box::new(Text::new(&ui,"Mono 16")?);
    let text6 = Box::new(Text::new(&ui,"Mono 32")?);

    let vstack = Box::new(VStack::new_from_vec(&ui,vec![text1,text2,text3,text4,text5,text6])?);
    vstack.halign.set(HAlignment::Center);

    // create more widgets
    let mut text8 = Box::new(Text::new(&ui,"File")?);
    let mut text9 = Box::new(Text::new(&ui,"Edit")?);
    let mut text10 = Box::new(Text::new(&ui,"Selection")?);
    let mut text11 = Box::new(Text::new(&ui,"View")?);
    text8.padding = vec2!(4,2);
    text9.padding = vec2!(4,2);
    text10.padding = vec2!(4,2);
    text11.padding = vec2!(4,2);
    
    // create HStack
    let hstack = Rc::new(HStack::new_from_vec(&ui,vec![text8,text9,text10,text11,vstack])?);
    hstack.valign.set(VAlignment::Center);

    let id = ui.open_frame(rect!(50,50,640,350),"HStack Test",&hstack);

    ui.run();

    ui.close(id);

    Ok(())
}
