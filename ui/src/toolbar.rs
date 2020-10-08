// E - UI - ToolBar
// Desmond Germans, 2020

// A toolbar is a bar with clickable icons, often connected to actions.

use{
    crate::*,
    std::cell::Cell,
};

#[derive(Copy,Clone)]
pub enum ToolBarHit {
    Nothing,
    Item(usize),
}

pub enum ToolBarItem {
    Action(Texture2D<pixel::ARGB8>),
    Separator,
}

/// Tool bar.
pub struct ToolBar {
    r: Cell<Rect<i32>>,
    hit: Cell<ToolBarHit>,
    items: Vec<ToolBarItem>,
    pressed: Cell<Option<usize>>,
}

const TOOLBAR_SEPARATOR_WIDTH: i32 = 10;

impl ToolBar {
    pub fn new(items: Vec<ToolBarItem>) -> Result<ToolBar,SystemError> {
        Ok(ToolBar {
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(ToolBarHit::Nothing),
            items: items,
            pressed: Cell::new(None),
        })
    }

    pub fn find_hit(&self,draw: &Draw,p: Vec2<i32>) -> ToolBarHit {
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

    fn calc_min_size(&self,_draw: &Draw) -> Vec2<i32> {
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

    fn draw(&self,draw: &Draw) {
        let mut r = rect!(0i32,0i32,0i32,self.r.get().sy());
        for i in 0..self.items.len() {
            let item = &self.items[i];
            match item {
                ToolBarItem::Action(texture) => {
                    let size = texture.size();
                    let size = vec2!(size.x() as i32,size.y() as i32);
                    r.set_sx(size.x());
                    draw.draw_texture(r.o(),texture,BlendMode::Replace);
                    r.set_ox(r.ox() + size.x());
                },
                ToolBarItem::Separator => {
                    r.set_ox(r.ox() + TOOLBAR_SEPARATOR_WIDTH);
                },
            }
        }
    }

    fn handle(&self,ui: &UI,window: &Window,draw: &Draw,event: Event) {
        match event {
            Event::MousePress(p,b) => {

            },
            Event::MouseRelease(p,b) => {

            },
            Event::MouseMove(p) => {
                self.hit.set(self.find_hit(draw,p));
            },
            _ => { },
        }
    }
}
