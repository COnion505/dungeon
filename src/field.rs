use crate::state::State;
use crate::cell::Cell;
use crate::common::Direction;

#[derive(Debug, Clone)]
pub struct Field{
    pub cells: Vec<Cell>,
    pub current_cell: Cell,
}

impl Field{
    pub fn new() -> Field{
        let cells: Vec<Cell> = Vec::new();
        Field{
            cells,
            current_cell: Cell{state: State::Unknown, x: 0, y: 0},
        }
    }
    pub fn set_start_cell(&mut self) {
        if self.cells.is_empty(){
            //println!("start dungeon game.");
            self.cells.push(
                Cell{
                    state: State::Start,
                    x: 0,
                    y: 0,
                });
            self.current_cell = self.get_start_cell();
        }else{
            //println!("already started.");
        }
    }
    pub fn get_start_cell(&self)-> Cell{
        let mut return_cell: Cell = Cell{state: State::Start, x: 0, y: 0};
        for cell in &self.cells{
            if cell.get_state() == State::Start {
                return_cell = *cell;
            }
        }
        return_cell
    }
	#[allow(dead_code)]
    pub fn get_current_cell(&self) -> Cell{
        self.current_cell
    }
    pub fn go(&mut self, dir: Direction, state_if_not_exist: State){
		let (x, y) = match dir{
			Direction::Up => (self.current_cell.get_x(), self.current_cell.get_y()-1),
			Direction::Down => (self.current_cell.get_x(), self.current_cell.get_y()+1),
			Direction::Right => (self.current_cell.get_x()+1, self.current_cell.get_y()),
			Direction::Left => (self.current_cell.get_x()-1, self.current_cell.get_y()),
		};

		if self.check_exist(x, y){
            if self.get_cell_by_pos(x, y).get_state() != State::Wall &&
               self.get_cell_by_pos(x, y).get_state() != State::FieldLimit{
                    self.move_to(x, y);
                }
		} else {
			self.create_cell(x, y, state_if_not_exist);
			self.move_to(x, y);
		}
    }

    pub fn create_cell(&mut self, x: i32, y: i32, state: State){
        if ! self.check_exist(x, y){
            self.cells.push(Cell{
                state,
                x,
                y,
            });
        }
    }
    pub fn move_to(&mut self, x: i32, y: i32) {
        if self.check_exist(x, y){
            self.current_cell = self.get_cell_by_pos(x, y);
        }
    }
    pub fn get_cell_by_pos(&self, x: i32, y: i32) -> Cell{
        let mut return_cell = Cell{state: State::Unknown, x, y};
        if self.check_exist(x,y){
            for cell in &self.cells{
                if cell.get_x() == x && cell.get_y() == y{
                    return_cell = *cell;
                }
            }
        } 
        return_cell
        
    }
    pub fn check_exist(&self, x: i32, y: i32) -> bool{
        let mut return_bool = false;
        for cell in &self.cells{
            if cell.get_x() == x && cell.get_y() == y {
                return_bool = true;
            }
        }
        return_bool
    }
    pub fn make_road(&mut self, loop_count: i32, print_console: bool){
		use rand::prelude::*;
        #[allow(unused)]
		use std::thread::sleep;
        #[allow(unused)]
		use std::time::Duration;
        let mut rng = rand::thread_rng();
        for _ in 0..loop_count{
            let rand_num: i32 = rng.gen_range(0..4);
            match rand_num{
                0 => {self.go(Direction::Up, State::Space);},
                1 => {self.go(Direction::Down, State::Space);},
                2 => {self.go(Direction::Right, State::Space);},
                3 => {self.go(Direction::Left, State::Space);},
                _ => {},
            }
            if print_console {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                self.print(print_console);
                sleep(Duration::from_millis(10));
            }
        }
        let index: usize = self.get_index_of(self.current_cell.get_x(), self.current_cell.get_y());
        self.cells.get_mut(index).unwrap().set_state(State::Goal);
        //println!("done make road. loop_count:{}", loop_count);
    }
    pub fn get_index_of(&self, x: i32, y: i32) -> usize{
        let mut return_index = 0;
        for (i, cell) in self.cells.iter().enumerate(){
            if cell.get_x() == x && cell.get_y() == y{
                return_index = i;
            }
        }
       return_index 
    }
    pub fn set_field_limit(&mut self){
        let min_x = self.min_x()-1;
        let max_x = self.max_x()+1;
        let min_y = self.min_y()-1;
        let max_y = self.max_y()+1;
        for y in min_y..=max_y{
            for x in min_x..=max_x{
                if x == min_x || x == max_x || y == min_y || y == max_y{
                    self.create_cell(x, y, State::FieldLimit);
                } 
            }
        }
    }
    pub fn generate(&mut self, size: i32, print_console: bool){
        self.set_start_cell();
        self.make_road(size, print_console);
		//road以外のセル生成
        for y in self.min_y()..=self.max_y(){
            for x in self.min_x()..=self.max_x(){
                if ! self.check_exist(x, y){
                    //self.create_cell(x, y, State::rand());
                    self.create_cell(x, y, State::Wall);
                }
            }
        }
		self.current_cell = self.get_start_cell();
        self.set_field_limit();
        let _field_height = self.get_height();
        let _field_width = self.get_width();
        //println!("field width: {}", _field_width);
        //println!("field height: {}", _field_height);
        //self.print_all_state_count();
        //self.print();
        //self.print_code();
    }
    pub fn get_height(&self) -> i32{
         self.max_y() - self.min_y() + 1
    }
    pub fn get_width(&self) -> i32{
         self.max_x() - self.min_x() + 1
    }
    pub fn min_x(&self) -> i32{
        let mut min = 9999;
        for cell in &self.cells{
            if cell.get_x() < min {
                min = cell.get_x();
            }
        }
        min
    }
    pub fn max_x(&self) -> i32{
        let mut max = -9999;
        for cell in &self.cells{
            if cell.get_x() > max {
                max = cell.get_x();
            }
        }
        max
    }
    pub fn min_y(&self) -> i32{
        let mut min = 9999;
        for cell in &self.cells{
            if cell.get_y() < min {
                min = cell.get_y();
            }
        }
        min
    }
    pub fn max_y(&self) -> i32{
        let mut max = -9999;
        for cell in &self.cells{
            if cell.get_y() > max{
                max = cell.get_y();
            }
        }
        max
    }
    #[allow(unused)]
    pub fn count_cell(&self, state: State) -> i32{
        let mut counter = 0;
        for cell in &self.cells{
            if cell.get_state() == state{
                counter += 1;
            }
        }
        counter
    }
    #[allow(unused)]
    pub fn print_all_state_count(&self){
        let cells_count = self.cells.len() as i32;
        for s in State::states().into_iter(){
            let count = self.count_cell(s);
            let _ratio = ((count as f32 / cells_count as f32 * 10000.0) as i32) as f32 / 100.0;
            //println!("{} {}: {}counts, {}%", s.get_name(), s.print(), count, _ratio);
        }
    }
    #[allow(unused)]
    pub fn print(&self, print_console: bool) -> String{
        let mut s = String::new();
        //println!("print current field...");
        for y in self.min_y()..=self.max_y(){
            for x in self.min_x()..=self.max_x(){
                //s.push_str(format!("{}", self.get_cell_by_pos(x, y).get_state().print()).as_str());
				if self.current_cell.get_x() == x && self.current_cell.get_y() == y {
					s.push_str(&self.get_cell_by_pos(x, y).get_state().print_on_player());
				} else {
					s.push_str(&self.get_cell_by_pos(x, y).get_state().print());
				}
            }
                s.push('\n');
        }
        if print_console {
            println!("{}", &s);
        }
        s
    }
    #[allow(unused)]
	pub fn print_code(&self) -> String{
        let mut s = String::new();
        //println!("print current field as code...");
        for y in self.min_y()..=self.max_y(){
            for x in self.min_x()..=self.max_x(){
                if x == self.get_current_cell().get_x() && y == self.get_current_cell().get_y() {
                    s.push('9');
                } else {
                    s.push_str(&self.get_cell_by_pos(x, y).get_state().to_code().to_string());
                }
				if x != self.max_x() {
					s.push(',');
				}
            }
            s.push('\n');
        }
        //println!("{}", &s);
        s
	}
}