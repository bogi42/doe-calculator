use crate::attributes::AttributeType;
use crate::interactive::{EditableItem, PrintableItem};
use crate::Character;
use advanced_inputs::{AdvInput, PromptableEnum};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum_macros::{Display, EnumIter, EnumString};

/// Represents the type of Weapon
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display)]
pub enum WeaponType {
    Melee,
    Ranged,
    SoulWeapon,
}
impl PromptableEnum for WeaponType {}

/// Represents a weapon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub name: String,
    pub weapon_type: WeaponType,
    pub used_attribute: AttributeType,
    pub factor: f64,
    pub description: String,
}
impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}) - Damage Factor: {:.1}",
            self.name,
            self.weapon_type.display_name(),
            self.factor
        )
    }
}

fn get_attribute(weapon_type: &WeaponType) -> AttributeType {
    match weapon_type {
        WeaponType::Melee => AttributeType::BasicDamageMelee,
        WeaponType::Ranged => AttributeType::BasicDamageRanged,
        WeaponType::SoulWeapon => AttributeType::BasicDamageMagic,
    }
}

impl Weapon {
    pub fn new(name: String, weapon_type: WeaponType, factor: f64, description: String) -> Self {
        Weapon {
            name,
            used_attribute: get_attribute(&weapon_type),
            weapon_type,
            factor,
            description,
        }
    }

    pub fn get_output(&self, character: &Character) -> f64 {
        self.factor * character.calculate_final_attribute(&self.used_attribute)
    }
}

impl EditableItem for Weapon {
    /// Returns a Weapon from User Input, `None` if not applicable
    fn from_input() -> Option<Self> {
        println!("{:^100}", "Create a new Weapon:".bold().underline());
        let mut advi = AdvInput::new();
        let name = match advi.get_string("Weapon Name > ".green()) {
            Some(s) => s,
            None => return None,
        };
        let weapon_type = match advi.get_enum_input::<WeaponType>("Type of Weapon > ".green(), true)
        {
            Some(t) => t,
            None => return None,
        };
        let factor = match advi.get_f64("Factor > ".green()) {
            Some(n) => n,
            None => return None,
        };
        let description = match advi.get_string("(Optional) Description > ".green()) {
            Some(s) => s,
            None => return None,
        };
        Some(Self::new(name, weapon_type, factor, description))
    }

    fn change(&mut self) {
        println!("{:^100}", "Create a new Weapon:".bold().underline());
        let mut advi = AdvInput::new();
        self.name = match advi.get_string_initial("Weapon Name > ".green(), &self.name) {
            Some(v) => v,
            None => self.name.clone(),
        };
        self.weapon_type = match advi.get_enum_input_initial::<WeaponType>(
            "Type of Weapon > ".green(),
            Some(self.weapon_type.clone()),
            true,
        ) {
            Some(v) => v,
            None => self.weapon_type.clone(),
        };
        self.used_attribute = get_attribute(&self.weapon_type);
        self.factor = match advi.get_f64_initial("Factor > ".green(), self.factor) {
            Some(v) => v,
            None => self.factor,
        };
        self.description =
            match advi.get_string_initial("(Optional) Description > ".green(), &self.description) {
                Some(v) => v,
                None => self.description.clone(),
            };
    }

    fn item_kind_name() -> &'static str {
        "Weapon"
    }
}

impl PrintableItem for Weapon {
    fn pretty_print(&self, character: &Character) -> String {
        format!(
            "<b>{}:</b> {:.2} Damage<br/>╰╼({}, Factor: {:.1})<br/>{}",
            self.name,
            self.get_output(&character),
            self.weapon_type.display_name(),
            self.factor,
            if self.description.is_empty() {
                String::new()
            } else {
                format!("╰╼<i>{}</i><br/>", self.description)
            }
        )
    }
}
