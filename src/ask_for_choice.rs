use std::time::Duration;

use crossterm::event::{ poll, read, Event, KeyCode, KeyEvent, KeyEventKind };

pub(crate) fn get_choice(items: &[&str]) -> Result<usize, String> {
    if items.len() == 0 {
        return Err("No items to choose from".to_string());
    }

    let mut choice: i32 = 0;
    loop {
        // Write the same for above but memorize the string and print it out at the end with all parameters
        let mut str: String = "".to_string();

        for (i, item) in items.iter().enumerate() {
            // Concat strings "{} {}. {}\n"
            str = format!(
                "{} {}{}. {}",
                str,
                if choice == (i as i32) {
                    " ->"
                } else {
                    "   "
                },
                i,
                item
            );
        }

        println!("\x1b[2J\x1b[H\nChoose an option by navigating with arrow keys and pressing Enter:\n{}", str); // Clear the screen and print the string

        let key = get_key();

        match key {
            GetKeyResult::IsDown => {
                choice += 1;
                if choice > ((items.len() - 1) as i32) {
                    choice = 0;
                }
            }
            GetKeyResult::IsUp => {
                choice -= 1;
                if choice < 0 {
                    choice = (items.len() - 1) as i32;
                }
            }
            GetKeyResult::IsEnter => {
                return Ok(choice as usize);
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
            | Event::Key(KeyEvent { code: KeyCode::Down, kind: KeyEventKind::Release, .. })
            | Event::Key(KeyEvent { code: KeyCode::Right, kind: KeyEventKind::Release, .. }) => {
                return Ok(GetKeyResult::IsDown);
            }
            | Event::Key(KeyEvent { code: KeyCode::Up, kind: KeyEventKind::Release, .. })
            | Event::Key(KeyEvent { code: KeyCode::Left, kind: KeyEventKind::Release, .. }) => {
                return Ok(GetKeyResult::IsUp);
            }
            Event::Key(KeyEvent { code: KeyCode::Enter, kind: KeyEventKind::Release, .. }) => {
                return Ok(GetKeyResult::IsEnter);
            }
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
