
// Enums are useful because they allow you to define a type with a fixed set of values, which can make your code more readable and easier to understand.
//  They also allow you to define different data structures within a single type, which can be more efficient than using multiple types.

#[derive(Debug)]
enum matchPossibilities{
    win,
    draw,
    lose
}

#[derive(Debug)]
struct matchResult {
    result: matchPossibilities,
    team: String
}

fn main() {

    let mutd = matchResult {
        result: matchPossibilities::win,
        team: String::from("manchester utd")
    };

    let chelsea = matchResult {
        result: matchPossibilities::lose,
        team: String::from("Chelsea")
    };

    // println!("{:#?}", mutd);
    // println!("{:#?}", chelsea);

    let some_number = Some(5);
    let some_char = Some("elisha");
    let absent_no: Option<i32> = None;
    
}
