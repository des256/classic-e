// E - UI - Atlassed
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::RefCell,
};

pub struct Texture2DArrayAtlas<T> where T: gpu::GLFormat {
    pub array: gpu::Texture2DArray<T>,
    occ: RefCell<Vec<Mat<bool>>>,
}

impl<T: gpu::GLFormat> Texture2DArrayAtlas<T> {
    pub fn new(graphics: &Rc<gpu::Graphics>,size: Vec2<usize>) -> Result<Texture2DArrayAtlas<T>,SystemError> {
        Ok(Texture2DArrayAtlas::<T> {
            array: gpu::Texture2DArray::<T>::new(graphics,vec3!(size.x,size.y,1))?,
            occ: RefCell::new(vec![Mat::new(vec2!(size.x,size.y))]),
        })
    }
}

pub struct Texture2DSub<T: gpu::GLFormat> {
    pub atlas: Rc<Texture2DArrayAtlas<T>>,
    pub layer: usize,
    pub r: Rect<usize>,
}

impl<T: gpu::GLFormat> Texture2DSub<T> {
    pub fn new(atlas: &Rc<Texture2DArrayAtlas<T>>,size: Vec2<usize>) -> Result<Texture2DSub<T>,SystemError> {
        for layer in 0..atlas.array.size.z {
            for y in 0..atlas.array.size.y {
                for x in 0..atlas.array.size.x {
                    let mut occupied = false;
                    for yp in y..y + size.y {
                        for xp in x..x + size.x {
                            if atlas.occ.borrow()[layer].get(vec2!(xp,yp)) {
                                occupied = true;
                                break;
                            }
                        }
                        if occupied {
                            break;
                        }
                    }
                    if !occupied {
                        for yp in y..y + size.y {
                            for xp in x..x + size.x {
                                atlas.occ.borrow_mut()[layer].set(vec2!(xp,yp),true);
                            }
                        }
                        return Ok(Texture2DSub {
                            atlas: Rc::clone(atlas),
                            layer: layer,
                            r: rect!(x,y,size.x,size.y),
                        });
                    }
                }
            }
        }
        // TODO: append new layer to array and try there
        Err(SystemError::Generic)
    }

    pub fn new_from_mat(atlas: &Rc<Texture2DArrayAtlas<T>>,mat: &Mat<T>) -> Result<Texture2DSub<T>,SystemError> {
        let texture = Texture2DSub::new(atlas,mat.size)?;
        atlas.array.load_mat(texture.layer,texture.r.o,mat);
        Ok(texture)
    }
}

impl<T: gpu::GLFormat> Drop for Texture2DSub<T> {
    fn drop(&mut self) {
        for yp in self.r.o.y..(self.r.o.y + self.r.s.y) {
            for xp in self.r.o.x..(self.r.o.x + self.r.s.x) {
                self.atlas.occ.borrow_mut()[self.layer].set(vec2!(xp,yp),false);
            }
        }
    }
}