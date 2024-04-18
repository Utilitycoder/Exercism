#[derive(Debug)]
pub struct HighScores{
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        // create a new HighScores struct with the given scores
        HighScores {
            scores: scores.to_vec(),
        }     
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores //This is a slice of the vector
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned() // Cloned the last element of the vector
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned() // iterated through the vector and found the highest value
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let new_scores = &self.scores; // Fetched and turned into a slice of u32
        let mut sorted_scores = new_scores.to_vec(); // Cloned the slice into a new vector
        sorted_scores.sort(); // Sorted the vector
        sorted_scores.reverse(); // Reversed the vector
        sorted_scores.truncate(3); // Truncated the vector to only 3 elements
        sorted_scores
    }
}
