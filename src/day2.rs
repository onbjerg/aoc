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