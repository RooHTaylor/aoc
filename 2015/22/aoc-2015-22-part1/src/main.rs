use shared::*;
use clap::Parser;
use std::process;

/// Advent Of Code 2015 Day 22 Part 1 solution
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file containing boss stats
    #[arg(short, long)]
    filename: Option<String>,

    /// Toggle example stats
    #[arg(short, default_value_t = false)]
    example: bool,

    /// Toggle debug messages
    #[arg(long, default_value_t = false)]
    debug: bool,
}

fn main() {
    // Parse args
    let args = Args::parse();

    // Load input file if provided.
    let mut file_contents: Option<String> = None;
    if let Some(filename) = args.filename {
        file_contents = Some(load_input_file(&filename));
    }

    let (boss_hp, boss_damage, player_hp, player_mana, spellbook) = build_game(args.debug, args.example, &file_contents);

    find_lowest_mana_win(args.debug, boss_hp, boss_damage, player_hp, player_mana, &spellbook);
}

// DFS search the game options to find the min cost to win.
// Each function iteration represents one turn
fn run_game_dfs(
    debug: bool,
    mut boss_hp: usize,
    boss_damage: usize,
    mut player_hp: usize,
    mut player_mana: usize,
    mut player_armor: usize,
    cost: usize,
    mut effects: Vec<Effect>,
    spellbook: &Vec<Spell>,
    min_cost: &mut usize,
    player_turn: bool,
) {

    let mut turn_name = "Boss";
    if player_turn {
        turn_name = "Player";
    }
    if debug {
        println!("--{turn_name} turn --");
        println!("- Player has {player_hp} hit points, {player_armor} armor, {player_mana} mana");
        println!("- Boss has {boss_hp} hit points");
    }

    // Loop through effects and apply or expire.
    for i in 0..effects.len() {
        // We should not encounter effects with timer 0 here. Skip them if we do
        if effects[i].timer == 0 {
            continue;
        }

        // This effect deals damage
        if let Some(d) = effects[i].damage {

            // boss_hp will either return a non-zero usize, or you killed him and win
            boss_hp = match boss_hp.checked_sub(d) {
                Some(n) => {
                    if n == 0 {
                        if debug { println!("{} deals {} damage. Boss dies. You Win!", effects[i].name, d); }
                        if cost < *min_cost {
                            *min_cost = cost;
                        }
                        return;
                    }
                    n
                },
                None => {
                    if debug { println!("{} deals {} damage. Boss dies. You Win!", effects[i].name, d); }
                    if cost < *min_cost {
                        *min_cost = cost;
                    }
                    return;
                }
            };

            if debug { print!("{} deals {} damage; ", effects[i].name, d); }
        }

        // This effect grants mana
        if let Some(m) = effects[i].mana {

            player_mana += m;

            if debug { print!("{} provides {} mana; ", effects[i].name, m); }
        }

        if debug { if &effects[i].name == "Shield" { print!("Shield ")} }

        effects[i].timer = effects[i].timer.checked_sub(1).unwrap_or(0);

        if debug { println!("timer is now {}", effects[i].timer); }

        if effects[i].timer == 0 {
            if debug { println!("{} wears off", effects[i].name); }
            // Remove armor effect if expired
            if let Some(a) = effects[i].armor {
                player_armor = player_armor.checked_sub(a).unwrap_or(0);
            }
        }
    }

    // flush expired effects
    effects.retain(|e| e.timer > 0);

    if player_turn {
        // Player selects a new spell
        for spell in spellbook {
            // If an effect is already running from this spell, we can't cast it
            // again. Also skip spells we can't afford
            if !effects.iter().all(|e| e.name != spell.name) || spell.cost > player_mana {
                continue;
            }

            let mut cost_clone = cost;
            let mut player_hp_clone = player_hp;
            let mut player_armor_clone = player_armor;
            let mut player_mana_clone = player_mana;
            let mut boss_hp_clone = boss_hp;
            let mut effects_clone = effects.clone();

            // Can do this unchecked, because we check for cost above
            player_mana_clone -= spell.cost;
            cost_clone += spell.cost;

            if debug { print!("Player casts {}", spell.name); }

            // Apply armor effect
            if let Some(a) = spell.effect.armor {
                player_armor_clone += a;
                if debug { print!(", increasing armor by {a}"); }
            }

            // Apply immediate damage
            if spell.effect.timer == 0 {
                if let Some(d) = spell.effect.damage {
                    // boss_hp will either return a non-zero usize, or you killed him and win
                    boss_hp_clone = match boss_hp_clone.checked_sub(d) {
                        Some(n) => {
                            if n == 0 {
                                if debug { println!(". {} deals {} damage. Boss dies. You Win!", spell.effect.name, d); }
                                if cost_clone < *min_cost {
                                    *min_cost = cost_clone;
                                }
                                return;
                            }
                            n
                        },
                        None => {
                            if debug { println!(". {} deals {} damage. Boss dies. You Win!", spell.effect.name, d); }
                            if cost_clone < *min_cost {
                                *min_cost = cost_clone;
                            }
                            return;
                        }
                    };

                    if debug { print!(", dealing {d} damage"); }

                    // Apply healing
                    if let Some(h) = spell.effect.hp {
                        player_hp_clone += h;
                        if debug { print!(", and healing {h} hit points"); }
                    }

                    if debug { println!(""); }
                }
            } else {
                // Push non-zero time effects into the clone vec
                effects_clone.push(spell.effect.clone());
            }

            run_game_dfs(debug, boss_hp_clone, boss_damage, player_hp_clone, player_mana_clone, player_armor_clone, cost_clone, effects_clone, spellbook, min_cost, !player_turn);
        }

        if debug { println!("Not able to cast a spell! You lost!"); }
        return;

    } else {
        // Boss does damage
        let damage = match boss_damage.checked_sub(player_armor) {
            Some(n) => {
                if n < 1 {
                    1
                } else {
                    n
                }
            },
            None => {
                1
            }
        };
        player_hp = match player_hp.checked_sub(damage) {
            Some(n) => {
                if n == 0 {
                    if debug { println!("Boss attacks for {} - {} = {} damage! You die. Boss Wins!", boss_damage, player_armor, damage); }
                    return;
                }
                n
            },
            None => {
                if debug { println!("Boss attacks for {} - {} = {} damage! You die. Boss Wins!", boss_damage, player_armor, damage); }
                return;
            }
        };

        if debug { println!("Boss attacks for {} - {} = {} damage!", boss_damage, player_armor, damage); }

        run_game_dfs(debug, boss_hp, boss_damage, player_hp, player_mana, player_armor, cost, effects, spellbook, min_cost, !player_turn);
    }
}

// Using the provided stats, run games iterating on each possible spell
// usage to find the lowest possible mana cost to win.
fn find_lowest_mana_win(
    debug: bool,
    boss_hp: usize,
    boss_damage: usize,
    player_hp: usize,
    player_mana: usize,
    spellbook: &Vec<Spell>,
) {
    let mut min_cost: usize = usize::MAX;

    run_game_dfs(debug, boss_hp, boss_damage, player_hp, player_mana, 0, 0, vec![], spellbook, &mut min_cost, true);

    if min_cost < usize::MAX {
        println!("The lowest mana cost to win is {min_cost}");
    } else {
        println!("We never won a game!");
    }
}

// Build the game data.
// Load boss stats from input file if provided or use defaults.
// Prepare the player data and spells.
fn build_game(
    debug: bool,
    example: bool,
    file_contents: &Option<String>,
) -> (usize, usize, usize, usize, Vec<Spell>) {

    // If the input file is provided, load the boss stats.
    // Use hard coded defaults if file is not provided or cannot be read.
    let mut boss_hp: usize = 13;
    let mut boss_damage: usize = 8;
    if let Some(input) = file_contents {
        if debug { println!("Parsing input file"); }
        (boss_hp, boss_damage) = parse_input(debug, input);
    }

    let mut player_hp: usize = 50;
    let mut player_mana: usize = 500;
    if example {
        if debug { println!("Running an example game using hard coded stats:"); }
        player_hp = 10;
        player_mana = 250;
        boss_hp = 13;
        boss_damage = 8;
    }

    let spellbook = build_spellbook();

    (boss_hp, boss_damage, player_hp, player_mana, spellbook)
}

// Parse the input file to retrieve boss stats.
//
// Exits(1)
// This function will exit if the boss data cannot be parsed, or if the data is
// set to 0.
fn parse_input(debug: bool, input: &str) -> (usize, usize) {
    let mut boss_hp: usize = 0;
    let mut boss_damage: usize = 0;

    for line in input.lines() {
        if debug { println!("Parsing: {line}"); }
        if line.is_empty() { continue; }

        for stat_string in vec!["Hit Points:", "Damage:"] {
            let parts = match line.split_once(stat_string) {
                Some(s) => s,
                None => { continue; },
            };

            match stat_string {
                "Hit Points:" => {
                    boss_hp = match parts.1.trim().parse::<usize>() {
                        Ok(n) => n,
                        Err(_) => {
                            eprintln!("Could not parse boss hit points!");
                            process::exit(1);
                        }
                    }
                },
                "Damage:" => {
                    boss_damage = match parts.1.trim().parse::<usize>() {
                        Ok(n) => n,
                        Err(_) => {
                            eprintln!("Could not parse boss damage!");
                            process::exit(1);
                        }
                    }
                },
                // This catch-all should never be reached, since the Vec is hard coded
                _ => { }
            }
        }
    }

    if boss_hp == 0 || boss_damage == 0 {
        eprintln!("Boss damage or hit-points not set! Check the file to make sure they aren't 0.");
        process::exit(1);
    }
    
    (boss_hp, boss_damage)
}

fn build_spellbook() -> Vec<Spell> {
    vec![
        Spell {
            name: "Magic Missile".to_string(),
            cost: 53,
            effect: Effect {
                name: "Magic Missile".to_string(),
                damage: Some(4),
                armor: None,
                mana: None,
                hp: None,
                // timer of 0 will run instantly
                timer: 0,
            },
        },
        Spell {
            name: "Drain".to_string(),
            cost: 73,
            effect: Effect {
                name: "Drain".to_string(),
                damage: Some(2),
                armor: None,
                mana: None,
                hp: Some(2),
                // timer of 0 will run instantly
                timer: 0,
            },
        },
        Spell {
            name: "Shield".to_string(),
            cost: 113,
            effect: Effect {
                name: "Shield".to_string(),
                damage: None,
                armor: Some(7),
                mana: None,
                hp: None,
                timer: 6,
            },
        },
        Spell {
            name: "Poison".to_string(),
            cost: 173,
            effect: Effect {
                name: "Poison".to_string(),
                damage: Some(3),
                armor: None,
                mana: None,
                hp: None,
                timer: 6,
            },
        },
        Spell {
            name: "Recharge".to_string(),
            cost: 229,
            effect: Effect {
                name: "Recharge".to_string(),
                damage: None,
                armor: None,
                mana: Some(101),
                hp: None,
                timer: 5,
            },
        },
    ]
}

#[derive(Debug)]
struct Spell {
    name: String,
    cost: usize,
    effect: Effect,
}

#[derive(Debug, Clone)]
struct Effect {
    name: String,
    damage: Option<usize>,
    armor: Option<usize>,
    mana: Option<usize>,
    hp: Option<usize>,
    timer: usize,
}