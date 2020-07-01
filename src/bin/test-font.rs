// E - Font test
// Desmond Germans, 2020

use e::UI;
use e::Event;
use e::isize_2;
use e::isize_r;
use std::rc::Rc;
use std::cell::RefCell;
use e::f32_2;
use e::Font;
use e::ARGB8;
use e::Pixel;
use e::BlendMode;

struct App {
    running: bool,
    font: Font,
}

fn handler(event: Event,app: &mut App) {
    match event {
        Event::Paint(graphics,_) => {
            graphics.clear(0.0,0.2,0.5,1.0);
            graphics.set_color(ARGB8::new_rgba(255,127,0,255));
            graphics.set_blend(BlendMode::Over);

            //graphics.draw_text(f32_2 { x: 10.0,y: 188.0, },"This is a test, just to see",&app.font,f32_2 { x: 44.0,y:44.0, },0.0);
            //graphics.draw_text(f32_2 { x: 10.0,y: 132.0, },"how rendering text looks like",&app.font,f32_2 { x: 44.0,y:44.0, },0.0);
            //graphics.draw_text(f32_2 { x: 10.0,y: 76.0, },"inside a window, so yeah,",&app.font,f32_2 { x: 44.0,y:44.0, },0.0);
            //graphics.draw_text(f32_2 { x: 10.0,y: 20.0, },"it's kinda cool, right?",&app.font,f32_2 { x: 44.0,y:44.0, },0.0);

            graphics.draw_text(f32_2 { x: 10.0,y: 132.0, },"WHO",&app.font,f32_2 { x: 44.0,y:44.0, },0.0);
            graphics.draw_text(f32_2 { x: 10.0,y: 76.0, },"ARE",&app.font,f32_2 { x: 44.0,y:44.0, },0.0);
            graphics.draw_text(f32_2 { x: 10.0,y: 20.0, },"YOU?",&app.font,f32_2 { x: 44.0,y:44.0, },0.0);
        },
        Event::Close => {
            app.running = false;
        },
        _ => { },
    }
}

fn main() {
    let mut ui = match UI::new() {
        Ok(ui) => ui,
        Err(_) => { panic!("Cannot open UI."); },
    };
    let font = ui.graphics().load_font("font.fnt").expect("what?");
    ui.graphics().set_scale(f32_2 { x: 1.0,y: 1.0, });
    let app = Rc::new(RefCell::new(App {
        running: true,
        font: font,
    }));
    let cloned_app = app.clone();
    ui.create_window(
        &isize_r::new(isize_2::new(50,50),isize_2::new(640,360)),
        "Test Window",
        move |event| {
            let mut app = cloned_app.borrow_mut();
            handler(event,&mut *app);
        }
    );
    while app.borrow().running {
        ui.wait();
        ui.pump();
    }
}