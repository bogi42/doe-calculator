use crate::interactive::{EditableItem, PrintableItem};
use crate::Character;
use advanced_inputs::AdvInput;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub topic: String,
    pub description: String,
}
impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:\n{}", self.topic.bold(), self.description)
    }
}
impl Note {
    pub fn new(topic: String, description: String) -> Self {
        Note { topic, description }
    }
}

impl EditableItem for Note {
    fn from_input() -> Option<Self> {
        let mut advi = AdvInput::new();
        let topic = match advi.get_string("Topic > ".green()) {
            Some(v) => v,
            None => return None,
        };
        let desc = match advi.get_string("Description > ".green()) {
            Some(v) => v,
            None => return None,
        };
        Some(Self::new(topic, desc))
    }
    fn change(&mut self) {
        let mut advi = AdvInput::new();
        self.topic = match advi.get_string_initial("Topic > ".green(), &self.topic) {
            Some(s) => s,
            None => self.topic.clone(),
        };
        self.description =
            match advi.get_string_initial("Description > ".green(), &self.description) {
                Some(s) => s,
                None => self.description.clone(),
            };
    }
    fn item_kind_name() -> &'static str {
        "Note"
    }
}

impl PrintableItem for Note {
    fn pretty_print(&self, _character: &Character) -> String {
        format!("| **{}:**\n| {}", &self.topic, &self.description)
    }
}
