use crate::interactive::{EditableItem, PrintableItem};
use crate::Character;
use advanced_inputs::AdvInput;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterTrait {
    pub description: String,
}
impl fmt::Display for CharacterTrait {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}
impl CharacterTrait {
    pub fn new(description: String) -> Self {
        CharacterTrait { description }
    }
}

impl EditableItem for CharacterTrait {
    fn from_input() -> Option<Self> {
        let mut advi = AdvInput::new();
        if let Some(desc) = advi.get_string("Description > ".green()) {
            Some(Self::new(desc))
        } else {
            None
        }
    }
    fn change(&mut self) {
        let mut advi = AdvInput::new();
        self.description =
            match advi.get_string_initial("Description > ".green(), &self.description) {
                Some(s) => s,
                None => self.description.clone(),
            };
    }
    fn item_kind_name() -> &'static str {
        "Trait"
    }
}

impl PrintableItem for CharacterTrait {
    fn pretty_print(&self, _character: &Character) -> String {
        format!("| - {}", &self.description)
    }
}
