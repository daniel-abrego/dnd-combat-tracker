use std::io;

const MAIN_MENU_OPTIONS: [&str; 4] = ["NEXT", "EDIT", "ADD EFFECT", "ADD COMBATANT"];

struct Initiative {
    round: u32,
    fight_name: String,
    combatants: Vec<Combatant>,
    curr_combatant_idx: i32
}

struct Combatant {
    name: String,
    dex_mod: u32
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
    initiative.fight_name = String::from("TESTING MAIN MENU");

    print_options(&mut MAIN_MENU_OPTIONS);
    loop {
        let option_result = take_option();
        let option = option_result.unwrap();

        match option.trim() {
            "1" => next(initiative),
            "2" => edit(initiative),
            "3" => add_effect(),
            "4" => add_combatant(initiative),
            "r" => {print_header(String::from("MAIN MENU"), '-'); print_options(&mut MAIN_MENU_OPTIONS)},
            "x" => break,
            _ => println!("unrecognized argument: {}", option),
        }
    }
}

fn next(initiative: &mut Initiative) {
    println!("NEXT()");
    initiative.curr_combatant_idx += 1;
}

fn edit(initiative: &mut Initiative) {
    print_header(String::from("EDIT"), '-');
    let mut options = ["CHANGE ROUND"];
    print_options(&mut options);
    loop {
        let option_result = take_option();
        let option = option_result.unwrap();

        match option.trim() {
            "1" => change_round(initiative),
            _ => break,
        }
    }
    refresh(&mut MAIN_MENU_OPTIONS);
}

fn refresh(list_of_options: &mut [&str]) {
    print_header(String::from("MAIN MENU"), '-');
    print_options(list_of_options);
}

fn change_round(initiative: &mut Initiative) {
    loop {
        println!("Enter the new round number: ");
        let option_result = take_option();
        let option = option_result.unwrap().trim().parse::<u32>();
        if !option.is_err() {
            initiative.round = option.unwrap();
            break;
        }
    }
}

fn add_effect() {
    println!("ADD_EFFECT()");
}

fn add_combatant(initiative: &mut Initiative) {
    println!("ADD_COMBATANT()");
}

fn take_option() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
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