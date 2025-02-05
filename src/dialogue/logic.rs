use crate::dialogue::dialogue1::dialogue1;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct DialogueOption {
    pub text: String,
    pub next: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DialogueNode {
    pub id: String,
    pub speaker: String,
    pub text: String,
    pub options: Vec<DialogueOption>,
}

pub fn parse_dialogue() {
    let dialogues = HashMap::new();
    dialogue1(dialogues);
//    println!("{:?}", central_info);
}
