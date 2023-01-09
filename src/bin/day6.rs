use std::collections::HashSet;

use anyhow::Result;

fn main() -> Result<()> {
    let s = include_str!("./day6.prod");
    let v = Vec::from(s);

    // #1 = marker made of 4 distinct chars
    for (idx, win) in v.windows(4).enumerate() {
        let hs: HashSet<u8> = win.to_vec().into_iter().collect();
        if hs.len() == 4 {
            println!("#1: {}", idx+4);
            break;
        }
    }

    // #2 = message made of 14 distinct chars
    for (idx, win) in v.windows(14).enumerate() {
        let hs: HashSet<u8> = win.to_vec().into_iter().collect();
        if hs.len() == 14 {
            println!("#2: {}", idx+14);
            break;
        }
    }

    Ok(())
}
