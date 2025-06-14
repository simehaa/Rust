fn main() {
    let mut post = Post::new();
    post.add_text( "Hello, World!");
    let post = post.request_review();
    let post = post.reject();
    let post = post.request_review();
    let post = post.approve();
    let post = post.approve();
    println!("Post approved and published:\n{}", post.content());
    
    /*
    Conclusion:
    Compared to the blog post by state patter, this is much less code, and the comiler is
    much stricter to what I can do here in the main function (as an external user of the
    Post structure). I can out-of-the-box not do anything illegal to the post object, like
    adding content to anything other than a DraftPost, or viewing the content of anything
    other than a fully published Post.
     */
}

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
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
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> SecondReviewPost {
        SecondReviewPost {
            content: self.content,
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

pub struct SecondReviewPost {
    content: String,
}

impl SecondReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
