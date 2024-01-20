pub fn raindrops(n: u32) -> String {
    let mut result = String::new();
    
    // If n is a factor of 3, push Pling to the result
    if n % 3 == 0 {
        result.push_str("Pling");
    }

    // If n is a factor of 5, push Plang to the result
    if n % 5 == 0 {
        result.push_str("Plang");
    }

    // If n is a factor of 7, push Plong to the result
    if n % 7 == 0 {
        result.push_str("Plong");
    }
    // If there's no factors, return the number as a string
    if result.is_empty() {
        result.push_str(&n.to_string());
    }
    result
}
