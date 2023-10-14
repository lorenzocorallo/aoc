use std::collections::HashSet;

use anyhow::Result;

#[derive(Debug)]
enum Direction {
    U,
    R,
    D,
    L,
}

#[derive(Debug)]
struct Motion {
    length: i8,
    direction: Direction,
}

fn main() -> Result<()> {
    let rows: Vec<&str> = include_str!("./day9.prod").lines().collect();

    let motions: Vec<Motion> = rows
        .into_iter()
        .map(|x| {
            let split: Vec<&str> = x.split(" ").collect();
            let direction = match split[0] {
                "U" => Direction::U,
                "D" => Direction::D,
                "L" => Direction::L,
                "R" => Direction::R,
                _ => unreachable!(),
            };

            return Motion {
                direction,
                length: split[1].parse().expect("Cool input bro!"),
            };
        })
        .collect();

    let mut head: (i8, i8) = (0, 0);
    let mut tail: (i8, i8) = (0, 0);

    let mut record: HashSet<(i8, i8)> = HashSet::new();

    for motion in motions.iter() {
        let delta: (i8, i8) = match motion.direction {
            Direction::U => (0, 1),
            Direction::D => (0, -1),
            Direction::R => (1, 0),
            Direction::L => (-1, 0),
        };

        for _ in 0..motion.length {
            head.0 += delta.0;
            head.1 += delta.1;

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
