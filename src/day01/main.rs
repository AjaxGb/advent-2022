use advent_2022::IteratorUtils;

fn main() {
    let max_elves = include_str!("input.txt")
        .split("\n\n")
        .map(|elf| elf.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
        .max_n::<3>();

    println!("Max 3 elves: {:?}", max_elves);
    
    let max_elves_total: u32 = max_elves.into_iter().sum();
    
    println!("Max 3 elves total: {}", max_elves_total);
}
