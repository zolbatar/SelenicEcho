use crate::parser::enums::Verbs;
use std::collections::HashMap;

pub struct VerbLookup {
    lookups: HashMap<String, Verbs>,
}

impl VerbLookup {
    pub fn new() -> Self {
        let mut lookups = HashMap::new();
        lookups.insert("CLOSE".to_string(), Verbs::Close);
        lookups.insert("LOCK".to_string(), Verbs::Close);

        lookups.insert("DROP".to_string(), Verbs::Drop);
        lookups.insert("LEAVE".to_string(), Verbs::Drop);
        lookups.insert("DISCARD".to_string(), Verbs::Drop);

        lookups.insert("GIVE".to_string(), Verbs::Give);
        lookups.insert("OFFER".to_string(), Verbs::Give);

        lookups.insert("LOOK".to_string(), Verbs::Look);
        lookups.insert("EXAMINE".to_string(), Verbs::Look);
        lookups.insert("INSPECT".to_string(), Verbs::Look);
        lookups.insert("OBSERVE".to_string(), Verbs::Look);

        lookups.insert("GO".to_string(), Verbs::Go);
        lookups.insert("MOVE".to_string(), Verbs::Go);
        lookups.insert("WALK".to_string(), Verbs::Go);
        lookups.insert("ENTER".to_string(), Verbs::Go);

        lookups.insert("OPEN".to_string(), Verbs::Open);
        lookups.insert("UNLOCK".to_string(), Verbs::Open);

        lookups.insert("RUN".to_string(), Verbs::Run);
        lookups.insert("DASH".to_string(), Verbs::Run);
        lookups.insert("SPRINT".to_string(), Verbs::Run);

        lookups.insert("TAKE".to_string(), Verbs::Take);
        lookups.insert("PICKUP".to_string(), Verbs::Take);
        lookups.insert("GRAB".to_string(), Verbs::Take);

        lookups.insert("TALK".to_string(), Verbs::Talk);
        lookups.insert("SAY".to_string(), Verbs::Talk);
        lookups.insert("SPEAK".to_string(), Verbs::Talk);

        lookups.insert("USE".to_string(), Verbs::Use);
        lookups.insert("ACTIVATE".to_string(), Verbs::Use);
        lookups.insert("OPERATE".to_string(), Verbs::Use);
        lookups.insert("MANIPULATE".to_string(), Verbs::Use);

        VerbLookup {
            lookups,
        }
    }

    pub fn find_verb(&self, key: &String) -> Option<&Verbs> {
        self.lookups.get(&key.to_uppercase())
    }
}
