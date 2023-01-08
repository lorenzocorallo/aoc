use anyhow::Result;

fn main() -> Result<()> {
    // let r = include_str!("./day1.test");
    let r = include_str!("./day1.prod");
    let v: Vec<&str> = r.split("\n\n").collect();
    let mut elves: Vec<i32> = v.iter().map(|elv| {
        elv
            .split("\n")
            .map(|s| s.parse::<i32>().unwrap_or(0))
            .sum::<i32>()
    }).collect();

    elves.sort_by(|a,b| b.cmp(a));

    // part one
    println!("#1: {:?}", elves[0]);

    // part two
    let sum3: i32 = elves[..3].iter().sum::<i32>();
    println!("#2: {:?}", sum3);

    Ok(())
}
