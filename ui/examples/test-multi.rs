// E - Text test
// Desmond Germans, 2020

use base::*;
use platform::*;
use gpu::*;
use ui::*;
use std::rc::Rc;

const FONT_DIR: &str = "/home/desmond/e/static/fonts";
//const ICON_DIR: &str = "/home/desmond/e/static/images/icons";
//const IMAGE_PATH: &str = "/home/desmond/e/static/images/world.png";

fn dispatch(graphics: &Graphics,window: &Rc<Window>,event: Event) {
    match event {
        Event::Render => {
            graphics.bind_target(window);
            graphics.clear(0xFF001122);
            graphics.flush();
            graphics.present(window.id);
        },
        _ => { },
    }
}

fn main() -> Result<(),SystemError> {
    let system = System::new()?;
    let graphics = Graphics::new(&system)?;
    let ui = UI::new(&system,&graphics,FONT_DIR)?;

    let syswin0 = Window::new_popup(&system,rect!(500,100,300,50))?;
    let syswin0c = Rc::clone(&syswin0);
    let graphics0c = Rc::clone(&graphics);
    syswin0.set_handler(move |event| dispatch(&graphics0c,&syswin0c,event));
    let syswin1 = Window::new_popup(&system,rect!(600,200,300,50))?;
    let syswin1c = Rc::clone(&syswin1);
    let graphics1c = Rc::clone(&graphics);
    syswin1.set_handler(move |event| dispatch(&graphics1c,&syswin1c,event));
    let syswin2 = Window::new_popup(&system,rect!(700,300,300,50))?;
    let syswin2c = Rc::clone(&syswin2);
    let graphics2c = Rc::clone(&graphics);
    syswin2.set_handler(move |event| dispatch(&graphics2c,&syswin2c,event));

    let syswin0c = Rc::clone(&syswin0);
    let syswin1c = Rc::clone(&syswin1);
    let syswin2c = Rc::clone(&syswin2);
    let main_contents: Vec<Rc<dyn Widget>> = vec![
        Text::new(&ui,"Click to enable/disable the other windows")?,
        Toggle::new(&ui,move |state| { println!("toggle 0 {}",state); if state { syswin0c.show(); } else { syswin0c.hide(); } })?,
        Toggle::new(&ui,move |state| { println!("toggle 1 {}",state); if state { syswin1c.show(); } else { syswin1c.hide(); } })?,
        Toggle::new(&ui,move |state| { println!("toggle 2 {}",state); if state { syswin2c.show(); } else { syswin2c.hide(); } })?,
    ];
    let stack = Stack::new_vertical(&ui,main_contents)?;

    let window = UIWindow::new_frame(&ui,rect!(50,50,200,100),"Window Test",stack as Rc<dyn Widget>)?;
    window.show();
    ui.run();
    window.hide();
    drop(window);
    Ok(())
}
