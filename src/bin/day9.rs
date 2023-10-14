use std::collections::HashSet;

use anyhow::Result;

#[derive(Debug)]
struct Motion {
    length: i8,
    direction: (i8, i8),
}

fn main() -> Result<()> {
    let rows: Vec<&str> = include_str!("./day9.prod").lines().collect();

    let motions: Vec<Motion> = rows
        .into_iter()
        .map(|x| {
            let (dir, len) = x.split_once(" ").unwrap();
            let direction = match dir {
                "U" => (0, 1),
                "D" => (0, -1),
                "R" => (1, 0),
                "L" => (-1, 0),
                _ => unreachable!(),
            };

            return Motion {
                direction,
                length: len.parse().expect("Cool input bro!"),
            };
        })
        .collect();

    let mut head: (i8, i8) = (0, 0);
    let mut tail: (i8, i8) = (0, 0);

    let mut record: HashSet<(i8, i8)> = HashSet::new();

    for motion in motions.iter() {
        for _ in 0..motion.length {
            head.0 += motion.direction.0;
            head.1 += motion.direction.1;

            let diff = ((head.0 - tail.0), (head.1 - tail.1));
            match diff {
                (-1..=1, -1..=1) => (),
                (0, -2) => tail.1 -= 1,
                (0, 2) => tail.1 += 1,
                (-2, 0) => tail.0 -= 1,
                (2, 0) => tail.0 += 1,

                (1, 2) | (2, 1) => {
                    tail.0 += 1;
                    tail.1 += 1;
                }

                (1, -2) | (2, -1) => {
                    tail.0 += 1;
                    tail.1 -= 1;
                }

                (-1, 2) | (-2, 1) => {
                    tail.0 -= 1;
                    tail.1 += 1;
                }

                (-1, -2) | (-2, -1) => {
                    tail.0 -= 1;
                    tail.1 -= 1;
                }

                _ => unimplemented!("i'm strange"),
            }

            record.insert(tail);
        }
    }

    println!("{:?}", record.len());

    Ok(())
}
