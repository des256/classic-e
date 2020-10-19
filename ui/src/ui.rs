// E - UI
// Desmond Germans, 2020

// The UI manages the various open windows of the interface.

use crate::*;
use std::{
    cell::{
        Cell,
        RefCell,
    },
    rc::Rc,
};

/// UI Widget window connection.
pub struct UIWindow {
    ui: Rc<UI>,
    pub(crate) window: Rc<Window>,
    widget: Rc<dyn Widget>,
    capturing: Cell<bool>,
}

/// UI context.
pub struct UI {
    system: Rc<System>,
    graphics: Rc<Graphics>,
    pub(crate) draw: Rc<Draw>,
    _proto_sans: Rc<FontProto>,
    _proto_serif: Rc<FontProto>,
    _proto_mono: Rc<FontProto>,
    pub(crate) font: Rc<Font>,
    uiwindows: RefCell<Vec<Rc<UIWindow>>>,
    new_uiwindows: RefCell<Vec<Rc<UIWindow>>>,
    drop_uiwindows: RefCell<Vec<usize>>,
    _current_capturing_id: Cell<Option<u64>>,  // window that is currently capturing the mouse (TBD)
    running: Cell<bool>,
}

impl UI {
    /// Create new UI context.
    ///
    /// **Arguments**
    ///
    /// * `system` - System context.
    /// * `graphics` - Graphics context.
    /// * `font_path` - Path to read font prototypes from.
    ///
    /// **Returns**
    ///
    /// New UI context.
    pub fn new(system: &Rc<System>,graphics: &Rc<Graphics>,font_path: &str) -> Result<Rc<UI>,SystemError> {

        // create drawing context
        let draw = Draw::new(&graphics)?;

        // font prototypes
        let proto_sans = Rc::new(
            FontProto::new(
                graphics,
                &format!("{}/sans.fnt",font_path)
            )?
        );
        let proto_serif = Rc::new(
            FontProto::new(
                graphics,
                &format!("{}/serif.fnt",font_path)
            )?
        );
        let proto_mono = Rc::new(
            FontProto::new(
                graphics,
                &format!("{}/mono.fnt",font_path)
            )?
        );

        // the main font for now
        let font = Rc::new(Font::new(&proto_sans,16)?);

        Ok(Rc::new(UI {
            system: Rc::clone(system),
            graphics: Rc::clone(graphics),
            draw: draw,
            _proto_sans: proto_sans,
            _proto_serif: proto_serif,
            _proto_mono: proto_mono,
            font: font,
            uiwindows: RefCell::new(Vec::new()),
            new_uiwindows: RefCell::new(Vec::new()),
            drop_uiwindows: RefCell::new(Vec::new()),
            _current_capturing_id: Cell::new(None),
            running: Cell::new(true),
        }))
    }

    /// Terminate UI main loop.
    pub fn terminate(&self) {
        self.running.set(false);
    }

    /// UI main loop.
    pub fn run(&self) {
        self.running.set(true);
        while self.running.get() {

            // wait for events
            self.system.wait();

            // process the events
            self.system.flush();

            // redraw all windows
            for uiwindow in self.uiwindows.borrow().iter() {
                uiwindow.draw_widget();
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
    fn register(self) -> Rc<UIWindow> {
        let rced_self = Rc::new(self);
        let handler_rced_self = Rc::clone(&rced_self);
        rced_self.window.set_handler(move |platform_event| UIWindow::dispatch_platform_event(&handler_rced_self,platform_event));
        rced_self.widget.set_rect(rced_self.window.r.get());
        rced_self.ui.new_uiwindows.borrow_mut().push(Rc::clone(&rced_self));
        rced_self
    }

    /// Open new frame window with widget.
    ///
    /// **Arguments**
    ///
    /// * `ui` - UI context.
    /// * `r` - Initial frame window rectangle.
    /// * `title` - Title of the frame window.
    /// * `widget` - Widget for the frame window.
    ///
    /// **Returns**
    ///
    /// New frame UI window.
    pub fn new_frame(ui: &Rc<UI>,r: Rect<i32>,title: &str,widget: Rc<dyn Widget>) -> Result<Rc<UIWindow>,SystemError> {
        Ok(UIWindow {
            ui: Rc::clone(&ui),
            window: Window::new_frame(&ui.system,r,title)?,
            widget: widget,
            capturing: Cell::new(false),
        }.register())
    }

    /// Open new popup window with widget.
    ///
    /// **Arguments**
    ///
    /// * `ui` - UI context.
    /// * `r` - Initial popup window rectangle.
    /// * `widget` - Widget for the popup window.
    ///
    /// **Returns**
    ///
    /// New popup UI window.
    pub fn new_popup(ui: &Rc<UI>,r: Rect<i32>,widget: Rc<dyn Widget>) -> Result<Rc<UIWindow>,SystemError> {
        Ok(UIWindow {
            ui: Rc::clone(&ui),
            window: Window::new_popup(&ui.system,r)?,
            widget: widget,
            capturing: Cell::new(false),
        }.register())
    }

    /// Close the window.
    ///
    /// This unregisters the window from the UI context, allowing Rust to hide and drop the window.
    pub fn close(&self) {
        self.window.clear_handler();
        let uiwindows = self.ui.uiwindows.borrow();
        for i in 0..uiwindows.len() {
            let cur_uiwindow = &uiwindows[i];
            if Rc::as_ptr(cur_uiwindow) == self as *const UIWindow {
                self.ui.drop_uiwindows.borrow_mut().push(i);
                break;
            }
        }
    }
    
    /// Set window rectangle.
    ///
    /// **Arguments**
    ///
    /// * `r` - New window rectangle.
    pub fn set_rect(&self,r: Rect<i32>) {
        self.window.set_rect(&r);
    }

    /// Show the window.
    pub fn show(&self) {
        self.window.show();
    }

    /// Hide the window.
    pub fn hide(&self) {
        self.window.hide();
    }

    fn dispatch_platform_event(window: &Rc<UIWindow>,platform_event: platform::Event) {
        let mut new_capturing = false;
        match platform_event {
            platform::Event::KeyPress(k) => {
                window.widget.keypress(&window.ui,window,k);
            },
            platform::Event::KeyRelease(k) => {
                window.widget.keyrelease(&window.ui,window,k);
            },
            platform::Event::MousePress(p,b) => {
                new_capturing = window.widget.mousepress(&window.ui,&window,p,b);
            },
            platform::Event::MouseRelease(p,b) => {
                new_capturing = window.widget.mouserelease(&window.ui,&window,p,b);
            },
            platform::Event::MouseWheel(w) => {
                new_capturing = window.widget.mousewheel(&window.ui,&window,w);
            },
            platform::Event::MouseMove(p) => {
                new_capturing = window.widget.mousemove(&window.ui,&window,p);
            },
            platform::Event::Configure(r) => {
                window.widget.set_rect(r);
            },
            platform::Event::Render => {
#[cfg(target_os="linux")]
                {
                    // In X11, window resizing is done in the main loop, so just
                    // collect the need to rerender in the dirty variable, and
                    // handle the actual rendering in UI::run()
                }
#[cfg(target_os="windows")]
                {
                    // In Windows, window resizing uses its own loop, so
                    // DispatchMessage() doesn't return until the window is
                    // resized, but WndProc() gets called to redraw the window
                    // anyway, handle the drawing here
                    window.draw_widget();
                }
            },
            platform::Event::Close => {
                // TODO
            },
        }

        // capture the mouse in this window
        if new_capturing != window.capturing.get() {
            if new_capturing {
                window.ui.system.capture_mouse(window.window.id);
            }
            else {
                window.ui.system.release_mouse();
            }
            window.capturing.set(new_capturing);
        }
    }

    fn draw_widget(&self) {

        // select and clear window to draw in
        self.ui.graphics.bind_target(&self.window);
        self.ui.graphics.clear(0xFF001122);

        // prepare draw context
        self.ui.draw.set_window_size(self.window.r.get().s);
        self.ui.draw.reset_offset();

        // draw the widget hierarchy
        self.widget.draw();

        // and flush and present the window
        self.ui.graphics.flush();
        self.ui.graphics.present(self.window.id);
    }
}
