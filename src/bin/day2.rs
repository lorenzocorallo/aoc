use anyhow::Result;

fn main() -> Result<()> {
    // A, X = Rock (1 pt)
    // B, Y = Paper (2 pt)
    // C, Z = Scissors (3 pt)
    // X wins C, Y wins A, Z wins B (6 pt)
    // X loses B, Y loses C, Z loses A (0 pt)
    // X draws A, Y draws B, Z draws C (3 pt)
    
    // let r = include_str!("./day2.test"); 
    let r = include_str!("./day2.prod"); 
    let mut lines: Vec<&str> = r.split("\n").collect();
    lines.pop();
    let games: Vec<(char, char)> = lines.iter().map(|str| {
        let chars = str.chars().collect::<Vec<_>>();
        return (chars[0], chars[2]);
    }).collect();

    let sum: i32 = games.iter().map(|game| {
        let base: i32 = match game.1 {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => 0
        };
        let result: i32 = match game.1 {
            'X' => match game.0 {
                'C' => 6, // win
                'A' => 3, // draw
                _ => 0 // includes loses B
            },
            'Y' => match game.0 {
                'A' => 6, // win
                'B' => 3, // draw
                _ => 0 // includes loses B
            },
            'Z' => match game.0 {
                'B' => 6,
                'C' => 3,
                _ => 0 // includes loses B
            },
            _ => 0
        };
        return base + result;
    }).sum();

    println!("#1: {:?}", sum);

    // part 2
    // A, B, C = Same as before
    // X = lose 
    // Y = draw 
    // Z = win 

    let sum2: i32 = games.iter().map(|game| {
        match game.1 {
            'X' => { // lose
                match game.0 {
                    'A' => 3, // Z
                    'B' => 1, // X
                    'C' => 2, // Y
                    _ => 0
                }
            },
            'Y' => { // draw 
                match game.0 {
                    'A' => 3 + 1, // A
                    'B' => 3 + 2, // B 
                    'C' => 3 + 3, // C
                    _ => 0
                }
            },
            'Z' => { // win
                match game.0 {
                    'A' => 6 + 2, // B
                    'B' => 6 + 3, // C
                    'C' => 6 + 1, // A
                    _ => 0
                }
            },
            _ => 0
        }
    }).sum();
    println!("#2: {:?}", sum2);

    Ok(())
}
