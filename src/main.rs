use std::io::{self, Write};

const MAIN_MENU_OPTIONS: &[&str; 4] = &["NEXT", "EDIT", "ADD EFFECT", "ADD COMBATANT"];
const EDIT_MENU_OPTIONS: &[&str; 2] = &["CHANGE ROUND", "CHANGE COMBAT NAME"];

struct Initiative {
    round: u32,
    fight_name: String,
    combatants: Vec<Combatant>,
    curr_combatant_idx: i32
}

struct Combatant {
    name: String,
    initiative: u32,
    dex_mod: i32
}

// enum CONDITIONS {
//     BLINDED,
//     CHARMED,
//     DEAFENED,
//     EXHAUSTION,
//     FRIGHTENED,
//     GRAPPLED,
//     INCAPACITATED,
//     INVISIBLE,
//     PARALYZED,
//     PETRIFIED,
//     POISONED,
//     PRONE,
//     RESTRAINED,
//     STUNNED,
//     UNCONSCIOUS
// }

fn main() {
    println!("Hello, world!");
    print_header(String::from("DND-COMBAT-TRACKER"), '*');

    let mut initiative = Initiative {
        round: 0,
        fight_name: String::from("Test"),
        combatants: Vec::new(),
        curr_combatant_idx: -1
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
    print_initiative(&initiative);
    initiative.fight_name = String::from("TESTING MAIN MENU");

    print_options(MAIN_MENU_OPTIONS);
    loop {
        let option_result = take_option();
        let option = option_result.unwrap();

        match option.trim() {
            "1" => next(initiative),
            "2" => edit(initiative),
            "3" => add_effect(),
            "4" => {add_combatant(initiative); refresh(String::from("MAIN MENU"), MAIN_MENU_OPTIONS, &initiative)},
            "r" => {print_header(String::from("MAIN MENU"), '-'); print_initiative(&initiative); print_options(MAIN_MENU_OPTIONS)},
            "x" => break,
            _ => println!("unrecognized argument: {}", option),
        }
    }
}

fn next(initiative: &mut Initiative) {
    println!("NEXT()");
    initiative.curr_combatant_idx += 1;
    refresh(String::from("MAIN MENU"), MAIN_MENU_OPTIONS, &initiative);
}

fn edit(initiative: &mut Initiative) {
    print_header(String::from("EDIT"), '-');
    print_options(EDIT_MENU_OPTIONS);
    loop {
        let option_result = take_option();
        let option = option_result.unwrap();

        match option.trim() {
            "1" => change_initiative_round(initiative),
            "2" => change_initiative_name(initiative),
            _ => break,
        }
    }
    refresh(String::from("MAIN MENU"), MAIN_MENU_OPTIONS, &initiative);
}

fn refresh(menu_name: String, list_of_options: &[&str], initiative: &Initiative) {
    print_header(menu_name, '-');
    print_initiative(initiative);
    print_options(list_of_options);
}

fn change_initiative_round(initiative: &mut Initiative) {
    loop {
        print!("Enter the new round number: ");
        let option_result = take_option();
        let option = option_result.unwrap().trim().parse::<u32>();
        if !option.is_err() {
            initiative.round = option.unwrap();
            break;
        }
    }
    refresh(String::from("MAIN MENU"), MAIN_MENU_OPTIONS, &initiative);
}

fn change_initiative_name(initiative: &mut Initiative) {
    print!("Enter new round name: ");
    let option_result = take_option();
    initiative.fight_name = option_result.unwrap();
}

fn add_effect() {
    println!("ADD_EFFECT()");
}

fn add_combatant(initiative: &mut Initiative) {
    print!("Enter your combatant name: ");
    let combatant_name = take_option();
    print!("Enter your combatant initiative: ");
    let combatant_initiative = take_option();
    print!("Enter your combatant dex modifier: [defaults 0] ");
    let combatant_dex_mod = take_option();
    let new_combatant = Combatant {
        name: combatant_name.unwrap(),
        initiative: combatant_initiative.unwrap().trim().parse::<u32>().unwrap(),
        dex_mod: combatant_dex_mod.unwrap().trim().parse::<i32>().unwrap(),
    };
    //TODO move through initiative Vec and find spot for new combatant based on initiative and dex_mod to resolve ties
    let mut insert_idx = 0;
    for i in 0..initiative.combatants.len() {
        if initiative.combatants[i].initiative == new_combatant.initiative {
            if initiative.combatants[i].dex_mod < new_combatant.dex_mod {
                insert_idx = i;
                break;
            }
        }
        else if initiative.combatants[i].initiative < new_combatant.initiative {
            insert_idx = i;
            break;
        }
    }

    initiative.combatants.insert(insert_idx, new_combatant);
}

fn take_option() -> io::Result<String> {
    io::Write::flush(&mut io::stdout()).expect("FLUSH FAILED");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn print_initiative(initiative: &Initiative) {
    println!("{:-^80}", "");
    println!("{:^80}", initiative.fight_name);
    println!("{:^80}", initiative.round);
    for i in 0..initiative.combatants.len() {
        print!("[{}]: {}\n", initiative.combatants[i].initiative, initiative.combatants[i].name);
    }
    println!("{:-^80}", "");
}

fn print_options(list_of_options: &[&str]) {
    for i in 0..list_of_options.len() {
        println!("[{}]: {}", i + 1, list_of_options[i]);
    }
    println!("[r] to refresh screen");
    println!("[x] to exit");
    print!("Enter Selection: ");
}