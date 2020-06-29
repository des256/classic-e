// e button
// by Desmond Germans, 2019

use crate::*;

pub enum ButtonHit {
    None,
    Button,
}

pub struct Button {
    outer_fid: usize,
    inner_fid: usize,
    _text: String,
    pressed: bool,
}

impl Button {
    pub fn new(ui: &mut UI,wid: usize,quads: &mut Vec<UIQuad>,p: f32_2,text: &str) -> Button {

        // allocate frames
        let outer_fid = ui.alloc_frame(wid).unwrap();
        let inner_fid = ui.alloc_frame(wid).unwrap();

        // background quad (setup later)
        let bgqid = quads.len();
        quads.push(UIQuad::new());

        // character quads
        let max = ui.font.push_text(quads,inner_fid,&text);

        // update frames
        let mut frame = UIFrame::new();
        frame.set_geometry(f32_r::new(p.x,p.y,max.x,max.y));
        frame.set_parent(0);
        ui.set_frame(wid,outer_fid,&frame);
        let mut frame = UIFrame::new();
        frame.set_geometry(f32_r::new(0.0,0.0,max.x,max.y));
        frame.set_parent(outer_fid);
        ui.set_frame(wid,inner_fid,&frame);

        // update background quad
        quads[bgqid].set_geometry(f32_r::new(0.0,0.0,max.x as f32,max.y as f32));
        quads[bgqid].set_state(QuadState::Color(0xFF2266AA));
        quads[bgqid].set_frame(inner_fid);

        ui.upload_quads(wid,&quads);

        Button {
            outer_fid: outer_fid,
            inner_fid: inner_fid,
            _text: text.to_string(),
            pressed: false,
        }
    }

    pub fn pressed(&self) -> bool {
        self.pressed
    }

    pub fn set_pressed(&mut self,ui: &mut UI,wid: usize,pressed: bool) {
        self.pressed = pressed;
        if pressed {
            let mut frame = ui.frame(wid,self.inner_fid).unwrap();
            let geometry = frame.geometry();
            frame.set_geometry(f32_r::new(5.0,5.0,geometry.s.x,geometry.s.y));
            ui.set_frame(wid,self.inner_fid,&frame);
            ui.upload_frames(wid);
            ui.invalidate(wid);
        }
        else {
            let mut frame = ui.frame(wid,self.inner_fid).unwrap();
            let geometry = frame.geometry();
            frame.set_geometry(f32_r::new(0.0,0.0,geometry.s.x,geometry.s.y));
            ui.set_frame(wid,self.inner_fid,&frame);
            ui.upload_frames(wid);
            ui.invalidate(wid);
        }
    }

    pub fn hittest(&mut self,ui: &mut UI,wid: usize,p: f32_2) -> ButtonHit {
        let mut lp = p;
        let frame = ui.frame(wid,self.outer_fid).unwrap();
        frame.localize(&mut lp);
        let geometry = frame.geometry();
        if (lp.x >= 0.0) && (lp.y >= 0.0) && (lp.x < geometry.s.x) && (lp.y < geometry.s.y) {
            ButtonHit::Button
        }
        else {
            ButtonHit::None
        }
    }

    pub fn mouse_press(&mut self,ui: &mut UI,wid: usize,p: f32_2,b: u8) {
        println!("mouse press");
        if let ButtonHit::Button = self.hittest(ui,wid,p) {
            println!("press");
            self.set_pressed(ui,wid,true);
        }
    }

    pub fn mouse_release(&mut self,ui: &mut UI,wid: usize,p: f32_2,b: u8) {
        println!("release");
        self.set_pressed(ui,wid,false);
    }
}
