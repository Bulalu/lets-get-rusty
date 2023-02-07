use  std::env;


fn main() {
    // println!("Hello, world!");
    get_image_binary();

}

fn get_image_binary() {
    let pinata_api_key = env::var("PINATA_API_KEY").unwrap();
    let pinata_secret_key = env::var("PINATA_API_SECRET").unwrap();
    println!("{:?}", pinata_api_key)
}