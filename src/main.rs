use std::io;

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
    print_header(String::from("DND-COMBAT-TRACKER"), '*');

    let mut initiative = Initiative {
        round: 0,
        fight_name: String::from("Test"),
        combatants: Vec::new()
    };

    main_menu(&mut initiative);
}

fn print_header(header: String, key: char) {
    if key == '*' {
        print_major_header(header);
    } else {
        print_minor_header(header);
    }
}

fn print_major_header(header: String) {
    println!("{:*^80}", "");
    println!("{:^80}", header);
    println!("{:*^80}", "");
}

fn print_minor_header(minor_header: String) {
    println!("{:-^80}", "");
    println!("{:^80}", minor_header);
    println!("{:-^80}", "");
}

/* 
 * MAIN MENU
 */
fn main_menu(initiative: &mut Initiative) {
    print_header(String::from("MAIN MENU"), '-');
    initiative.fight_name = String::from("TESTING MAIN MENU");

    let mut options = ["NEXT", "EDIT", "ADD EFFECT", "ADD COMBATANT"];
    print_options(&mut options);

    let option_result = take_option();
    let option = option_result.unwrap();

    match option.trim() {
        "1" => println!("next()"),
        "2" => println!("edit()"),
        "3" => println!("add_effect()"),
        "4" => println!("add_combatant()"),
        _ => println!("unrecognized argument: {}", option),
    }

    // initiative.combatants.push(String::from("Test1"));
    // initiative.combatants.push(String::from("Test1"));
    // initiative.combatants.push(String::from("Test1"));
    // initiative.combatants.push(String::from("Test1"));

    // println!("{} [{}]", initiative.fight_name, initiative.round);
    // for (i, combatant) in initiative.combatants.iter().enumerate() {
    //     println!("{}: {}", i + 1, combatant);
    // }
}

fn take_option() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    println!();
    Ok(buffer)
}

fn print_options(list_of_options: &mut [&str]) {
    for i in 0..list_of_options.len() {
        println!("[{}]: {}", i + 1, list_of_options[i]);
    }
    println!("[r] to refresh screen");
    println!("[x] to exit");
    println!("Enter Selection: ");
}