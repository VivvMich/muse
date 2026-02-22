use std::collections::HashMap;
use crate::instrument::lfo::ModParam;

#[derive(Hash, Eq, PartialEq)]
pub enum Wave {
    SIN,
    SQR,
    TRI,
    SAW,
}

pub struct OscSignal {
    pub volume: f32, // 0.0, 1.0
    pub pw: Option<ModParam>,
}

