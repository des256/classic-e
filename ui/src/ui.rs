// E - UI
// Desmond Germans, 2020

use crate::*;
use std::{
    cell::{
        Cell,
        RefCell,
    },
    rc::Rc,
};

pub struct UIWindow {
    pub ui: Rc<UI>,
    pub window: Rc<Window>,
    pub widget: Rc<dyn Widget>,
    pub dirty: Cell<bool>,
}

/// UI context.
pub struct UI {
    pub system: Rc<System>,
    pub graphics: Rc<Graphics>,
    pub draw: RefCell<Draw>,  // global draw context
    pub uiwindows: RefCell<Vec<Rc<UIWindow>>>,
    pub new_uiwindows: RefCell<Vec<Rc<UIWindow>>>,
    pub drop_uiwindows: RefCell<Vec<usize>>,
    pub current_capturing_id: Cell<Option<u64>>,  // window that is currently capturing the mouse (TBD)
    pub running: Cell<bool>,
}

impl UI {
    pub fn new(system: &Rc<System>,graphics: &Rc<Graphics>,font_path: &str) -> Result<UI,SystemError> {

        // create global draw context
        let draw = Draw::new(graphics,font_path)?;

        Ok(UI {
            system: Rc::clone(system),
            graphics: Rc::clone(graphics),
            draw: RefCell::new(draw),
            uiwindows: RefCell::new(Vec::new()),
            new_uiwindows: RefCell::new(Vec::new()),
            drop_uiwindows: RefCell::new(Vec::new()),
            current_capturing_id: Cell::new(None),
            running: Cell::new(true),
        })
    }

    pub fn terminate(&self) {
        self.running.set(false);
    }

    pub fn run(&self) {
        self.running.set(true);
        while self.running.get() {

            // extract current list of platform windows (TBD: can we do this nicer somehow?)
            let mut windows: Vec<Rc<Window>> = Vec::new();
            for uiwindow in self.uiwindows.borrow().iter() {
                windows.push(Rc::clone(&uiwindow.window));
            }

            // wait for events
            self.system.wait();

            // process the events
            self.system.flush(&windows);

            // redraw the dirty windows
            for uiwindow in self.uiwindows.borrow().iter() {
                if uiwindow.dirty.get() {
                    uiwindow.draw_widget();
                    uiwindow.dirty.set(false);
                }
            }

            // start updating uiwindows
            let mut uiwindows = self.uiwindows.borrow_mut();

            // remove dropped windows
            self.drop_uiwindows.borrow_mut().sort_unstable();
            for index in self.drop_uiwindows.borrow().iter().rev() {
                uiwindows.remove(*index);
            }
            self.drop_uiwindows.borrow_mut().clear();

            // append new windows
            uiwindows.append(&mut self.new_uiwindows.borrow_mut());
        }
    }
}

impl UIWindow {
    pub fn new_frame(ui: &Rc<UI>,r: Rect<i32>,title: &str,widget: &Rc<dyn Widget>) -> Result<Rc<UIWindow>,SystemError> {
        Ok(UIWindow {
            ui: Rc::clone(&ui),
            window: Rc::new(Window::new_frame(&ui.system,r,title)?),
            widget: Rc::clone(&widget),
            dirty: Cell::new(true),  
        }.register())
    }

    pub fn new_popup(ui: &Rc<UI>,r: Rect<i32>,parent: &Rc<UIWindow>,widget: &Rc<dyn Widget>) -> Result<Rc<UIWindow>,SystemError> {
        Ok(UIWindow {
            ui: Rc::clone(&ui),
            window: Rc::new(Window::new_popup(&ui.system,&parent.window,r)?),
            widget: Rc::clone(&widget),
            dirty: Cell::new(true),
        }.register())
    }

    fn register(self) -> Rc<UIWindow> {

        // make Rc of this UIWindow
        let rced_self = Rc::new(self);

        // clone the Rc for the handler closure
        let handler_rced_self = Rc::clone(&rced_self);

        // set handler closure to call dispatch_platform_event
        rced_self.window.set_handler(move |platform_event| handler_rced_self.dispatch_platform_event(platform_event));

        // append this UIWindow to the UI's new_uiwindows list
        rced_self.ui.new_uiwindows.borrow_mut().push(Rc::clone(&rced_self));

        rced_self
    }

    fn dispatch_platform_event(&self,platform_event: platform::Event) {
        match platform_event {
            platform::Event::KeyPress(k) => {
                self.widget.handle(&self.ui,&self.window,Event::KeyPress(k));
            },
            platform::Event::KeyRelease(k) => {
                self.widget.handle(&self.ui,&self.window,Event::KeyRelease(k));
            },
            platform::Event::MousePress(p,b) => {
                self.widget.handle(&self.ui,&self.window,Event::MousePress(p,b));
            },
            platform::Event::MouseRelease(p,b) => {
                self.widget.handle(&self.ui,&self.window,Event::MouseRelease(p,b));
            },
            platform::Event::MouseWheel(w) => {
                self.widget.handle(&self.ui,&self.window,Event::MouseWheel(w));
            },
            platform::Event::MouseMove(p) => {
                self.widget.handle(&self.ui,&self.window,Event::MouseMove(p));
            },
            platform::Event::Configure(r) => {
                self.window.r.set(rect!(vec2!(0,0),r.s()));  // TBD: would be nice if this happens in platform, and not here
                self.widget.set_rect(rect!(vec2!(0,0),r.s()));
            },
            platform::Event::Render => {
#[cfg(target_os="linux")]
                {
                    // In X11, window resizing is done in the main loop, so just
                    // collect the need to rerender in the dirty variable, and
                    // handle the actual rendering in UI::run()
                    self.dirty.set(true);
                }
#[cfg(target_os="windows")]
                {
                    // In Windows, window resizing uses its own loop, so
                    // DispatchMessage() doesn't return until the window is
                    // resized, but WndProc() gets called to redraw the window
                    // anyway, handle the drawing here
                    self.draw_widget();
                }
            },
            platform::Event::Close => {
                // TODO
            },
        }
    }

    fn draw_widget(&self) {

        // select and clear window to draw in
        self.ui.graphics.bind_target(&self.window);
        self.ui.graphics.clear(0xFF001122);

        // prepare draw context
        let draw = self.ui.draw.borrow();
        draw.set_window_size(self.window.r.get().s());
        draw.reset_offset();

        // draw the widget hierarchy
        self.widget.draw(&draw);

        // and flush and present the window
        self.ui.graphics.flush();
        self.ui.graphics.present(self.window.id);
    }
}

impl Drop for UIWindow {
    fn drop(&mut self) {
        // find this window in the UI's uiwindows list
        let uiwindows = self.ui.uiwindows.borrow();
        for i in 0..uiwindows.len() {
            let cur_uiwindow = &uiwindows[i];
            if Rc::as_ptr(cur_uiwindow) == self as *const UIWindow {
                // found, so add this window to the UI's drop_uiwindows list
                self.ui.drop_uiwindows.borrow_mut().push(i);
                break;
            }
        }
    }
}
