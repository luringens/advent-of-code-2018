use std::collections::HashMap;
const DATA: &'static str = include_str!("../input");

fn main() {
    task_1();
    task_2();
}

fn task_1() {
    let mut two = 0;
    let mut three = 0;
    for line in DATA.lines() {
        let mut map = HashMap::new();
        for c in line.chars() {
            match map.get_mut(&c) {
                Some(num) => *num += 1,
                None => { map.insert(c, 1); },
            }
        }
        if map.iter().find(|(_, n)| **n == 2).is_some() {
            two += 1;
        }
        if map.iter().find(|(_, n)| **n == 3).is_some() {
            three += 1;
        }
    }

    println!("Task 1: {}", two * three);
}

fn task_2() {
    for (i, line_1) in DATA.lines().enumerate() {
        for line_2 in DATA.lines().skip(i + 1) {
            if compare(line_1, line_2) {
                println!("Task 2: {}", same_chars(line_1, line_2));
                return;
            }
        }
    }
    println!("Task 2: Failed");
}

fn compare(fst: &str, snd: &str) -> bool {
    fst.chars()
       .zip(snd.chars())
       .filter(|(a, b)| a == b)
       .count() == fst.len() - 1
}

fn same_chars(fst: &str, snd: &str) -> String {
    fst.chars()
       .zip(snd.chars())
       .filter(|(a, b)| a == b)
       .map(|(a, _)| a)
       .collect()
       //.fold(String::new(), |acc, (a, _)| { acc.push(a); acc })
}
