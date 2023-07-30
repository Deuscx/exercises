pub fn reverse(input: &str) -> String {
    // unimplemented!("Write a function to reverse {input}");
    let mut result = String::new();
    for c in input.chars().rev() {
        result.push(c);
    }
    result
}
