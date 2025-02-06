use crate::location;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Location {
    pub id: LocationID,
    pub text: String,
}

#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy)]
pub enum LocationID {
    InitialLocation,
}

pub fn get_locations() -> HashMap<LocationID, Location> {
    let mut map = HashMap::new();
    let l1 = location!(LocationID::InitialLocation, "BEEP...\nBEEP...BEEP...BEEP\n");
    map.insert(l1.id, l1);
    map
}
