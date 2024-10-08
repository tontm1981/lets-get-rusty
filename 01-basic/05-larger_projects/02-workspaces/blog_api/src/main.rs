use blog_shared::Post;

fn main() {
    let post = Post::new("Dummy Title".to_owned(), "dummy content Lorem ipsum".to_owned());
    println!("{post:?}");
}
