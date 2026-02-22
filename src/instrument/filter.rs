use crate::instrument::lfo::ModParam;

pub enum FilterType{
    HighPass,
    LowPass,
    PassBand,
}


pub struct Filter{
    pub filter_type: FilterType,
    pub cutoff_frequency: ModParam,
    pub resonance: ModParam,
}