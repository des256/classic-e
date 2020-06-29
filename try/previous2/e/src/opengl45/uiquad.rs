// e UI quad (OpenGL 4.5)
// by Desmond Germans, 2019

use std::fmt::*;
use crate::*;

#[derive(Copy,Clone)]
#[repr(C)]
pub struct UIQuad {  // also used by vertex shader
    pub geo: f32_4,  // the geometry: x, y, width, height
    pub tex: f32_4,  // texture coordinates: u, v, u width, v height
    col: u32,    // color
    pub fmlf: u16_4, // frame, mode, layer, flags
}

pub const UIQUAD_ALLOCATED: u16 = 0x0001;

pub const UIQUAD_SIZE: usize = 44;

pub enum QuadState {
    Color(u32),
    Texture(u8,f32_r),
    Modulate(u32,u8,f32_r),
    MSDF(u32,u8,f32_r),
}

impl UIQuad {
    pub fn new() -> UIQuad {
        UIQuad {
            geo: f32_4 { x: 0.0,y: 0.0,z: 1.0,w: 1.0, },
            tex: f32_4 { x: 0.0,y: 0.0,z: 1.0,w: 1.0, },
            col: 0xFFFF0000,
            fmlf: u16_4 { x: 0,y: 0,z: 0,w: UIQUAD_ALLOCATED, },
        }
    }

    pub fn geometry(&self) -> f32_4 {
        self.geo
    }

    pub fn state(&self) -> QuadState {
        if self.fmlf.y == 0 {
            QuadState::Color(self.col)
        }
        else if self.fmlf.y == 1 {
            QuadState::Texture(self.fmlf.z as u8,f32_r::new(self.tex.x,self.tex.y,self.tex.z,self.tex.w))
        }
        else if self.fmlf.y == 2 {
            QuadState::Modulate(self.col,self.fmlf.z as u8,f32_r::new(self.tex.x,self.tex.y,self.tex.z,self.tex.w))
        }
        else {
            QuadState::MSDF(self.col,self.fmlf.z as u8,f32_r::new(self.tex.x,self.tex.y,self.tex.z,self.tex.w))
        }
    }

    pub fn frame(&self) -> usize {
        self.fmlf.x as usize
    }

    pub fn allocated(&self) -> bool {
        (self.fmlf.w & UIQUAD_ALLOCATED) != 0
    }

    pub fn set_geometry(&mut self,r: f32_r) {
        self.geo.x = r.o.x;
        self.geo.y = r.o.y;
        self.geo.z = r.s.x;
        self.geo.w = r.s.y;
    }

    pub fn set_state(&mut self,s: QuadState) {
        match s {
            QuadState::Color(col) => {
                self.fmlf.y = 0;
                self.col = col;
            },
            QuadState::Texture(l,tex) => {
                self.fmlf.y = 1;
                self.fmlf.z = l as u16;
                self.tex = f32_4::new(tex.o.x,tex.o.y,tex.s.x,tex.s.y);
            },
            QuadState::Modulate(col,l,tex) => {
                self.fmlf.y = 2;
                self.col = col;
                self.fmlf.z = l as u16;
                self.tex = f32_4::new(tex.o.x,tex.o.y,tex.s.x,tex.s.y);
            },
            QuadState::MSDF(col,l,tex) => {
                self.fmlf.y = 3;
                self.col = col;
                self.fmlf.z = l as u16;
                self.tex = f32_4::new(tex.o.x,tex.o.y,tex.s.x,tex.s.y);
            }
        }
    }

    pub fn set_frame(&mut self,f: usize) {
        self.fmlf.x = f as u16;
    }

    pub fn set_allocated(&mut self,v: bool) {
        if v {
            self.fmlf.w |= UIQUAD_ALLOCATED;
        }
        else {
            self.fmlf.w &= !UIQUAD_ALLOCATED;
        }
    }
}

impl Display for QuadState {
    fn fmt(&self,f: &mut Formatter) -> Result {
        match self {
            QuadState::Color(col) => {
                write!(f,"(color {:08X})",col)
            },
            QuadState::Texture(l,tex) => {
                write!(f,"(texture {} {})",l,tex)
            },
            QuadState::Modulate(col,l,tex) => {
                write!(f,"(modulate {:08X} {} {})",col,l,tex)
            },
            QuadState::MSDF(col,l,tex) => {
                write!(f,"(MSDF {:08X} {} {})",col,l,tex)
            }
        }
    }
}

impl Display for UIQuad {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"(geometry: {}, state: {}, frame: {}, allocated: {}",self.geometry(),self.state(),self.frame(),self.allocated())
    }
}
