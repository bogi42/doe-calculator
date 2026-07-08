use crate::attributes::Modifier; // Import necessary types from the 'attributes' module
use crate::interactive::{change_items, EditableItem, PrintableItem};
use crate::Character;
use advanced_inputs::{AdvInput, PromptableEnum};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum_macros::{Display, EnumIter, EnumString};

/// Represents the type of gear (e.g., equipment, a title, a consumable).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display)]
pub enum GearType {
    Equipment,
    Title,
    BoneGlyph,
    Bond,
    Attunement,
}
impl PromptableEnum for GearType {}

/// Represents a piece of gear, which can apply one or more modifiers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gear {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub gear_type: GearType,
    pub modifiers: Vec<Modifier>, // A vector of modifiers applied by this gear
}
impl fmt::Display for Gear {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mods = if self.modifiers.is_empty() {
            String::new()
        } else {
            self.modifiers
                .iter()
                .map(|m| format!("\n  - {}", m))
                .collect::<Vec<String>>()
                .join("")
        };
        write!(
            f,
            "{} ({}) - {} specific modifiers\n{}\n{}",
            self.name.bold().cyan(),
            self.gear_type.display_name(),
            self.modifiers.len(),
            self.description.italic(),
            mods.italic()
        )
    }
}

impl Gear {
    pub fn new(
        name: String,
        description: String,
        gear_type: GearType,
        modifiers: Vec<Modifier>,
    ) -> Self {
        Gear {
            name: name.into(),
            description,
            gear_type,
            modifiers,
        }
    }
}
impl EditableItem for Gear {
    /// Returns a Gear from User input, or None if not applicable
    fn from_input() -> Option<Self> {
        println!("{:^100}", "Create a new Gear:".bold().underline());
        let mut advi = AdvInput::new();
        let name = match advi.get_string("Enter Name > ".green()) {
            Some(s) => s,
            None => return None,
        };
        let description = match advi.get_string("Description > ".green()) {
            Some(s) => s,
            None => return None,
        };
        let gear_type = match advi.get_enum_input("What type? (TAB) > ".green(), true) {
            Some(g) => g,
            None => return None,
        };
        /* build a list of modifiers */
        let mut modifiers: Vec<Modifier> = Vec::new();
        println!("{}", "Building List of Modifiers:".underline());
        loop {
            match Modifier::from_input() {
                Some(m) => modifiers.push(m),
                None => break,
            }
        }
        if modifiers.is_empty() {
            None
        } else {
            Some(Gear::new(name, description, gear_type, modifiers))
        }
    }

    fn change(&mut self) {
        println!("{:^100}", "Change Gear:".bold().underline());
        let mut advi = AdvInput::new();
        self.name = match advi.get_string_initial("Name > ".green(), &self.name) {
            Some(s) => s,
            None => self.name.clone(),
        };
        self.description =
            match advi.get_string_initial("Description > ".green(), &self.description) {
                Some(s) => s,
                None => self.description.clone(),
            };
        self.gear_type = match advi.get_enum_input_initial::<GearType>(
            "What type? (TAB) > ".green(),
            Some(self.gear_type.clone()),
            true,
        ) {
            Some(g) => g,
            None => self.gear_type.clone(),
        };
        change_items::<Modifier>(&mut self.modifiers, &self.name);
    }

    fn item_kind_name() -> &'static str {
        "Gear"
    }
}

impl PrintableItem for Gear {
    fn pretty_print(&self, _character: &Character) -> String {
        /* build list of Modifiers */
        let mut mod_list: Vec<String> = Vec::new();
        for modifier in &self.modifiers {
            mod_list.push(format!("| - {}", modifier));
        }
        let description_line = if self.description.is_empty() {
            String::new()
        } else {
            format!("| *{}*", self.description)
        };
        format!(
            "| **{}** ({})\n{}{}",
            self.name,
            self.gear_type.display_name(),
            description_line,
            mod_list.join("\n")
        )
    }
}
