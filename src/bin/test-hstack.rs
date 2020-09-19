// E - HStack test
// Desmond Germans, 2020

use e::*;
use std::rc::Rc;

fn main() {
    let system = Rc::new(System::new().expect("Cannot open system."));
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open GPU."));
    let mut ui = ui::UI::new(&system,&graphics,"static/fonts").expect("Cannot open UI.");

    // create variety of fonts
    let sans16 = Rc::new(ui::Font::new(&ui.state.proto_sans,16).expect("unable to load font"));
    let sans32 = Rc::new(ui::Font::new(&ui.state.proto_sans,32).expect("unable to load font"));
    let serif16 = Rc::new(ui::Font::new(&ui.state.proto_serif,16).expect("unable to load font"));
    let serif32 = Rc::new(ui::Font::new(&ui.state.proto_serif,32).expect("unable to load font"));
    let mono16 = Rc::new(ui::Font::new(&ui.state.proto_mono,16).expect("unable to load font"));
    let mono32 = Rc::new(ui::Font::new(&ui.state.proto_mono,32).expect("unable to load font"));
    
    let mut text1 = Box::new(ui::Text::new(&ui.state,"Sans 16",&sans16));
    let mut text2 = Box::new(ui::Text::new(&ui.state,"Sans 32",&sans32));
    let mut text3 = Box::new(ui::Text::new(&ui.state,"Serif 16",&serif16));
    let mut text4 = Box::new(ui::Text::new(&ui.state,"Serif 32",&serif32));
    let mut text5 = Box::new(ui::Text::new(&ui.state,"Mono 16",&mono16));
    let mut text6 = Box::new(ui::Text::new(&ui.state,"Mono 32",&mono32));
    text1.color = 0xFFFF7700;
    text2.color = 0xFF77FF00;
    text3.color = 0xFF00FF77;
    text4.color = 0xFF0077FF;
    text5.color = 0xFF7700FF;
    text6.color = 0xFFFF0077;

    let mut vstack = Box::new(ui::VStack::new_from_vec(&ui.state,vec![text1,text2,text3,text4,text5,text6]));
    vstack.halign = ui::HAlignment::Center;

    // create more widgets
    let mut text8 = Box::new(ui::Text::new(&ui.state,"File",&ui.state.font));
    let mut text9 = Box::new(ui::Text::new(&ui.state,"Edit",&ui.state.font));
    let mut text10 = Box::new(ui::Text::new(&ui.state,"Selection",&ui.state.font));
    let mut text11 = Box::new(ui::Text::new(&ui.state,"View",&ui.state.font));
    text8.padding = i32x2::from_xy(4,2);
    text9.padding = i32x2::from_xy(4,2);
    text10.padding = i32x2::from_xy(4,2);
    text11.padding = i32x2::from_xy(4,2);
    
    // create HStack
    let hstack = Rc::new(ui::HStack::new_from_vec(&ui.state,vec![text8,text9,text10,text11,vstack]));
    //hstack.valign = ui::VAlignment::Center;

    ui.open_frame(i32r::from_os(i32x2::from_xy(50,50),i32x2::from_xy(640,350)),"HStack Test",&hstack);

    ui.run();

    ui.close(&hstack);
}
