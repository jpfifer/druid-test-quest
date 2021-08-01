mod spells;
mod enemies;

pub use enemies::Enemy;

pub struct Classes;

impl Classes {
    pub fn all_classes() -> Vec<&'static str> {
        CLASSES.into()
    }

    pub fn index_of(class: &str) -> Option<usize> {
        CLASSES.iter().zip(0..CLASSES.len()).find_map(|(c, idx)| {
            if *c == class {
                Some(idx)
            } else {
                None
            }
        })
    }
}

const CLASSES: [&str; 18] = [
    "Ur-Paladin",
    "Voodoo Princess",
    "Robot Monk",
    "Mu-Fu Monk",
    "Mage Illusioner",
    "Shiv-Knight",
    "Inner Mason",
    "Fighter/Organist",
    "Puma Burgular",
    "Runeloremaster",
    "Hunter Strangler",
    "Battle-Felon",
    "Tickle-Mimic",
    "Slow Poisoner",
    "Bastard Lunatic",
    "Lowling",
    "Birdrider",
    "Vermineer",
];

pub struct Races;

impl Races {
    pub fn all_races() -> Vec<String> {
        RACES.iter().map(|e| e.to_string()).collect()
    }

    pub fn index_of(race: &str) -> Option<usize> {
        RACES.iter().position(|r| *r == race)
    }
}

const RACES: [&str; 21] = [
    "Half Orc",
    "Half Man",
    "Half Halfling",
    "Double Hobbit",
    "Hob-Hobbit",
    "Low Elf",
    "Dung Elf",
    "Talking Pony",
    "Gyrognome",
    "Lesser Dwarf",
    "Crested Dwarf",
    "Eel Man",
    "Panda Man",
    "Trans-Kobold",
    "Enchanted Motorcycle",
    "Will o' the Wisp",
    "Battle-Finch",
    "Double Wookiee",
    "Skraeling",
    "Demicanadian",
    "Land Squid",
];

