use anyhow::Result;

// code inspired by 
// https://github.com/ThePrimeagen/aoc/blob/2022/src/bin/day7.rs
fn main() -> Result<()> {
    let total_space = 70_000_000;
    let needed_space = 30_000_000;
    let mut used_space = 0;

    let r = std::fs::read_to_string("src/bin/day7.prod")?;
    let mut stack: Vec<(&str, usize)> = vec![
        ("/", 0),
    ];

    let mut dir_sizes: Vec<usize> = Vec::new();

    let mut total = 0;
    for str in r.lines().filter(|x| !x.is_empty()) {
        if str == "$ ls" || str == "$ cd /" { continue; }

        if str.starts_with("$ cd") {
            let path = &str[5..];
            if path == ".." {
                let (_, amount) = stack.pop().unwrap();
                // println!("item: {:?}", item.1);
                if amount <= 100_000 {
                    total += amount;
                }
                dir_sizes.push(amount);
                stack.last_mut().unwrap().1 += amount;
            } else {
                stack.push((path, 0));
            }
            continue;
        }
        let (size, _) = str.split_once(" ").unwrap();
        if let Ok(size) = size.parse::<usize>() {
            used_space += size;
            stack.last_mut().unwrap().1 += size;
        } else if size == "dir" {
            // ignore "dir"
        }
    }

    while stack.len() > 1 {
        let (_, amount) = stack.pop().unwrap();
        if amount <= 100_000 {
            total += amount;
        }
        dir_sizes.push(amount);
        stack.last_mut().unwrap().1 += amount;
    }

    println!("#1: {}", total);


    let space_to_free = needed_space - (total_space - used_space); 
    let mut dir_sizes: Vec<&usize> = dir_sizes.iter().filter(|x| **x > space_to_free).collect();
    dir_sizes.sort_by(|a, b| a.cmp(b));
    println!("#2 {}", dir_sizes[0]);

    Ok(())
}
