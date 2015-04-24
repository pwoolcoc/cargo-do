use std::env;
// use std::io::process::{Command,InheritFd,ExitStatus,ExitSignal};
use std::process::{exit, Command, Stdio, ExitStatus};

use self::State::{Quoted, Escaped, Normal};

enum State {
    Quoted(char),
    Escaped,
    Normal
}

fn trimmed(s: &str) -> String {
    s.trim().to_string()
}

fn extract_commands(input: &str) -> Vec<String> {
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
    // this gets invoked like this:
    //
    //   $ cargo-do cargo do [args]
    //
    // so we throw away the `cargo-do` and the `do`.
    // But, we keep the `cargo`, because we don't want
    // to just assume that `cargo` is the name of the cargo
    // binary, nor do we want to assume that it is on our path
    let mut args = env::args();
    let binname = args.nth(1).unwrap();
    let args = args.skip(1).collect::<Vec<_>>().connect(" ");

    for command in extract_commands(&args).iter() {
        let status = Command::new(&binname)
                        .args(&[command])
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .status();
        match status {
            Ok(st) => {
                if !st.success() {
                    match st.code() {
                        Some(i) => { exit(i); },
                        None => { exit(127); }
                    }
                }

                exit(0);
            },
            Err(_) => {
                exit(127);
            },
        }
    }
}

