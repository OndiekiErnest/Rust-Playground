pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_name() {
        let rt = greeting("John");
        assert!(
            rt.contains("John"),
            "Greeting did not contain name, value was `{rt}`"
        );
    }
}
