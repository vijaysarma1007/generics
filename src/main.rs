fn main() {
    println!("{}", identity::<i32>(5));
    println!("{}", identity::<String>(String::from("Random text")));

    make_tuple::<&str, i32>("hello", 3);
}

fn make_tuple<T, U>(first:T, second:U) -> (T, U) {
    (first, second)
}

fn identity<T>(value: T) -> T {
    value
}
 