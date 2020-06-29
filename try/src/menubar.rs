use crate::ui::Vec2;
use crate::ui::Widget;
use crate::ui::View;
use crate::ui::Font;

pub struct Item<'a> {
    name: &'a str,
}

pub struct MenuBar<'a> {
    view: View,
    items: Vec<Item<'a>>,
}

const MENUBAR_FONT: &Font = &crate::ui::SANS;
const ITEM_PADDING_X: f32 = 0.1;
const ITEM_PADDING_Y: f32 = 0.01;

impl<'a> Widget for MenuBar<'a> {
    fn movesize(&mut self,x: f32, y: f32, xs: f32, ys: f32) {
        self.view.movesize(x,y,xs,ys);
    }

    fn minsize(&self) -> Vec2 {
        let mut total_x: f32 = 0.0;
        let mut total_y: f32 = 0.0;
        for item in &self.items {
            total_x += ITEM_PADDING_X + MENUBAR_FONT.advance_kerning(item.name) + ITEM_PADDING_X;
            let height = ITEM_PADDING_Y + MENUBAR_FONT.height() + ITEM_PADDING_Y;
            if height > total_y {
                total_y = height;
            }
        }
        Vec2 {
            x: total_x,
            y: total_y,
        }
    }

    fn draw(&self) {
        println!("MenuBar::draw");
        self.view.draw();
    }

    fn build(&mut self) {
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        self.view.quads.clear();
        for item in &self.items {
            let width = ITEM_PADDING_X + MENUBAR_FONT.advance_kerning(item.name) + ITEM_PADDING_X;
            let height = ITEM_PADDING_Y + MENUBAR_FONT.height() + ITEM_PADDING_Y;
            self.view.push(x,0.0,width,height,0.0,0.0,0.0,0.0,0xFFFF0000,1,0);
            // TODO: add characters
            x += width;
        }
    }

    fn keypress(&self,_k: u32) {
    }

    fn keyrelease(&self,_k: u32) {
    }

    fn mousepress(&self,_x: f32,_y: f32,_b: u32) {
    }

    fn mouserelease(&self,_x: f32,_y: f32,_b: u32) {
    }

    fn mousemove(&self,_x: f32,_y: f32) {
    }

    fn mousewheel(&self,_dx: f32,_dy: f32) {
    }
}

impl<'a> MenuBar<'a> {
    pub fn new() -> MenuBar<'a> {
        MenuBar {
            view: View::new(0.0,0.0,1.0,1.0),
            items: Vec::new(),
        }
    }

    pub fn push(&mut self,name: &'a str) {
        self.items.push(Item { name: name });
        self.build();
    }
}
