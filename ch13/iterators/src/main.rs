fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
    println!("v1 = {:?}", v1);

    let v1_iter2 = v1.iter();
    let total: i32 = v1_iter2.sum();

    println!("v1 = {:?}", v1);
    println!("total = {}", total);
}
