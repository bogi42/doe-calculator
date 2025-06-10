use crate::attributes::Modifier;
use crate::interactive::{change_items, EditableItem};
use advanced_inputs::AdvInput;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;
/// Represents a character's profession.
/// For simplicity, we'll assume professions grant fixed points to attributes on level up.
/// These "level up points" will be applied directly to the character's base attributes,
/// rather than being treated as dynamic modifiers from this struct.
/// This struct primarily tracks the profession's name and its level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profession {
    pub name: String,
    pub level: u32,
    pub lvlup_body: f64,
    pub lvlup_mind: f64,
    pub lvlup_spirit: f64,
    pub lvlup_free: f64,
    pub profession_specific_modifiers: Vec<Modifier>,
}
impl fmt::Display for Profession {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let level = if self.level > 0 {
            &format!("Lvl: {}", self.level)
        } else {
            "Class"
        };
        write!(
            f,
            "{} ({}) - {} specific modifiers\n- per Level: {} Body, {} Mind, {} Spirit, {} Free",
            self.name.bold().cyan(),
            level,
            self.profession_specific_modifiers.len(),
            self.lvlup_body,
            self.lvlup_mind,
            self.lvlup_spirit,
            self.lvlup_free
        )
    }
}

impl Profession {
    pub fn new(
        name: String,
        level: u32,
        lvlup_body: f64,
        lvlup_mind: f64,
        lvlup_spirit: f64,
        lvlup_free: f64,
        profession_specific_modifiers: Vec<Modifier>,
    ) -> Self {
        Profession {
            name,
            level,
            lvlup_body,
            lvlup_mind,
            lvlup_spirit,
            lvlup_free,
            profession_specific_modifiers,
        }
    }
}

impl EditableItem for Profession {
    /// Returns a Profession from User ipnut, or None if not applicable
    fn from_input() -> Option<Self> {
        println!("{:^100}", "Create a new Profession:".bold().underline());
        let mut advi = AdvInput::new();
        let name = match advi.get_string("Enter Name > ".cyan()) {
            Some(s) => s,
            None => return None,
        };
        let level = match advi.get_index("Level (0 for class) > ".cyan()) {
            Some(n) => n as u32,
            None => return None,
        };
        let lvlup_body = match advi.get_f64("Points to Body on Level-Up > ".cyan()) {
            Some(n) => n,
            None => return None,
        };
        let lvlup_mind = match advi.get_f64("Points to Mind on Level-Up > ".cyan()) {
            Some(n) => n,
            None => return None,
        };
        let lvlup_spirit = match advi.get_f64("Points to Spirit on Level-Up > ".cyan()) {
            Some(n) => n,
            None => return None,
        };
        let lvlup_free = match advi.get_f64("Free Points on Level-Up > ".cyan()) {
            Some(n) => n,
            None => return None,
        };
        /* build a list of modifiers */
        let mut profession_specific_modifiers: Vec<Modifier> = Vec::new();
        println!("{}", "Building List of Modifiers:".underline());
        loop {
            match Modifier::from_input() {
                Some(m) => profession_specific_modifiers.push(m),
                None => break,
            }
        }
        Some(Self::new(
            name,
            level,
            lvlup_body,
            lvlup_mind,
            lvlup_spirit,
            lvlup_free,
            profession_specific_modifiers,
        ))
    }

    fn change(&mut self) {
        println!("{:^100}", "Change Profession:".bold().underline());
        let mut advi = AdvInput::new();
        self.name = match advi.get_string_initial("Name > ".green(), &self.name) {
            Some(s) => s,
            None => self.name.clone(),
        };
        self.level =
            match advi.get_index_initial("Level (0 for class) > ".cyan(), self.level as usize) {
                Some(n) => n as u32,
                None => self.level,
            };
        self.lvlup_body = match advi.get_f64_initial("Body on Level-Up > ".cyan(), self.lvlup_body)
        {
            Some(n) => n,
            None => self.lvlup_body,
        };
        self.lvlup_mind = match advi.get_f64_initial("Mind on Level-Up > ".cyan(), self.lvlup_mind)
        {
            Some(n) => n,
            None => self.lvlup_mind,
        };
        self.lvlup_spirit =
            match advi.get_f64_initial("Spirit on Level-Up > ".cyan(), self.lvlup_spirit) {
                Some(n) => n,
                None => self.lvlup_spirit,
            };
        self.lvlup_free =
            match advi.get_f64_initial("Free Points on Level-Up > ".cyan(), self.lvlup_free) {
                Some(n) => n,
                None => self.lvlup_free,
            };
        change_items(&mut self.profession_specific_modifiers, &self.name);
    }

    fn item_kind_name() -> &'static str {
        "Profession"
    }
}
