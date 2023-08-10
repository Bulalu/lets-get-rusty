use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108]; 
static C: [u8; 10] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104];
fn main() {

    // let a = 10;
    // let b = Box::new(20);
    // let c = Rc::new(Box::new(30));
    // let d = Arc::new(Mutex::new(40));

    // println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);
    let a = 42;
    let b = &B;
    let c = &C;
   
    println!("a: {}, b: {:p}, c: {:p}", a, b, c);
}

fn grep() {
    let search_term = "picture";
    let quote = "Every face, every shop, bedroom window, public-house, and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books. What do we seek through millions of pages?";
    for (idx,line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            let line_num = idx + 1;
            println!("{}: {}",line_num, line);
        }
    }
}

fn read_from_file() {
    let path = "./foo.txt";
    let  file = File::open(path).expect("Could not open file");
    let mut reader = BufReader::new(file);

    println!("Reading file line by line");
    println!("{:?}", reader)

}
