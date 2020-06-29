// e::engine::Track
// by Desmond Germans, 2019

use audio::Frame;
use crate::Channel;
//use crate::audio::Bunch;

#[derive(Clone)]
pub struct Track {
    channels: Vec<Channel>,
    icount: usize,
    fpt: usize,
}

impl Track {
    pub fn new() -> Track {
        Track {
            channels: Vec::new(),
            icount: 0,
            fpt: 0,
        }
    }

    pub fn render(&mut self,quantum: &mut [Frame]) {

        for i in 0..quantum.len() {
            quantum[i].l = 0.0;
            quantum[i].r = 0.0;
        }

        let len = quantum.len();
        let until_tick = self.fpt - self.icount;
        if len >= until_tick {
            // TODO: render until_tick frames
            for channel in &self.channels {
                channel.tick();
            }
            if len > until_tick {
                // TODO: render len - until_tick frames
                self.icount = len - until_tick;
            }
        }
        else {
            // TODO: render len frames
            self.icount += len;
        }
    }
}
