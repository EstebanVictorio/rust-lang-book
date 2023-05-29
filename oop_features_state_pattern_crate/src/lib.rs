#[cfg(test)]
mod tests {
    use super::blog::post::Post;
    #[test]
    fn it_creates_post() {
        let post = Post::new();
        assert_eq!(2 + 2, 4);
    }
}

pub mod blog;
