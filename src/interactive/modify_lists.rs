use crate::clear_screen;
use crate::gear::Gear;
use crate::skill::Skill;
use crate::spell::Spell;
use crate::traits::CharacterTrait;
use crate::weapon::Weapon;
use crate::Character;
use advanced_inputs::{AdvInput, PromptableEnum};
use colored::Colorize;
use std::fmt::Display as FmtDisplay;
use strum_macros::{Display, EnumIter, EnumString};

// This trait defines the common interface for the generic change function

pub trait EditableItem: FmtDisplay + Sized {
    fn change(&mut self);
    fn from_input() -> Option<Self>;
    fn item_kind_name() -> &'static str;
}

pub trait PrintableItem: Sized {
    fn pretty_print(&self, character: &Character) -> String;
}

#[derive(Debug, Clone)]
pub enum PrintableItemVariant {
    Gear(Gear),
    CharacterTrait(CharacterTrait),
    Spell(Spell),
    Skill(Skill),
    Weapon(Weapon),
}
impl PrintableItem for PrintableItemVariant {
    fn pretty_print(&self, c: &Character) -> String {
        match self {
            Self::Gear(v) => v.pretty_print(c),
            Self::CharacterTrait(v) => v.pretty_print(c),
            Self::Spell(v) => v.pretty_print(c),
            Self::Skill(v) => v.pretty_print(c),
            Self::Weapon(v) => v.pretty_print(c),
        }
    }
}

#[derive(Debug, Clone, PartialEq, EnumIter, EnumString, Display)]
enum Movement {
    Next,
    Previous,
    Remove,
    Edit,
    Add,
    Exit,
    SkipToEnd,
}
impl PromptableEnum for Movement {}

#[derive(Debug, Clone, PartialEq, EnumIter, EnumString, Display)]
enum SmallMovement {
    Add,
    Exit,
}
impl PromptableEnum for SmallMovement {}

/// guides the user through a list of items, allowing modificion, removal, addition, or skipping.
/// T must implement the `EditableItem` trait.
pub fn change_items<T>(items: &mut Vec<T>, placement_name: &str)
where
    T: EditableItem,
{
    let mut advi = AdvInput::new();
    let mut current_index: usize = 0;
    let item_type_name = T::item_kind_name();
    loop {
        if items.is_empty() {
            println!(
                "Currently no {}s in this list. You can either Add one, or Exit",
                item_type_name
            );
            if let Some(choice) = advi.get_enum_input_default::<SmallMovement>(
                "> ".green(),
                true,
                Some(SmallMovement::Add),
            ) {
                match choice {
                    SmallMovement::Add => {
                        if let Some(modifier) = T::from_input() {
                            items.push(modifier);
                        } else {
                            break;
                        }
                    }
                    SmallMovement::Exit => break,
                }
            }
        }
        clear_screen();
        println!(
            "{:^100}\n",
            format!("Edit {}s on {}", item_type_name, placement_name)
                .bright_blue()
                .bold()
                .underline()
        );
        current_index %= items.len(); // Make sure index is valid
        println!(
            "---Viewing {} {} out of {}:\n",
            item_type_name,
            format!("{}", (current_index + 1)).bright_magenta().bold(),
            format!("{}", items.len()).magenta().bold()
        );
        println!("{}\n", items[current_index]);
        if let Some(choice) =
            advi.get_enum_input_default::<Movement>("(Tab) >".green(), true, Some(Movement::Next))
        {
            match choice {
                Movement::Next => current_index += 1,
                Movement::Previous => {
                    if current_index == 0 {
                        current_index = items.len() - 1;
                    } else {
                        current_index -= 1;
                    }
                }
                Movement::Remove => {
                    _ = items.remove(current_index);
                    println!("{} {}", item_type_name.red(), "has been removed!".red());
                }
                Movement::Exit => break,
                Movement::Edit => items[current_index].change(),
                Movement::Add => {
                    if let Some(m) = T::from_input() {
                        items.push(m);
                        println!(
                            "{} {} {}",
                            "new".green(),
                            item_type_name.green(),
                            "added to end of list".green()
                        );
                        current_index = items.len() - 1;
                    }
                }
                Movement::SkipToEnd => current_index = items.len() - 1,
            }
        }
    }
}
