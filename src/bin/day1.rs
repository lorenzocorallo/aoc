use anyhow::Result;

fn main() -> Result<()> {
    // let r = include_str!("./day1.test");
    let r = include_str!("./day1.prod");
    let v: Vec<&str> = r.split("\n\n").collect();
    let elves: Vec<i32> = v.iter().map(|elv| {
        elv
            .split("\n")
            .map(|s| s.parse::<i32>().unwrap_or(0))
            .sum::<i32>()
    }).collect();

    let max = elves.iter().max().unwrap();

    println!("{:?}", max);

    Ok(())
}
