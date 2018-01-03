use std::collections::HashSet;
use std::iter::FromIterator;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn check_password(password: &String) -> bool {
    let words = password.split_whitespace();
    let mut word_set = HashSet::new();
    for word in words {
        if word_set.contains(word) {
            return false;
        }
        word_set.insert(word);
    }
    return true;
}

fn check_password2(password: &String) -> bool {
    let words = password.split_whitespace();
    let mut word_set = HashSet::new();
    for word in words {
        let word = sort_chars(&word);
        if word_set.contains(&word) {
            return false;
        }
        word_set.insert(word);
    }
    return true;
}

fn sort_chars(a: &str) -> String {
    let mut chars = a.chars().collect::<Vec<char>>();
    chars.sort_unstable();
    String::from_iter(chars)
}

fn main() {
    let path = std::env::args().nth(1).expect("no input file given!");
    let f = File::open(path).expect("unable to open file");
    let f = BufReader::new(f);
    // TODO: copy iterator instead of collect()?
    let lines = f.lines().map(|x| x.unwrap()).collect::<Vec<String>>();
    let num_valid = lines.iter().filter(|x| check_password(x)).count();
    let num_valid2 = lines.iter().filter(|x| check_password2(x)).count();
    println!("{}\n{}", num_valid, num_valid2);
}
