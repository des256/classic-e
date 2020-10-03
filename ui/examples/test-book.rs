// E - Book test
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
    let mut ui = UI::new(&system,&graphics,FONT_DIR)?;

    let mut button = Button::new(&ui.state,"Page Button")?;
    button.padding = vec2!(40,20);

    let text1 = Text::new(&ui.state,"This")?;
    let text2 = Text::new(&ui.state,"is a vertical")?;
    let text3 = Text::new(&ui.state,"stack with")?;
    let text4 = Text::new(&ui.state,"a bunch of")?;
    let text5 = Text::new(&ui.state,"texts that just align")?;
    let text6 = Text::new(&ui.state,"nicely.")?;
    let text7 = Text::new(&ui.state,"Almost before we knew it, we had left the ground.")?;
    let vstack = VStack::new_from_vec(&ui.state,vec![
        Box::new(text1),
        Box::new(text2),
        Box::new(text3),
        Box::new(text4),
        Box::new(text5),
        Box::new(text6),
        Box::new(text7),
    ])?;
    vstack.halign.set(HAlignment::Center);

    let book = Rc::new(Book::new_from_vec(&ui.state,vec![
        Page { name: "Hello".to_string(),widget: Box::new(button), },
        Page { name: "World".to_string(),widget: Box::new(vstack), },
    ])?);

    ui.open_frame(rect!(50,50,640,350),"Book Test",&book);

    ui.run();

    ui.close(&book);

    Ok(())
}
