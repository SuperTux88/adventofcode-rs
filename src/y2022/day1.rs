pub fn main() {
    let input = std::fs::read_to_string("input/2022/day1.txt").unwrap();
    let elves = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|c| c.parse::<i32>().unwrap()));
    let mut elves_calories: Vec<i32> = elves.map(|elf| elf.sum()).collect();
    elves_calories.sort();

    println!("max calories: {}", elves_calories.last().unwrap());
    println!("top 3 calories: {}", elves_calories.iter().rev().take(3).sum::<i32>());
}
