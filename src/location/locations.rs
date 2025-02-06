use crate::location;
use crate::narration::narrations::NarrationID;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Location {
    pub id: LocationID,
    pub narration_id: NarrationID,
}

#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy)]
pub enum LocationID {
    InitialLocation,
}

pub fn get_locations() -> HashMap<LocationID, Location> {
    let mut map = HashMap::new();
    {
        let l = location!(LocationID::InitialLocation, NarrationID::Awake);
        map.insert(l.id, l);
    }
    map
}
