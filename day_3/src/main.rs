use std::str::FromStr;

const DATA: &'static str = include_str!("../input");

fn main() {
    let claims: Vec<Claim> = DATA.lines().map(|s| s.parse().unwrap()).collect();
    let mut map = std::collections::HashMap::new();

    for claim in claims.iter() {
        for x in (claim.x)..(claim.x + claim.width) {
            for y in (claim.y)..(claim.y + claim.height) {
                let mut handled = false;
                if let Some(v) = map.get_mut(&(x, y)) {
                    *v += 1;
                    handled = true;
                }

                if !handled {
                    map.insert((x, y), 1);
                }
            }
        }
    }

    let overlapped = map.values().filter(|v| **v > 1).count();
    println!("Task 1: {}", overlapped);

    for claim in claims.iter() {
        let mut overlapping = false;

        'c: for x in (claim.x)..(claim.x + claim.width) {
            for y in (claim.y)..(claim.y + claim.height) {
                let v = map.get(&(x, y)).unwrap();
                if *v > 1 {
                    overlapping = true;
                    break 'c;
                }
            }
        }

        if !overlapping {
            println!("Task 2: {}", claim.id);
            return;
        }
    }

    println!("Task 2: Failed");
}

#[derive(Debug, Clone, Copy)]
struct Claim {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl FromStr for Claim {
    type Err = ();

    /// Parse strings like `#1 @ 100,366: 24x27`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let id: i32 = parts.next().unwrap().get(1..).unwrap().parse().unwrap();
        parts.next();
        let xy: Vec<i32> = parts
            .next()
            .unwrap()
            .trim_matches(':')
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let (x, y) = (xy[0], xy[1]);

        let wh: Vec<i32> = parts
            .next()
            .unwrap()
            .split('x')
            .map(|s| s.parse().unwrap())
            .collect();
        let (width, height) = (wh[0], wh[1]);

        Ok(Self {
            id,
            x,
            y,
            width,
            height,
        })
    }
}
