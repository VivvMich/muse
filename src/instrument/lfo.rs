use crate::instrument::signal::Wave;

pub struct ModParam {
    pub value: f32,
    pub lfo: Option<LFO>,
}


pub struct LFO {
    rate: f32,
    shape: Wave,
    depth: f32
}