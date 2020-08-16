// E - HStack test
// Desmond Germans, 2020

use e::*;
use e::ui::Color;
use e::ui::BackColor;
use e::ui::Padding;
use e::ui::VAlign;
use std::rc::Rc;

fn main() {

    // initialize system
    let system = Rc::new(System::new().expect("Cannot open system."));

    // initialize graphics context
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open GPU."));

    // initialize UI
    let ui = Rc::new(ui::UI::new(&system,&graphics).expect("Cannot open UI."));

    // create variety of fonts
    let sans16 = Rc::new(ui::Font::new(&ui.proto_sans,16).expect("unable to load font"));
    let sans32 = Rc::new(ui::Font::new(&ui.proto_sans,32).expect("unable to load font"));
    let serif16 = Rc::new(ui::Font::new(&ui.proto_serif,16).expect("unable to load font"));
    let serif32 = Rc::new(ui::Font::new(&ui.proto_serif,32).expect("unable to load font"));
    let mono16 = Rc::new(ui::Font::new(&ui.proto_mono,16).expect("unable to load font"));
    let mono32 = Rc::new(ui::Font::new(&ui.proto_mono,32).expect("unable to load font"));

    // create text widgets
    let text1 = Rc::new(ui::Text::new(&ui,"Sans 16",&sans16).expect("Cannot create text."));
    let text2 = Rc::new(ui::Text::new(&ui,"Sans 32",&sans32).expect("Cannot create text."));
    let text3 = Rc::new(ui::Text::new(&ui,"Serif 16",&serif16).expect("Cannot create text."));
    let text4 = Rc::new(ui::Text::new(&ui,"Serif 32",&serif32).expect("Cannot create text."));
    let text5 = Rc::new(ui::Text::new(&ui,"Mono 16",&mono16).expect("Cannot create text."));
    let text6 = Rc::new(ui::Text::new(&ui,"Mono 32",&mono32).expect("Cannot create text."));
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
    
    // create VStack
    let vstack = Rc::new(ui::VStack::new(&ui,vec![text1,text2,text3,text4,text5,text6]));

    // create more widgets
    let text8 = Rc::new(ui::Text::new(&ui,"File",&ui.font).expect("Cannot create text."));
    text8.set_back_color(0xFF001133);
    text8.set_padding(vec2!(4,2));
    let text9 = Rc::new(ui::Text::new(&ui,"Edit",&ui.font).expect("Cannot create text."));
    text9.set_back_color(0xFF001133);
    text9.set_padding(vec2!(4,2));
    let text10 = Rc::new(ui::Text::new(&ui,"Selection",&ui.font).expect("Cannot create text."));
    text10.set_back_color(0xFF001133);
    text10.set_padding(vec2!(4,2));
    let text11 = Rc::new(ui::Text::new(&ui,"View",&ui.font).expect("Cannot create text."));
    text11.set_back_color(0xFF001133);
    text11.set_padding(vec2!(4,2));

    // create HStack
    let widget = Rc::new(ui::HStack::new(&ui,vec![text8,text9,text10,text11,vstack]));
    widget.set_valign(ui::VAlignment::Center);

    // open window to host the text widget
    ui.open(&(widget as Rc<dyn ui::Widget>),rect!(50,50,640,360),"Test Window");

    // run UI loop
    ui.run();
}
