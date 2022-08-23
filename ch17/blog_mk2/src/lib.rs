use std::error::Error;

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new()
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approvals: 0,
            approvals_needed: 2,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
    approvals: u32,
    approvals_needed: u32,
}

impl PendingReviewPost {
    pub fn approve(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approvals: self.approvals + 1,
            approvals_needed: self.approvals_needed,
        }
    }

    pub fn publish(self) -> Result<Post, ()> {
        if &self.approvals >= &self.approvals_needed {
            return Ok(Post {
                content: self.content,
            });
        }
        panic!("Not enough approvals.");
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_review_double_approve_works() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");

        let post = post.request_review();

        let post = post.approve();

        let post = post.approve();

        let post = post.publish();

        assert_eq!("I ate a salad for lunch today", post.unwrap().content());
    }

    #[test]
    #[should_panic(expected = "Not enough approvals.")]
    fn post_review_single_approve_fails() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");

        let post = post.request_review();

        let post = post.approve();

        let post = post.publish();
    }
}
