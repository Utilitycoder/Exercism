/// Annotates a minefield with the number of adjacent mines for each cell.
///
/// Given a minefield represented as a slice of strings, where each string represents a row of cells,
/// this function returns a vector of strings representing the annotated minefield.
/// The annotation consists of replacing each cell containing a mine ('*') with '*', and replacing each
/// empty cell with the count of adjacent mines.
///
/// # Arguments
///
/// * `minefield` - A slice of strings representing the minefield.

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = Vec::new();
    for (i, row) in minefield.iter().enumerate() {
        let mut row_result = String::new();
        for (j, cell) in row.chars().enumerate() {
            if cell == '*' {
                row_result.push('*');
            } else {
                let mut count = 0;
                for x in i.saturating_sub(1)..=i + 1 {
                    for y in j.saturating_sub(1)..=j + 1 {
                        if x < minefield.len() && y < row.len() && minefield[x].chars().nth(y) == Some('*') {
                            count += 1;
                        }
                    }
                }
                if count > 0 {
                    row_result.push_str(&count.to_string());
                } else {
                    row_result.push(' ');
                }
            }
        }
        result.push(row_result);
    }
    result
}
