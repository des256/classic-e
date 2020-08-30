// E - VStack test
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

    let text1 = Rc::new(RefCell::new(ui::Text::new(&ui.anchor,"This",&ui.anchor.font)));
    let text2 = Rc::new(RefCell::new(ui::Text::new(&ui.anchor,"is a vertical",&ui.anchor.font)));
    let text3 = Rc::new(RefCell::new(ui::Text::new(&ui.anchor,"stack with",&ui.anchor.font)));
    let text4 = Rc::new(RefCell::new(ui::Text::new(&ui.anchor,"a bunch of",&ui.anchor.font)));
    let text5 = Rc::new(RefCell::new(ui::Text::new(&ui.anchor,"texts that just align",&ui.anchor.font)));
    let text6 = Rc::new(RefCell::new(ui::Text::new(&ui.anchor,"nicely.",&ui.anchor.font)));
    let text7 = Rc::new(RefCell::new(ui::Text::new(&ui.anchor,"Almost before we knew it, we had left the ground.",&ui.anchor.font)));
    text1.borrow_mut().color = 0xFFFF7700;
    text2.borrow_mut().color = 0xFF77FF00;
    text3.borrow_mut().color = 0xFF00FF77;
    text4.borrow_mut().color = 0xFF0077FF;
    text5.borrow_mut().color = 0xFF7700FF;
    text6.borrow_mut().color = 0xFFFF0077;
    text7.borrow_mut().color = 0xFFFF7700;

    let vstack = Rc::new(RefCell::new(ui::VStack::new_from_vec(&ui.anchor,vec![text1,text2,text3,text4,text5,text6,text7])));
    vstack.borrow_mut().halign = ui::HAlignment::Center;

    ui.open_frame(rect!(50,50,640,360),"Test Window",&(vstack as Rc<RefCell<dyn ui::Widget>>));
    ui.run();        
}
