mod day1;
mod day2;

fn main() {
  println!("Day 1");
  println!("- Part 1: {:?}", day1::part1(include_str!("../input/day1.txt").into()));
  println!("- Part 2: {:?}", day1::part2(include_str!("../input/day1.txt").into()));

  println!("Day 2");
  println!("- Part 1: {:?}", day2::part1(include_str!("../input/day2.txt").into()));
}
