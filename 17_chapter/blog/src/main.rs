use blog::OopPost;
use blog::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today!");

    let post = post.request_review();

    let mut post = post.reject();

    post.add_text("\nAnd some nuts!");
    let post = post.request_review();

    let post = post.approve();

    let post = post.approve();

    assert_eq!(
        "I ate a salad for lunch today!\nAnd some nuts!",
        post.content()
    );

    // below is the OOP-like implementation

    let mut post = OopPost::new();
    post.add_text("I ate a salad for lunch today!");

    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());
    post.add_text("\nCurrently under review");

    post.approve();
    assert_eq!("", post.content());
    post.add_text("\nCurrently undergoing approval");

    post.approve();
    post.add_text("\nPost is approved");
    assert_eq!("I ate a salad for lunch today!", post.content());
}
