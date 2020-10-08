// E - Menu test
// Desmond Germans, 2020

use base::*;
use platform::*;
use gpu::*;
use ui::*;
use std::rc::Rc;

const FONT_DIR: &str = "/home/desmond/e/static/fonts";

fn main() -> Result<(),SystemError> {
    let system = Rc::new(System::new()?);
    let graphics = Rc::new(Graphics::new(&system)?);
    let ui = Rc::new(UI::new(&system,&graphics,FONT_DIR)?);

    let menubar = Rc::new(MenuBar::new(&ui,vec![
        MenuItem { name: "File".to_string(), menu: Rc::new(Menu { }), },
        MenuItem { name: "Edit".to_string(), menu: Rc::new(Menu { }), },
        MenuItem { name: "Selection".to_string(), menu: Rc::new(Menu { }), },
        MenuItem { name: "View".to_string(), menu: Rc::new(Menu { }), },
        MenuItem { name: "Go".to_string(), menu: Rc::new(Menu { }), },
        MenuItem { name: "Run".to_string(), menu: Rc::new(Menu { }), },
        MenuItem { name: "Terminal".to_string(), menu: Rc::new(Menu { }), },
        MenuItem { name: "Help".to_string(), menu: Rc::new(Menu { }), },
    ])?);

    let id = ui.open_frame(rect!(50,50,640,350),"Menu Test",&menubar);

    ui.run();

    ui.close(id);

    Ok(())
}
