// e::engine::Channel
// by Desmond Germans, 2019

//use audio::Frame;
use crate::Bunch;
use crate::Synth;

/// Note.
#[derive(Clone)]
pub struct Note {
    note: u8, // 0 = C0, ..., 95 = B7
    velocity: f32, // 0.0 = none, ..., 1.0 = full
    length: u64,
}

/// Channel.
#[derive(Clone)]
pub struct Channel {
    synth: Synth,  // the synthesizer
    notes: Bunch<Vec<Note>>,  // group of notes played together
}

impl Channel {
    pub fn new() -> Channel {
        Channel {
            synth: Synth::new(),
            notes: Bunch::new(),
        }
    }

    pub fn tick(&self) {
        // TODO: prepare next tick
    }
}
