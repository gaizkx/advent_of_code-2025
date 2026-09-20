use std::fs;

fn find_maximum_joltage(batteries: &str) -> u32
{
    let mut batteries = batteries.chars()
        .map(|c| c.to_digit(10).unwrap());

    let mut a = batteries.next().unwrap();
    let mut b = 0;

    while let Some(cand) = batteries.next() {
        if b > a {
            a = b;
            b = cand;
        }

        if cand > b {
            b = cand;
        }
    }

    a * 10 + b
}

fn main() {
    let input = fs::read_to_string("./inputs/day_3.txt").unwrap();

    let output: u32 = input.lines()
        .map(|l| find_maximum_joltage(l))
        .sum();

    println!("Día 3, parte 1: {output}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_maximum_joltage_test() {
        assert_eq!(find_maximum_joltage("987654321111111"), 98_u32);
        assert_eq!(find_maximum_joltage("811111111111119"), 89_u32);
        assert_eq!(find_maximum_joltage("234234234234278"), 78_u32);
        assert_eq!(find_maximum_joltage("818181911112111"), 92_u32);
    }
}
