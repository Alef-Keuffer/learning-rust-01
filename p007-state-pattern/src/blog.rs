pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.state
            .as_ref()
            .unwrap()
            .add_text(&mut self.content, &text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }

    #[cfg_attr(test, allow(dead_code))]
    #[cfg(test)]
    fn name(&self) -> StateName {
        self.state.as_ref().unwrap().name()
    }
}

#[cfg_attr(test, allow(dead_code))]
#[cfg(test)]
#[derive(Debug, PartialEq)]
enum StateName {
    Draft,
    PendingReview,
    Published,
}

trait State {
    // --snip--
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn add_text(&self, _content: &mut String, _text: &str) -> () {}
    fn add_text2(&self, _post: &mut Post, _text: &str) -> () {}

    #[cfg(test)]
    fn name(&self) -> StateName;
}

// --snip--

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approve_count: 0 })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text(&self, content: &mut String, text: &str) {
        content.push_str(text);
    }

    #[cfg(test)]
    fn name(&self) -> StateName {
        StateName::Draft
    }
}

struct PendingReview {
    approve_count: u8,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        if self.approve_count == 0 {
            Box::new(PendingReview { approve_count: 1 })
        } else {
            Box::new(Published {})
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    #[cfg(test)]
    fn name(&self) -> StateName {
        StateName::PendingReview
    }
}

struct Published {}

impl State for Published {
    // --snip--
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    #[cfg(test)]
    fn name(&self) -> StateName {
        StateName::Published
    }
}

// use core::fmt::Debug;
// impl Debug for dyn State {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "{}", self.s)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    /// A blog post starts as an empty draft
    #[test]
    fn post_starts_as_draft() {
        let post = Post::new();
        assert_eq!(post.state.unwrap().name(), StateName::Draft);
    }

    /// When the draft is done, a review of the post is requested.
    #[test]
    fn request_review_works() {
        let mut post = Post::new();
        post.request_review();
        assert_eq!(post.state.unwrap().name(), StateName::PendingReview);
    }

    /// Changes the post’s state from PendingReview back to Draft.
    #[test]
    fn reject_works() {
        let mut post = Post::new();
        post.request_review();
        post.reject();
        assert_eq!(post.state.as_ref().unwrap().name(), StateName::Draft);
    }

    /// When the post is approved, it gets published.
    /// Require two calls to approve before the state can be changed to
    /// Published.
    #[test]
    fn approve_works() {
        let mut post = Post::new();
        post.request_review();
        post.approve();
        assert_ne!(post.state.as_ref().unwrap().name(), StateName::Published);
        post.approve();
        assert_eq!(post.state.as_ref().unwrap().name(), StateName::Published);
    }

    /// if we try to approve a draft blog post before we’ve requested a
    /// review, the post should remain an unpublished draft.
    #[test]
    fn try_approve_draft() {
        let mut post = Post::new();
        post.approve();
        assert_eq!(post.state.as_ref().unwrap().name(), StateName::Draft);
    }
}
