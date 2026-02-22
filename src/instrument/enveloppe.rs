use crate::instrument::lfo::ModParam;

pub struct Enveloppe {
    pub gate: f32,
    pub attack: ModParam,
    pub sustain: ModParam,
    pub release: ModParam,
}