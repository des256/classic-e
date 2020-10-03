// E - UI - Image
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::Cell,
};

/// Image widget.
pub struct Image {
    state: Rc<UIState>,
    r: Cell<Rect<i32>>,
    pub texture: Texture2D<pixel::ARGB8>,
    pub padding: Vec2<i32>,
}

impl Image {
    pub fn new(state: &Rc<UIState>,mat: Mat<pixel::ARGB8>) -> Result<Image,SystemError> {
        Ok(Image {
            state: Rc::clone(state),
            r: Cell::new(Rect::<i32>::zero()),
            texture: Texture2D::<pixel::ARGB8>::new_from_mat(&state.graphics,mat)?,
            padding: Vec2::<i32>::zero(),
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
        let size = self.texture.size();
        vec2!(size.x() as i32,size.y() as i32) + 2 * self.padding
    }

    fn draw(&self) {
        self.state.draw_texture(self.r.get().o(),&self.texture,BlendMode::Replace);
    }

    fn handle_mouse_press(&self,_p: Vec2<i32>,_b: MouseButton) {
    }

    fn handle_mouse_release(&self,_p: Vec2<i32>,_b: MouseButton) {
    }

    fn handle_mouse_move(&self,_p: Vec2<i32>) {
    }

    fn handle_mouse_wheel(&self,_w: MouseWheel) {
    }
}