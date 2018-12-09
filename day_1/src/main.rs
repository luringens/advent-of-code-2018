use std::str::FromStr;
const DATA: &'static str = include_str!("../input");

fn main() {
    let result = DATA.lines().map(|s| line_to_num(s)).sum::<i32>();
    println!("Task 1: {}", result);

    let mut map = std::collections::HashMap::new();
    let mut iter = DATA.lines();
    let mut sum = 0;
    map.insert(0, 1);

    'outer: loop {
        let next = iter.next().map(|s| line_to_num(s));
        match next {
            Some(n) => {
                sum += n;
                if map.get(&sum).is_some() {
                    break 'outer;
                } else {
                    map.insert(sum, 1);
                }
            }
            None => iter = DATA.lines(),
        }
    }

    println!("Task 2: {}", sum);
}

fn line_to_num(input: &str) -> i32 {
    match input.split_at(1) {
        ("+", num) => i32::from_str(num).expect("Failed to parse"),
        ("-", num) => -i32::from_str(num).expect("Failed to parse"),
        _ => panic!("Unexpected input."),
    }
}
