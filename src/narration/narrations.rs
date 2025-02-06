use crate::{location, narration};
use std::collections::HashMap;

const AWAKE: &str = include_str!("awake.txt");

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum NarrationID {
    Awake,
}

pub struct Narration {
    pub id: NarrationID,
    pub text: String,
}

pub fn get_narrations() -> HashMap<NarrationID, Narration> {
    let mut map = HashMap::<NarrationID, Narration>::new();
    {
        let n = narration!(NarrationID::Awake, AWAKE);
        map.insert(n.id, n);
    }
    map
}
