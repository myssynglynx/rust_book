struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("first sentence = {}", i.part);
    let book = &novel;
    println!("novel = {}", book);
    println!("first sentence = {}", i.part);
    let part = i.announce_and_return_part("hello!");
    println!("part = {}", part);
}
