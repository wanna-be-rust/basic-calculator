use std::time::Duration;

use crossterm::event::{ poll, read, Event, KeyCode, KeyEvent };

pub(crate) fn get_choice(items: &[&str]) -> Result<usize, String> {
    if items.len() == 0 {
        return Err("No items to choose from".to_string());
    }

    let mut choice = 0;
    loop {
        // print!("\x1b[2J\x1b[H"); // Clear the screen
        println!("Choose an option by navigating with arrow keys and pressing Enter:");
        for (i, item) in items.iter().enumerate() {
            print!("{} {}. {}\n", if choice == i { "->" } else { "  " }, i, item);
        }

        let key = get_key();

        match key {
            GetKeyResult::IsDown => {
                choice += 1;
                if choice > items.len() {
                    choice = 0;
                }
            }
            GetKeyResult::IsUp => {
                choice -= 1;
                if choice < 1 {
                    choice = items.len() - 1;
                }
            }
            GetKeyResult::IsEnter => {
                return Ok(choice);
            }
        }
    }
}

enum GetKeyResult {
    IsDown,
    IsUp,
    IsEnter,
}

impl GetKeyResult {
    fn new(event: Event) -> Result<Self, String> {
        match event {
            | Event::Key(KeyEvent { code: KeyCode::Down, .. })
            | Event::Key(KeyEvent { code: KeyCode::Right, .. }) => {
                Ok(GetKeyResult::IsDown)
            }
            | Event::Key(KeyEvent { code: KeyCode::Up, .. })
            | Event::Key(KeyEvent { code: KeyCode::Left, .. }) => {
                Ok(GetKeyResult::IsUp)
            }
            Event::Key(KeyEvent { code: KeyCode::Enter, .. }) => { Ok(GetKeyResult::IsEnter) }
            _ => { Err("Not a valid key".to_string()) }
        }
    }
}

fn get_key() -> GetKeyResult {
    loop {
        // `poll()` waits for an `Event` for a given time period
        if poll(Duration::from_millis(0)).unwrap() {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            let event = read().unwrap();

            match GetKeyResult::new(event) {
                Ok(obj) => {
                    return obj;
                }
                Err(_) => {
                    continue;
                }
            }
        }
    }
}
