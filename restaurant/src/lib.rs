
mod back_of_house {
    pub struct  Breakfast {
        pub toast: String,
        seasonal_fruits: String
    }

    impl Breakfast {
        pub fn summer(toast: &String) -> Breakfast {
            Breakfast { toast: String::from(toast), seasonal_fruits: String::from("Pineapple") }
        }
    }
}


pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer(&String::from("toast"));


}