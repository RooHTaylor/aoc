use shared::*;
use std::{
    env,
    process
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut debug: bool = false;
    let mut file_name: String = String::new();
    let mut i: usize = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--debug" => {
                debug = true;
            },
            "-f" => {
                if i+1 >= args.len() {
                    eprintln!("Not enough arguments provided for file name");
                    process::exit(1);
                }
                file_name = args[i+1].to_string();
                i += 1;
            },
            _ => {
                eprintln!("Usage: {} [-f <filename>] [--debug]", args[0]);
                process::exit(1);
            }
        }
        i += 1;
    }

    let file_contents = load_input_file(&file_name);
    if debug { println!("{file_contents}"); }

    // Possible items
    let item_list: Vec<Item> = build_item_list();

    let (boss_hp, boss_damage, boss_armor) = parse_input(debug, &file_contents);

    let player_hp: isize = 100;

    let mut min_cost: usize = usize::MAX;

    for weapon in &item_list {
        if weapon.t != ItemType::WEAPON { continue; }
        for armor in &item_list {
            if armor.t != ItemType::ARMOR { continue; }
            for ring1 in &item_list {
                if ring1.t != ItemType::RING { continue; }
                for ring2 in &item_list {
                    if ring2.t != ItemType::RING || (ring2.name == ring1.name && ring1.name != "None") { continue; }
                    if debug { println!("weapon: {}, armor: {}, ring1: {}, ring2: {}", weapon.name, armor.name, ring1.name, ring2.name); }
                    
                    let p_damage = weapon.damage + armor.damage + ring1.damage + ring2.damage;
                    let p_armor = weapon.armor + armor.armor + ring1.armor + ring2.armor;
                    let cost = weapon.cost + armor.cost + ring1.cost + ring2.cost;
                    if run_game(debug, player_hp, p_damage, p_armor, boss_hp, boss_damage, boss_armor) {
                        if debug { println!("You WIN!"); }
                        if cost < min_cost {
                            min_cost = cost;
                        }
                    } else {
                        if debug { println!("You LOST!"); }
                    }
                }
            }
        }
    }

    if min_cost < usize::MAX {
        println!("The least gold you can spend is {}", min_cost);
    } else {
        println!("You never won a game");
    }
}

fn run_game(
    debug: bool,
    mut player_hp: isize,
    player_damage: isize,
    player_armor: isize,
    mut boss_hp: isize,
    boss_damage: isize,
    boss_armor: isize
)-> bool {

    loop {
        boss_hp -= (player_damage - boss_armor).max(1 as isize);
        if debug { println!("Player deals {} damage; boss hp {}", (player_damage - boss_armor).max(1 as isize), boss_hp); }

        player_hp -= (boss_damage - player_armor).max(1 as isize);
        if debug { println!("Boss deals {} damage; player hp {}", (boss_damage - player_armor).max(1 as isize), player_hp); }

        if player_hp <= 0 || boss_hp <= 0 {
            break;
        }
    }

    if boss_hp <= 0 {
        true
    } else {
        false
    }
}

fn parse_input(_debug: bool, input: &str) -> (isize, isize, isize) {
    let mut hp: isize = 0;
    let mut damage: isize = 0;
    let mut armor: isize = 0;

    for line in input.lines() {
        if line.is_empty() { continue; }

        for sstring in vec!["Hit Points:", "Damage:", "Armor:"] {
            let parts = match line.split_once(sstring) {
                Some(s) => s,
                None => { continue; },
            };
            match sstring {
                "Hit Points:" => {
                    hp = match parts.1.trim().parse::<isize>() {
                        Ok(n) => n,
                        Err(_) => {
                            eprintln!("Could not parse boss hit points!");
                            process::exit(1);
                        }
                    }
                },
                "Damage:" => {
                    damage = match parts.1.trim().parse::<isize>() {
                        Ok(n) => n,
                        Err(_) => {
                            eprintln!("Could not parse boss damage!");
                            process::exit(1);
                        }
                    }
                },
                "Armor:" => {
                    armor = match parts.1.trim().parse::<isize>() {
                        Ok(n) => n,
                        Err(_) => {
                            eprintln!("Could not parse boss armor!");
                            process::exit(1);
                        }
                    }
                },
                _ => {
                    eprintln!("Impossible string encountered!");
                    process::exit(1);
                }
            }
        }
    }

    (hp, damage, armor)
}

fn build_item_list() -> Vec<Item> {
    let item_list = vec![
        // WEAPONS
        Item {
            t: ItemType::WEAPON,
            name: "Dagger".to_string(),
            cost: 8,
            damage: 4,
            armor: 0,
        },
        Item {
            t: ItemType::WEAPON,
            name: "Shortsword".to_string(),
            cost: 10,
            damage: 5,
            armor: 0,
        },
        Item {
            t: ItemType::WEAPON,
            name: "Warhammer".to_string(),
            cost: 25,
            damage: 6,
            armor: 0,
        },
        Item {
            t: ItemType::WEAPON,
            name: "Longsword".to_string(),
            cost: 40,
            damage: 7,
            armor: 0,
        },
        Item {
            t: ItemType::WEAPON,
            name: "Greataxe".to_string(),
            cost: 74,
            damage: 8,
            armor: 0,
        },
        // ARMOR
        Item {
            t: ItemType::ARMOR,
            name: "None".to_string(),
            cost: 0,
            damage: 0,
            armor: 0,
        },
        Item {
            t: ItemType::ARMOR,
            name: "Leather".to_string(),
            cost: 13,
            damage: 0,
            armor: 1,
        },
        Item {
            t: ItemType::ARMOR,
            name: "Chainmail".to_string(),
            cost: 31,
            damage: 0,
            armor: 2,
        },
        Item {
            t: ItemType::ARMOR,
            name: "Splintmail".to_string(),
            cost: 53,
            damage: 0,
            armor: 3,
        },
        Item {
            t: ItemType::ARMOR,
            name: "Bandedmail".to_string(),
            cost: 75,
            damage: 0,
            armor: 4,
        },
        Item {
            t: ItemType::ARMOR,
            name: "Platemail".to_string(),
            cost: 102,
            damage: 0,
            armor: 5,
        },
        // RINGS
        Item {
            t: ItemType::RING,
            name: "None".to_string(),
            cost: 0,
            damage: 0,
            armor: 0,
        },
        Item {
            t: ItemType::RING,
            name: "Damage+1".to_string(),
            cost: 25,
            damage: 1,
            armor: 0,
        },
        Item {
            t: ItemType::RING,
            name: "Damage+2".to_string(),
            cost: 50,
            damage: 2,
            armor: 0,
        },
        Item {
            t: ItemType::RING,
            name: "Damage+3".to_string(),
            cost: 100,
            damage: 3,
            armor: 0,
        },
        Item {
            t: ItemType::RING,
            name: "Defense+1".to_string(),
            cost: 20,
            damage: 0,
            armor: 1,
        },
        Item {
            t: ItemType::RING,
            name: "Defense+2".to_string(),
            cost: 40,
            damage: 0,
            armor: 2,
        },
        Item {
            t: ItemType::RING,
            name: "Defense+3".to_string(),
            cost: 80,
            damage: 0,
            armor: 3,
        },
    ];

    item_list
}

#[derive(PartialEq)]
enum ItemType {
    WEAPON,
    ARMOR,
    RING,
}

struct Item {
    t: ItemType,
    name: String,
    cost: usize,
    damage: isize,
    armor: isize,
}