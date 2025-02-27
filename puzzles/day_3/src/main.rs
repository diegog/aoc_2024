use regex::Regex;

const INPUT: &str = include_str!("input.txt");

fn part_1(input: &str) -> i32 {
    let r = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    r.captures_iter(input)
        .map(|c| {
            let x = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let y = c.get(2).unwrap().as_str().parse::<i32>().unwrap();

            x*y
        }).sum()
}

fn part_2(input: &str) -> i32 {
    let r = Regex::new(r"(mul)\(([0-9]+),([0-9]+)\)|(do\(\))|(don't\(\))").unwrap();
    let mut mul_enabled = true;
    r.captures_iter(input)
        .map(|c| {
            if c.get(1).is_some() {
                let x = c.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let y = c.get(3).unwrap().as_str().parse::<i32>().unwrap();
                if mul_enabled { return x * y }
            }
            if c.get(4).is_some() {
                mul_enabled = true;
            }
            if c.get(5).is_some() {
                mul_enabled = false;
            }
            0
        }).sum()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_1_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const TEST_2_INPUT: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_1_INPUT), 161);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_2_INPUT), 48);
    }
}
