use std::env;
use dotenv::dotenv;

mod dungeon;
mod cell;
mod state;
mod field;
mod common;
mod field3d;


fn main() {
    // 第一引数で生成するマップのサイズを数字で受け取る。
    // デフォルトは100。数字はループの回数。
    // スタートからゴールまで移動する距離。
//    let args: Vec<String> = env::args().collect();
//    let input: String = match args.get(1){
//        Some(i) => String::from(i),
//        None => String::from("100"),
//    };
//	dungeon::run(input);

    dotenv().ok();
    let loop_count: String = match env::var("LOOP_COUNT") {
        Ok(i) => i,
        Err(_) => String::from("100"),
    };
    dungeon::run(loop_count);
}
