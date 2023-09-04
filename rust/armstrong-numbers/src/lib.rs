pub fn is_armstrong_number(num: u32) -> bool {
    let received_num = num as u64;
    // Convert to string to get length
    let num_str = received_num.to_string();
    // Convert back to u32 to get length
    let num_len = num_str.len() as u32;
    let sum: u64 = num_str
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(num_len) as u64)
        .sum();
    sum == received_num
}
