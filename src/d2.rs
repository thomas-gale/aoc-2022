pub fn run(input: String) -> u32 {
    let mut score = 0;
    input.lines().for_each(|line| {
        let strat: Vec<&str> = line.split_whitespace().collect();
        match strat[0] {
            "A" => match strat[1] {
                "X" => score += 3,
                "Y" => score += 1 + 3,
                "Z" => score += 2 + 6,
                _ => (),
            },
            "B" => match strat[1] {
                "X" => score += 1,
                "Y" => score += 2 + 3,
                "Z" => score += 3 + 6,
                _ => (),
            },
            "C" => match strat[1] {
                "X" => score += 2,
                "Y" => score += 3 + 3,
                "Z" => score += 1 + 6,
                _ => (),
            },
            _ => (),
        }
    });
    score
}
