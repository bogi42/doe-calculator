mod attributes;
mod character;
mod gear;
mod interactive;
mod profession;
mod skill;
mod spell;
mod traits;
mod weapon;
use attributes::AttributeType;
use character::Character;
use interactive::{clear_screen, Interactive};
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input = Interactive::new();
    /* Open a character (or close program)
     * If user passed an argument, we assume it's a file path to a character file */
    let mut character = if args.len() > 1 {
        Character::from_file(PathBuf::from(&args[1]))
    } else {
        match input.open_character() {
            Some(c) => c,
            None => {
                return;
            }
        }
    };
    if args.len() > 2 {
        println!("{}", character.pretty_print());
    } else {
        input.start_session(&mut character);
        println!("Bye, thanks!");
    }
}
