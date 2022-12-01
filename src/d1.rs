pub fn run(input: String) -> u32 {
    let mut top_three_elf = vec![0, 0, 0];
    let mut curr_elf = 0;
    for line in input.lines() {
        if line == "" {
            if curr_elf > top_three_elf[0] {
                top_three_elf[0] = curr_elf;
                top_three_elf.sort();
            }
            curr_elf = 0;
            continue;
        }
        curr_elf += line.parse::<u32>().unwrap();
    }
    if curr_elf > top_three_elf[0] {
        top_three_elf[0] = curr_elf;
        top_three_elf.sort();
    }
    top_three_elf.iter().sum()
}
