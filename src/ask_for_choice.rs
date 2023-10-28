use std::time::Duration;

use crossterm::event::{ poll, read, Event, KeyCode, KeyEvent };

pub(crate) fn get_choice(items: &[&str]) -> Result<usize, String> {
    if items.len() == 0 {
        return Err("No items to choose from".to_string());
    }

    let mut choice = 1;
    loop {
        print!("\x1b[2J\x1b[H"); // Clear the screen
        println!("Choose an option by navigating with arrow keys and pressing Enter:");
        for (i, item) in items.iter().enumerate() {
            println!("{} {}. {}", if choice == i + 1 { "->" } else { "  " }, i + 1, item);
        }

        let key = get_key();

        if key.is_down {
            choice += 1;
            if choice > items.len() {
                choice = 1;
            }
        } else if key.is_up {
            choice -= 1;
            if choice < 1 {
                choice = 3;
            }
        } else if key.is_enter {
            return Ok(choice);
        }
    }
}

struct GetKeyResult {
    is_down: bool,
    is_up: bool,
    is_enter: bool,
}

impl GetKeyResult {
    fn new(event: Event) -> Result<Self, String> {
        let obj = GetKeyResult {
            is_down: event == Event::Key(KeyEvent::from(KeyCode::Down)) ||
            event == Event::Key(KeyEvent::from(KeyCode::Right)),
            is_up: event == Event::Key(KeyEvent::from(KeyCode::Up)) ||
            event == Event::Key(KeyEvent::from(KeyCode::Left)),
            is_enter: event == Event::Key(KeyEvent::from(KeyCode::Enter)),
        };
        if obj.is_down || obj.is_up || obj.is_enter {
            return Ok(obj);
        }
        Err("Not a valid key".to_string())
    }
}

fn get_key() -> GetKeyResult {
    loop {
        // `poll()` waits for an `Event` for a given time period
        if poll(Duration::from_millis(500)).unwrap() {
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
