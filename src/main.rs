use std::os;
use std::io::process::{Command,InheritFd,ExitStatus,ExitSignal};

use self::State::{Quoted, Escaped, Normal};

enum State {
    Quoted(char),
    Escaped,
    Normal
}

fn trimmed(s: &String) -> String {
    from_str::<String>(s.trim_chars(' ')).unwrap()
}

fn extract_commands(input: String) -> Vec<String> {
    let mut state: State = Normal;
    let mut commands: Vec<String> = vec![];
    let mut it = input.chars();
    let mut s = String::new();
    loop {
        let c = match it.next() {
            Some(ch) => ch.clone(),
            None => {
                commands.push(trimmed(&s));
                return commands;
            }
        };
        match state {
            Normal => {
                match c {
                    '\\' => {
                        state = Escaped;
                    },
                    '\'' | '"' => {
                        state = Quoted(c);
                        s.push(c);
                    },
                    ',' => {
                        commands.push(trimmed(&s));
                        s.clear();
                    },
                    _ => {
                        s.push(c);
                    },
                }
            },
            Quoted(terminator) => {
                if c == terminator {
                    state = Normal;
                }
                s.push(c);
            },
            Escaped => {
                state = Normal;
                s.push(c);
            },
        }
    }
}

fn main() {
    let mut a = os::args();
    // remove the binary name
    let _ = a.remove(0);
    let args = a.connect(" ");
    for command in extract_commands(args).iter() {
        // need to improve this, it assumes cargo is in our path..
        let status = Command::new("cargo")
                        .args(&[command])
                        .stdin(InheritFd(0))
                        .stdout(InheritFd(1))
                        .stderr(InheritFd(2))
                        .status();
        match status {
            Ok(ExitStatus(0)) => (),
            Ok(ExitStatus(i)) => {
                os::set_exit_status(i)
            },
            Ok(ExitSignal(i)) => {
                os::set_exit_status(i)
            },
            Err(_) => {
                os::set_exit_status(127)
            },
        }
    }
}

