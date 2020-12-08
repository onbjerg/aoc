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
}

mod day2 {
  use std::ops::Range;

  #[derive(Debug, PartialEq)]
  pub struct Policy {
    letter: char,
    range: Range<usize>
  }

  impl Policy {
    pub fn new(letter: char, range: Range<usize>) -> Self {
      Self {
        letter,
        range
      }
    }

    pub fn validate(&self, password: String) -> bool {
      self.range.contains(&password.matches(self.letter).count())
    }
  }

  impl From<String> for Policy {
    fn from(line: String) -> Self {
      let (range, letter) = {
        let mut details = line.split_whitespace();
        let range = {
          let mut raw = details
            .next()
            .expect("No policy range provided")
            .split('-');

          Range {
            start: raw.next()
              .expect("No min in policy range")
              .parse::<usize>()
              .expect("Min in range is not a number"),
            end: raw.next()
              .expect("No max in range")
              .parse::<usize>()
              .expect("Max in range is not a number")
          }
        };
        let letter = details.next()
          .expect("No letter in policy")
          .chars()
          .nth(0)
          .expect("No letter in policy");

        (range, letter)
      };

      Policy::new(letter, range)
    }
  }

  pub fn part1(input: String) -> u64 {
    input
      .lines()
      .map(|line| {
        let mut raw = line.split(':');
        let (raw_policy, password) = (
          raw.next().expect("No policy provided"),
          raw.next().expect("No password provided")
        );
        let policy = Policy::from(String::from(raw_policy));

        policy.validate(password.into())
      })
      .filter(|ok| *ok)
      .count() as u64
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn policy_parse_ok() {
      assert_eq!(Policy::from(String::from("1-3 a")), Policy::new('a', Range { start: 1, end: 3 }));
      assert_eq!(Policy::from(String::from("5-8 b")), Policy::new('a', Range { start: 1, end: 3 }));
    }

    #[test]
    #[should_panic]
    fn policy_parse_no_letter() {
      Policy::from(String::from("1-3"));
    }

    #[test]
    #[should_panic]
    fn policy_parse_invalid_range() {
      Policy::from(String::from("a-c"));
      Policy::from(String::from("1-f"));
      Policy::from(String::from("foo-bar"));
    }

    #[test]
    fn part1_aoc_examples() {
      assert_eq!(part1("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc".into()), 2);
      assert_eq!(part1("1-3 a: abcde\n1-3 b: cdefg\n1-2 c: ccccccccc".into()), 1);
      assert_eq!(part1("1-3 a: b\n1-3 b: c\n1-3 c: d".into()), 0);
    }
  }
}

fn main() {
  println!("Day 1");
  println!("- Part 1: {:?}", day1::part1(include_str!("../input/day1.txt").into()));
  println!("- Part 2: {:?}", day1::part2(include_str!("../input/day1.txt").into()));

  println!("Day 2");
  println!("- Part 1: {:?}", day2::part1(include_str!("../input/day2.txt").into()));
}
