#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate redis;

use redis::Commands;
use std::env;

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

#[get("/write/<x>/<y>/<color>")]
fn index(x: u16, y: u16, color: u8) -> &'static str {
        if color > 15 {
            return "Invalid color";
        }
        
        if x >= WIDTH as u16 {
            return "Invalid x";
        }
        if y >= HEIGHT as u16 {
            return "Invalid y";
        }

        let mut board = get_board();

        let pos: usize = x as usize + y as usize * WIDTH;

        board[pos] = color;

        set_board(board);

        "Success"
}

fn get_con() -> redis::Connection {
 let client = redis::Client::open(&env::var("REDIS").unwrap() as &str).unwrap();
    let con = client.get_connection().unwrap();

    con

}

fn main() {

    {

        use std::{thread, time};

        thread::sleep(time::Duration::from_secs(3));
    }

    println!("Starting API service");
    let con = get_con();

    let empty_board: Vec<u8> = vec![0u8; WIDTH * HEIGHT];

    let _: () = con.set("board", empty_board).unwrap();
    

            rocket::ignite().mount("/", routes![index]).launch();

}

fn get_board() -> Vec<u8> {
    let con = get_con();

    con.get("board").unwrap()
}

fn set_board(board: Vec<u8>) {
    let con = get_con();

    let _: () = con.set("board", board).unwrap();
}
