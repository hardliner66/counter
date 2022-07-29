use std::{
    collections::BTreeMap,
    io::{self, Write},
    thread, time,
};
use termion::{input::TermRead, raw::IntoRawMode};

fn main() {
    // Set terminal to raw mode to allow reading stdin one key at a time
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    // Use asynchronous stdin
    let mut stdin = termion::async_stdin().keys();

    let mut my_map = BTreeMap::new();

    loop {
        // Read input (if any)
        let input = stdin.next();

        // If a key was pressed
        if let Some(Ok(key)) = input {
            match key {
                termion::event::Key::Ctrl('c') => break,
                termion::event::Key::Char(key) if ('a'..='z').contains(&key) => {
                    *my_map.entry(key).or_insert(0) += 1;
                    write!(
                        stdout,
                        "{}{}",
                        termion::clear::All,
                        termion::cursor::Goto(1, 1),
                    )
                    .unwrap();

                    let s = format!("{:#?}", my_map);
                    for (c, l) in s.lines().enumerate() {
                        write!(stdout, "{}{}", termion::cursor::Goto(1, c as u16 + 1), l,).unwrap();
                    }

                    stdout.lock().flush().unwrap();
                }
                _ => continue,
            }
        } else {
            thread::sleep(time::Duration::from_millis(5));
        }
    }
}
