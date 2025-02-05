use crate::dialogue;
use crate::dialogue::logic::{DialogueNode, DialogueNodeID, DialogueOption, DialoguePersonID};
use std::collections::HashMap;

pub fn dialogue1(hash_map: &mut HashMap<DialogueNodeID, DialogueNode>) {
    let d = dialogue!(
        DialogueNodeID::Dialogue1,
        DialoguePersonID::Central,
        "System Status: Active. All crew members accounted for.",
        [
            ("No, they're gone. The base is empty.", DialogueNodeID::Dialogue1),
            ("You need to let me into the lab.", DialogueNodeID::Dialogue1),
            ("You're malfunctioning. Something is wrong.", DialogueNodeID::Dialogue1)
        ]
    );
    hash_map.insert(d.id, d);
}
