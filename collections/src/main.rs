
fn v1() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.pop();
    println!("{:?}", v)
}

fn v2() {
    let v = vec![1,2,3,4,5,6];

    let third = &v[5];
    println!("The third element is {third}");

    let third = v.get(6);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }
}

fn v3() {
    let v = vec![1,2,3,4,5,6];

    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    println!("{:?}", does_not_exist)

}

fn v4() {

    let v = vec![13,20,90];

    for i in &v {
        println!("{i}")
    }
}
 
fn v5() {
    let mut v = vec![10,20,30];

    for i in &mut v {
        *i += 10;
    }

    println!("{:? }", v)
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

fn v6() {

 

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    println!("{:#?}", row);
}

fn main() {
    v6()
}
