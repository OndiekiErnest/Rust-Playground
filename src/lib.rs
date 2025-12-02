pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_name() {
        let name = "John";
        let rt = greeting(name);
        assert!(
            rt.contains(name),
            "Greeting did not contain the name `{name}`, return value was `{rt}`"
        );
    }
}
