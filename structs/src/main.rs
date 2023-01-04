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

  

    let user1 = create_user(String::from("Elisha Bulalu"), String::from("me@ai.xyz"),  90);
    println!("{:?}", user1);
    let user2 = User {
        active: false,
        ..user1
    };
    println!("{:?}", user2);

    let black = Color(0,0,0);
    println!("{:?}", black);
}

fn create_user(username: String, email: String, followers: u64) -> User {
    User {
        active: true,
        username,
        email,
        followers: followers
    }

}
