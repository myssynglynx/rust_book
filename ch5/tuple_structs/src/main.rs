fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let mut black = Color(0, 0, 0);
    let mut origin = Point(0, 0, 0);

    println!("black.1: {}", black.1);
    println!("origin.1: {}", origin.1);

    black.1 = 2;
    origin.1 = black.1;

    println!("black.1: {}", black.1);
    println!("origin.1: {}", origin.1);
}
