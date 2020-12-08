mod day1;
mod day2;
mod day3;

fn main() {
    println!("Day 1");
    println!(
        "- Part 1: {:?}",
        day1::part1(include_str!("../input/day1.txt").into())
    );
    println!(
        "- Part 2: {:?}",
        day1::part2(include_str!("../input/day1.txt").into())
    );

    println!("Day 2");
    println!(
        "- Part 1: {:?}",
        day2::part1(include_str!("../input/day2.txt").into())
    );
    println!(
        "- Part 2: {:?}",
        day2::part2(include_str!("../input/day2.txt").into())
    );

    println!("Day 3");
    println!(
        "- Part 1: {:?}",
        day3::part1(include_str!("../input/day3.txt").into())
    );
    println!(
        "- Part 2: {:?}",
        day3::part2(include_str!("../input/day3.txt").into())
    );
}
