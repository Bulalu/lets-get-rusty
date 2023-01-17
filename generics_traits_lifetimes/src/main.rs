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

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U
}

//TRAITS

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
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
        y:"yo bro"
    };
    println!(
        "{:?}", test
    )
}



