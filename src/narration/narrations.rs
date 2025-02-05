use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum NarrationID {
    Narration1,
}

pub struct Narration {
    pub text: String,
}

pub fn get_narrations() -> HashMap<NarrationID, Narration> {
    let map = HashMap::<NarrationID, Narration>::new();
    map
}
