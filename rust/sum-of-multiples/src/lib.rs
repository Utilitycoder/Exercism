pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // todo!("Sum the multiples of all of {factors:?} which are less than {limit}")
    let mut multiples = Vec::new();
    // loop all numbers from 1 to limit
    for i in 1..limit {
        // loop all factors
        for j in factors {
            // if the factor is not 0 and the number is divisible by the factor
            if *j != 0 && i % j == 0 {
                // add the number to the multiples vector
                multiples.push(i);
                break;
            }
        }
    }
    // return the sum of the multiples vector
    multiples.iter().sum()
}
