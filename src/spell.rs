use crate::attributes::AttributeType;
use crate::interactive::{EditableItem, PrintableItem};
use crate::Character;
use advanced_inputs::{AdvInput, PromptableEnum};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum_macros::{Display, EnumIter, EnumString};
use thousands::Separable;

/// Represents the type of spell
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display)]
pub enum SpellType {
    Damage,
    Healing,
    Control,
}
impl PromptableEnum for SpellType {}

/// Represents the Spell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spell {
    pub name: String,
    pub spell_type: SpellType,
    pub mana_cost: f64,
    pub cooldown: String,
    pub used_attribute: Option<AttributeType>,
    pub factor: f64,
    #[serde(default)]
    pub control_effect: String,
    pub additional_effect: String,
}
impl fmt::Display for Spell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} [{}] - {} Mana, {}",
            self.name, self.spell_type, self.mana_cost, self.cooldown
        )
    }
}

impl Spell {
    pub fn new(
        name: String,
        spell_type: SpellType,
        mana_cost: f64,
        cooldown: String,
        used_attribute: Option<AttributeType>,
        factor: f64,
        control_effect: String,
        additional_effect: String,
    ) -> Self {
        Spell {
            name,
            spell_type,
            mana_cost,
            cooldown,
            used_attribute,
            factor,
            control_effect,
            additional_effect,
        }
    }

    pub fn get_output(&self, character: &Character) -> String {
        match self.spell_type {
            SpellType::Control => self.control_effect.clone(),
            SpellType::Damage | SpellType::Healing => {
                let value = character
                    .calculate_final_attribute(&self.used_attribute.as_ref().unwrap())
                    * self.factor;
                format!(
                    "{} {}",
                    format!("{:.2}", value).separate_with_commas(),
                    self.spell_type
                )
            }
        }
    }
}

impl EditableItem for Spell {
    /// Returns a Spell from User input, NOne if not applicable
    fn from_input() -> Option<Self> {
        println!("{:^100}", "Create a new Spell:".bold().underline());
        let mut advi = AdvInput::new();
        let name = match advi.get_string("Spell Name > ".green()) {
            Some(s) => s,
            None => return None,
        };
        let spell_type = match advi.get_enum_input::<SpellType>("Type of Spell > ".green(), true) {
            Some(t) => t,
            None => return None,
        };
        let mana_cost = match advi.get_f64("Mana Cost > ".green()) {
            Some(n) => n,
            None => return None,
        };
        let cooldown = match advi.get_string("Cooldown > ".green()) {
            Some(s) => s,
            None => return None,
        };
        let used_attribute = if spell_type == SpellType::Control {
            None
        } else {
            match advi.get_enum_input::<AttributeType>("Attribute to use > ".green(), false) {
                Some(a) => Some(a),
                None => return None,
            }
        };
        let factor = if spell_type == SpellType::Control {
            0.0
        } else {
            match advi.get_f64("Factor > ".green()) {
                Some(n) => n,
                None => return None,
            }
        };
        let control_effect = if spell_type == SpellType::Control {
            match advi.get_string("Control Effect > ".green()) {
                Some(s) => s,
                None => return None,
            }
        } else {
            String::new()
        };
        let additional_effect = match advi.get_string("Additional Effect > ".green()) {
            Some(s) => s,
            None => return None,
        };
        Some(Self::new(
            name,
            spell_type,
            mana_cost,
            cooldown,
            used_attribute,
            factor,
            control_effect,
            additional_effect,
        ))
    }

    /// changes current Spell from User input
    fn change(&mut self) {
        println!("{:^100}", "Change Spell:".bold().underline());
        let mut advi = AdvInput::new();
        self.name = match advi.get_string_initial("Spell Name > ".green(), &self.name) {
            Some(s) => s,
            None => self.name.clone(),
        };
        self.spell_type = match advi.get_enum_input_initial::<SpellType>(
            "Type of Spell > ".green(),
            Some(self.spell_type.clone()),
            true,
        ) {
            Some(t) => t,
            None => self.spell_type.clone(),
        };
        self.mana_cost = match advi.get_f64_initial("Mana Cost > ".green(), self.mana_cost) {
            Some(n) => n,
            None => self.mana_cost,
        };
        self.cooldown = match advi.get_string_initial("Cooldown > ".green(), &self.cooldown) {
            Some(s) => s,
            None => self.cooldown.clone(),
        };
        self.used_attribute = if self.spell_type == SpellType::Control {
            None
        } else {
            match advi.get_enum_input_initial::<AttributeType>(
                "Attribute to use > ".green(),
                self.used_attribute.clone(),
                false,
            ) {
                Some(a) => Some(a),
                None => self.used_attribute.clone(),
            }
        };
        self.factor = if self.spell_type == SpellType::Control {
            0.0
        } else {
            match advi.get_f64_initial("Factor > ".green(), self.factor) {
                Some(n) => n,
                None => self.factor,
            }
        };
        self.control_effect = if self.spell_type == SpellType::Control {
            match advi.get_string_initial("Control Effect > ".green(), &self.control_effect) {
                Some(s) => s,
                None => self.control_effect.clone(),
            }
        } else {
            String::new()
        };
        self.additional_effect = match advi
            .get_string_initial("Additional Effect > ".green(), &self.additional_effect)
        {
            Some(s) => s,
            None => self.additional_effect.clone(),
        };
    }
    fn item_kind_name() -> &'static str {
        "Spell"
    }
}

impl PrintableItem for Spell {
    fn pretty_print(&self, character: &Character) -> String {
        format!(
            "| **{}:** {}\n| ╰╼({} Mana / {}){}",
            self.name,
            self.get_output(&character),
            format!("{:.0}", self.mana_cost).separate_with_commas(),
            self.cooldown,
            if self.additional_effect.is_empty() {
                String::new()
            } else {
                format!("\n| ╰╼({})", self.additional_effect)
            }
        )
    }
}
