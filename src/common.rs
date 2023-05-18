extern crate variant_count;
use variant_count::VariantCount;

#[derive(Debug, Clone,Copy, VariantCount, std::cmp::PartialEq, Eq)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right,
}
pub fn input(message: &str)-> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("{message}");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("failed input.");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    s
}
