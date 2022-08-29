/*
    We will go ahead and implement the object-oriented state pattern in Rust. This pattern 
    requires a set of states a value can have internally. The value's behavior changes based
    on its state. The states are represented by a set of state objects in our
    first approach.
*/

pub struct Post { // has private fields
    state: Option<Box<dyn State>>, // allows states of any type
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
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        /*
            as_ref gives us a reference to value inside
            the Option, instead of taking ownership.

            At this point, when we call content on the &Box<dyn State>, 
            deref coercion will take effect on the & and the Box so the 
            content method will ultimately be called on the type that 
            implements the State trait.
        */
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
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        "" // add default implementation, so we only need to selectively implement this method
    }
}
/*
    Define different states using different structs;
    each struct implements the State trait in a different
    way. This lets different state objects emulate different
    behaviour like we want.
*/

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

/*
    Another approach to this problem involves encoding states as types.
    This means we have a Post whose new method returns a DraftPost. The 
    request_review method on this DraftPost consumes self (thereby discarding
    the DraftPost) and creates a new PendingReviewPost. This approach reduces
    the duplication of the content method across different types due to the 
    shared State trait; instead, these state types implement exactly what 
    they need. For example, the DraftPost has no content method defined, but
    a Post does.
*/

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
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
