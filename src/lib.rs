pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Self {
        if value < 1 || value > 100 {
            panic!("The guess value is out of range (1 - 100)!");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "out of range")]
    fn guess_greater_than_100() {
        Guess::new(101);
    }
}
