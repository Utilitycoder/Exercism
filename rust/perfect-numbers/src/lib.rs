use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    } else {
        // Match the numbers between 1 and num, filter out the ones that are can divide num
        // and sum them up. Then compare the sum to num and return the appropriate Classification
        match (1..num).filter(|x| num % x == 0).sum::<u64>().cmp(&num) {
            Ordering::Less => Some(Classification::Deficient),
            Ordering::Equal => Some(Classification::Perfect),
            Ordering::Greater => Some(Classification::Abundant),
        }
    }
}
