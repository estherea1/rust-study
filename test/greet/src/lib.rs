pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("esther");
        assert!(
            result.contains("esther"),
            "不包含，result值是{}",result
        );
    }
}