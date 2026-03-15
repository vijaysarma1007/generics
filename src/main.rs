#[derive(Debug)]
struct TeasureChest<T> {
    captain: String,
    treasure: T,
}

impl TeasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string();
        println!("cleaned chest");
    }
}

impl TeasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

impl<T> TeasureChest<T> {
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

enum Cheesesteak<T> {
    Plain,
    Topping(T),
}

fn main() {
    let mushrooms = Cheesesteak::Topping::<String>(String::from("mushroom"));
    let onions = Cheesesteak::Topping::<String>(String::from("onions"));
    let topping = "cottage cheese".to_string();
    let paneer = Cheesesteak::Topping::<&String>(&topping);

    let mut plain: Cheesesteak<String> = Cheesesteak::Plain;
    //not allowed -> plain = Cheesesteak::Topping("String");

    let gold_chest = TeasureChest::<[&str; 3]> {
        captain: String::from("Jack Sparrow"),
        treasure: ["gold", "silver", "platinum"],
    };

    let result = gold_chest.amount_of_treasure();
    println!("size of the chest: {}", result);
    let captain1 = gold_chest.capital_captain();
    println!("captial captain: {}", captain1);

    let mut silver_chest = TeasureChest::<String> {
        captain: String::from("Bloodsail"),
        treasure: String::from("Silver"),
    };

    silver_chest.clean_treasure();
    let captain2 = silver_chest.capital_captain();
    println!("captial captain: {}", captain2);

    println!("{}", identity::<i32>(5));
    println!("{}", identity::<String>(String::from("Random text")));

    make_tuple::<&str, i32>("hello", 3);
}

fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

fn identity<T>(value: T) -> T {
    value
}
