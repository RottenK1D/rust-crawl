#![allow(dead_code)]
pub fn normalize_url(url: &str) -> &str {
    url
}

//run test on normalize_url

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_url() {
        assert_eq!(
            normalize_url("https://www.rust-lang.org/"),
            "https://www.rust-lang.org/"
        );
    }
}
