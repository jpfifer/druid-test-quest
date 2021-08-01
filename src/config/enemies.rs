use rand::Rng;
use druid::Data;

#[derive(Debug, Copy, Clone, Data)]
pub struct Enemy {
    name: &'static str,
    level: i32,
    drop: &'static str,
}

impl Enemy {
    pub fn random_enemy() -> Self {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..ENEMIES.len());
        ENEMIES[index]
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn item(&self) -> &str {
        self.drop
    }
}

const ENEMIES: [Enemy; 231] = [
    Enemy {
        name: "Anhkheg",
        level: 6,
        drop: "chitin",
    },
    Enemy {
        name: "Ant",
        level: 0,
        drop: "antenna",
    },
    Enemy {
        name: "Ape",
        level: 4,
        drop: "banana",
    },
    Enemy {
        name: "Baluchitherium",
        level: 14,
        drop: "ear",
    },
    Enemy {
        name: "Beholder",
        level: 10,
        drop: "eyestalk",
    },
    Enemy {
        name: "Black Pudding",
        level: 10,
        drop: "saliva",
    },
    Enemy {
        name: "Blink Dog",
        level: 4,
        drop: "eyelid",
    },
    Enemy {
        name: "Cub Scout",
        level: 1,
        drop: "neckerchief",
    },
    Enemy {
        name: "Girl Scout",
        level: 2,
        drop: "cookie",
    },
    Enemy {
        name: "Boy Scout",
        level: 3,
        drop: "merit badge",
    },
    Enemy {
        name: "Eagle Scout",
        level: 4,
        drop: "merit badge",
    },
    Enemy {
        name: "Bugbear",
        level: 3,
        drop: "skin",
    },
    Enemy {
        name: "Bugboar",
        level: 3,
        drop: "tusk",
    },
    Enemy {
        name: "Boogie",
        level: 3,
        drop: "slime",
    },
    Enemy {
        name: "Camel",
        level: 2,
        drop: "hump",
    },
    Enemy {
        name: "Carrion Crawler",
        level: 3,
        drop: "egg",
    },
    Enemy {
        name: "Catoblepas",
        level: 6,
        drop: "neck",
    },
    Enemy {
        name: "Centaur",
        level: 4,
        drop: "rib",
    },
    Enemy {
        name: "Centipede",
        level: 0,
        drop: "leg",
    },
    Enemy {
        name: "Cockatrice",
        level: 5,
        drop: "wattle",
    },
    Enemy {
        name: "Couatl",
        level: 9,
        drop: "wing",
    },
    Enemy {
        name: "Crayfish",
        level: 0,
        drop: "antenna",
    },
    Enemy {
        name: "Demogorgon",
        level: 53,
        drop: "tentacle",
    },
    Enemy {
        name: "Jubilex",
        level: 17,
        drop: "gel",
    },
    Enemy {
        name: "Manes",
        level: 1,
        drop: "tooth",
    },
    Enemy {
        name: "Orcus",
        level: 27,
        drop: "wand",
    },
    Enemy {
        name: "Succubus",
        level: 6,
        drop: "whip",
    },
    Enemy {
        name: "Vrock",
        level: 8,
        drop: "neck",
    },
    Enemy {
        name: "Hezrou",
        level: 9,
        drop: "leg",
    },
    Enemy {
        name: "Glabrezu",
        level: 10,
        drop: "collar",
    },
    Enemy {
        name: "Nalfeshnee",
        level: 11,
        drop: "tusk",
    },
    Enemy {
        name: "Marilith",
        level: 7,
        drop: "arm",
    },
    Enemy {
        name: "Balor",
        level: 8,
        drop: "whip",
    },
    Enemy {
        name: "Yeenoghu",
        level: 25,
        drop: "flail",
    },
    Enemy {
        name: "Asmodeus",
        level: 52,
        drop: "leathers",
    },
    Enemy {
        name: "Baalzebul",
        level: 43,
        drop: "pants",
    },
    Enemy {
        name: "Barbed Devil",
        level: 8,
        drop: "flame",
    },
    Enemy {
        name: "Bone Devil",
        level: 9,
        drop: "hook",
    },
    Enemy {
        name: "Dispater",
        level: 30,
        drop: "matches",
    },
    Enemy {
        name: "Erinyes",
        level: 6,
        drop: "thong",
    },
    Enemy {
        name: "Geryon",
        level: 30,
        drop: "cornucopia",
    },
    Enemy {
        name: "Malebranche",
        level: 5,
        drop: "fork",
    },
    Enemy {
        name: "Ice Devil",
        level: 11,
        drop: "snow",
    },
    Enemy {
        name: "Lemure",
        level: 3,
        drop: "blob",
    },
    Enemy {
        name: "Pit Fiend",
        level: 13,
        drop: "seed",
    },
    Enemy {
        name: "Anklyosaurus",
        level: 9,
        drop: "tail",
    },
    Enemy {
        name: "Brontosaurus",
        level: 30,
        drop: "brain",
    },
    Enemy {
        name: "Diplodocus",
        level: 24,
        drop: "fin",
    },
    Enemy {
        name: "Elasmosaurus",
        level: 15,
        drop: "neck",
    },
    Enemy {
        name: "Gorgosaurus",
        level: 13,
        drop: "arm",
    },
    Enemy {
        name: "Iguanadon",
        level: 6,
        drop: "thumb",
    },
    Enemy {
        name: "Megalosaurus",
        level: 12,
        drop: "jaw",
    },
    Enemy {
        name: "Monoclonius",
        level: 8,
        drop: "horn",
    },
    Enemy {
        name: "Pentasaurus",
        level: 12,
        drop: "head",
    },
    Enemy {
        name: "Stegosaurus",
        level: 18,
        drop: "plate",
    },
    Enemy {
        name: "Triceratops",
        level: 16,
        drop: "horn",
    },
    Enemy {
        name: "Tyranosauraus Rex",
        level: 18,
        drop: "forearm",
    },
    Enemy {
        name: "Djinn",
        level: 7,
        drop: "lamp",
    },
    Enemy {
        name: "Doppleganger",
        level: 4,
        drop: "face",
    },
    Enemy {
        name: "Black Dragon",
        level: 7,
        drop: "*",
    },
    Enemy {
        name: "Plaid Dragon",
        level: 7,
        drop: "sporrin",
    },
    Enemy {
        name: "Blue Dragon",
        level: 9,
        drop: "*",
    },
    Enemy {
        name: "Beige Dragon",
        level: 9,
        drop: "*",
    },
    Enemy {
        name: "Brass Dragon",
        level: 7,
        drop: "watch",
    },
    Enemy {
        name: "Tin Dragon",
        level: 8,
        drop: "*",
    },
    Enemy {
        name: "Bronze Dragon",
        level: 9,
        drop: "medal",
    },
    Enemy {
        name: "Chromatic Dragon",
        level: 16,
        drop: "scale",
    },
    Enemy {
        name: "Copper Dragon",
        level: 8,
        drop: "loafer",
    },
    Enemy {
        name: "Gold Dragon",
        level: 8,
        drop: "filling",
    },
    Enemy {
        name: "Green Dragon",
        level: 8,
        drop: "*",
    },
    Enemy {
        name: "Platinum Dragon",
        level: 21,
        drop: "*",
    },
    Enemy {
        name: "Red Dragon",
        level: 10,
        drop: "cocktail",
    },
    Enemy {
        name: "Silver Dragon",
        level: 10,
        drop: "*",
    },
    Enemy {
        name: "White Dragon",
        level: 6,
        drop: "tooth",
    },
    Enemy {
        name: "Dragon Turtle",
        level: 13,
        drop: "shell",
    },
    Enemy {
        name: "Dryad",
        level: 2,
        drop: "acorn",
    },
    Enemy {
        name: "Dwarf",
        level: 1,
        drop: "drawers",
    },
    Enemy {
        name: "Eel",
        level: 2,
        drop: "sashimi",
    },
    Enemy {
        name: "Efreet",
        level: 10,
        drop: "cinder",
    },
    Enemy {
        name: "Sand Elemental",
        level: 8,
        drop: "glass",
    },
    Enemy {
        name: "Bacon Elemental",
        level: 10,
        drop: "bit",
    },
    Enemy {
        name: "Cheese Elemental",
        level: 14,
        drop: "curd",
    },
    Enemy {
        name: "Hair Elemental",
        level: 16,
        drop: "follicle",
    },
    Enemy {
        name: "Swamp Elf",
        level: 1,
        drop: "lilypad",
    },
    Enemy {
        name: "Brown Elf",
        level: 1,
        drop: "tusk",
    },
    Enemy {
        name: "Sea Elf",
        level: 1,
        drop: "jerkin",
    },
    Enemy {
        name: "Ettin",
        level: 10,
        drop: "fur",
    },
    Enemy {
        name: "Frog",
        level: 0,
        drop: "leg",
    },
    Enemy {
        name: "Violet Fungi",
        level: 3,
        drop: "spore",
    },
    Enemy {
        name: "Gargoyle",
        level: 4,
        drop: "gravel",
    },
    Enemy {
        name: "Gelatinous Cube",
        level: 4,
        drop: "jam",
    },
    Enemy {
        name: "Ghast",
        level: 4,
        drop: "vomit",
    },
    Enemy {
        name: "Ghost",
        level: 10,
        drop: "*",
    },
    Enemy {
        name: "Ghoul",
        level: 2,
        drop: "muscle",
    },
    Enemy {
        name: "Humidity Giant",
        level: 12,
        drop: "drops",
    },
    Enemy {
        name: "Beef Giant",
        level: 11,
        drop: "steak",
    },
    Enemy {
        name: "Quartz Giant",
        level: 10,
        drop: "crystal",
    },
    Enemy {
        name: "Porcelain Giant",
        level: 9,
        drop: "fixture",
    },
    Enemy {
        name: "Rice Giant",
        level: 8,
        drop: "grain",
    },
    Enemy {
        name: "Cloud Giant",
        level: 12,
        drop: "condensation",
    },
    Enemy {
        name: "Fire Giant",
        level: 11,
        drop: "cigarettes",
    },
    Enemy {
        name: "Frost Giant",
        level: 10,
        drop: "snowman",
    },
    Enemy {
        name: "Hill Giant",
        level: 8,
        drop: "corpse",
    },
    Enemy {
        name: "Stone Giant",
        level: 9,
        drop: "hatchling",
    },
    Enemy {
        name: "Storm Giant",
        level: 15,
        drop: "barometer",
    },
    Enemy {
        name: "Mini Giant",
        level: 4,
        drop: "pompadour",
    },
    Enemy {
        name: "Gnoll",
        level: 2,
        drop: "collar",
    },
    Enemy {
        name: "Gnome",
        level: 1,
        drop: "hat",
    },
    Enemy {
        name: "Goblin",
        level: 1,
        drop: "ear",
    },
    Enemy {
        name: "Grid Bug",
        level: 1,
        drop: "carapace",
    },
    Enemy {
        name: "Jellyrock",
        level: 9,
        drop: "seedling",
    },
    Enemy {
        name: "Beer Golem",
        level: 15,
        drop: "foam",
    },
    Enemy {
        name: "Oxygen Golem",
        level: 17,
        drop: "platelet",
    },
    Enemy {
        name: "Cardboard Golem",
        level: 14,
        drop: "recycling",
    },
    Enemy {
        name: "Rubber Golem",
        level: 16,
        drop: "ball",
    },
    Enemy {
        name: "Leather Golem",
        level: 15,
        drop: "fob",
    },
    Enemy {
        name: "Gorgon",
        level: 8,
        drop: "dank weed",
    },
    Enemy {
        name: "Gray Ooze",
        level: 3,
        drop: "gravy",
    },
    Enemy {
        name: "Green Slime",
        level: 2,
        drop: "sample",
    },
    Enemy {
        name: "Griffon",
        level: 7,
        drop: "nest",
    },
    Enemy {
        name: "Banshee",
        level: 7,
        drop: "larynx",
    },
    Enemy {
        name: "Harpy",
        level: 3,
        drop: "mascara",
    },
    Enemy {
        name: "Hell Hound",
        level: 5,
        drop: "tongue",
    },
    Enemy {
        name: "Hippocampus",
        level: 4,
        drop: "mane",
    },
    Enemy {
        name: "Hippogriff",
        level: 3,
        drop: "egg",
    },
    Enemy {
        name: "Hobgoblin",
        level: 1,
        drop: "patella",
    },
    Enemy {
        name: "Homonculus",
        level: 2,
        drop: "fluid",
    },
    Enemy {
        name: "Hydra",
        level: 8,
        drop: "gyrum",
    },
    Enemy {
        name: "Imp",
        level: 2,
        drop: "tail",
    },
    Enemy {
        name: "Invisible Stalker",
        level: 8,
        drop: "*",
    },
    Enemy {
        name: "Iron Peasant",
        level: 3,
        drop: "chaff",
    },
    Enemy {
        name: "Jumpskin",
        level: 3,
        drop: "shin",
    },
    Enemy {
        name: "Kobold",
        level: 1,
        drop: "candle",
    },
    Enemy {
        name: "Leprechaun",
        level: 1,
        drop: "wallet",
    },
    Enemy {
        name: "Leucrotta",
        level: 6,
        drop: "hoof",
    },
    Enemy {
        name: "Lich",
        level: 11,
        drop: "crown",
    },
    Enemy {
        name: "Lizard Man",
        level: 2,
        drop: "tail",
    },
    Enemy {
        name: "Lurker",
        level: 10,
        drop: "sac",
    },
    Enemy {
        name: "Manticore",
        level: 6,
        drop: "spike",
    },
    Enemy {
        name: "Mastodon",
        level: 12,
        drop: "tusk",
    },
    Enemy {
        name: "Medusa",
        level: 6,
        drop: "eye",
    },
    Enemy {
        name: "Multicell",
        level: 2,
        drop: "dendrite",
    },
    Enemy {
        name: "Pirate",
        level: 1,
        drop: "booty",
    },
    Enemy {
        name: "Berserker",
        level: 1,
        drop: "shirt",
    },
    Enemy {
        name: "Caveman",
        level: 2,
        drop: "club",
    },
    Enemy {
        name: "Dervish",
        level: 1,
        drop: "robe",
    },
    Enemy {
        name: "Merman",
        level: 1,
        drop: "trident",
    },
    Enemy {
        name: "Mermaid",
        level: 1,
        drop: "gills",
    },
    Enemy {
        name: "Mimic",
        level: 9,
        drop: "hinge",
    },
    Enemy {
        name: "Mind Flayer",
        level: 8,
        drop: "tentacle",
    },
    Enemy {
        name: "Minotaur",
        level: 6,
        drop: "map",
    },
    Enemy {
        name: "Yellow Mold",
        level: 1,
        drop: "spore",
    },
    Enemy {
        name: "Morkoth",
        level: 7,
        drop: "teeth",
    },
    Enemy {
        name: "Mummy",
        level: 6,
        drop: "gauze",
    },
    Enemy {
        name: "Naga",
        level: 9,
        drop: "rattle",
    },
    Enemy {
        name: "Nebbish",
        level: 1,
        drop: "belly",
    },
    Enemy {
        name: "Neo-Otyugh",
        level: 11,
        drop: "organ ",
    },
    Enemy {
        name: "Nixie",
        level: 1,
        drop: "webbing",
    },
    Enemy {
        name: "Nymph",
        level: 3,
        drop: "hanky",
    },
    Enemy {
        name: "Ochre Jelly",
        level: 6,
        drop: "nucleus",
    },
    Enemy {
        name: "Octopus",
        level: 2,
        drop: "beak",
    },
    Enemy {
        name: "Ogre",
        level: 4,
        drop: "talon",
    },
    Enemy {
        name: "Ogre Mage",
        level: 5,
        drop: "apparel",
    },
    Enemy {
        name: "Orc",
        level: 1,
        drop: "snout",
    },
    Enemy {
        name: "Otyugh",
        level: 7,
        drop: "organ",
    },
    Enemy {
        name: "Owlbear",
        level: 5,
        drop: "feather",
    },
    Enemy {
        name: "Pegasus",
        level: 4,
        drop: "aileron",
    },
    Enemy {
        name: "Peryton",
        level: 4,
        drop: "antler",
    },
    Enemy {
        name: "Piercer",
        level: 3,
        drop: "tip",
    },
    Enemy {
        name: "Pixie",
        level: 1,
        drop: "dust",
    },
    Enemy {
        name: "Man-o-war",
        level: 3,
        drop: "tentacle",
    },
    Enemy {
        name: "Purple Worm",
        level: 15,
        drop: "dung",
    },
    Enemy {
        name: "Quasit",
        level: 3,
        drop: "tail",
    },
    Enemy {
        name: "Rakshasa",
        level: 7,
        drop: "pajamas",
    },
    Enemy {
        name: "Rat",
        level: 0,
        drop: "tail",
    },
    Enemy {
        name: "Remorhaz",
        level: 11,
        drop: "protrusion",
    },
    Enemy {
        name: "Roc",
        level: 18,
        drop: "wing",
    },
    Enemy {
        name: "Roper",
        level: 11,
        drop: "twine",
    },
    Enemy {
        name: "Rot Grub",
        level: 1,
        drop: "eggsac",
    },
    Enemy {
        name: "Rust Monster",
        level: 5,
        drop: "shavings",
    },
    Enemy {
        name: "Satyr",
        level: 5,
        drop: "hoof",
    },
    Enemy {
        name: "Sea Hag",
        level: 3,
        drop: "wart",
    },
    Enemy {
        name: "Silkie",
        level: 3,
        drop: "fur",
    },
    Enemy {
        name: "Shadow",
        level: 3,
        drop: "silhouette",
    },
    Enemy {
        name: "Shambling Mound",
        level: 10,
        drop: "mulch",
    },
    Enemy {
        name: "Shedu",
        level: 9,
        drop: "hoof",
    },
    Enemy {
        name: "Shrieker",
        level: 3,
        drop: "stalk",
    },
    Enemy {
        name: "Skeleton",
        level: 1,
        drop: "clavicle",
    },
    Enemy {
        name: "Spectre",
        level: 7,
        drop: "vestige",
    },
    Enemy {
        name: "Sphinx",
        level: 10,
        drop: "paw",
    },
    Enemy {
        name: "Spider",
        level: 0,
        drop: "web",
    },
    Enemy {
        name: "Sprite",
        level: 1,
        drop: "can",
    },
    Enemy {
        name: "Stirge",
        level: 1,
        drop: "proboscis",
    },
    Enemy {
        name: "Stun Bear",
        level: 5,
        drop: "tooth",
    },
    Enemy {
        name: "Stun Worm",
        level: 2,
        drop: "trode",
    },
    Enemy {
        name: "Su-monster",
        level: 5,
        drop: "tail",
    },
    Enemy {
        name: "Sylph",
        level: 3,
        drop: "thigh",
    },
    Enemy {
        name: "Titan",
        level: 20,
        drop: "sandal",
    },
    Enemy {
        name: "Trapper",
        level: 12,
        drop: "shag",
    },
    Enemy {
        name: "Treant",
        level: 10,
        drop: "acorn",
    },
    Enemy {
        name: "Triton",
        level: 3,
        drop: "scale",
    },
    Enemy {
        name: "Troglodyte",
        level: 2,
        drop: "tail",
    },
    Enemy {
        name: "Troll",
        level: 6,
        drop: "hide",
    },
    Enemy {
        name: "Umber Hulk",
        level: 8,
        drop: "claw",
    },
    Enemy {
        name: "Unicorn",
        level: 4,
        drop: "blood",
    },
    Enemy {
        name: "Vampire",
        level: 8,
        drop: "pancreas",
    },
    Enemy {
        name: "Wight",
        level: 4,
        drop: "lung",
    },
    Enemy {
        name: "Will-o'-the-Wisp",
        level: 9,
        drop: "wisp",
    },
    Enemy {
        name: "Wraith",
        level: 5,
        drop: "finger",
    },
    Enemy {
        name: "Wyvern",
        level: 7,
        drop: "wing",
    },
    Enemy {
        name: "Xorn",
        level: 7,
        drop: "jaw",
    },
    Enemy {
        name: "Yeti",
        level: 4,
        drop: "fur",
    },
    Enemy {
        name: "Zombie",
        level: 2,
        drop: "forehead",
    },
    Enemy {
        name: "Wasp",
        level: 0,
        drop: "stinger",
    },
    Enemy {
        name: "Rat",
        level: 1,
        drop: "tail",
    },
    Enemy {
        name: "Bunny",
        level: 0,
        drop: "ear",
    },
    Enemy {
        name: "Moth",
        level: 0,
        drop: "dust",
    },
    Enemy {
        name: "Beagle",
        level: 0,
        drop: "collar",
    },
    Enemy {
        name: "Midge",
        level: 0,
        drop: "corpse",
    },
    Enemy {
        name: "Ostrich",
        level: 1,
        drop: "beak",
    },
    Enemy {
        name: "Billy Goat",
        level: 1,
        drop: "beard",
    },
    Enemy {
        name: "Bat",
        level: 1,
        drop: "wing",
    },
    Enemy {
        name: "Koala",
        level: 2,
        drop: "heart",
    },
    Enemy {
        name: "Wolf",
        level: 2,
        drop: "paw",
    },
    Enemy {
        name: "Whippet",
        level: 2,
        drop: "collar",
    },
    Enemy {
        name: "Uruk",
        level: 2,
        drop: "boot",
    },
    Enemy {
        name: "Poroid",
        level: 4,
        drop: "node",
    },
    Enemy {
        name: "Moakum",
        level: 8,
        drop: "frenum",
    },
    Enemy {
        name: "Fly",
        level: 0,
        drop: "*",
    },
    Enemy {
        name: "Hogbird",
        level: 3,
        drop: "curl",
    },
    Enemy {
        name: "Wolog",
        level: 4,
        drop: "lemma",
    },
];