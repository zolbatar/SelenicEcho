use crate::dialogue;
use crate::dialogue::logic::{DialogueNode, DialogueOption};
use std::collections::HashMap;

pub fn dialogue1(mut hash_map: HashMap<String, DialogueNode>) {
    let d = dialogue!(
        "central_intro",
        "Central",
        "System Status: Active. All crew members accounted for.",
        [
            ("No, they're gone. The base is empty.", "central_deny_reality"),
            ("You need to let me into the lab.", "central_access_check"),
            ("You're malfunctioning. Something is wrong.", "central_malfunction_check")
        ]
    );
    hash_map.insert(d.id.clone(), d);
}
