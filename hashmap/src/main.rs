use std::collections::HashMap;


fn ex1() {
    let mut scores = HashMap::new();
    scores.insert("man united".to_string(), 3);
    scores.insert("man city".to_string(), 5);

    println!("{:?}", scores);

    let team_name = String::from("man united");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{}", score)
}

fn ex2() {
    let mut scores = HashMap::new();

    scores.insert("man united".to_string(), 3);

    scores.entry(String::from("real madrid")).or_insert(9);
    scores.entry(String::from("man united")).or_insert(5);

    println!("{:?}", scores);

}

fn ex3() {
    let text = "hello world, let's the rusty rusty";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count +=1
    }

    println!("{:?}", map)
}

fn main() {
    test_1()
}


// Given a list of integers, 
// use a vector and return the median (when sorted, the value in the middle position) 
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

fn test_1() {
    let mut v = vec![1,2,4,5,6,3,2];

    let length = v.len();

     
    println!("{:?}",v.sort())
}



