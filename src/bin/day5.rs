use anyhow::Result;

fn main() -> Result<()> {
    let r: Vec<&str> = include_str!("./day5.prod").lines().collect();
    let blank_idx = r.iter().position(|x| x.len() == 0).unwrap();
    let schema = &r[0..blank_idx - 1].iter()
        .map(|l| {
            let mut s: Vec<char> = l.chars().collect();
            let mut rem_count = 0;
            for (idx, _) in l.chars().enumerate() {
                if idx >= 3 && (idx - 3) % 4 == 0 {
                    s.remove(idx - rem_count);
                    rem_count += 1;
                }
            }
            return s;
        })
        .map(|s| {
            let mut v: Vec<char> = Vec::new(); 
            for n in 0..(s.len() / 3) {
                let n = n * 3;
                let col = &s[n..n+3][1];
                v.push(*col);
            }
            return v;
        })
        .collect::<Vec<Vec<char>>>();
    let cols_num = &r[blank_idx-1]
        .split("")
        .filter(|c| !c.replace(" ", "").is_empty())
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let moves = &r[blank_idx+1..].iter()
        .map(|str| {
            str.split(" ")
                .filter(|c| c.parse::<usize>().is_ok())
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<usize>>() // [num, from_col, to_col]
        })
        .collect::<Vec<Vec<usize>>>();

    
    let cols = cols_num.iter().map(|n| {
        schema.iter().rev().map(|v| {
            return v[n-1];
        })
            .filter(|e| !e.is_whitespace())
            .collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();

    let mut cols_1 = cols.clone(); 
    let mut cols_2 = cols.clone();
    for m in moves {
        let num = m[0];
        let from_col_idx = m[1] - 1;
        let to_col_idx = m[2] - 1;

        // #1
        for _ in 0..num {
            let el = cols_1[from_col_idx].pop();
            if el.is_some() {
                cols_1[to_col_idx].push(el.unwrap());
            }
        }

        // #2
        let fc = &cols_2.to_vec()[from_col_idx];
        let fc_len = fc.len();
        let els = &fc[fc_len-num..fc_len];
        cols_2[to_col_idx].append(&mut els.to_vec());
        cols_2[from_col_idx] = fc[..fc_len-num].to_vec();
    }
    
    let last_seq_1 = cols_1.iter().map(|v| v.last().unwrap()).collect::<String>();
    let last_seq_2 = cols_2.iter().map(|v| v.last().unwrap()).collect::<String>();
    println!("#1: {}", last_seq_1);
    println!("#2: {}", last_seq_2);


    Ok(())
}
