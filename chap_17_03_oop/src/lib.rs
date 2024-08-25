pub mod betterblog {
    pub struct Post {
        content: String,
    }

    pub struct PendingReviewPost {
        content: String,
    }

    pub struct DraftPost {
        content: String,
    }

    impl DraftPost {
        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }

    impl PendingReviewPost {
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }
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
}

pub mod blog {
    const DEFAULT_REQUIRED_APPROVALS: u8 = 2;
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Default for Post {
        fn default() -> Self {
            Self {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }
    }

    impl Post {
        pub fn new() -> Self {
            Post {
                ..Default::default()
            }
        }
        pub fn add_text(&mut self, text: &str) {
            self.content
                .push_str(self.state.as_ref().unwrap().add_text(text));
        }

        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }

        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }

        pub fn reject(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.reject())
            }
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
    }

    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn reject(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn add_text<'a>(&self, _text: &'a str) -> &'a str {
            ""
        }
        fn content<'a>(&self, _post: &'a Post) -> &'a str {
            ""
        }
    }

    struct Draft {}
    struct PendingReview {
        approvals: u8,
        approvals_required: u8,
    }
    struct Published {}

    impl State for Draft {
        fn add_text<'a>(&self, text: &'a str) -> &'a str {
            text
        }
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            PendingReview::new()
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }
    impl PendingReview {
        fn new() -> Box<Self> {
            Box::new(PendingReview {
                approvals: 0,
                approvals_required: DEFAULT_REQUIRED_APPROVALS,
            })
        }
    }
    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft {})
        }
        fn approve(mut self: Box<Self>) -> Box<dyn State> {
            self.approvals += 1;
            if self.approvals >= self.approvals_required {
                return Box::new(Published {});
            }
            self
        }
    }
    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }
}

#[cfg(test)]
mod blog_tests {
    use super::blog::Post;

    #[test]
    fn create_and_publish() {
        let mut post = Post::new();

        post.add_text("I have a big honkin dog...");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I have a big honkin dog...", post.content());
    }
    #[test]
    fn create_and_edit_in_review() {
        let mut post = Post::new();

        post.add_text("I have a big honkin dog...");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.add_text("I have a HUGEEEEE honkin dog...");
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I have a big honkin dog...", post.content());
    }
    #[test]
    fn create_and_reject() {
        let mut post = Post::new();

        post.add_text("I have a BIG OL honkin dog...");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.reject();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve(); // Two approvals required.
        assert_eq!("", post.content());
        post.approve();
        assert_eq!("I have a BIG OL honkin dog...", post.content());
    }
}
