use std::fs::File;
use std::io::BufReader;


#[derive(Debug)]

struct User {
    active: bool,
    username: String,
    email: String,
    followers: u64
}

#[derive(Debug)]
struct Color(i32, i32, i32);

fn main() {
    let file_path = include_str!("data/s.json");
    let file = File::open(file_path);


    println!("{:#?}", file);



}
