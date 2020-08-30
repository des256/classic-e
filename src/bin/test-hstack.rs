// E - HStack test
// Desmond Germans, 2020

use e::*;
use std::{
    rc::Rc,
    cell::RefCell,
};

fn main() {
    let system = Rc::new(System::new().expect("Cannot open system."));
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open GPU."));
    let ui = Rc::new(ui::UI::new(&system,&graphics,"static/fonts").expect("Cannot open UI."));

    // create variety of fonts
    let sans16 = Rc::new(ui::Font::new(&ui.anchor.proto_sans,16).expect("unable to load font"));
    let sans32 = Rc::new(ui::Font::new(&ui.anchor.proto_sans,32).expect("unable to load font"));
    let serif16 = Rc::new(ui::Font::new(&ui.anchor.proto_serif,16).expect("unable to load font"));
    let serif32 = Rc::new(ui::Font::new(&ui.anchor.proto_serif,32).expect("unable to load font"));
    let mono16 = Rc::new(ui::Font::new(&ui.anchor.proto_mono,16).expect("unable to load font"));
    let mono32 = Rc::new(ui::Font::new(&ui.anchor.proto_mono,32).expect("unable to load font"));
    
    let text1 = Rc::new(RefCell::new(ui::Text::new(&ui.anchor,"Sans 16",&sans16)));
    let text2 = Rc::new(RefCell::new(ui::Text::new(&ui.anchor,"Sans 32",&sans32)));
    let text3 = Rc::new(RefCell::new(ui::Text::new(&ui.anchor,"Serif 16",&serif16)));
    let text4 = Rc::new(RefCell::new(ui::Text::new(&ui.anchor,"Serif 32",&serif32)));
    let text5 = Rc::new(RefCell::new(ui::Text::new(&ui.anchor,"Mono 16",&mono16)));
    let text6 = Rc::new(RefCell::new(ui::Text::new(&ui.anchor,"Mono 32",&mono32)));
    text1.borrow_mut().color = 0xFFFF7700;
    text2.borrow_mut().color = 0xFF77FF00;
    text3.borrow_mut().color = 0xFF00FF77;
    text4.borrow_mut().color = 0xFF0077FF;
    text5.borrow_mut().color = 0xFF7700FF;
    text6.borrow_mut().color = 0xFFFF0077;

    let vstack = Rc::new(RefCell::new(ui::VStack::new_from_vec(&ui.anchor,vec![text1,text2,text3,text4,text5,text6])));
    vstack.borrow_mut().halign = ui::HAlignment::Center;

    // create more widgets
    let text8 = Rc::new(RefCell::new(ui::Text::new(&ui.anchor,"File",&ui.anchor.font)));
    let text9 = Rc::new(RefCell::new(ui::Text::new(&ui.anchor,"Edit",&ui.anchor.font)));
    let text10 = Rc::new(RefCell::new(ui::Text::new(&ui.anchor,"Selection",&ui.anchor.font)));
    let text11 = Rc::new(RefCell::new(ui::Text::new(&ui.anchor,"View",&ui.anchor.font)));
    text8.borrow_mut().padding = vec2!(4,2);
    text9.borrow_mut().padding = vec2!(4,2);
    text10.borrow_mut().padding = vec2!(4,2);
    text11.borrow_mut().padding = vec2!(4,2);
    
    // create HStack
    let hstack = Rc::new(RefCell::new(ui::HStack::new_from_vec(&ui.anchor,vec![text8,text9,text10,text11,vstack])));
    hstack.borrow_mut().valign = ui::VAlignment::Center;

    ui.open_frame(rect!(50,50,640,360),"HStack Test",&(hstack as Rc<RefCell<dyn ui::Widget>>));
    ui.run();        
}
