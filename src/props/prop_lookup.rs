use crate::props::enums::PropsID;
use std::collections::HashMap;

pub struct PropLookup {
    lookups: HashMap<String, PropsID>,
}

impl PropLookup {
    pub fn new() -> PropLookup {
        let mut lookups = HashMap::new();
        lookups.insert("HELMET".to_string(), PropsID::Helmet);
        lookups.insert("SUIT OXYGEN SENSOR".to_string(), PropsID::SuitOxygenSensor);

        PropLookup {
            lookups,
        }
    }
}
