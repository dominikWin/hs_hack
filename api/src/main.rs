#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate redis;
extern crate tempfile;

use redis::Commands;
use std::env;
use std::fs::File;
use std::io::*;

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

#[get("/write/<x>/<y>/<color>")]
fn write(x: u16, y: u16, color: u8) -> &'static str {
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

fn get_color(c: u8) -> (u8, u8, u8) {
    match c {
        0 => (255,255,255),  // white
        1 => (230,126,34),   // orange
        2 => (176,58,46),    // red
        3 => (244,208,63),   // yellow
        4 => (52,152,219),   // blue
        5 => (82,190,128),   // green
        6 => (31,97,141),    // dark blue
        7 => (125,60,152),   // purple
        8 => (233,30,99),    // pink
        9 => (195,155,211),  // lavender
        10 => (118,215,196), // teal
        11 => (214,234,248), // light blue
        12 => (210,180,140), // tan
        13 => (23,32,42),    // black
        14 => (129,199,132), // light green
        15 => (160,64,0),    // brown
        _ => panic!()
    }
} 

#[get("/board.png")]
fn board() -> String {
    let board = get_board();
    let size = 12;
    let mut content = String::new();
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let (r,g,b) = get_color(board[x as usize + y as usize * WIDTH]);
            content += &format!("<rect x=\"{x}\" y = \"{y}\" width = \"{size}\" height = \"{size}\" style=\"fill:rgb({r},{g},{b});\" />", x = x * size, y = y * size, size = size, r=r, g=g,b=b);
        }
    }
    format!("<svg width = \"{w}\" height = \"{h}\">
                {content} 
            </svg>", content = content, w = WIDTH * size, h = HEIGHT * size)
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
    

            rocket::ignite().mount("/", routes![write,board]).launch();

}

fn get_board() -> Vec<u8> {
    let con = get_con();

    con.get("board").unwrap()
}

fn set_board(board: Vec<u8>) {
    let con = get_con();

    let _: () = con.set("board", board).unwrap();
}
