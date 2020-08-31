// E - VStack test
// Desmond Germans, 2020

use e::*;
use std::rc::Rc;

fn main() {
    
    let system = Rc::new(System::new().expect("Cannot open system."));
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open GPU."));
    let ui = Rc::new(ui::UI::new(&system,&graphics,"static/fonts").expect("Cannot open UI."));

    let mut text1 = Box::new(ui::Text::new(&ui.anchor,"This",&ui.anchor.font));
    let mut text2 = Box::new(ui::Text::new(&ui.anchor,"is a vertical",&ui.anchor.font));
    let mut text3 = Box::new(ui::Text::new(&ui.anchor,"stack with",&ui.anchor.font));
    let mut text4 = Box::new(ui::Text::new(&ui.anchor,"a bunch of",&ui.anchor.font));
    let mut text5 = Box::new(ui::Text::new(&ui.anchor,"texts that just align",&ui.anchor.font));
    let mut text6 = Box::new(ui::Text::new(&ui.anchor,"nicely.",&ui.anchor.font));
    let mut text7 = Box::new(ui::Text::new(&ui.anchor,"Almost before we knew it, we had left the ground.",&ui.anchor.font));
    text1.color = 0xFFFF7700;
    text2.color = 0xFF77FF00;
    text3.color = 0xFF00FF77;
    text4.color = 0xFF0077FF;
    text5.color = 0xFF7700FF;
    text6.color = 0xFFFF0077;
    text7.color = 0xFFFF7700;

    let mut vstack = Box::new(ui::VStack::new_from_vec(&ui.anchor,vec![text1,text2,text3,text4,text5,text6,text7]));
    vstack.halign = ui::HAlignment::Center;

    let id = ui.open_frame(rect!(50,50,640,350),"VStack Test",vstack);

    ui.run();

    ui.close_frame(id);
}
