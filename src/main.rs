use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// conditionally implement a trait for any type that implements another trait
impl<T: Display> ToString for Pair<T> {
    fn to_string(&self) -> String {
        format!("Pair(x={}, y={})", self.x, self.y)
    }
}

fn main() {
    let pair = Pair::new(23, 89);

    pair.cmp_display();

    println!("x={}, y={}", pair.x, pair.y);
    println!("{}", pair.to_string());
}
