use std::fs::File;
use std::io::prelude::*;


use crate::field::Field;
use crate::common;
use crate::common::Direction;
use crate::state::State;


#[allow(unused)]
#[derive(Debug, Clone)]
struct Dungeon {
    name: String,
    field: Field,
}


pub fn run(loop_count: String, print_console: bool){
    let loop_count:i32 = match loop_count.parse() {
        Ok(i) if i < 100 => 100,
        Ok(i) if i > 10000 => 10000,
        Ok(i) => i,
        Err(_) => 100,
    };

    let mut f = Field::new();
    f.generate(loop_count, print_console);

    let mut d = Dungeon {
        name: String::from("dungeon"),
        field: f, 
    };
    let mut file = File::create("map-code.txt")
        .expect("can't ceate map-code.txt");
    file.write_all(d.field.print_code().as_bytes())
        .expect("can't write to file.");
    
    let mut file2 = File::create("map-text.txt")
        .expect("can't create map-text.txt");
    file2.write_all(d.field.print(false).as_bytes())
        .expect("can't write to file.");

    for _ in 0..loop_count {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("loop count: {}", loop_count);
        println!("{}",d.field.print(false));
        //println!("{}",d.field.print_code());
        if d.field.get_current_cell().get_state() == State::Goal {
            println!("GOAL!!");
            return;
        }
        let dir = common::input("input:");
        match dir.as_str() {
           "w" => {d.field.go(Direction::Up, State::Space);}, 
           "s" => {d.field.go(Direction::Down, State::Space);}, 
           "d" => {d.field.go(Direction::Right, State::Space);}, 
           
           
           _ => {},
        }
    }
    println!("YOU DIED!!");

}

