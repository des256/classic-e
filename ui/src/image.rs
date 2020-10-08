// E - UI - Image
// Desmond Germans, 2020

// An image is just that, an image.

use{
    crate::*,
    std::cell::Cell,
};

/// Image.
pub struct Image {
    r: Cell<Rect<i32>>,
    image: Texture2D<pixel::ARGB8>,
}

impl Image {
    pub fn new(graphics: &Graphics,image: Mat<pixel::ARGB8>) -> Result<Image,SystemError> {
        Ok(Image {
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

    fn calc_min_size(&self,draw: &Draw) -> Vec2<i32> {
        let size = self.image.size();
        vec2!(size.x() as i32,size.y() as i32)
    }

    fn draw(&self,draw: &Draw) {
        draw.draw_texture(vec2!(0,0),&self.image,BlendMode::Replace);
    }

    fn handle(&self,ui: &UI,window: &Window,draw: &Draw,event: Event) {
    }
}
