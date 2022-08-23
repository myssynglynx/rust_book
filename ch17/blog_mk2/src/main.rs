use blog_mk2::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    let post = post.approve();

    let post = post.publish();

    assert_eq!("I ate a salad for lunch today", post.unwrap().content());
}
