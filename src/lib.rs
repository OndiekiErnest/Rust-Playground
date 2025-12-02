pub fn divide(num: u32, denom: u32) -> Result<u32, String> {
    if denom == 0 {
        return Err(String::from("Division by zero error"));
    }
    Ok(num / denom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide_by_zero() -> Result<(), String> {
        let rt = divide(2, 0)?;

        assert_eq!(rt, 2);
        Ok(())
    }
}
