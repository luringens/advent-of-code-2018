const DATA: &'static str = include_str!("../input");

fn main() {
    let collapsed = collapse(DATA.trim());
    println!("Task 1: {}", collapsed.len());

    let mut letters: Vec<char> = collapsed.chars().filter(|l| l.is_lowercase()).collect();
    letters.sort_unstable();
    letters.dedup();

    let mut shortest = collapsed.len();
    for letter_lower in letters {
        let letter_upper = letter_lower.to_uppercase().next().unwrap();
        let filtered: String = collapsed.chars().filter(|c| *c != letter_lower && *c != letter_upper).collect();
        let result = collapse(filtered.as_ref());
        shortest = shortest.min(result.len());
    }

    println!("Task 2: {}", shortest);
}

fn collapse(input: &str) -> String {
    let mut stack: Vec<char> = Vec::new();
    for next in input.chars() {
        if let Some(prev) = stack.pop() {
            let equal_char = prev.to_lowercase().to_string() == next.to_lowercase().to_string();
            let different_case = prev.is_lowercase() && next.is_uppercase()
                || next.is_lowercase() && prev.is_uppercase();
            if !(equal_char && different_case) {
                stack.push(prev);
                stack.push(next);
            }
        } else {
            stack.push(next);
        }
    }

    stack.into_iter().collect()
}