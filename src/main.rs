mod day1 {
  use std::collections::HashSet;

  pub fn part1(input: String) -> Option<u64> {
    let numbers: Vec<u64> = input
      .lines()
      .map(|s| s.parse().expect("Parse error"))
      .collect();
    let mut complements = HashSet::new();

    for num in numbers {
      let complement = 2020 - num;
      if complements.contains(&complement) {
        return Some(complement * num);
      } else {
        complements.insert(num);
      }
    }

    None
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn small() {
      assert_eq!(part1("2010\n10".into()), Some(20100));
    }

    #[test]
    fn bigish() {
      assert_eq!(part1("150\n1500\n1111\n2010\n123\n10".into()), Some(20100));
    }

    #[test]
    fn no_results() {
      assert_eq!(part1("1\n2\n3\n4\n5\n6".into()), None);
    }
  }
}

fn main() {
  println!("Day 1");
  println!("- Part 1: {:?}", day1::part1(include_str!("../input/day1_part1.txt").into()));
}
