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
    pub fn new(ui: &Rc<UI>,graphics: &Graphics,image: Mat<pixel::ARGB8>) -> Result<Image,SystemError> {
        Ok(Image {
            ui: Rc::clone(&ui),
            r: Cell::new(rect!(0,0,0,0)),
            image: graphics.create_texture2d_from_mat(image)?,
        })
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
        let size = self.image.size();
        vec2!(size.x() as i32,size.y() as i32)
    }

    fn draw(&self) {
        self.ui.draw_texture(vec2!(0,0),&self.image,BlendMode::Replace);
    }

    fn keypress(&self,ui: &UI,window: &Window,k: u8) {
    }

    fn keyrelease(&self,ui: &UI,window: &Window,k: u8) {
    }

    fn mousepress(&self,ui: &UI,window: &Window,p: Vec2<i32>,b: MouseButton) -> bool {
        false
    }

    fn mouserelease(&self,ui: &UI,window: &Window,p: Vec2<i32>,b: MouseButton) -> bool {
        false
    }

    fn mousemove(&self,ui: &UI,window: &Window,p: Vec2<i32>) -> bool {
        false
    }

    fn mousewheel(&self,ui: &UI,window: &Window,w: MouseWheel) -> bool {
        false
    }
}
