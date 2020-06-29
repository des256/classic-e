// e::audio::Audio: main audio subsystem
// by Desmond Germans, 2019

#[doc(no_inline)]
extern crate alsa;
use alsa::{PCM,pcm::{HwParams,Format,Access},PollDescriptors};
use std::{thread,os::raw::c_int};

pub struct Frame {
    pub l: f32,
    pub r: f32,
}

pub struct Audio {
    mixer_thread: Option<thread::JoinHandle<()>>,
    //tx: mpsc::Sender<AudioMsg>,
    //rx_return: mpsc::Receiver<AudioReturnMsg>,
}

fn clamp(v: f32, min: f32, max: f32) -> f32 {

    if v < min { return min; }
    if v > max { return max; }
    v
}

impl Audio {
    pub fn new() -> Audio {
        //let (tx, rx) = mpsc::channel();
        //let (_tx_return, rx_return) = mpsc::channel();
        let mixer_thread = Some(thread::spawn(move || {
            let pcm_maybe = PCM::new("hw:0,0",alsa::Direction::Playback,false);
            let pcm = match pcm_maybe {
                Ok(pcm) => { pcm },
                Err(_e) => { panic!("Audio: unable to open PCM device"); },
            };
            let mut fps: f32 = 44100.0;
            {
                let hwp = HwParams::any(&pcm).unwrap();
                hwp.set_channels(2).unwrap();
                hwp.set_rate_near(fps as u32,alsa::ValueOr::Nearest).unwrap();
                hwp.set_format(Format::s16()).unwrap();
                hwp.set_access(Access::RWInterleaved).unwrap();
                hwp.set_buffer_size(1024).unwrap();
                hwp.set_period_size(256, alsa::ValueOr::Nearest).unwrap();
                pcm.hw_params(&hwp).unwrap();
            }
            {
                let hwp = pcm.hw_params_current().unwrap();
                let swp = pcm.sw_params_current().unwrap();
                let bufsize = hwp.get_buffer_size().unwrap();
                let periodsize = hwp.get_period_size().unwrap();
                swp.set_start_threshold(bufsize - periodsize).unwrap();
                swp.set_avail_min(periodsize).unwrap();
                pcm.sw_params(&swp).unwrap();
                fps = hwp.get_rate().unwrap() as f32;
            }
            let mut fds = pcm.get().unwrap();
            let io = pcm.io_i16().unwrap();
            match pcm.start() {
                Err(_e) => { pcm.prepare().unwrap(); },
                _ => { },
            };
            let buffer: Box<[i16]> = Box::new([0; 4 * 44100]);
            let mut running = true;
            while running {
                //loop {
                //    match rx.try_recv() {
                //        Ok(msg) => { },
                //        _ => { break; },
                //    }
                //}
                alsa::poll::poll(&mut fds,100).unwrap();
                let avail = match pcm.avail_update() {
                    Ok(n) => n as usize,
                    Err(e) => {
                        match e.errno() {
                            Some(errno) => { pcm.recover(errno as c_int,true).unwrap(); },
                            _ => { },
                        };
                        pcm.avail_update().unwrap() as usize
                    }
                };
                if avail > 0 {
                    // TODO: clear buffer
                    //for i in 0..avail {
                    //    buffer[i * 2] = (clamp(accu[i].l,-1.0,1.0) * 32767.0) as i16;
                    //    buffer[i * 2 + 1] = (clamp(accu[i].r,-1.0,1.0) * 32767.0) as i16;
                    //}
                    match io.writei(&buffer[0..avail * 2]) {
                        Ok(_n) => { },
                        Err(e) => { println!("Audio: write error {}",e); },
                    };
                }
            }
        }));

        Audio {
            mixer_thread: mixer_thread,
            //tx: tx,
            //rx_return: rx_return,
        }
    }
}

impl Drop for Audio {
    fn drop(&mut self) {
        //self.tx.send(AudioMsg::Terminate).unwrap();
        if let Some(handle) = self.mixer_thread.take() {
            handle.join().unwrap();
        }
    }
}
