use crate::props::enums::{PropTypes, PropsID};
use crate::props::item::Item;
use crate::props::prop_lookup::PropLookup;

pub struct Items {
    items: Vec<Item>,
    prop_lookup: PropLookup,
}

impl Items {
    pub fn new() -> Items {
        let items = vec![
            Item {
                props_id: PropsID::Helmet,
                types: vec![PropTypes::Interactable],
            },
            Item {
                props_id: PropsID::SuitOxygenSensor,
                types: vec![PropTypes::Interactable],
            },
        ];

        Items {
            items,
            prop_lookup: PropLookup::new(),
        }
    }
}
