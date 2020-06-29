use crate::ui::Vec2;
use crate::ui::View;
use crate::ui::Widget;

pub struct Row<'a> {
    view: View,
    children: Vec<&'a mut Widget>,
}

impl<'a> Row<'a> {
    pub fn new() -> Row<'a> {
        Row {
            view: View::new(0.0,0.0,1.0,1.0),
            children: Vec::new(),
        }
    }

    pub fn push(&mut self,child: &'a mut Widget) {
        self.children.push(child);
        self.build();
    }
}

pub struct Column<'a> {
    view: View,
    children: Vec<&'a mut Widget>,
}

impl<'a> Column<'a> {
    pub fn new() -> Column<'a> {
        Column {
            view: View::new(0.0,0.0,1.0,1.0),
            children: Vec::new(),
        }
    }

    pub fn push(&mut self,child: &'a mut Widget) {
        self.children.push(child);
        self.build();
    }
}

impl<'a> Widget for Row<'a> {
    fn movesize(&mut self,x: f32,y: f32,xs: f32,ys: f32) {
        self.view.movesize(x,y,xs,ys);
    }

    fn minsize(&self) -> Vec2 {
        let mut xs: f32 = 0.0;
        let mut ys: f32 = 0.0;
        for child in &self.children {
            let ms = child.minsize();
            xs += ms.x;
            if ms.y > ys {
                ys = ms.y;
            }
        }
        Vec2 { x: xs, y: ys }
    }

    fn draw(&self) {
        self.view.draw();
    }

    fn build(&mut self) {
        let mut x: f32 = 0.0;
        for child in &mut self.children {
            let ms = child.minsize();
            child.movesize(x,0.0,ms.x,ms.y);
            x += ms.x;
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

impl<'a> Widget for Column<'a> {
    fn movesize(&mut self,x: f32, y: f32, xs: f32, ys: f32) {
        self.view.movesize(x,y,xs,ys);
    }

    fn minsize(&self) -> Vec2 {
        let mut xs: f32 = 0.0;
        let mut ys: f32 = 0.0;
        for child in &self.children {
            let ms = child.minsize();
            xs += ms.x;
            if ms.y > ys {
                ys = ms.y;
            }
        }
        Vec2 { x: xs, y: ys }
    }

    fn draw(&self) {
        println!("Column::draw");
        self.view.draw();
    }

    fn build(&mut self) {
        let mut y: f32 = 0.0;
        for child in &mut self.children {
            let ms = child.minsize();
            child.movesize(0.0,y,ms.x,ms.y);
            y += ms.y;
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
