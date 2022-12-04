trait Animal {
    fn baby_name() -> String;
}

struct Dog;
struct Cat;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Cat {
    fn baby_name() -> String {
        String::from("Mittens")
    }
}

impl Animal for Cat {
    fn baby_name() -> String {
        String::from("kitten")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    println!("A baby cat is called a {}", <Cat as Animal>::baby_name());
    println!("A baby dog is named {}", Dog::baby_name());
    println!("A baby cat is named {}", Cat::baby_name());
}
