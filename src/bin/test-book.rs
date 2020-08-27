// E - Book test
// Desmond Germans, 2020

use e::*;
use std::rc::Rc;

fn main() {

    // initialize system
    let system = Rc::new(System::new().expect("Cannot open system."));

    // initialize graphics context
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open GPU."));

    // initialize UI
    let ui = Rc::new(ui::UI::new(&system,&graphics,"static/fonts").expect("Cannot open UI."));

    // create widgets for the pages
    let button = Rc::new(ui::Button::new(&ui,"Page Button",&ui.font).expect("Cannot create button."));
    button.padding.set(vec2!(40,20));

    let text1 = Rc::new(ui::Text::new(&ui,"This",&ui.font).expect("Cannot create text."));
    let text2 = Rc::new(ui::Text::new(&ui,"is a vertical",&ui.font).expect("Cannot create text."));
    let text3 = Rc::new(ui::Text::new(&ui,"stack with",&ui.font).expect("Cannot create text."));
    let text4 = Rc::new(ui::Text::new(&ui,"a bunch of",&ui.font).expect("Cannot create text."));
    let text5 = Rc::new(ui::Text::new(&ui,"texts that just align",&ui.font).expect("Cannot create text."));
    let text6 = Rc::new(ui::Text::new(&ui,"nicely.",&ui.font).expect("Cannot create text."));
    let text7 = Rc::new(ui::Text::new(&ui,"Almost before we knew it, we had left the ground.",&ui.font).expect("Cannot create text."));
    text1.color.set(0xFFFF7700);
    text2.color.set(0xFF77FF00);
    text3.color.set(0xFF00FF77);
    text4.color.set(0xFF0077FF);
    text5.color.set(0xFF7700FF);
    text6.color.set(0xFFFF0077);
    text7.color.set(0xFFFF7700);
    let vstack = Rc::new(ui::VStack::new_from_vec(&ui,vec![text1,text2,text3,text4,text5,text6,text7]).expect("Cannot create VStack."));
    vstack.halign.set(ui::HAlignment::Center);

    // create book
    let book = Rc::new(ui::Book::new_from_vec(&ui,vec![
        ("Hello".to_string(),button),
        ("World".to_string(),vstack)
    ]).expect("Cannot create book."));

    // open window to host the book
    ui.open(&(book as Rc<dyn ui::Widget>),rect!(50,50,640,360),"Test Window");

    // run UI loop
    ui.run();
}
