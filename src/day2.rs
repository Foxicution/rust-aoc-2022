//TODO: Add tests, improve these solutions

/// A very simple day 2 rust solution.
/// This solution takes the input file and splits it into lines.
/// Then it splits each line into a tuple of (str, str).
/// Finally it uses a match statement to determine the result of a match
/// and returns the sum of all the results
/// ```
/// use advent_of_code_2022_rust::day2::day2;
/// let input = "A Y\n\
/// B X\n\
/// C Z";
/// let result = day2(input);
/// assert_eq!(result, 15);
/// ```
pub fn day2(input: &str) -> i32 {
    input
        .split("\n")
        .map(|line| match line.split_once(" ").unwrap_or(("", "")) {
            ("A", "X") => 1 + 3,
            ("B", "X") => 1,
            ("C", "X") => 1 + 6,
            ("A", "Y") => 2 + 6,
            ("B", "Y") => 2 + 3,
            ("C", "Y") => 2,
            ("A", "Z") => 3,
            ("B", "Z") => 3 + 6,
            ("C", "Z") => 3 + 3,
            _ => 0,
        })
        .sum()
}

pub fn day2_part2(input: &str) -> i32 {
    input
        .split("\n")
        .map(|line| match line.split_once(" ").unwrap_or(("", "")) {
            ("A", "X") => 3,
            ("B", "X") => 1,
            ("C", "X") => 2,
            ("A", "Y") => 1 + 3,
            ("B", "Y") => 2 + 3,
            ("C", "Y") => 3 + 3,
            ("A", "Z") => 2 + 6,
            ("B", "Z") => 3 + 6,
            ("C", "Z") => 1 + 6,
            _ => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::day2;

    #[test]
    fn test_day2() {
        let input = "A Y\n\
            B X\n\
            C Z";

        assert_eq!(day2(input), 15);
    }
}
