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

pub fn part2(input: String) -> Option<u64> {
  let numbers: Vec<u64> = input
  .lines()
  .map(|s| s.parse().expect("Parse error"))
  .collect();
  let mut complements = HashSet::new();

  for a in &numbers {
    for b in &numbers {
      if 2020 - a < *b {
        continue
      }

      let complement = 2020 - a - b;
      if complements.contains(&complement) {
        return Some(complement * a * b);
      } else {
        complements.insert(b);
      }
    }
  }

  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_small() {
    assert_eq!(part1("2010\n10".into()), Some(20100));
  }

  #[test]
  fn part1_bigish() {
    assert_eq!(part1("150\n1500\n1111\n2010\n123\n10".into()), Some(20100));
  }

  #[test]
  fn part1_no_results() {
    assert_eq!(part1("1\n2\n3\n4\n5\n6".into()), None);
  }
  
  #[test]
  fn part2_small() {
    assert_eq!(part2("979\n366\n675".into()), Some(241861950));
  }
  
  #[test]
  fn part2_bigish() {
    assert_eq!(part2("10\n979\n233\n366\n1234\n675\n8".into()), Some(241861950));
  }
  
  #[test]
  fn part2_no_results() {
    assert_eq!(part2("1\n2\n3\n4\n5\n6".into()), None);
  }
}