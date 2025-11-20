use std::collections::HashMap;

#[derive(Debug)]
enum Value {
    Str(String),
    Int(i32),
}

fn create_person() -> HashMap<String, Value> {
    let mut person = HashMap::new();
    person.insert("name".into(), Value::Str("John".into()));
    person.insert("age".into(), Value::Int(40));
    person
}

fn main() {
    let person = create_person();
    // Access and print the values
    if let Some(name) = person.get("name") {
        match name {
            Value::Str(s) => println!("Name: {}", s),
            _ => (),
        }
    }

    if let Some(age) = person.get("age") {
        match age {
            Value::Int(a) => println!("Age: {}", a),
            _ => (),
        }
    }
}
