use std::path::Path;

pub fn is_dir(absolute_path: &str) -> bool {
    Path::new(absolute_path).is_dir()
}

#[cfg(test)]
mod test {
    // use crate::is_dir;
    #[tokio::test]
    #[ignore]
    async fn test_is_dir() {
        unimplemented!();
    }
}
