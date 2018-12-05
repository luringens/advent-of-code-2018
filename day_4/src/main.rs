use chrono::prelude::*;
use std::collections::HashMap;

const DATA: &'static str = include_str!("../input");
const DATE_FORMAT: &'static str = "%Y-%m-%d %H:%M";

fn main() {
    let data = parse_data();
    let most_asleep = data.iter()
                          .max_by_key(|(_, v)| {
                              v.iter()
                               .map(|(d1, d2)| {
                                    d2.signed_duration_since(*d1)
                                      .num_minutes()
                               })
                               .sum::<i64>()
                          })
                          .unwrap();

    let mut mins = [0; 60];
    for (d1, d2) in most_asleep.1 {
        for min in (d1.minute())..(d2.minute()) {
            mins[min as usize] += 1;
        }
    }
    let max = mins.iter().enumerate().max_by_key(|(_, v)| *v).unwrap().0;

    println!("Task 1: {}", *most_asleep.0 as usize * max);

    let most_asleep_2 = data
        .iter()
        .map(|(k, v)| {
            let mut mins = [0; 60];
            for (d1, d2) in v {
                for min in (d1.minute())..(d2.minute()) {
                    mins[min as usize] += 1;
                }
            }
            let max = mins.iter().enumerate().max_by_key(|(_, v)| *v).unwrap();
            (k, max.0, *max.1)
        })
        .max_by_key(|(_, _, v)| *v)
        .unwrap();

    println!("Task 2: {}", *most_asleep_2.0 as usize * most_asleep_2.1);
}

fn parse_data() -> HashMap<i32, Vec<(NaiveDateTime, NaiveDateTime)>> {
    let mut data: Vec<&str> = DATA.lines().collect();
    data.sort_unstable();
    
    let mut map: HashMap<i32, Vec<(NaiveDateTime, NaiveDateTime)>> = HashMap::new();
    let mut current_guard_id = 0;
    let mut current_guard_asleep = NaiveDateTime::from_timestamp(0, 0);
    for line in DATA.lines() {
        
        let date = NaiveDateTime::parse_from_str(&line[1..17], DATE_FORMAT).unwrap();
        
        match line.split_whitespace().last().unwrap() {
            "shift" => current_guard_id = line.split_whitespace()
                                              .find(|s| s.starts_with('#'))
                                              .map(|s| s[1..].parse().unwrap())
                                              .unwrap(),
            "asleep" => current_guard_asleep = date,
            "up" => {
                if map.contains_key(&current_guard_id) {
                    map.get_mut(&current_guard_id).unwrap().push((current_guard_asleep.clone(), date));
                } else {
                    map.insert(current_guard_id, vec!((current_guard_asleep.clone(), date)));
                }
            }
            _ => panic!("Unknown line end."),
        }
    }

    map
}
