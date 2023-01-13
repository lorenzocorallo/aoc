use anyhow::Result;

fn main() -> Result<()> {
    let rows: Vec<Vec<u32>> = include_str!("./day8.prod")
        .lines()
        .into_iter()
        .map(|str| str.chars().map(|char| char.to_digit(10).expect("cool input bro")).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();


    let mut highest_view_score: usize = 0;
    let mut count = 2 * rows.len() + 2 * (rows[0].len() - 2);
    for (x, row)  in rows.iter().enumerate() {
        if x == 0 || x == rows.len() - 1 { continue; }
        for (y, h1) in row.iter().enumerate() {
            if y == 0 || y == row.len() - 1 { continue; }
            
            let mut col: Vec<u32> = Vec::new();
            for r in rows.iter() {
                col.push(r[y])
            }

            let mut intvl_1 = row[..y].to_vec();
            intvl_1.reverse();
            let intvl_2 = row[y+1..].to_vec();
            let mut intvl_3 = col[..x].to_vec();
            intvl_3.reverse();
            let intvl_4 = col[x+1..].to_vec();
        
            let mut view_score: Vec<usize> = Vec::new();
            let mut count_updated = false;
            for interval in vec![intvl_1, intvl_2, intvl_3, intvl_4].into_iter() {
                let mut ok = true;
                let mut vs = 0;
                for h2 in interval.iter() {
                    if !ok { break; }
                    vs += 1;
                    if h1 <= h2 {
                        ok = false;
                    }
                }
                view_score.push(vs);
                if ok && !count_updated {
                    count += 1;
                    count_updated = true;
                }
            }
            let view_score = view_score.into_iter().fold(1, |acc, e| acc * e);
            if view_score > highest_view_score {
                highest_view_score = view_score
            }

        }
    }
    println!("#1: {}", count);
    println!("#2: {}", highest_view_score);
    
    Ok(())
}
