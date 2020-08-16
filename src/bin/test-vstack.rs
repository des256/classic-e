// E - Text test
// Desmond Germans, 2020

use e::*;
use e::ui::Color;
use e::ui::BackColor;
use std::rc::Rc;

fn main() {

    // initialize system
    let system = Rc::new(System::new().expect("Cannot open system."));

    // initialize graphics context
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open GPU."));

    // initialize UI
    let ui = Rc::new(ui::UI::new(&system,&graphics).expect("Cannot open UI."));

    // create text widgets
    let text1 = Rc::new(ui::Text::new(&ui,"This",16).expect("Cannot create text."));
    let text2 = Rc::new(ui::Text::new(&ui,"is a vertical",16).expect("Cannot create text."));
    let text3 = Rc::new(ui::Text::new(&ui,"stack with",16).expect("Cannot create text."));
    let text4 = Rc::new(ui::Text::new(&ui,"a bunch of",16).expect("Cannot create text."));
    let text5 = Rc::new(ui::Text::new(&ui,"texts that just align",16).expect("Cannot create text."));
    let text6 = Rc::new(ui::Text::new(&ui,"nicely.",16).expect("Cannot create text."));
    let text7 = Rc::new(ui::Text::new(&ui,"Almost before we knew it, we had left the ground.",16).expect("Cannot create text."));
    text1.set_color(0xFFFF7700);
    text1.set_back_color(0xFF001133);
    text2.set_color(0xFF77FF00);
    text2.set_back_color(0xFF001133);
    text3.set_color(0xFF00FF77);
    text3.set_back_color(0xFF001133);
    text4.set_color(0xFF0077FF);
    text4.set_back_color(0xFF001133);
    text5.set_color(0xFF7700FF);
    text5.set_back_color(0xFF001133);
    text6.set_color(0xFFFF0077);
    text6.set_back_color(0xFF001133);
    text7.set_color(0xFFFF7700);
    text7.set_back_color(0xFF001133);
    
    // create VStack
    let widget = Rc::new(ui::VStack::new(&ui,vec![text1,text2,text3,text4,text5,text6,text7]));

    // open window to host the text widget
    ui.open(&(widget as Rc<dyn ui::Widget>),rect!(50,50,640,360),"Test Window");

    // run UI loop
    ui.run();
}
