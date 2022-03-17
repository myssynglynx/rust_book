fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    //change(&mut s);

    println!("r1 = {}\nr2 = {}", r1, r2);

    let r3 = &mut s;

    change(r3);

    println!("r3 = {}", r3);
}

fn change(s: &mut String) {
    s.push_str(", world");
}

