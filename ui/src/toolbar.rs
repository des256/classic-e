// E - UI - ToolBar
// Desmond Germans, 2020

// A toolbar is a bar with clickable icons, often connected to actions.

use{
    crate::*,
    std::{
        cell::{
            Cell,
            RefCell,
        },
        rc::Rc,
    },
};

#[doc(hidden)]
#[derive(Copy,Clone,Debug)]
pub enum ToolBarHit {
    Nothing,
    Item(usize),
}

/// Tool bar style.
pub struct ToolBarStyle {
    pub item_text_color: u32,
    pub item_color: u32,
    pub item_hover_color: u32,
}

/// Tool bar item.
pub enum ToolBarItem {
    Action(Texture2D<pixel::ARGB8>),
    Separator,
}

/// Tool bar.
pub struct ToolBar {
    ui: Rc<UI>,
    style: RefCell<ToolBarStyle>,
    r: Cell<Rect<i32>>,
    hit: Cell<ToolBarHit>,
    capturing: Cell<bool>,
    items: Vec<ToolBarItem>,
    pressed: Cell<Option<usize>>,
}

const TOOLBAR_SEPARATOR_WIDTH: i32 = 10;

impl ToolBar {
    pub fn new(ui: &Rc<UI>,items: Vec<ToolBarItem>) -> Result<ToolBar,SystemError> {
        Ok(ToolBar {
            ui: Rc::clone(&ui),
            style: RefCell::new(ToolBarStyle {
                item_text_color: 0xAAAAAA,
                item_color: 0x444444,
                item_hover_color: 0x224488,
            }),
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(ToolBarHit::Nothing),
            capturing: Cell::new(false),
            items: items,
            pressed: Cell::new(None),
        })
    }

    pub fn find_hit(&self,p: Vec2<i32>) -> ToolBarHit {
        let mut r = rect!(0i32,0i32,0i32,0i32);
        for i in 0..self.items.len() {
            let item = &self.items[i];
            match item {
                ToolBarItem::Action(texture) => {
                    let size = texture.size();
                    let size = vec2!(size.x() as i32,size.y() as i32);
                    r.set_s(size);
                    if r.contains(&p) {
                        return ToolBarHit::Item(i);
                    }        
                    r.set_ox(r.ox() + size.x());
                },
                ToolBarItem::Separator => {
                    r.set_ox(r.ox() + TOOLBAR_SEPARATOR_WIDTH);
                },
            }
        }
        ToolBarHit::Nothing
    }
}

impl Widget for ToolBar {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        let mut total_size = vec2!(0i32,0i32);
        for item in self.items.iter() {
            match item {
                ToolBarItem::Action(mat) => {
                    let size = mat.size();
                    let size = vec2!(size.x() as i32,size.y() as i32);
                    total_size += vec2!(size.x(),0);
                    if size.y() > total_size.y() {
                        total_size.set_y(size.y());
                    }
                },
                ToolBarItem::Separator => {
                    total_size += vec2!(TOOLBAR_SEPARATOR_WIDTH,0);
                },
            }
        }

        total_size
    }

    fn draw(&self) {
        let style = self.style.borrow();
        let mut r = rect!(0i32,0i32,0i32,self.r.get().sy());
        for i in 0..self.items.len() {
            let item = &self.items[i];
            let color = if let ToolBarHit::Item(n) = self.hit.get() {
                if n == i {
                    style.item_hover_color
                }
                else {
                    style.item_color
                }
            }
            else {
                style.item_color
            };
            match item {
                ToolBarItem::Action(texture) => {
                    let size = texture.size();
                    let size = vec2!(size.x() as i32,size.y() as i32);
                    r.set_sx(size.x());
                    self.ui.draw_rectangle(r,color,BlendMode::Replace);
                    self.ui.draw_texture(r.o(),texture,BlendMode::Over);
                    r.set_ox(r.ox() + size.x());
                },
                ToolBarItem::Separator => {
                    r.set_sx(TOOLBAR_SEPARATOR_WIDTH);
                    self.ui.draw_rectangle(r,style.item_color,BlendMode::Replace);
                    r.set_ox(r.ox() + TOOLBAR_SEPARATOR_WIDTH);
                },
            }
        }
        r.set_sx(self.r.get().sx() - r.ox());
        if r.sx() > 0 {
            self.ui.draw_rectangle(r,style.item_color,BlendMode::Replace);
        }
    }

    fn keypress(&self,ui: &UI,window: &Window,k: u8) {
    }

    fn keyrelease(&self,ui: &UI,window: &Window,k: u8) {
    }

    fn mousepress(&self,ui: &UI,window: &Window,p: Vec2<i32>,b: MouseButton) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                ToolBarHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                ToolBarHit::Item(n) => {
                    true
                },
            }
        }
        else {
            match self.hit.get() {
                ToolBarHit::Nothing => {
                    false
                },
                ToolBarHit::Item(n) => {
                    println!("ToolBar: start clicking on item {}",n);
                    self.capturing.set(true);
                    true
                },
            }
        }
    }

    fn mouserelease(&self,ui: &UI,window: &Window,p: Vec2<i32>,b: MouseButton) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                ToolBarHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                ToolBarHit::Item(n) => {
                    println!("ToolBar: stop clicking on item {}",n);
                    self.capturing.set(false);
                    self.mousemove(ui,window,p)
                }
            }
        }
        else {
            match self.hit.get() {
                ToolBarHit::Nothing => {
                    false
                },
                ToolBarHit::Item(n) => {
                    false
                },
            }
        }
    }

    fn mousemove(&self,ui: &UI,window: &Window,p: Vec2<i32>) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                ToolBarHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                ToolBarHit::Item(n) => {
                    println!("ToolBar: still clicking on item {}",n);
                    true
                },
            }
        }
        else {
            self.hit.set(self.find_hit(p));
            match self.hit.get() {
                ToolBarHit::Nothing => {
                    false
                },
                ToolBarHit::Item(n) => {
                    true
                },
            }
        }
    }

    fn mousewheel(&self,ui: &UI,window: &Window,w: MouseWheel) -> bool {
        false
    }
}
