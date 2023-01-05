
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

#[derive(Debug)]
enum PlayerPosition {
    Striker,
    Winger
}
enum Players {
    Ronaldo,
    Mbappe,
    Rashford(PlayerPosition),
    Benzema
}


fn player_ratings(players: Players) -> u8 {
    match players {
        Players::Benzema => 90,
        Players::Mbappe => 91,
        Players::Rashford(state)=> {
            println!("Player Position is {:?}!", state);
            88
        },
        Players::Ronaldo => 89
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
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

    // let results = player_ratings(Players::Rashford(PlayerPosition::Striker));
    // println!("{}", results)

    let five = Some(5);
    let six =  plus_one(five);
    let none = plus_one(None);

    println!("{:?}", six);
    println!("{:?}", none);


    
}
