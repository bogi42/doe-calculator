use crate::interactive::EditableItem;
use advanced_inputs::{AdvInput, PromptableEnum};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum_macros::{Display, EnumIter, EnumString};

/// Represents the different types of attributes a character can have.
/// This includes both basic attributes (like asBody, Mind, Spirit)
/// and derived attributes (like Health, Strength, Intelligence).
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, EnumString, Display,
)]
pub enum AttributeType {
    // Basic Attributes
    Body,
    Mind,
    Spirit,

    // Derived Attributes (examples, you can add many more)
    Health,
    HealthRegen,
    Mana,
    ManaRegen,
    Stamina,
    StaminaRegen,
    PhysicalResistance,
    MentalResilience,
    BasicDamageMelee,
    BasicDamageRanged,
    BasicDamageMagic,
}
impl PromptableEnum for AttributeType {}

/// Defines whether a modifier adds a fixed value or a percentage.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display)]
pub enum ModifierType {
    Fixed,
    Percentage,
}
impl PromptableEnum for ModifierType {}

/// Represents a single modification to an attribute.
/// This can come from gear, professions, titles, temporary buffs, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Modifier {
    pub target_attribute: AttributeType,
    pub modifier_type: ModifierType,
    pub value: f64, // Using f64 for flexibility with percentages
}
impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (value, desc) = match self.modifier_type {
            ModifierType::Fixed => (
                if self.value == 0.0 {
                    format!("Level")
                } else {
                    format!("{}", self.value)
                },
                "Points",
            ),
            ModifierType::Percentage => (format!("{:.2}", self.value * 100.0), "Percent"),
        };
        write!(
            f,
            "{} {} to {}",
            value,
            desc,
            self.target_attribute.display_name()
        )
    }
}

impl Modifier {
    pub fn new(target_attribute: AttributeType, modifier_type: ModifierType, value: f64) -> Self {
        Modifier {
            target_attribute,
            modifier_type,
            value,
        }
    }
}

impl EditableItem for Modifier {
    /// this functions returns a valid Modifier based on User Input, or None if modifier wasn't valid
    fn from_input() -> Option<Self> {
        let mut advi = AdvInput::new();
        println!("{}", "Build new modifier".underline().bold());
        let target_attribute = match advi
            .get_enum_input::<AttributeType>("Choose Attribute (Tab-completion) > ".green(), false)
        {
            Some(t) => t,
            None => return None,
        };
        let modifier_type = match advi.get_enum_input::<ModifierType>(
            "Choose Modifier Type (Tab-completion) > ".green(),
            true,
        ) {
            Some(m) => m,
            None => return None,
        };
        let value = match modifier_type {
            ModifierType::Fixed => match advi.get_f64("Enter value > ".cyan()) {
                Some(v) => v,
                None => return None,
            },
            ModifierType::Percentage => {
                match advi.get_f64_range("Choose percentage (0.01 - 1.00) > ".cyan(), 0.01, 1.00) {
                    Some(v) => v,
                    None => return None,
                }
            }
        };
        Some(Modifier::new(target_attribute, modifier_type, value))
    }

    /// this function changes the current Modifier based on User Input
    fn change(&mut self) {
        let mut advi = AdvInput::new();
        self.target_attribute = match advi.get_enum_input_initial::<AttributeType>(
            format!(
                "Choose Attribute (Tab-Completition) [{}]",
                self.target_attribute
            ),
            Some(self.target_attribute.clone()),
            false,
        ) {
            Some(t) => t,
            None => self.target_attribute.clone(),
        };
        self.modifier_type = match advi.get_enum_input_initial::<ModifierType>(
            format!(
                "Choose ModifierType (Tab-Completition) [{}]",
                self.modifier_type
            ),
            Some(self.modifier_type.clone()),
            false,
        ) {
            Some(t) => t,
            None => self.modifier_type.clone(),
        };
        self.value = match advi.get_f64_initial("Value > ".cyan(), self.value) {
            Some(n) => n,
            None => self.value,
        };
    }

    fn item_kind_name() -> &'static str {
        "Modifier"
    }
}
