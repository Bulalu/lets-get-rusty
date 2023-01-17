use std::cmp::PartialOrd;

fn largest<T:PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for value in list {
        if value > largest {
            largest = value
        }
    }

    largest
}

struct Point<T, U> {
    x: T,
    y: U
}
fn main() {
    println!("Hello, world!");
    let v = vec![1,2,4,5,6];
    let chars = vec!["t", "x", "g", "z"];
    let largest1 = largest(&v);
    let largest2 = largest(& chars);
    println!("{largest1}");
    println!("{largest2}");

    let test = Point {
        x:5,
        y:4.0
    };
}



