use std::collections::HashMap;
use crate::instrument::signal::{Wave, OscSignal};
// ALL COMMAND

// MAIN COMMAND
pub enum Command {
    START,
    STOP,
    ADD(String, SynthModule),
    MOD(SynthModule),
    MIX,
    DELETE(SynthModule),
    EXIT,
}

pub enum SynthModule {
    Oscillator{
        offset_tune: f32,
        signals: HashMap<Wave, OscSignal>
    },
    LFO,
    Filter,
    Effect,
    Envelope
}