use crate::dialogue::logic::{get_dialogues, DialogueNode, DialogueNodeID};
use crate::location::locations::{get_locations, Location, LocationID};
use crate::narration::narrations::{get_narrations, Narration, NarrationID};
use crate::printer::Printer;
use crate::props::items::Items;
use std::collections::HashMap;

pub struct GameState {
    pub dialogues: HashMap<DialogueNodeID, DialogueNode>,
    pub narrations: HashMap<NarrationID, Narration>,
    pub locations: HashMap<LocationID, Location>,
    pub items: Items,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            dialogues: get_dialogues(),
            narrations: get_narrations(),
            locations: get_locations(),
            items: Items::new(),
        }
    }

    pub fn start(&self, printer: &mut Printer) {
        printer.print_location(LocationID::InitialLocation, &self);
    }
}
