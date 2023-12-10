use std::collections::HashSet;
use std::io;

fn main() {
    let vowels: HashSet<char> = vec!['a', 'e', 'i', 'o', 'u'].into_iter().collect();
    loop {
        println!("input word");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let first = input.chars().next().unwrap();
        let rest: String = input.chars().skip(1).collect();
        if vowels.contains(&first) {
            println!("{first}{rest}-hay");
        } else {
            println!("{rest}-{first}ay");
        }
    }
}
