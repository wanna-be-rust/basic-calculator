use std::error::Error;
// mod ask_for_choice;

// ask_for_choice::get_choice(&["Free", "Guided"]).unwrap();

fn main() {
    println!("Welcome to a basic calculator!");

    let choices = ["FREE", "GUIDED"];

    for choice in choices {
        println!("- {}", choice);
    }

    let user_selection: usize = handle_user_input();

    let selected_choice = match_mode(user_selection, &choices);

    if selected_choice == choices[0] {
        println!("Guided Mode Started");

        init_guided_mode()
    } else {
        println!("Free Mode Started")
    }
}

fn handle_user_input() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let user_choice = input.trim();
    let user_selection = user_choice.parse::<usize>().unwrap();

    user_selection
}

fn match_mode<'a>(selection: usize, choices: &'a [&str]) -> &'a str {
    let current_selection: &str;
    match selection {
        1 => {
            println!("You selected {}", choices[0]);
            current_selection = choices[0];
        }
        2 => {
            println!("You selected {}", choices[1]);
            current_selection = choices[1];
        }
        _ => {
            println!("Invalid option. Please try again");
            let sel = handle_user_input();
            current_selection = match_mode(sel, choices);
        }
    }

    current_selection
}

fn init_guided_mode() {
    print!("\x1B[2J\x1B[1;1H");

    let first: i32;
    let second: i32;

    loop {
        let first_number: Result<i32, Box<dyn Error>> = handle_number_input();

        if first_number.is_ok() {
            first = first_number.unwrap();
            break;
        }
    }

    loop {
        let second_number: Result<i32, Box<dyn Error>> = handle_number_input();
        if second_number.is_ok() {
            second = second_number.unwrap();
            break;
        }
    }

    println!("{} + {} = {}", first, second, first + second)
}

fn handle_number_input() -> Result<i32, Box<dyn Error>> {
    println!("Insert a number: ");

    let input: usize = handle_user_input();
    let current_number: i32 = match input.to_string().parse::<i32>() {
        Ok(value) => value,
        Err(err) => {
            println!("Not a numeric input!");
            return Err(Box::new(err));
        }
    };

    Ok(current_number)
}
