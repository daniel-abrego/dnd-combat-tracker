struct Initiative {
    round: u32,
    fight_name: String,
    combatants: Vec<String>
}

struct Combatant {
    name: String,
    dex_mod: u32,
    conditions: Vec<CONDITIONS>
}

enum CONDITIONS {
    BLINDED,
    CHARMED,
    DEAFENED,
    EXHAUSTION,
    FRIGHTENED,
    GRAPPLED,
    INCAPACITATED,
    INVISIBLE,
    PARALYZED,
    PETRIFIED,
    POISONED,
    PRONE,
    RESTRAINED,
    STUNNED,
    UNCONSCIOUS
}

fn main() {
    println!("Hello, world!");
    print_header(String::from("DND-COMBAT-TRACKER"));
    let mut initiative = Initiative {
        round: 0,
        fight_name: String::from("Test"),
        combatants: Vec::new()
    };

    initiative.combatants.push(String::from("Test1"));
    initiative.combatants.push(String::from("Test1"));
    initiative.combatants.push(String::from("Test1"));
    initiative.combatants.push(String::from("Test1"));

    println!("{} [{}]", initiative.fight_name, initiative.round);
    for (i, combatant) in initiative.combatants.iter().enumerate() {
        println!("{}: {}", i + 1, combatant);
    }
}

fn print_header(header: String) {
    println!("{:*^80}", "");
    println!("{:^80}", header);
    println!("{:*^80}", "");
}