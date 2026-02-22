use std::collections::HashMap;
use crate::instrument::oscilator::Oscillator;
use crate::instrument::filter::{FilterType};
use crate::instrument::filter::Filter;
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
    Oscillator(Oscillator),
    Filter(Filter),
    Envelope,
    LFO,
    Effect,
}