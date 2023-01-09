use std::ops::RangeInclusive;

use anyhow::Result;

fn get_range(s: &str) -> RangeInclusive<i32> {
    let c = s.split_once("-").unwrap();
    let c: (i32, i32) = (c.0.parse().unwrap(), c.1.parse().unwrap());
    return c.0..=c.1;
}

trait Compare<T> {
    fn is_subrange(&self, r: &T) -> bool;
    fn has_overlaps(&self, r: &T) -> bool; 
}

impl Compare<RangeInclusive<i32>> for RangeInclusive<i32> {
    fn is_subrange(&self, r: &RangeInclusive<i32>) -> bool {
        r.start() <= self.start() && self.end() <= r.end()
    }

    fn has_overlaps(&self, r: &RangeInclusive<i32>) -> bool {
        for n in self.clone().into_iter() {
            if r.contains(&n) {
                return true;
            }
        }
        return false;
    }
}

fn main() -> Result<()> {
    let r: Vec<&str> = include_str!("./day4.prod").lines().collect(); 

    let pairs: Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> = r.iter().map(|str| {
        let s: (&str, &str) = str.split_once(",").unwrap();
        let r1 = get_range(s.0);
        let r2 = get_range(s.1);
        (r1, r2)
    }).collect();

    let mut count_1: isize = 0;
    let mut count_2: isize = 0;
    for pair in pairs.iter() {
        let ok1 = pair.0.is_subrange(&pair.1);
        let ok2 = pair.1.is_subrange(&pair.0);
        if ok1 || ok2 {
            count_1 += 1;
        } 

        let has_overlaps = pair.0.has_overlaps(&pair.1); 
        if has_overlaps {
            count_2 += 1;
        }
    }

    println!("#1: {:?}", count_1);
    println!("#2: {:?}", count_2);
    
    Ok(())
}
