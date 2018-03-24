#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate redis;
extern crate tempfile;
#[macro_use]
extern crate bmp;

use redis::Commands;
use std::env;
use std::fs::File;

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

#[get("/board.bmp")]
fn board() -> Vec::<u8> {
    use bmp::{Image, Pixel};
    let board = get_board();
    let mut img = Image::new(WIDTH as u32, HEIGHT as u32);
    use std::io::*;
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let (r,g,b) = get_color(board[x as usize + y as usize * WIDTH]);
            img.set_pixel(x as u32, y as u32, px!(r, g, b));
        }
    }

    let mut c = Cursor::new(Vec::new());

    let _ = img.to_writer(&mut c);

    let mut data = Vec::new();

    c.read_to_end(&mut data).unwrap();

    data
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
