pub fn get_the_answer() -> i32 {
  42
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(get_the_answer(), 42);
    }
}
