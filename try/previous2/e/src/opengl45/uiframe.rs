// e UI frame (OpenGL 4.5)
// by Desmond Germans, 2019

use std::fmt::*;
use crate::*;

pub const UIFRAME_ALLOCATED: u32 = 0x00000001;
pub const UIFRAME_VISIBLE: u32 = 0x00000002;

#[derive(Copy,Clone)]
#[repr(C)]
pub struct UIFrame {  // also used by geometry shader
    geometry: f32_4,  // frame geometry: x, y, width, height
    scroll: f32_4,    // frame scrolling: x offset, y offset, x scale, y scale
    fpuu: u32_4,      // flags (UIFRAME_*), parent (0 = root), -, -
}

pub const UIFRAME_SIZE: usize = 48;
pub const MAX_UIFRAMES: usize = 256;
impl UIFrame {
    pub fn new() -> UIFrame {
        UIFrame {
            geometry: f32_4 { x: 0.0,y: 0.0,z: 1.0,w: 1.0, },
            scroll: f32_4 { x: 0.0,y: 0.0,z: 1.0,w: 1.0, },
            fpuu: u32_4 { x: UIFRAME_VISIBLE | UIFRAME_ALLOCATED,y: 0,z: 0,w: 0, },
        }
    }

    pub fn geometry(&self) -> f32_r {
        f32_r::new(self.geometry.x,self.geometry.y,self.geometry.z,self.geometry.w)
    }

    pub fn offset(&self) -> f32_2 {
        f32_2::new(self.scroll.x,self.scroll.y)
    }

    pub fn zoom(&self) -> f32_2 {
        f32_2::new(self.scroll.z,self.scroll.w)
    }

    pub fn parent(&self) -> usize {
        self.fpuu.y as usize
    }

    pub fn allocated(&self) -> bool {
        (self.fpuu.x & UIFRAME_ALLOCATED) != 0
    }

    pub fn visible(&self) -> bool {
        (self.fpuu.x & UIFRAME_VISIBLE) != 0
    }

    pub fn set_geometry(&mut self,r: f32_r) {
        self.geometry.x = r.o.x as f32;
        self.geometry.y = r.o.y as f32;
        self.geometry.z = r.s.x as f32;
        self.geometry.w = r.s.y as f32;
    }

    pub fn set_offset(&mut self,p: f32_2) {
        self.scroll.x = p.x;
        self.scroll.y = p.y;
    }

    pub fn set_zoom(&mut self,p: f32_2) {
        self.scroll.z = p.x;
        self.scroll.w = p.y;
    }

    pub fn set_parent(&mut self,p: usize) {
        self.fpuu.y = p as u32;
    }

    pub fn set_allocated(&mut self,v: bool) {
        if v {
            self.fpuu.x |= UIFRAME_ALLOCATED;
        }
        else {
            self.fpuu.x &= !UIFRAME_ALLOCATED;
        } 
    }

    pub fn set_visible(&mut self,v: bool) {
        if v {
            self.fpuu.x |= UIFRAME_VISIBLE;
        }
        else {
            self.fpuu.x &= !UIFRAME_VISIBLE;
        }
    }

    pub fn localize(&self,p: &mut f32_2) {
        p.x -= self.geometry().o.x;
        p.y -= self.geometry().o.y;
        p.x -= self.offset().x;
        p.y -= self.offset().y;
        p.x /= self.zoom().x;
        p.y /= self.zoom().y;
    }

    pub fn globalize(&self,p: &mut f32_2) {
        p.x *= self.zoom().x;
        p.y *= self.zoom().y;
        p.x += self.offset().x;
        p.y += self.offset().y;
        p.x += self.geometry().o.x;
        p.y += self.geometry().o.y;
    }
}

impl Display for UIFrame {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"(geometry: {}, offset: {}, zoom: {}, parent: {}, visible: {}, allocated: {})",self.geometry(),self.offset(),self.zoom(),self.parent(),self.visible(),self.allocated())
    }
}
