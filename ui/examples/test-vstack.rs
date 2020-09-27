// E - VStack test
// Desmond Germans, 2020

use base::*;
use platform::*;
use gpu::*;
use ui::*;
use std::rc::Rc;

fn main() {
    
    let system = Rc::new(System::new().expect("Cannot open system."));
    let graphics = Rc::new(Graphics::new(&system).expect("Cannot open GPU."));
    let mut ui = UI::new(&system,&graphics,"../static/fonts").expect("Cannot open UI.");

    let mut text1 = Box::new(Text::new(&ui.state,"This",&ui.state.font));
    let mut text2 = Box::new(Text::new(&ui.state,"is a vertical",&ui.state.font));
    let mut text3 = Box::new(Text::new(&ui.state,"stack with",&ui.state.font));
    let mut text4 = Box::new(Text::new(&ui.state,"a bunch of",&ui.state.font));
    let mut text5 = Box::new(Text::new(&ui.state,"texts that just align",&ui.state.font));
    let mut text6 = Box::new(Text::new(&ui.state,"nicely.",&ui.state.font));
    let mut text7 = Box::new(Text::new(&ui.state,"Almost before we knew it, we had left the ground.",&ui.state.font));
    text1.color = 0xFFFF7700;
    text2.color = 0xFF77FF00;
    text3.color = 0xFF00FF77;
    text4.color = 0xFF0077FF;
    text5.color = 0xFF7700FF;
    text6.color = 0xFFFF0077;
    text7.color = 0xFFFF7700;

    let vstack = Rc::new(VStack::new_from_vec(&ui.state,vec![text1,text2,text3,text4,text5,text6,text7]));
    //vstack.halign = HAlignment::Center;

    ui.open_frame(rect!(50,50,640,350),"VStack Test",&vstack);

    ui.run();

    ui.close(&vstack);
}
