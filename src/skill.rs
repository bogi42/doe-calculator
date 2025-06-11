use crate::interactive::{EditableItem, PrintableItem};
use crate::Character;
use advanced_inputs::AdvInput;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a skill
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub stamina_cost: f64,
    pub cooldown: String,
    pub main_effect: String,
    pub additional_effect: String,
}
impl fmt::Display for Skill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} [{}] - {}",
            self.name,
            self.get_cost_cd(),
            self.main_effect
        )
    }
}

impl Skill {
    pub fn new(
        name: String,
        stamina_cost: f64,
        cooldown: String,
        main_effect: String,
        additional_effect: String,
    ) -> Self {
        Skill {
            name,
            stamina_cost,
            cooldown,
            main_effect,
            additional_effect,
        }
    }

    pub fn get_cost_cd(&self) -> String {
        if self.stamina_cost == 0.0 {
            "(passive)".to_string()
        } else {
            format!(
                "({:.0} Stamina / {} Cooldown)",
                self.stamina_cost, self.cooldown
            )
        }
    }
}

impl EditableItem for Skill {
    fn from_input() -> Option<Self> {
        println!("{:^100}", "Create a new Skill:".bold().underline());
        let mut advi = AdvInput::new();
        let name = match advi.get_string("Skill Name > ".green()) {
            Some(s) => s,
            None => return None,
        };
        let stamina_cost = match advi.get_f64("Stamina Cost > ".green()) {
            Some(n) => n,
            None => return None,
        };
        let cooldown = if stamina_cost > 0.0 {
            match advi.get_string("Cooldown > ".green()) {
                Some(s) => s,
                None => return None,
            }
        } else {
            String::new()
        };
        let main_effect = match advi.get_string("Main Effect > ".green()) {
            Some(s) => s,
            None => return None,
        };
        let additional_effect = match advi.get_string("Addition Effect > ".green()) {
            Some(s) => s,
            None => return None,
        };
        Some(Self::new(
            name,
            stamina_cost,
            cooldown,
            main_effect,
            additional_effect,
        ))
    }

    fn change(&mut self) {
        println!("{:^100}", "Change Skill:".bold().underline());
        let mut advi = AdvInput::new();
        self.name = match advi.get_string_initial("Skill Name > ".green(), &self.name) {
            Some(v) => v,
            None => self.name.clone(),
        };
        self.stamina_cost = match advi.get_f64_initial("Stamina Cost > ".green(), self.stamina_cost)
        {
            Some(v) => v,
            None => self.stamina_cost,
        };
        self.cooldown = if self.stamina_cost > 0.0 {
            match advi.get_string_initial("Cooldown > ".green(), &self.cooldown) {
                Some(v) => v,
                None => self.cooldown.clone(),
            }
        } else {
            String::new()
        };
        self.main_effect =
            match advi.get_string_initial("Main Effect > ".green(), &self.main_effect) {
                Some(v) => v,
                None => self.main_effect.clone(),
            };
        self.additional_effect = match advi
            .get_string_initial("Additional Effect > ".green(), &self.additional_effect)
        {
            Some(v) => v,
            None => self.additional_effect.clone(),
        };
    }

    fn item_kind_name() -> &'static str {
        "Skill"
    }
}

impl PrintableItem for Skill {
    fn pretty_print(&self, _character: &Character) -> String {
        format!(
            "<b>{}:</b> {}<br/>╰╼{}<br/>{}",
            self.name,
            self.main_effect,
            self.get_cost_cd(),
            if self.additional_effect.is_empty() {
                String::new()
            } else {
                format!("╰╼({})<br/>", self.additional_effect)
            }
        )
    }
}
