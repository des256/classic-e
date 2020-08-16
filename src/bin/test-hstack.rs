// E - HStack test
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
    let text1 = Rc::new(ui::Text::new(&ui,"Size 12",12).expect("Cannot create text."));
    let text2 = Rc::new(ui::Text::new(&ui,"Size 14",14).expect("Cannot create text."));
    let text3 = Rc::new(ui::Text::new(&ui,"Size 16",16).expect("Cannot create text."));
    let text4 = Rc::new(ui::Text::new(&ui,"Size 18",18).expect("Cannot create text."));
    let text5 = Rc::new(ui::Text::new(&ui,"Size 20",20).expect("Cannot create text."));
    let text6 = Rc::new(ui::Text::new(&ui,"Size 22",22).expect("Cannot create text."));
    let text7 = Rc::new(ui::Text::new(&ui,"Size 24",24).expect("Cannot create text."));
    text1.set_color(0xFFFF0000);
    text1.set_back_color(0xFF001133);
    text2.set_color(0xFFFFFF00);
    text2.set_back_color(0xFF001133);
    text3.set_color(0xFF00FF00);
    text3.set_back_color(0xFF001133);
    text4.set_color(0xFF00FFFF);
    text4.set_back_color(0xFF001133);
    text5.set_color(0xFF0000FF);
    text5.set_back_color(0xFF001133);
    text6.set_color(0xFFFF00FF);
    text6.set_back_color(0xFF001133);
    text7.set_color(0xFFFF0000);
    text7.set_back_color(0xFF001133);
    
    // create VStack
    let vstack = Rc::new(ui::VStack::new(&ui,vec![text1,text2,text3,text4,text5,text6,text7]));

    // create more widgets
    let text8 = Rc::new(ui::Text::new(&ui,"File",10).expect("Cannot create text."));
    text8.set_back_color(0xFF001133);
    let text9 = Rc::new(ui::Text::new(&ui,"Edit",10).expect("Cannot create text."));
    text9.set_back_color(0xFF001133);
    let text10 = Rc::new(ui::Text::new(&ui,"Selection",10).expect("Cannot create text."));
    text10.set_back_color(0xFF001133);
    let text11 = Rc::new(ui::Text::new(&ui,"View",10).expect("Cannot create text."));
    text11.set_back_color(0xFF001133);

    // create HStack
    let widget = Rc::new(ui::HStack::new(&ui,vec![text8,text9,text10,text11,vstack]));
    //widget.set_calign(ui::VAlignment::Center);

    // open window to host the text widget
    ui.open(&(widget as Rc<dyn ui::Widget>),rect!(50,50,640,360),"Test Window");

    // run UI loop
    ui.run();
}
