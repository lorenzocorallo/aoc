use std::collections::HashSet;

use anyhow::Result;

trait ToNumber {
    fn to_number(&self) -> i32;
}

impl ToNumber for char {
    fn to_number(&self) -> i32 {
        let ascii = *self as i32;
        let delta = if self.is_uppercase() { 27 - 65 } else { 1 - 97 }; 
        return ascii + delta;
    } 
}

fn main() -> Result<()> {
    let r = include_str!("./day3.prod");
    let rucksacks: Vec<&str> = r
        .lines()
        .collect();

    // nui = non-unique items
    let nui_sum: i32 = rucksacks.iter().map(|v| {
        let (v0, v1) = v.split_at(v.len() / 2);
        let h0: HashSet<char> = HashSet::from_iter(v0.chars());
        let h1: HashSet<char> = HashSet::from_iter(v1.chars());
        let intersection: Vec<char> = h0.intersection(&h1).cloned().collect();
        return intersection[0].to_number();
    }).sum();

    
    println!("#1: {:?}", nui_sum);
    
    let mut badges: Vec<char> = Vec::new();
    for n in 0..(rucksacks.len() / 3) {
        let group = &rucksacks[n*3..(n*3)+3];
        let mut hs: [HashSet<char>; 3] = [HashSet::new(), HashSet::new(), HashSet::new()];
        for (idx, rs) in group.iter().enumerate() {
            let chars: Vec<char> = rs.chars().collect();
            for char in chars.iter() {
                hs[idx].insert(*char);
            }
        }
        let badge = hs[0].intersection(&hs[1].intersection(&hs[2]).cloned().collect()).cloned().next().unwrap();
        badges.push(badge);
    } 
    println!("#2: {:?}", badges.iter().map(|c| c.to_number()).sum::<i32>());
    Ok(())
}
