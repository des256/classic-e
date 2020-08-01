// E - HStack test
// Desmond Germans, 2020

use e::*;
use e::ui::Widget;
use std::rc::Rc;

fn main() {

    // initialize system
    let system = Rc::new(System::new().expect("Cannot open system."));

    // initialize graphics context
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open GPU."));

    // initialize UI
    let ui = Rc::new(ui::UI::new(&system,&graphics).expect("Cannot open UI."));

    // create window
    let window = Rc::new(Window::new(
        &system,
        rect!(50,50,640,360),
        "Test Window"
    ).expect("unable to create window"));

    // create UI drawing context
    let dc = Rc::new(ui::DC::new(&ui).expect("what?"));

    // create text widgets
    let text1 = Rc::new(ui::Text::new(&ui,"This").expect("Cannot create text."));
    let text2 = Rc::new(ui::Text::new(&ui,"is a vertical").expect("Cannot create text."));
    let text3 = Rc::new(ui::Text::new(&ui,"stack with").expect("Cannot create text."));
    let text4 = Rc::new(ui::Button::new(&ui,"a bunch of").expect("Cannot create text."));
    let text5 = Rc::new(ui::Text::new(&ui,"texts that just align").expect("Cannot create text."));
    let text6 = Rc::new(ui::Text::new(&ui,"nicely.").expect("Cannot create text."));
    let text7 = Rc::new(ui::Text::new(&ui,"Almost before we knew it, we had left the ground.").expect("Cannot create text."));
    text1.set_color(vec4!(1.0,0.5,0.0,1.0));
    text2.set_color(vec4!(0.5,1.0,0.0,1.0));
    text3.set_color(vec4!(0.0,1.0,0.5,1.0));
    text4.set_color(vec4!(0.0,0.5,1.0,1.0));
    text5.set_color(vec4!(0.5,0.0,1.0,1.0));
    text6.set_color(vec4!(1.0,0.0,0.5,1.0));
    text7.set_color(vec4!(1.0,0.5,0.0,1.0));

    // create VStack
    let vstack = Rc::new(ui::VStack::new(&ui,vec![text1,text2,text3,text4,text5,text6,text7]));
    vstack.set_calign(ui::HAlignment::Right);

    // create more widgets
    let text8 = Rc::new(ui::Text::new(&ui,"File").expect("Cannot create text."));
    let text9 = Rc::new(ui::Text::new(&ui,"Edit").expect("Cannot create text."));
    let text10 = Rc::new(ui::Text::new(&ui,"Selection").expect("Cannot create text."));
    let text11 = Rc::new(ui::Text::new(&ui,"View").expect("Cannot create text."));
    text8.set_padding(vec2!(4,2));
    text9.set_padding(vec2!(4,2));
    text10.set_padding(vec2!(4,2));
    text11.set_padding(vec2!(4,2));

    // create HStack
    let hstack = Rc::new(ui::HStack::new(&ui,vec![text8,text9,text10,text11,vstack]));
    hstack.set_calign(ui::VAlignment::Center);

    // main loop
    let mut running = true;
    while running {

        // wait for event to happen
        system.wait();

        // keep track of graphics changes
        let mut rendered = false;

        // process all current events
        for event in system.poll(&window) {
            match event {

                Event::Render => {
                    graphics.bind_target(&window);
                    graphics.clear(vec4!(0.0,0.3,0.4,1.0));
                    let size = window.size.get();
                    let fsize = vec2!(size.x as f32,size.y as f32);
                    let nsize = vec2!(size.x as i32,size.y as i32);
                    dc.set_size(fsize);
                    let hstack_size = hstack.measure();
                    let pos = (nsize - hstack_size) / 2;
                    hstack.draw(&dc,rect!(pos.x,pos.y,hstack_size.x,hstack_size.y));
                    rendered = true;
                },

                Event::Resize(s) => {
                    window.size.set(vec2!(s.x as usize,s.y as usize));
                },

                Event::Close => {
                    running = false;
                },

                _ => { },
            }
        }

        // if anything was updated, swap buffers
        if rendered {
            graphics.present();
        }
    }
}
