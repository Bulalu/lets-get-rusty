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
    let mut v = vec![1,2,4,5,6,6,3,6,3,2,9,8,7,8,8,8];
    median_and_mode(v);
}


// Given a list of integers, 
// use a vector and return the median (when sorted, the value in the middle position) 
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

fn median_and_mode(numbers: Vec<i32>) -> (f64, i32)  {

    let mut numbers = numbers;
 

    // // median solution
    numbers.sort();
    let length = numbers.len();
    // println!("{:?}",numbers);

    let median = if length % 2 == 0 {
        //calculate the anumberserage of the two numbers in the middle
         (numbers[length / 2] + numbers[length / 2 - 1]) as f64  / 2.0
       
    } else {

        numbers[length/2] as f64
    };

    

    // // mode solution
    let mut mode = 0;
    let mut mode_count = 0;
    let mut counts = HashMap::new();

    for &number in &numbers {
        *counts.entry(number).or_insert(0) += 1;
    }

    for (&number, &count) in &counts {

        if count > mode_count {
            mode = number;
            mode_count = count;
        }
    }

    

    (median , mode)



}



