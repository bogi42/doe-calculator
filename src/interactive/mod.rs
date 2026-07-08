mod modify_lists;
use crate::{character::Attunement, AttributeType, Character};
use advanced_inputs::{AdvInput, PromptableEnum};
use colored::Colorize;
pub use modify_lists::{change_items, EditableItem, PrintableItem, PrintableItemVariant};
use std::env;
use std::fs;
use std::io::Write;
use strum_macros::{Display, EnumIter, EnumString};

pub struct Interactive {
    ed: AdvInput,
}

impl Interactive {
    pub fn new() -> Self {
        Interactive {
            ed: AdvInput::new(),
        }
    }

    pub fn open_character(&mut self) -> Option<Character> {
        println!("{}", "Character Screen - choose option".bold().underline());
        println!("");
        if let Some(choice) = self.ed.get_enum_input_default::<OpenCharacter>(
            "(TAB) > ".green(),
            true,
            Some(OpenCharacter::Exit),
        ) {
            match choice {
                OpenCharacter::Exit => None,
                OpenCharacter::LoadFromFile => Some(self.load_file()),
                OpenCharacter::CreateNew => Some(self.create_new()),
            }
        } else {
            None
        }
    }

    pub fn start_session(&mut self, mc: &mut Character) {
        loop {
            clear_screen();
            mc.display_stats();
            println!("\n{:^100}", "[Character Menu]".bold().underline());
            if let Some(choice) = self.ed.get_enum_input_default::<CharacterMenu>(
                "(TAB) > ".green(),
                true,
                Some(CharacterMenu::Exit),
            ) {
                match choice {
                    CharacterMenu::Save => {
                        mc.save();
                    }
                    CharacterMenu::Exit => {
                        break;
                    }
                    CharacterMenu::Exp => {
                        if let Some(nmb) = self.ed.get_index("How much EXP? ".bright_green()) {
                            mc.experience += nmb as u32;
                        }
                    }
                    CharacterMenu::SetAttribute => {
                        if let Some(ba) = self
                            .ed
                            .get_enum_input::<AttributeType>("Base Attribute > ".green(), false)
                        {
                            match ba {
                                AttributeType::Body
                                | AttributeType::Mind
                                | AttributeType::Spirit
                                | AttributeType::Soul => {
                                    let old_value = mc.get_base_attribute(&ba);
                                    if let Some(nmb) =
                                        self.ed.get_f64_initial("> ".green(), old_value)
                                    {
                                        mc.set_base_attribute(ba, nmb);
                                    }
                                }
                                _ => {
                                    println!(
                                        "{}",
                                        format!("Attribute {} cannot be set directly", ba).red()
                                    );
                                    _ = self.ed.get_string("Press Enter to continue".italic());
                                }
                            }
                        }
                    }
                    CharacterMenu::Attunement => {
                        if let Some(ba) = self.ed.get_enum_input::<AttributeType>(
                            "For which Attribute > ".green(),
                            false,
                        ) {
                            match ba {
                                AttributeType::Body => match &mut mc.body_attunement {
                                    Some(att) => att.change(),
                                    None => mc.body_attunement = Attunement::from_input(),
                                },
                                AttributeType::Mind => match &mut mc.mind_attunement {
                                    Some(att) => att.change(),
                                    None => mc.mind_attunement = Attunement::from_input(),
                                },
                                AttributeType::Spirit => match &mut mc.spirit_attunement {
                                    Some(att) => att.change(),
                                    None => mc.spirit_attunement = Attunement::from_input(),
                                },
                                AttributeType::Soul => match &mut mc.soul_attunement {
                                    Some(att) => att.change(),
                                    None => mc.soul_attunement = Attunement::from_input(),
                                },
                                _ => {
                                    println!(
                                        "{}",
                                        format!("Attribute {} cannot have an attunement", ba).red()
                                    );
                                }
                            }
                        }
                    }
                    CharacterMenu::Professions => change_items(&mut mc.professions, &mc.name),
                    CharacterMenu::Gears => change_items(&mut mc.equipped_gear, &mc.name),
                    CharacterMenu::Traits => change_items(&mut mc.traits, &mc.name),
                    CharacterMenu::Skills => change_items(&mut mc.skills, &mc.name),
                    CharacterMenu::Spells => change_items(&mut mc.spells, &mc.name),
                    CharacterMenu::Weapons => change_items(&mut mc.weapons, &mc.name),
                    CharacterMenu::Notes => change_items(&mut mc.notes, &mc.name),
                    CharacterMenu::LevelUp => mc.level_up(),
                    CharacterMenu::PrintCharacterSheet => {
                        let mut outfile = mc.file_path.clone();
                        outfile.set_extension("md");
                        let mut file =
                            fs::File::create(&outfile).expect("Trouble creating output file");
                        file.write_all(mc.pretty_print().as_bytes())
                            .expect("Trouble writing output file");
                        println!("Markdown-Output saved to:\n- {:#?}", outfile);
                        _ = self.ed.get_string("Press Enter to continue".italic());
                    }
                }
            }
        }
    }

    fn create_new(&mut self) -> Character {
        let mut file_path = env::current_dir().expect("trouble with filesystem");
        println!("  - create new character in: {:?}", file_path);
        let cname = match self.ed.get_string("Character Name > ".green()) {
            Some(s) => {
                if s.is_empty() {
                    "John Doe".to_string()
                } else {
                    s
                }
            }
            None => "John Doe".to_string(),
        };
        let file_name = sanitize_filename(&cname) + ".json";
        file_path.push(file_name);
        return Character::new(cname, file_path);
    }

    fn load_file(&mut self) -> Character {
        let file_path = env::current_dir().expect("Trouble with filesystem");
        println!("  - load character in: {:?}", file_path);
        match self.ed.get_json_file_input("Filename > ", file_path) {
            Ok(f) => Character::from_file(f),
            Err(f) => {
                /* create new character, get name from non-existing file */
                let cname = advanced_inputs::promptable_enum::add_spaces_before_caps(
                    f.file_stem().unwrap().to_str().unwrap(),
                );
                Character::new(cname, f)
            }
        }
    }
}

/* Menu Enums, used for the user navigation */
/// Menu to create a new character, open an existing one, or exit the program
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, EnumString, Display)]
pub enum OpenCharacter {
    LoadFromFile,
    CreateNew,
    Exit,
}
impl PromptableEnum for OpenCharacter {}

/// Character Menu (while viewing character)
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, EnumString, Display)]
pub enum CharacterMenu {
    SetAttribute,
    Exp,
    LevelUp,
    Attunement,
    Professions,
    Gears,
    Traits,
    Skills,
    Spells,
    Weapons,
    Notes,
    PrintCharacterSheet,
    Save,
    Exit,
}
impl PromptableEnum for CharacterMenu {}

pub fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn sanitize_filename(name: &str) -> String {
    let mut sanitized = String::new();
    let mut last_char_was_separator = true; // Tracks if the last character added was a separator (_ or - or .)

    for c in name.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => {
                // Keep alphanumeric characters
                sanitized.push(c);
                last_char_was_separator = false;
            }
            '.' | '-' => {
                // Allow periods and hyphens, but collapse multiples
                if !last_char_was_separator {
                    sanitized.push(c);
                    last_char_was_separator = true;
                }
            }
            _ if c.is_whitespace() => {
                // Replace spaces with underscores, collapse multiples
                if !last_char_was_separator {
                    sanitized.push('_');
                    last_char_was_separator = true;
                }
            }
            _ => {
                // Drop all other characters (e.g., '/', ':', '*', '?', '<', '>', '|', etc.)
                // Mark as separator to collapse potential following separators
                last_char_was_separator = true;
            }
        }
    }

    // Remove any trailing separators that might have been added
    while sanitized.ends_with('_') || sanitized.ends_with('-') || sanitized.ends_with('.') {
        sanitized.pop();
    }

    // If the string becomes empty after sanitization, provide a default name
    if sanitized.is_empty() {
        "untitled".to_string()
    } else {
        sanitized
    }
}
