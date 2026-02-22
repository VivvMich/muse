// Here we add each synth module in a register with id

use std::collections::HashMap;
use crate::parser::command::SynthModule;

pub struct ModuleRegistry {
    registry: HashMap<String, SynthModule>
}