// e::engine::Synth
// by Desmond Germans, 2019

// Synth is the basic subtractive synthesizer (kinda sorta a Juno 106):
//
// FIRST THE OSCILLATOR:
//
//     the pulse width can be modulated by 3 things:
//         dco_pulse = base pulse width
//         dco_pulse_env = pulse width envelope
//         dco_pulse_lfo = pulse width LFO
//
//     pulse_width = dco_pulse + dco_pulse_lfo_level * dco_pulse_lfo + dco_pulse_env_level * dco_pulse_env
//
//     the frequency of the oscillator can be altered with a vibrato:
//         dco_lfo = vibrato LFO
//
//     dco_freq = note frequency + dco_lfo_level * dco_lfo
//
//     the oscillator result is a mix of 4 waves:
//         pulse_wave = pulse with pulse_width pulse at dco_freq
//         saw_wave = saw wave at dco_freq
//         sub_wave = square wave at dco_freq / 2
//         noise = white noise
//
//     RESULT ==> dco = dco_pulse_level * pulse_wave + dco_saw_level * saw_wave + dco_sub_level * sub_wave + dco_noise_level * noise
//
// THEN THE HIGH-PASS FILTER:
//
//     RESULT ==> hpf = high-pass at hpf_cutoff over dco
//
// THEN THE LOW-PASS FILTER:
//
//     the VCF cutoff can be modulated by 3 things:
//         vcf_cutoff = base cutoff
//         vcf_cutoff_env = cutoff envelope
//         vcf_cutoff_lfo = cutoff LFO
//
//     cutoff = vcf_cutoff + vcf_cutoff_lfo_level * vcf_cutoff_lfo + vcf_cutoff_env_level * vcf_cutoff_env
//
//     RESULT ==> vcf = low-pass at cutoff and vcf_resonance over hpf
//
//     STILL TODO: velocity sensitivity, keyboard sensitivity
//
// AND FINALLY THE AMPLIFIER:
//
//     this is a combination of:
//         vca_env = final envelope
//         vca_lfo = tremolo LFO
//
//     RESULT ==> vca = vca_level * (vca_env + vca_lfo_level * vca_lfo) * vcf
//
//     STILL TODO: velocity sensitivity

extern crate rand;

use std::f32;

use audio::Frame;

const NOTES_HZ: [f32; 96] = [
      16.35,  17.32,  18.35,  19.45,  20.60,  21.83,  23.12,  24.50,  25.96,  27.50,  29.14,  30.87,
      32.70,  34.65,  36.71,  38.89,  41.20,  43.65,  46.25,  49.00,  51.91,  55.00,  58.27,  61.74,
      65.41,  69.30,  73.42,  77.78,  82.41,  87.31,  92.50,  98.00, 103.83, 110.00, 116.54, 123.47,
     130.81, 138.59, 146.83, 155.56, 164.81, 174.61, 185.00, 196.00, 207.65, 220.00, 233.08, 246.94,
     261.63, 277.18, 293.66, 311.13, 329.63, 349.23, 369.99, 392.00, 415.30, 440.00, 466.16, 493.88,
     523.25, 554.37, 587.33, 622.25, 659.25, 698.46, 739.99, 783.99, 830.61, 880.00, 932.33, 987.77,
    1046.50,1108.73,1174.66,1244.51,1318.51,1396.91,1479.98,1567.98,1661.22,1760.00,1864.66,1975.53,
    2093.00,2217.46,2349.32,2489.02,2637.02,2793.83,2959.96,3135.96,3322.44,3520.00,3729.31,3951.07,
];

const TAU: f32 = 6.28318530718;

const CUTOFF_KNOB: f32 = 1.0 / (TAU * 40000.0);  // personalization

#[derive(Clone,Copy)]
pub enum EnvPhase {
    Off,
    Attack,
    Decay,
    Sustain,
    Release,
}

#[derive(Clone,Copy)]
pub struct Voice {
    t: f32,                         // time since start of note (sec)
    dco: f32,                       // DCO current period index (0..1)
    dco_sub: bool,                  // positive or negative sub
    //dco_pulse: f32,                 // DCO current pulse width (0..1)
    dco_pulse_lfo: f32,             // DCO pulse width LFO period index (0..1)
    dco_pulse_env_phase: EnvPhase,  // DCO pulse width envelope current phase (0ADSR)
    dco_pulse_env_ofs: f32,         // DCO pulse width envelope time offset
    dco_lfo: f32,                   // DCO LFO period index (0..1)
    prev_dco: f32,                  // previous DCO output (for HPF calculation)
    prev_hpf: f32,                  // previous HPF output (for HPF calculation)
    vcf_lfo: f32,                   // VCF LFO current period index (0..1)
    vcf_env_phase: EnvPhase,        // VCF envelope current phase (0ADSR)
    vcf_env_ofs: f32,               // VCF envelope time offset
    prev_vcf: f32,                  // previous VCF output (for VCF calculation)
    prev_dvcf: f32,                 // previous VCF speed output (for VCF calculation)
    vca_lfo: f32,                   // VCA LFO period index (0..1)
    vca_env_phase: EnvPhase,        // VCA envelope current phase (0ADSR)
    vca_env_ofs: f32,               // VCA envelope time offset
}

impl Voice {
    pub fn new() -> Voice {
        Voice {
            t: 0.0,
            dco: 0.0,
            dco_sub: false,
            //dco_pulse: 0.5,
            dco_pulse_lfo: 0.0,
            dco_pulse_env_phase: EnvPhase::Off,
            dco_pulse_env_ofs: 0.0,
            dco_lfo: 0.0,
            prev_dco: 0.0,
            prev_hpf: 0.0,
            vcf_lfo: 0.0,
            vcf_env_phase: EnvPhase::Off,
            vcf_env_ofs: 0.0,
            prev_vcf: 0.0,
            prev_dvcf: 0.0,
            vca_lfo: 0.0,
            vca_env_phase: EnvPhase::Off,
            vca_env_ofs: 0.0,
        }
    }
}

#[derive(Clone,Copy)]
pub struct Patch {
    dco_pulse: f32,              // DCO base pulse width (0..1)
    dco_pulse_lfo_delay: f32,    // DCO pulse LFO delay (sec)
    dco_pulse_lfo_rate: f32,     // DCO pulse LFO rate (hz)
    dco_pulse_env_attack: f32,   // DCO pulse envelope attack (per sec)
    dco_pulse_env_decay: f32,    // DCO pulse envelope decay (per sec)
    dco_pulse_env_sustain: f32,  // DCO pulse envelope sustain (0..1)
    dco_pulse_env_release: f32,  // DCO pulse envelope release (per sec)
    dco_pulse_lfo_level: f32,    // DCO pulse mixer LFO level (0..1)
    dco_pulse_env_level: f32,    // DCO pulse mixer envelope level (-1..1)
    dco_lfo_delay: f32,          // DCO LFO delay (sec)
    dco_lfo_rate: f32,           // DCO LFO rate (hz) (vibrato)
    dco_lfo_level: f32,          // DCO LFO level (0..1)
    dco_pulse_level: f32,        // DCO mixer pulse level (0..1)
    dco_saw_level: f32,          // DCO mixer saw level (0..1)
    dco_sub_level: f32,          // DCO mixer sub level (0..1)
    dco_noise_level: f32,        // DCO mixer noise level (0..1)
    hpf_cutoff: f32,             // HPF cutoff frequency (0..1)
    vcf_cutoff: f32,             // VCF cutoff frequency (0..1)
    vcf_resonance: f32,          // VCF resonance level (0..1)
    vcf_lfo_delay: f32,          // VCF cutoff LFO delay (sec)
    vcf_lfo_rate: f32,           // VCF cutoff LFO rate (hz)
    //vcf_sense: f32,              // VCF velocity sensitivity (0..1)
    vcf_env_attack: f32,         // VCF cutoff envelope attack (per sec)
    vcf_env_decay: f32,          // VCF cutoff envelope decay (per sec)
    vcf_env_sustain: f32,        // VCF cutoff envelope sustain (0..1)
    vcf_env_release: f32,        // VCF cutoff envelope release (per sec)
    vcf_lfo_level: f32,          // VCF cutoff mixer LFO level (-1..1)
    vcf_env_level: f32,          // VCF cutoff mixer environment level (-1..1)
    //vcf_key_level: f32,          // VCF key level (-1..1)
    vca_lfo_delay: f32,          // VCA LFO delay (sec)
    vca_lfo_rate: f32,           // VCA LFO rate (hz) (tremolo)
    //vca_sense: f32,              // VCA velocity sensitivity (0..1)
    vca_env_attack: f32,         // VCA envelope attack (per sec)
    vca_env_decay: f32,          // VCA envelope decay (per sec)
    vca_env_sustain: f32,        // VCA envelope sustain (0..1)
    vca_env_release: f32,        // VCA envelope release (per sec)
    vca_lfo_level: f32,          // VCA mixer LFO level (-1..1)
    vca_level: f32,              // VCA envelope level (0..1) (main volume)
}

impl Patch {
    pub fn new() -> Patch {
        Patch {
            dco_pulse: 0.3,
            dco_pulse_lfo_delay: 0.0,
            dco_pulse_lfo_rate: 2.0,
            dco_pulse_env_attack: 0.05,
            dco_pulse_env_decay: 0.1,
            dco_pulse_env_sustain: 0.3,
            dco_pulse_env_release: 1.0,
            dco_pulse_lfo_level: 0.1,
            dco_pulse_env_level: 0.0,
            dco_lfo_delay: 0.2,
            dco_lfo_rate: 8.0,
            dco_lfo_level: 0.0,
            dco_pulse_level: 1.0,
            dco_saw_level: 1.0,
            dco_sub_level: 0.0,
            dco_noise_level: 0.2,
            hpf_cutoff: 0.0,
            vcf_cutoff: 0.1,
            vcf_resonance: 0.5,
            vcf_lfo_delay: 0.5,
            vcf_lfo_rate: 2.0,
            //vcf_sense: 0.0,
            vcf_env_attack: 0.05,
            vcf_env_decay: 0.1,
            vcf_env_sustain: 0.2,
            vcf_env_release: 0.5,
            vcf_lfo_level: 0.2,
            vcf_env_level: 0.4,
            //vcf_key_level: 0.0,
            vca_lfo_delay: 0.0,
            vca_lfo_rate: 0.0,
            //vca_sense: 0.0,
            vca_env_attack: 0.01,
            vca_env_decay: 0.2,
            vca_env_sustain: 0.3,
            vca_env_release: 8.0,
            vca_lfo_level: 0.0,
            vca_level: 0.8,
        }
    }
}

#[derive(Clone,Copy)]
pub struct Synth {
    voices: [Voice; 96],  // all the voices
    patch: Patch,  // the current patch
}

fn clamp(mut x: f32,min: f32,max: f32) -> f32 {
    if x < min {
        x = min;
    }
    if x > max {
        x = max;
    }
    x
}

fn process_lfo(t: f32,p: &mut f32,delay: f32,rate: f32) -> f32 {
    if t >= delay {
        *p = (*p + rate).fract();
    }
    (*p * TAU).sin()
}

fn process_envelope(t: f32,offset: &mut f32,phase: &mut EnvPhase,attack: f32,decay: f32,sustain: f32,release: f32) -> f32 {
    let mut v = 0.0;
    match *phase {
        EnvPhase::Attack => {
            v = (t - *offset) * attack;
            if v > 1.0 {
                v = 1.0;
                *offset = t;
                *phase = EnvPhase::Decay;
            }
        },
        EnvPhase::Decay => {
            v = 1.0 - (t - *offset) * decay;
            if v < sustain {
                v = sustain;
                *offset = t;
                *phase = EnvPhase::Sustain;
            }
        },
        EnvPhase::Sustain => {
            v = sustain;
        },
        EnvPhase::Release => {
            v = sustain - (t - *offset) * release;
            if v < 0.0 {
                v = 0.0;
                *phase = EnvPhase::Off;
            }
        },
        EnvPhase::Off => { },
    }
    v
}

impl Synth {
    pub fn new() -> Synth {
        Synth {
            voices: [Voice::new(); 96],
            patch: Patch::new(),
        }
    }

    pub fn load(&mut self,patch: &Patch) {
        self.patch = patch.clone();
    }

    pub fn render(&mut self,fps: f32,quantum: &mut [Frame]) {

        let spf = 1.0 / fps;

        for i in 0..quantum.len() {
            quantum[i].l = 0.0;
            quantum[i].r = 0.0;
        }

        for n in 0..96 {
            match self.voices[n].vca_env_phase {
                EnvPhase::Off => { },
                _ => {
                    let voice = &mut self.voices[n];
                    let patch = &self.patch;
                    //let mut debug: f32 = 0.0;
                    for i in 0..quantum.len() {
                        
                        // DCO pulse width LFO
                        let dco_pulse_lfo = process_lfo(voice.t,&mut voice.dco_pulse_lfo,patch.dco_pulse_lfo_delay,spf * patch.dco_pulse_lfo_rate);
                        
                        // DCO pulse width envelope
                        let dco_pulse_env = process_envelope(voice.t,&mut voice.dco_pulse_env_ofs,&mut voice.dco_pulse_env_phase,1.0 / patch.dco_pulse_env_attack,1.0 / patch.dco_pulse_env_decay,patch.dco_pulse_env_sustain,1.0 / patch.dco_pulse_env_release);

                        // DCO pulse width combined
                        let width = clamp(patch.dco_pulse + patch.dco_pulse_lfo_level * dco_pulse_lfo + patch.dco_pulse_env_level * dco_pulse_env,0.0,1.0);
                        //debug = width;

                        // DCO LFO
                        let dco_lfo = process_lfo(voice.t,&mut voice.dco_lfo,patch.dco_lfo_delay,spf * patch.dco_lfo_rate);

                        // DCO oscillate
                        voice.dco += (1.0 + patch.dco_lfo_level * dco_lfo) * spf * NOTES_HZ[n];  // <-- this might be inaccurate
                        while voice.dco > 1.0 {
                            voice.dco -= 1.0;
                            voice.dco_sub = !voice.dco_sub;
                        }

                        // DCO pulse wave
                        let mut pulse = -1.0;
                        if voice.dco > width {
                            pulse = 1.0;
                        }

                        // DCO saw wave
                        let saw = voice.dco;

                        // DCO sub wave
                        let mut sub = -1.0;
                        if voice.dco_sub {
                            sub = 1.0;
                        }

                        // DCO white noise
                        let noise = rand::random::<f32>().fract();

                        // DCO combined
                        let dco = patch.dco_pulse_level * pulse + patch.dco_saw_level * saw + patch.dco_sub_level * sub + patch.dco_noise_level * noise;
                        
                        // VCF HPF
                        let mut hpf = dco;
                        if patch.hpf_cutoff > 0.0 {
                            let q = fps * CUTOFF_KNOB;
                            let b = (1.0 / (q + 1.0)).ln() / 0.5f32.ln();
                            let a = patch.hpf_cutoff.powf(b);
                            hpf = a * (voice.prev_hpf + dco - voice.prev_dco);
                            voice.prev_dco = dco;
                            voice.prev_hpf = hpf;
                        }

                        // VCF LPF LFO
                        let vcf_lfo = process_lfo(voice.t,&mut voice.vcf_lfo,patch.vcf_lfo_delay,spf * patch.vcf_lfo_rate);

                        // VCF LPF envelope
                        let vcf_env = process_envelope(voice.t,&mut voice.vcf_env_ofs,&mut voice.vcf_env_phase,1.0 / patch.vcf_env_attack,1.0 / patch.vcf_env_decay,patch.vcf_env_sustain,1.0 / patch.vcf_env_release);

                        // VCF LPF combined
                        let cutoff = clamp(patch.vcf_cutoff +  // base
                            patch.vcf_lfo_level * vcf_lfo +    // LFO
                            patch.vcf_env_level * vcf_env,     // envelope
                            0.0,1.0);
                        // TODO: key level

                        let mut vcf = hpf;
                        if cutoff > 0.0 {
                            let q = fps * CUTOFF_KNOB;
                            let b = (q / (q + 1.0)).ln() / 0.5f32.ln();
                            let a = cutoff.powf(b);
                            voice.prev_dvcf += 2.0 * patch.vcf_resonance * a * (hpf - voice.prev_vcf);  // TODO: this resonance might not be entirely correct
                            vcf = voice.prev_vcf + a * (hpf - voice.prev_vcf) + voice.prev_dvcf;
                            voice.prev_vcf = vcf;
                        }
                        // TODO: VCF velocity sensitivity                        

                        // VCA LFO
                        let vca_lfo = process_lfo(voice.t,&mut voice.vca_lfo,patch.vca_lfo_delay,spf * patch.vca_lfo_rate);

                        // VCA envelope
                        let vca_env = process_envelope(voice.t,&mut voice.vca_env_ofs,&mut voice.vca_env_phase,1.0 / patch.vca_env_attack,1.0 / patch.vca_env_decay,patch.vca_env_sustain,1.0 / patch.vca_env_release);

                        let result = patch.vca_level * (vca_env + patch.vca_lfo_level * vca_lfo) * vcf;

                        // and add
                        quantum[i].l += result;
                        quantum[i].r += result;

                        voice.t += spf;
                    }
                },
            }
        }
    }

    pub fn note(&mut self,n: usize,_v: f32) {
        self.voices[n] = Voice::new();
        self.voices[n].dco_pulse_env_phase = EnvPhase::Attack;
        self.voices[n].vcf_env_phase = EnvPhase::Attack;
        self.voices[n].vca_env_phase = EnvPhase::Attack;
    }

    pub fn release(&mut self,n: usize) {
        self.voices[n].dco_pulse_env_ofs = self.voices[n].t;
        self.voices[n].dco_pulse_env_phase = EnvPhase::Release;
        self.voices[n].vcf_env_ofs = self.voices[n].t;
        self.voices[n].vcf_env_phase = EnvPhase::Release;
        self.voices[n].vca_env_ofs = self.voices[n].t;
        self.voices[n].vca_env_phase = EnvPhase::Release;
    }
}
