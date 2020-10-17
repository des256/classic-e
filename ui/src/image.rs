// E - UI - Image
// Desmond Germans, 2020

// An image is just that, an image.

use{
    crate::*,
    std::{
        cell::Cell,
        rc::Rc,
    },
};

/// Image.
pub struct Image {
    ui: Rc<UI>,
    r: Cell<Rect<i32>>,
    image: Texture2D<pixel::ARGB8>,
}

impl Image {
    pub fn new(ui: &Rc<UI>,graphics: &Rc<Graphics>,image: Mat<pixel::ARGB8>) -> Result<Rc<Image>,SystemError> {
        Ok(Rc::new(Image {
            ui: Rc::clone(&ui),
            r: Cell::new(rect!(0,0,0,0)),
            image: Texture2D::new_from_mat(&graphics,image)?,
        }))
    }
}

impl Widget for Image {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        let size = self.image.size;
        vec2!(size.x as i32,size.y as i32)
    }

    fn draw(&self) {
        self.ui.draw_texture(vec2!(0,0),&self.image,BlendMode::Replace);
    }

    fn keypress(&self,_ui: &UI,_window: &Rc<UIWindow>,_k: u8) {
    }

    fn keyrelease(&self,_ui: &UI,_window: &Rc<UIWindow>,_k: u8) {
    }

    fn mousepress(&self,_ui: &UI,_window: &Rc<UIWindow>,_p: Vec2<i32>,_b: MouseButton) -> bool {
        false
    }

    fn mouserelease(&self,_ui: &UI,_window: &Rc<UIWindow>,_p: Vec2<i32>,_b: MouseButton) -> bool {
        false
    }

    fn mousemove(&self,_ui: &UI,_window: &Rc<UIWindow>,_p: Vec2<i32>) -> bool {
        false
    }

    fn mousewheel(&self,_ui: &UI,_window: &Rc<UIWindow>,_w: MouseWheel) -> bool {
        false
    }
}
