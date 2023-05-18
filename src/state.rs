extern crate variant_count;
use variant_count::VariantCount;


#[derive(Debug, Clone,Copy, VariantCount, std::cmp::PartialEq, Eq)]
pub enum State{
    Unknown,
    Enemy,
    Friend,
    Wall,
    Hole,
    Goal,
    Start,
    Space,
    FieldLimit,
}
impl State{
    pub fn states() -> Vec<State> {
        vec![
            State::Unknown,
            State::Enemy,
            State::Friend,
            State::Wall,
            State::Hole,
            State::Goal,
            State::Start,
            State::Space,
            State::FieldLimit,
        ]
    }
    #[allow(unused)]
    pub fn get_name(&self) -> String{
        match self{
            State::Unknown => String::from("Unknown"),
            State::Enemy => String::from("Enemy"),
            State::Friend => String::from("Friend"),
            State::Wall => String::from("Wall"),
            State::Hole => String::from("Hole"),
            State::Goal => String::from("Goal"),
            State::Start => String::from("Start"),
            State::Space => String::from("Space"),
            State::FieldLimit => String::from("FieldLimit"),
        }
    }
    // fn ratio(&self) -> i32{
    //     match self{
    //         State::Unknown =>  5,
    //         State::Hole    =>  10,
    //         State::Friend  =>  15,
    //         State::Enemy   =>  20,
    //         State::Wall    =>  22,
    //         State::Space   =>  28,
    //         _ => 0,
    //     }
    // }
    pub fn ratios() -> Vec<(State, i32)> {
       vec![
        (State::Unknown, 3),
        (State::Hole, 5),
        (State::Friend, 10),
        (State::Enemy, 18),
        (State::Wall, 25),
       ] 
    }
    pub fn rand() -> State {
		use rand::prelude::*;
        let mut rng = rand::thread_rng();
        let i: i32 = rng.gen_range(0..100);
        let mut counter: i32 = 0;
        let mut return_state = State::Space;
        for (s, r) in State::ratios().iter() {
            counter += r;
            if counter > i {
                return_state = *s;
                break;
            }
        }
        return_state
    }
    pub fn to_code(self) -> i32{
        //match self{
        //    State::Unknown =>  State::Unknown as i32,
        //    State::Enemy   =>  State::Enemy as i32,
        //    State::Friend  =>  State::Friend as i32,
        //    State::Wall    =>  State::Wall as i32,
        //    State::Hole    =>  State::Hole as i32,
        //    State::Goal    =>  State::Goal as i32,
        //    State::Start   =>  State::Start as i32,
        //    State::Space   =>  State::Space as i32,
        //    State::FieldLimit   =>  State::FieldLimit as i32,
        //}
        self as i32
    }

    pub fn print(&self) -> String{
        match self{
            State::Unknown => String::from("?"),
            State::Enemy => String::from("!"),
            State::Friend => String::from("+"),
            State::Wall => String::from("#"),
            State::Hole => String::from("O"),
            State::Goal => String::from("g"),
            State::Start => String::from("S"),
            State::Space => String::from(" "),
            State::FieldLimit => String::from("*"),
        }
    }
    pub fn print_on_player(&self) -> String{
        match self{
            State::Enemy => String::from("i"),
            State::Friend => String::from("x"),
            State::Hole => String::from("Q"),
            State::Goal => String::from("G"),
            State::Start => String::from("$"),
            State::Space => String::from("P"),
			_ => String::from("/"),
        }
    }
}
