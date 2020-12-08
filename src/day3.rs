pub fn tree_finder(map: &String, x_step: usize, y_step: usize) -> u64 {
    map.lines().enumerate().fold(0, |trees, (i, line)| {
        let position = (line.len() * y_step + x_step * i) % line.len();

        if line.chars().nth(position).unwrap() == '#' {
            trees + 1
        } else {
            trees
        }
    })
}

pub fn part1(input: String) -> u64 {
    tree_finder(&input, 3, 1)
}

pub fn part2(input: String) -> u64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_aoc_examples() {
        assert_eq!(
            part1(
                "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
                    .into()
            ),
            7
        );
    }

    #[test]
    fn part2_aoc_examples() {
        assert_eq!(
            part2(
                "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
                    .into()
            ),
            0
        );
    }
}
