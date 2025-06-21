use crate::attributes::{AttributeType, ModifierType}; // Import AttributeType and ModifierType from attributes module
use crate::gear::{Gear, GearType}; // Import Gear and GearType from gear module
use crate::interactive::{PrintableItem, PrintableItemVariant};
use crate::notes::Note;
use crate::profession::Profession;
use crate::skill::Skill;
use crate::spell::Spell;
use crate::traits::CharacterTrait;
use crate::weapon::Weapon;
use advanced_inputs::{promptable_enum::add_spaces_before_caps, AdvInput};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap; // Import Profession from profession module
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use strum::IntoEnumIterator;
use thousands::Separable;

/// Represents the main character, holding their stats, gear, and professions.
#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub level: u32,
    pub experience: u32,
    // Stores the character's base attribute values. These are the values
    // before any external modifiers (from gear, titles, etc.) are applied.
    // These base values would typically increase with character level and
    // profession levels.
    pub base_attributes: HashMap<AttributeType, f64>,
    // Attunements for Body, Spirit and Mind are optional
    pub body_attunement: Option<Attunement>,
    pub mind_attunement: Option<Attunement>,
    pub spirit_attunement: Option<Attunement>,
    #[serde(default)]
    pub soul_attunement: Option<Attunement>,
    pub professions: Vec<Profession>,
    pub equipped_gear: Vec<Gear>,
    #[serde(default)]
    pub spells: Vec<Spell>,
    #[serde(default)]
    pub skills: Vec<Skill>,
    #[serde(default)]
    pub weapons: Vec<Weapon>,
    #[serde(default)]
    pub notes: Vec<Note>,
    pub traits: Vec<CharacterTrait>,
    pub file_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attunement {
    pub element: String,
    pub title: String,
}

impl Attunement {
    pub fn new(element: String, title: String) -> Self {
        Attunement { element, title }
    }

    pub fn from_input() -> Option<Self> {
        let mut advi = AdvInput::new();
        let element = match advi.get_string("Element > ".green()) {
            Some(s) => s,
            None => return None,
        };
        let title = match advi.get_string("Title > ".green()) {
            Some(s) => s,
            None => return None,
        };
        Some(Self::new(element, title))
    }

    pub fn change(&mut self) {
        let mut advi = AdvInput::new();
        let (old_element, old_title) = (self.element.clone(), self.title.clone());
        self.element = match advi.get_string_initial("Element > ", old_element.as_str()) {
            Some(s) => s,
            None => old_element,
        };
        self.title = match advi.get_string_initial("Element > ", old_title.as_str()) {
            Some(s) => s,
            None => old_title,
        };
    }
}

impl Character {
    /// Creates a new character with initial base attributes.
    pub fn new(name: String, file_path: PathBuf) -> Self {
        Character {
            name,
            level: 1,
            experience: 0,
            base_attributes: HashMap::new(),
            body_attunement: None,
            mind_attunement: None,
            spirit_attunement: None,
            soul_attunement: None,
            professions: Vec::new(),
            equipped_gear: Vec::new(),
            spells: Vec::new(),
            skills: Vec::new(),
            traits: Vec::new(),
            weapons: Vec::new(),
            notes: Vec::new(),
            file_path,
        }
    }

    /// Sets or updates a base attribute value for the character.
    pub fn set_base_attribute(&mut self, attr_type: AttributeType, value: f64) {
        /* only Body, Mind, and Spirit can be set directly */
        match attr_type {
            AttributeType::Body
            | AttributeType::Mind
            | AttributeType::Spirit
            | AttributeType::Soul => {
                self.base_attributes.insert(attr_type, value);
            }
            _ => return,
        }
    }

    /// returns the amount of needed exp for the next level
    pub fn get_exp_needed(&self) -> u32 {
        self.level * 100
    }

    /// Levels up the character.
    pub fn level_up(&mut self) {
        let exp_needed = self.get_exp_needed();
        if self.experience >= exp_needed {
            let mut advi = AdvInput::new();
            println!(
                "{}",
                format!("Leveling up from {} to {}:", self.level, self.level + 1).underline()
            );
            /* collect fixed level up points from Professions */
            let mut body_up: f64 = 0.0;
            let mut mind_up: f64 = 0.0;
            let mut spirit_up: f64 = 0.0;
            let mut soul_up: f64 = 0.0;
            let mut free_up: f64 = 0.0;
            for prof in &self.professions {
                body_up += prof.lvlup_body;
                mind_up += prof.lvlup_mind;
                spirit_up += prof.lvlup_spirit;
                soul_up += prof.lvlup_soul;
                free_up += prof.lvlup_free;
            }
            println!("- fixed progressions from Professions:");
            println!("  {}: {}", "Body".cyan(), format!("{}", body_up).green());
            println!("  {}: {}", "Mind".cyan(), format!("{}", mind_up).green());
            println!(
                "  {}: {}",
                "Spirit".cyan(),
                format!("{}", spirit_up).green()
            );
            println!("  {}: {}", "Soul".cyan(), format!("{}", soul_up).green());
            println!("  {}: {}", "Free".cyan(), format!("{}", free_up).green());
            println!(
                "Enter distribution for free points: {} (separated by spaces)",
                "body mind spirit soul".green()
            );
            loop {
                let istr: Vec<String> = match advi.get_string("> ".green()) {
                    Some(s) => s
                        .split_whitespace()
                        .map(|slice| slice.to_string())
                        .collect(),
                    None => return (), // quit function
                };
                if istr.len() != 4 {
                    eprintln!("Wrong number of arguments, try again");
                    continue;
                }
                let free_body: f64 = match istr[0].parse::<f64>() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("{}", "Invalid value for body, try again".red());
                        continue;
                    }
                };
                let free_mind: f64 = match istr[1].parse::<f64>() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("{}", "Invalid value for mind, try again".red());
                        continue;
                    }
                };
                let free_spirit: f64 = match istr[2].parse::<f64>() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("{}", "Invalid value for spirit, try again".red());
                        continue;
                    }
                };
                let free_soul: f64 = match istr[3].parse::<f64>() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("{}", "Invalid value for sould, try again".red());
                        continue;
                    }
                };
                if free_spirit + free_mind + free_body + free_soul != free_up {
                    eprintln!(
                        "{}",
                        format!(
                            "Numbers don't add up: {} +  {}  + {} + {} != {}",
                            free_body, free_mind, free_spirit, free_soul, free_up
                        )
                        .red()
                    );
                    continue;
                }
                /* numbers add up, great! */
                body_up += free_body;
                mind_up += free_mind;
                spirit_up += free_spirit;
                soul_up += free_soul;
                /* change base values */
                self.level += 1;
                self.experience -= exp_needed;
                self.add_to_attribute(AttributeType::Body, body_up);
                self.add_to_attribute(AttributeType::Mind, mind_up);
                self.add_to_attribute(AttributeType::Spirit, spirit_up);
                self.add_to_attribute(AttributeType::Soul, soul_up);
                if self.level <= 20 || self.level % 5 == 0 {
                    println!(
                        "{}",
                        "You should add/upgrade a Skill/Spell".bold().bright_cyan()
                    );
                }
                break;
            }
        } else {
            println!(
                "{}",
                format!(
                    "Cannot level up, insuffiecient experience ({}/{})",
                    self.experience,
                    self.get_exp_needed()
                )
                .bright_red()
                .bold()
            );
            let mut advi = AdvInput::new();
            _ = advi.get_string("[press enter to continue]".italic());
        }
    }

    pub fn add_to_attribute(&mut self, attribute: AttributeType, value: f64) {
        let current = self.get_base_attribute(&attribute);
        self.set_base_attribute(attribute, current + value);
    }

    pub fn get_base_attribute(&self, attribute: &AttributeType) -> f64 {
        /* only Body, Mind, Spirit, and Soul can be gotten directly */
        match attribute {
            AttributeType::Body
            | AttributeType::Mind
            | AttributeType::Spirit
            | AttributeType::Soul => *self.base_attributes.get(&attribute).unwrap_or(&0.0),
            _ => 0.0,
        }
    }

    pub fn get_attunement_string(&self, attr: &AttributeType) -> String {
        let att_option = match attr {
            AttributeType::Body => &self.body_attunement,
            AttributeType::Mind => &self.mind_attunement,
            AttributeType::Spirit => &self.spirit_attunement,
            AttributeType::Soul => &self.soul_attunement,
            _ => &None,
        };
        if let Some(att) = att_option {
            format!("{}: {}", att.element, att.title)
        } else {
            "None".to_string()
        }
    }

    /// Calculates the final value of a given attribute,
    /// applying base value, fixed modifiers, and percentage modifiers.
    ///
    /// The formula used is: (Base-Value + Fixed Modifiers) * (1 + sum of percentage modifiers).
    pub fn calculate_final_attribute(&self, attr_type: &AttributeType) -> f64 {
        let base_value = match attr_type {
            AttributeType::Body
            | AttributeType::Mind
            | AttributeType::Spirit
            | AttributeType::Soul => self.get_base_attribute(attr_type),
            AttributeType::Health => {
                self.calculate_final_attribute(&AttributeType::Body) * 10.0
                    + self.level as f64 * 5.0
            }
            AttributeType::HealthRegen => {
                self.calculate_final_attribute(&AttributeType::Body) / 10.0
            }
            AttributeType::Mana => {
                self.calculate_final_attribute(&AttributeType::Spirit) * 10.0
                    + self.calculate_final_attribute(&AttributeType::Mind) * 5.0
            }
            AttributeType::ManaRegen => {
                (self.calculate_final_attribute(&AttributeType::Mind)
                    + self.calculate_final_attribute(&AttributeType::Spirit))
                    / 20.0
            }
            AttributeType::Stamina => {
                self.calculate_final_attribute(&AttributeType::Body) * 10.0
                    + self.calculate_final_attribute(&AttributeType::Spirit) * 5.0
            }
            AttributeType::StaminaRegen => {
                (self.calculate_final_attribute(&AttributeType::Body)
                    + self.calculate_final_attribute(&AttributeType::Spirit))
                    / 20.0
            }
            AttributeType::PhysicalResistance | AttributeType::BasicDamageMelee => {
                self.calculate_final_attribute(&AttributeType::Body) * 2.0
            }
            AttributeType::MentalResilience | AttributeType::BasicDamageMagic => {
                self.calculate_final_attribute(&AttributeType::Mind)
                    + self.calculate_final_attribute(&AttributeType::Spirit)
            }
            AttributeType::BasicDamageRanged => {
                self.calculate_final_attribute(&AttributeType::Body)
                    + self.calculate_final_attribute(&AttributeType::Mind)
            }
            AttributeType::BasicDamageSoul => {
                self.calculate_final_attribute(&AttributeType::Soul) * 2.0
            }
        };

        let (tfm, tpm) = self.get_modifiers(attr_type);

        // Apply the formula: (Base-Value + Fixed Modifiers) * (1 + sum of percentage modifiers)
        (base_value + tfm) * (1.0 + tpm)
    }

    // returns the fixed and percentage modifiers for the given attribute type
    fn get_modifiers(&self, attribute: &AttributeType) -> (f64, f64) {
        let mut fixed_modifier = 0.0;
        let mut percentage_modifier = 0.0;
        for gear in &self.equipped_gear {
            for modifier in &gear.modifiers {
                if modifier.target_attribute == *attribute {
                    match modifier.modifier_type {
                        ModifierType::Fixed => {
                            fixed_modifier += if modifier.value == 0.0 {
                                self.level as f64
                            } else {
                                modifier.value
                            }
                        }
                        ModifierType::Percentage => percentage_modifier += modifier.value,
                    }
                }
            }
        }
        for prof in &self.professions {
            for modifier in &prof.profession_specific_modifiers {
                if modifier.target_attribute == *attribute {
                    match modifier.modifier_type {
                        ModifierType::Fixed => fixed_modifier += modifier.value,
                        ModifierType::Percentage => percentage_modifier += modifier.value,
                    }
                }
            }
        }
        (fixed_modifier, percentage_modifier)
    }

    /// A helper function to display character stats.
    pub fn display_stats(&self) {
        println!(
            "\n{:^100}",
            format!("Character Stats: {}", self.name)
                .bold()
                .underline()
                .bright_purple()
        );
        println!(
            " {:>25}{}{:>25}{}{:>25}{}",
            format!("{}", "Level:").bold(),
            format!("{:8.0}", self.level).cyan().bold(),
            format!("{}", "Experience:").bold(),
            format!("{:8.0}", self.experience).cyan().bold(),
            format!("{}", "to next Level:").italic(),
            format!("{:8.0}", &self.get_exp_needed())
                .cyan()
                .bold()
                .italic(),
        );
        /* attributes */
        println!(
            " {:>10}{:^15}{:8.0}{:>10}{:^15}{:8.0}{:>10}{:^15}{:8.0}",
            "Body:".bold().green(),
            format!(
                "(Base: {:.0})",
                self.get_base_attribute(&AttributeType::Body)
            )
            .italic(),
            self.calculate_final_attribute(&AttributeType::Body),
            "Mind:".bold().green(),
            format!(
                "(Base: {:.0})",
                self.get_base_attribute(&AttributeType::Mind)
            )
            .italic(),
            self.calculate_final_attribute(&AttributeType::Mind),
            "Spirit:".bold().green(),
            format!(
                "(Base: {:.0})",
                self.get_base_attribute(&AttributeType::Spirit)
            )
            .italic(),
            self.calculate_final_attribute(&AttributeType::Spirit)
        );
        /* Attunements */
        println!(
            " {:>33}{:>33}{:>33}",
            self.get_attunement_string(&AttributeType::Body).italic(),
            self.get_attunement_string(&AttributeType::Mind).italic(),
            self.get_attunement_string(&AttributeType::Spirit).italic()
        );
        print_line();
        /* Soul line */
        println!(
            " {:>10}{:^15}{:8.0}{:>66}",
            "Soul:".bold().green(),
            format!(
                "(Base: {:.0})",
                self.get_base_attribute(&AttributeType::Soul)
            )
            .italic(),
            self.calculate_final_attribute(&AttributeType::Soul),
            self.get_attunement_string(&AttributeType::Soul).italic()
        );
        print_line();
        /* Health, Mana, Stamina */
        println!(
            " {:>25}{:8.0}{:>25}{:8.0}{:>25}{:8.0}",
            "Health",
            self.calculate_final_attribute(&AttributeType::Health),
            "Mana",
            self.calculate_final_attribute(&AttributeType::Mana),
            "Stamina",
            self.calculate_final_attribute(&AttributeType::Stamina)
        );
        /* regeneration */
        println!(
            " {0:>25}{1:8.2}{0:>25}{2:8.2}{0:>25}{3:8.2}",
            "per second".italic(),
            self.calculate_final_attribute(&AttributeType::HealthRegen),
            self.calculate_final_attribute(&AttributeType::ManaRegen),
            self.calculate_final_attribute(&AttributeType::StaminaRegen)
        );
        print_line();
        /* Base Damages */
        println!(
            "Damage: {:>15}{:>8}{:>15}{:>8}{:>15}{:>8}{:15}{:>8}",
            "Melee",
            format!(
                "{:.0}",
                self.calculate_final_attribute(&AttributeType::BasicDamageMelee)
            )
            .separate_with_commas(),
            "Ranged",
            format!(
                "{:.0}",
                self.calculate_final_attribute(&AttributeType::BasicDamageRanged)
            )
            .separate_with_commas(),
            "Magic",
            format!(
                "{:.0}",
                self.calculate_final_attribute(&AttributeType::BasicDamageMagic)
            )
            .separate_with_commas(),
            "Soul",
            format!(
                "{:.0}",
                self.calculate_final_attribute(&AttributeType::BasicDamageSoul)
            )
            .separate_with_commas()
        );
        print_line();
        /* Protection & Traits */
        println!(
            " {:>25}{:8.0}{:>25}{:8.0}{:>12}{} {:>12}{}",
            "Physical Resistance",
            self.calculate_final_attribute(&AttributeType::PhysicalResistance),
            "Mental Resilience",
            self.calculate_final_attribute(&AttributeType::MentalResilience),
            "Traits:".bright_yellow(),
            format!("{:4.0}", self.traits.len()).bright_yellow(),
            "Notes:".bright_green(),
            format!("{:4.0}", self.notes.len()).bright_green()
        );
        print_line();
        /* Spells and skills */
        println!(
            "{:>42}{:8.0}{:>42}{:8.0}",
            "Spells:",
            self.spells.len(),
            "Skills:",
            self.skills.len()
        );
        print_line();
        /* professions */
        print!("{}", "Professions:".bold().underline());
        if self.professions.is_empty() {
            println!("{}", "  None".red());
        } else {
            for (i, profession) in self.professions.iter().enumerate() {
                /* new line before every second item (and this first) */
                if i % 2 == 0 {
                    print!("\n");
                }
                print!(
                    " {:>33} {:>15}",
                    profession.name,
                    if profession.level > 0 {
                        format!("(Lvl {})", profession.level).cyan()
                    } else {
                        "(Class)".bright_cyan()
                    }
                );
            }
            println!("");
        }
        print_line();
        print!("{}", "Equipped Gear types:".bold().underline());
        if self.equipped_gear.is_empty() {
            println!("{}", "  None".red());
        } else {
            let mut i: usize = 0;
            for gear in GearType::iter() {
                /* new line before every second item (and the first) */
                if i % 2 == 0 {
                    print!("\n");
                }
                print!(
                    " {:>39}: {}",
                    add_spaces_before_caps(&gear.to_string()),
                    format!("{:8}", self.count_gear(&gear)).cyan()
                );
                i += 1;
            }
            // After GearTypes, add Weapon
            if i % 2 == 0 {
                print!("\n");
            }
            println!(
                " {:>39}: {}",
                "Weapons",
                format!("{:8}", self.weapons.len()).cyan()
            );
        }
        print_line();
    }

    pub fn pretty_print(&self) -> String {
        let mut lines: Vec<String> = Vec::new();
        /* find Class */
        let class = if self.professions.len() == 0 {
            "No class".to_string()
        } else {
            match self.professions.iter().find(|p| p.level == 0) {
                Some(p) => p.name.clone(),
                None => "No class".to_string(),
            }
        };
        lines.push(format!("{}", "<span class=\"u\">Status</span>"));
        lines.push(format!("<b>Name:</b> {}<br/>", self.name));
        lines.push(format!("<b>Class:</b> {}<br/>", class));
        lines.push(format!(
            "<b>Level/Exp:</b> {} ({}/{} [{:.2} %])<br/>",
            self.level,
            self.experience.separate_with_commas(),
            self.get_exp_needed().separate_with_commas(),
            (self.experience as f64 / self.get_exp_needed() as f64 * 100.0)
        ));
        /* Professions */
        for prof in self.professions.iter().filter(|&p| p.level > 0) {
            lines.push(format!(
                "<b>Profession:</b> {} (Tier {})<br/>",
                prof.name, prof.level
            ));
        }
        /* Health, Mana, Stamina and their regeneration */
        for (lhs, rhs) in vec![
            (AttributeType::Health, AttributeType::HealthRegen),
            (AttributeType::Mana, AttributeType::ManaRegen),
            (AttributeType::Stamina, AttributeType::StaminaRegen),
        ] {
            lines.push(format!(
                "<b>{0}:</b> {1}/{1} ({2}/s)<br/>",
                lhs,
                format!("{:.0}", self.calculate_final_attribute(&lhs)).separate_with_commas(),
                format!("{:.2}", self.calculate_final_attribute(&rhs)).separate_with_commas()
            ));
        }
        lines.push(format!("<span class=\"u\">Primary Attributes</span>"));
        /* Base Attributes and their final values */
        for a in vec![
            AttributeType::Body,
            AttributeType::Mind,
            AttributeType::Spirit,
        ] {
            lines.push(format!(
                "<b>{}:</b> {} (base: {})<br/>",
                a,
                format!("{:.0}", self.calculate_final_attribute(&a)).separate_with_commas(),
                format!("{:.0}", self.get_base_attribute(&a)).separate_with_commas()
            ));
            lines.push(format!("╰╼({})<br/>", self.get_attunement_string(&a)));
        }
        lines.push(format!("<span class=\"u\">Derived Attributes</span>"));
        /* Resistances and Damage */
        for res in vec![
            AttributeType::PhysicalResistance,
            AttributeType::MentalResilience,
            AttributeType::BasicDamageMelee,
            AttributeType::BasicDamageRanged,
            AttributeType::BasicDamageMagic,
        ] {
            lines.push(format!(
                "<b>{}:</b> {}<br/>",
                add_spaces_before_caps(&format!("{}", res)),
                format!("{:.0}", self.calculate_final_attribute(&res)).separate_with_commas()
            ));
        }
        /* print multiple Lists of printabe items */
        for (item_type, item_list) in vec![
            (
                "Spell List",
                &self
                    .spells
                    .iter()
                    .map(|v| PrintableItemVariant::Spell(v.clone()))
                    .collect(),
            ),
            (
                "Skill List",
                &self
                    .skills
                    .iter()
                    .map(|v| PrintableItemVariant::Skill(v.clone()))
                    .collect(),
            ),
            (
                "Attunements",
                &self
                    .equipped_gear
                    .iter()
                    .filter(|g| g.gear_type == GearType::Attunement)
                    .map(|v| PrintableItemVariant::Gear(v.clone()))
                    .collect() as &Vec<PrintableItemVariant>,
            ),
            (
                "Bone Glyphs",
                &self
                    .equipped_gear
                    .iter()
                    .filter(|g| g.gear_type == GearType::BoneGlyph)
                    .map(|v| PrintableItemVariant::Gear(v.clone()))
                    .collect(),
            ),
            (
                "Bonds",
                &self
                    .equipped_gear
                    .iter()
                    .filter(|g| g.gear_type == GearType::Bond)
                    .map(|v| PrintableItemVariant::Gear(v.clone()))
                    .collect(),
            ),
            (
                "Weapons",
                &self
                    .weapons
                    .iter()
                    .map(|v| PrintableItemVariant::Weapon(v.clone()))
                    .collect(),
            ),
            (
                "Equipment",
                &self
                    .equipped_gear
                    .iter()
                    .filter(|g| g.gear_type == GearType::Equipment)
                    .map(|v| PrintableItemVariant::Gear(v.clone()))
                    .collect(),
            ),
            (
                "Character Traits",
                &self
                    .traits
                    .iter()
                    .map(|v| PrintableItemVariant::CharacterTrait(v.clone()))
                    .collect(),
            ),
            (
                "Titles",
                &self
                    .equipped_gear
                    .iter()
                    .filter(|g| g.gear_type == GearType::Title)
                    .map(|v| PrintableItemVariant::Gear(v.clone()))
                    .collect(),
            ),
            (
                "Notes",
                &self
                    .notes
                    .iter()
                    .map(|v| PrintableItemVariant::Note(v.clone()))
                    .collect(),
            ),
        ] {
            lines.push(format!("<span class=\"u\">{}</span>", item_type));
            for item in item_list {
                lines.push(item.pretty_print(&self));
            }
        }
        lines.join("\n")
    }

    pub fn save(&self) {
        let json_string = serde_json::to_string_pretty(&self).expect("trouble creating json");
        let mut file = fs::File::create(&self.file_path).expect("Trouble creating file");
        file.write_all(json_string.as_bytes())
            .expect("trouble writing file");
    }

    pub fn count_gear(&self, gear_type: &GearType) -> usize {
        self.equipped_gear
            .iter()
            .filter(|g| g.gear_type == *gear_type)
            .count()
    }

    pub fn from_file(file_path: PathBuf) -> Self {
        if !file_path.exists() {
            eprintln!("{}", "Gibt es nicht.".red());
            return Character::new("dummy".to_string(), file_path);
        }
        let mut file = fs::File::open(&file_path).expect("Trouble opening file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("trouble reading file");
        if contents.trim().is_empty() {
            return Character::new("dummy".to_string(), file_path);
        }
        let mut me: Self = serde_json::from_str(&contents).expect("trouble evaluation json");
        // fix file path after loading
        me.file_path = file_path;
        me
    }
}

pub fn print_line() {
    println!(
        "{0}{0}{0}{0}{0}{0}{0}{0}{0}{0}",
        "----------".magenta().bold()
    );
}
