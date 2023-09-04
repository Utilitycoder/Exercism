/// Function to reverse a string
/// It takes a string slice and returns a string
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect::<String>()
}
