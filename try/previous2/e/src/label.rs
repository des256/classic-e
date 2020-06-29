// e label
// by Desmond Germans, 2019

use crate::*;

pub struct Label {
    _fid: usize,
    _text: String,
}

impl Label {
    pub fn new(ui: &mut UI,wid: usize,quads: &mut Vec<UIQuad>,p: f32_2,text: &str) -> Label {

        // allocate frame
        let fid = ui.alloc_frame(wid).unwrap();

        // background quad (setup later)
        let bgqid = quads.len();
        quads.push(UIQuad::new());

        // character quads
        let max = ui.font.push_text(quads,fid,&text);

        // update frame
        let mut frame = UIFrame::new();
        frame.set_geometry(f32_r::new(p.x,p.y,max.x,max.y));
        frame.set_parent(0);
        ui.set_frame(wid,fid,&frame);

        // update background quad
        quads[bgqid].set_geometry(f32_r::new(0.0,0.0,max.x as f32,max.y as f32));
        quads[bgqid].set_state(QuadState::Color(0xFF113355));
        quads[bgqid].set_frame(fid);

        ui.upload_quads(wid,&quads);

        Label {
            _fid: fid,
            _text: text.to_string(),
        }
    }
}
