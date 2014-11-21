use std::os;
use std::io::process::{Command,InheritFd,ExitStatus,ExitSignal};

use self::State::{Quoted, Escaped, Normal};

enum State<'a> {
    Quoted(&'a str),
    Escaped,
    Normal
}

fn trimmed(s: &str) -> String {
    from_str(s.trim()).unwrap()
}

fn extract_commands<'a>(input: &'a str) -> Vec<String> {
    let mut state: State = Normal;
    let mut commands: Vec<String> = vec![];
    let mut it = input.graphemes(true);
    let mut s = String::new();
    loop {
        let c = match it.next() {
            Some(ch) => ch.clone(),
            None => {
                commands.push(trimmed(s.as_slice()));
                return commands;
            }
        };
        match state {
            Normal => {
                match c {
                    "\\" => {
                        state = Escaped;
                    },
                    "'" | "\"" => {
                        state = Quoted(c);
                        s.push_str(c);
                    },
                    "," => {
                        commands.push(trimmed(s.as_slice()));
                        s.clear();
                    },
                    _ => {
                        s.push_str(c);
                    },
                }
            },
            Quoted(terminator) => {
                if c == terminator {
                    state = Normal;
                }
                s.push_str(c);
            },
            Escaped => {
                state = Normal;
                s.push_str(c);
            },
        }
    }
}

fn main() {
    // remove the binary name "cargo-do", and the next 2 arguments,
    // which are just "cargo" and "do"
    let args = os::args().slice_from(3).connect(" ");
    for command in extract_commands(args.as_slice()).iter() {
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

