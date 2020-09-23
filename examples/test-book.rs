// E - Book test
// Desmond Germans, 2020

use e::*;
use std::rc::Rc;

fn main() {
    let system = Rc::new(System::new().expect("Cannot open system."));
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open GPU."));
    let mut ui = ui::UI::new(&system,&graphics,"static/fonts").expect("Cannot open UI.");

    let mut button = ui::Button::new(&ui.state,"Page Button",&ui.state.font);
    button.padding = Vec2::<i32>::new(40,20);

    let mut text1 = ui::Text::new(&ui.state,"This",&ui.state.font);
    let mut text2 = ui::Text::new(&ui.state,"is a vertical",&ui.state.font);
    let mut text3 = ui::Text::new(&ui.state,"stack with",&ui.state.font);
    let mut text4 = ui::Text::new(&ui.state,"a bunch of",&ui.state.font);
    let mut text5 = ui::Text::new(&ui.state,"texts that just align",&ui.state.font);
    let mut text6 = ui::Text::new(&ui.state,"nicely.",&ui.state.font);
    let mut text7 = ui::Text::new(&ui.state,"Almost before we knew it, we had left the ground.",&ui.state.font);
    text1.color = 0xFFFF7700;
    text2.color = 0xFF77FF00;
    text3.color = 0xFF00FF77;
    text4.color = 0xFF0077FF;
    text5.color = 0xFF7700FF;
    text6.color = 0xFFFF0077;
    text7.color = 0xFFFF7700;
    let mut vstack = ui::VStack::new_from_vec(&ui.state,vec![
        Box::new(text1),
        Box::new(text2),
        Box::new(text3),
        Box::new(text4),
        Box::new(text5),
        Box::new(text6),
        Box::new(text7),
    ]);
    vstack.halign = ui::HAlignment::Center;

    let book = Rc::new(ui::Book::new_from_vec(&ui.state,vec![
        ui::Page { name: "Hello".to_string(),widget: Box::new(button), },
        ui::Page { name: "World".to_string(),widget: Box::new(vstack), },
    ]));

    ui.open_frame(Rect::<i32>::new(50,50,640,350),"Book Test",&book);

    ui.run();

    ui.close(&book);
}