use crate::dialogue::dialogue1::dialogue1;
use std::collections::HashMap;

#[derive(Debug)]
pub struct DialogueOption {
    pub text: String,
    pub next: DialogueNodeID,
}

#[derive(Debug)]
pub struct DialogueNode {
    pub id: DialogueNodeID,
    pub speaker: DialoguePersonID,
    pub text: String,
    pub options: Vec<DialogueOption>,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum DialogueNodeID {
    Dialogue1,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum DialoguePersonID {
    Player,
    Central,
    Watcher,
    Fixer,
    Echo,
}

pub fn parse_dialogue() -> HashMap<DialogueNodeID, DialogueNode> {
    let mut dialogues = HashMap::new();
    dialogue1(&mut dialogues);
    //    println!("{:?}", central_info);
    dialogues
}
