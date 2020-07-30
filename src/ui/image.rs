// E - UI - Image
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

/// Image widget.
pub struct Image {
    _ui: Rc<ui::UI>,
    padding: Cell<Vec2<f32>>,
    tex: RefCell<Rc<gpu::Texture2D<pixel::ARGB8>>>,
}

impl Image {

    /// Create new image widget.
    /// # Arguments
    /// * `ui` - UI context for this widget.
    /// * `tex` - Texture containing image.
    /// # Returns
    /// * `Ok(image)` - The image widget.
    /// * `Err(_)` - The image widget could not be created.
    pub fn new(ui: &Rc<ui::UI>,tex: &Rc<gpu::Texture2D<pixel::ARGB8>>) -> Result<Image,SystemError> {
        Ok(Image {
            _ui: Rc::clone(ui),
            padding: Cell::new(vec2!(0.0,0.0)),
            tex: RefCell::new(Rc::clone(tex)),
        })
    }

    /// Set padding of the text widget.
    /// # Arguments
    /// * `padding` - New padding.
    pub fn set_padding(&self,padding: Vec2<f32>) {
        self.padding.set(padding);
    }
}

impl ui::Widget for Image {
    
    fn draw(&self,dc: &Rc<ui::DC>,space: Rect<f32>) {
        dc.draw(space.o + self.padding.get(),&self.tex.borrow());
    }

    fn measure(&self) -> Vec2<f32> {
        let size = self.tex.borrow().size;
        vec2!(size.x as f32,size.y as f32)
    }
}
