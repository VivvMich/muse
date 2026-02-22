use std::collections::HashMap;
use crate::instrument::lfo::ModParam;
use crate::instrument::signal::{OscSignal, Wave};

pub struct Oscillator{
    pub pitch: f32,
    pub offset_tune: ModParam,
    pub signals: HashMap<Wave, OscSignal>
}