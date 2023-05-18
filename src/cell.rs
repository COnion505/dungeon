use crate::state::State;


#[derive(Debug, Clone,Copy, std::cmp::PartialEq, Eq)]
pub struct Cell{
    pub state: State,
    pub x: i32,
    pub y: i32,
}
impl Cell{
	#[allow(unused)]
    pub fn new() -> Cell{
        Cell{
            state: State::Unknown,
            x: 0,
            y: 0,
        }
    }
	#[allow(unused)]
    pub fn view(&self){
        println!("state:{:?},\nx:{:?},\ny:{:?}",&self.state,&self.x,&self.y);
    }
    pub fn get_x(&self) -> i32{
        self.x
    }
    pub fn get_y(&self) -> i32{
        self.y
    }
    pub fn set_state(&mut self, state: State){
        self.state = state;
    }
    pub fn get_state(&self) -> State {
        self.state
    }
}