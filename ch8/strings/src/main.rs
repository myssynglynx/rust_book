fn main() {
    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    println!("s = {}", data);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
    println!("s1 = {}", s1);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    //let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s = {}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}

