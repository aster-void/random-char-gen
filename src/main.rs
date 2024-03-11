use std::fmt::Debug;
use std::str::FromStr;
use std::string::ParseError;

use rand::Rng;

fn main() {
    'recreate: loop {
        let options: Options = read_input();
        let mut show_options = true;
        'save_options: loop {
            let chars = generate_random_chars(&options);
            write(&chars, show_options);
            // only show options once
            if show_options {
                show_options = false;
            }
            match getch_answer() {
                Answer::Recreate => continue 'save_options,
                Answer::StartOver => continue 'recreate,
                Answer::Quit => break 'recreate,
                Answer::ParseErr => break 'recreate,
            }
        }
    }
}

fn _main() {
    _test();
}

fn _test() {
    println!("AAAA");
    print!("{}[2J", 27 as char);
    // std::process::Command::new("cmd.exe /c cls").status().unwrap();
    println!("BBBB");
}

#[derive(Copy, Clone)]
struct Options {
    len: usize,
    allowed_letters: Letters,
}

#[derive(Copy, Clone, Debug)]
struct Letters {
    lowercase: bool,
    uppercase: bool,
    numbers: bool,
    symbols: bool,
}

impl Letters {
    fn iter(&self) -> LettersAsIter {
        let letters = self.clone();
        return LettersAsIter { letters, iter: 0 };
    }
}

impl FromStr for Letters {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Letters, Self::Err> {
        let s = s.to_string();
        let allowed_letters = Letters {
            lowercase: s.has('1'),
            uppercase: s.has('2'),
            numbers: s.has('3'),
            symbols: s.has('4'),
        };
        return Ok(allowed_letters);
    }
}

struct LettersAsIter {
    letters: Letters,
    iter: i32,
}

impl Iterator for LettersAsIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.iter {
            0 => Some(self.letters.lowercase),
            1 => Some(self.letters.uppercase),
            2 => Some(self.letters.numbers),
            3 => Some(self.letters.symbols),
            _ => None,
        };
        self.iter += 1;
        return result;
    }
}

trait StringHas {
    fn has(&self, letter: char) -> bool;
}

impl StringHas for String {
    fn has(&self, letter: char) -> bool {
        return self.chars().any(|char| char == letter);
    }
}

enum Answer {
    Recreate,
    StartOver,
    Quit,
    ParseErr,
}

impl Answer {
    fn find(char: char) -> Answer {
        match char {
            'r' => Self::Recreate,
            's' => Self::StartOver,
            'q' => Self::Quit,
            _ => Self::ParseErr,
        }
    }
}

fn generate_random_chars(&options: &Options) -> Vec<char> {
    let lowercase = "abcdefghijklmnopqrstuvwxyz";
    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let symbols = "`~!@#$%^&*()_+-=[]\\{}|;':\",./<>?";
    let allowed = options.allowed_letters;
    let mut population = String::new();
    if allowed.lowercase {
        population += lowercase;
    }
    if allowed.uppercase {
        population += uppercase;
    }
    if allowed.numbers {
        population += numbers;
    }
    if allowed.symbols {
        population += symbols;
    }
    if population == "" {
        panic!("You bypassed something smh");
    }

    let mut rng = rand::thread_rng();
    let population = population.chars().collect::<Vec<char>>();
    let pop_len = population.len();
    let mut result = Vec::new();
    result.reserve(options.len);
    for _ in 0..options.len {
        let r = rng.gen_range(0..pop_len);
        result.push(population[r]);
    }
    return result;
}

fn getch_answer() -> Answer {
    let g = getch_rs::Getch::new();
    match g.getch() {
        Ok(getch_rs::Key::Char(char)) => Answer::find(char),
        _ => Answer::ParseErr,
    }
}

fn write(vec: &Vec<char>, should_show_options: bool) {
    let count = vec.len();
    let mut chars = String::new();
    for char in vec.iter() {
        chars.push(*char);
    }
    if should_show_options {
        println!(
            "\
{count} characters generated:"
        );
    }
    println!("{chars}");
    if should_show_options {
        println!(
            "
Press \"s\" to start over
Press \"r\" to recreate
Press any other key to exit..."
        );
    }
}

fn ask<T: FromStr>(msg: &str, err_msg: &str, validator: Option<fn(x: &T) -> bool>) -> T {
    let validator = match validator {
        Some(func) => func,
        None => |_: &T| true,
    };
    print!("{}", msg);
    loop {
        let result = prompt();
        let result: Option<T> = result.parse::<T>().ok();
        match result {
            Some(r) if validator(&r) => return r,
            _else => {
                print!("{}", err_msg);
                continue;
            }
        }
    }
}

fn prompt() -> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

fn read_input() -> Options {
    let len: usize = ask(
        "Enter the length desired [unsigned int only]: ",
        "Please input valid length ( > 0 )",
        Some(|&x| x > 0),
    );
    let allowed: Letters = ask(
        "Select the options specified below to generate:
        1 - lowercase [a-z]
        2 - uppercase [A-Z]
        3 - numbers [0-9]
        4 - symbols [\"'!@#$...]\n: ",
        "Invalid options. Please try again",
        Some(|letters: &Letters| letters.iter().any(|b| b == true)),
    );
    return Options {
        len,
        allowed_letters: allowed,
    };
}
