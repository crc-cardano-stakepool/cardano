use std::path::Path;

pub fn file_exists(absolute_path: &str) -> bool {
    Path::new(absolute_path).exists()
}

#[cfg(test)]
mod test {
    // use crate::file_exists;
    #[tokio::test]
    #[ignore]
    async fn test_file_exists() {
        unimplemented!();
    }
}
