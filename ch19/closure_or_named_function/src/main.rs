fn main() {
    let list_of_numbers_one = vec![1, 2, 3];
    let list_of_strings_closure: Vec<String> =
        list_of_numbers_one.iter().map(|i| i.to_string()).collect();

    let list_of_numbers_two = vec![1, 2, 3];
    let list_of_strings_closure: Vec<String> =
        list_of_numbers_two.iter().map(ToString::to_string).collect();
}
