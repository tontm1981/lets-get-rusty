use blog_shared::Post;

fn main() {
    let post = Post::new("Dummy post @web".to_owned(), "Dummy post content @ web package".to_owned());
    println!("{post:?}");
}
