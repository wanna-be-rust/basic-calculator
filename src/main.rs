mod ask_for_choice;

fn main() {
    println!("Hello, world!");
    // ask_for_choice(10)

    let choices = [
        "Is whatever I want",
        "Is whatever you want",
        "Is whatever he want",
        "Is whatever she want",
        "Is whatever we want",
        "Is whatever they want",
    ];
    println!("{}", ask_for_choice::get_choice(&choices).unwrap())
}
