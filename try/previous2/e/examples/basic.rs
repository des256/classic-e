// e examples: basic
// by Desmond Germans, 2019

use std::{thread,time};
extern crate e;

struct Application {
    wid: usize,
    label1: e::Label,
    label2: e::Label,
    label3: e::Label,
    button: e::Button,
}

impl Application {
    pub fn new(ui: &mut e::UI) -> Application {
        let wid = ui.create_app_window(e::isize_r::new(100,100,1920,1080),"Basic Example");
        let mut quads: Vec<e::UIQuad> = Vec::new();
        let label1 = e::Label::new(ui,wid,&mut quads,e::f32_2::new(10.0,10.0),"The quick brown fox jumps over the lazy dog.");
        let label2 = e::Label::new(ui,wid,&mut quads,e::f32_2::new(10.0,60.0),"But it remained unclear as to why the fox did this.");
        let label3 = e::Label::new(ui,wid,&mut quads,e::f32_2::new(10.0,110.0),"The dog, however, quite enjoyed watching the fox jump.");
        let button = e::Button::new(ui,wid,&mut quads,e::f32_2::new(10.0,160.0),"Ok");
        Application {
            wid: wid,
            label1: label1,
            label2: label2,
            label3: label3,
            button: button,
        }
    }

    pub fn wid(&self) -> usize {
        self.wid
    }

    pub fn handle_event(&mut self,ui: &mut e::UI,we: &e::WindowEvent) -> bool {
        match we.event {
            e::Event::MousePress(p,b) => {
                self.button.mouse_press(ui,self.wid,e::f32_2::new(p.x as f32,p.y as f32),b);
            },
            e::Event::MouseRelease(p,b) => {
                self.button.mouse_release(ui,self.wid,e::f32_2::new(p.x as f32,p.y as f32),b);
            },
            e::Event::Close => {
                return false;
            }
            _ => {
            },
        }
        true
    }
}

pub fn main() {
    let mut ui = e::UI::new();
    let mut application = Application::new(&mut ui);
    loop {
        thread::sleep(time::Duration::from_millis(10));
        while let Some(we) = ui.next_event() {
            if we.id == application.wid() {
                if !application.handle_event(&mut ui,&we) {
                    return;
                }
            }
        }
    }
}
