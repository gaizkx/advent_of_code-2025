use std::fs;

const PART_BATTERY_COUNT: usize = 12;

fn find_maximum_joltage_part1(batteries: &str) -> usize
{
    let mut batteries = batteries.chars()
        .map(|c| c.to_digit(10).unwrap() as usize);

    let mut a = batteries.next().unwrap();
    let mut b = 0;

    while let Some(cand) = batteries.next() {
        if b > a {
            a = b;
            b = cand;
            continue;
        }

        if cand > b {
            b = cand;
        }
    }

    a * 10_usize + b
}

fn find_maximum_joltage_part2(batteries: &str) -> usize
{
    let mut batteries = batteries.chars()
        .map(|c| c.to_digit(10).unwrap() as usize);

    // Crear el array de dígitos e inicializarlos
    let mut digits = [0; PART_BATTERY_COUNT];
    for i in 0..PART_BATTERY_COUNT {
        digits[i] = batteries.next().unwrap();
    }

    while let Some(cand) = batteries.next() {
        for i in 0..(PART_BATTERY_COUNT - 1)  {
            if digits[i] < digits[i+1] {
                for j in i..(PART_BATTERY_COUNT - 1) {
                    digits[j] = digits[j + 1];
                }
                digits[PART_BATTERY_COUNT - 1] = 0;

                break;
            }
        }

        if cand > digits[PART_BATTERY_COUNT - 1] {
            digits[PART_BATTERY_COUNT - 1] = cand;
        }
    }

    let mut result = 0;
    for i in 0..PART_BATTERY_COUNT {
        result += digits[i] * 10_usize.pow(((PART_BATTERY_COUNT - i) as u32)  - 1);
    }

    result
}

fn main() {
    let input = fs::read_to_string("./inputs/day_3.txt").unwrap();

    let part1_output: usize = input.lines()
        .map(|l| find_maximum_joltage_part1(l))
        .sum();

    println!("Día 3, parte 1: {part1_output}");

    let part2_output: usize = input.lines()
        .map(|l| find_maximum_joltage_part2(l))
        .sum();

    println!("Día 3, parte 2: {part2_output}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_maximum_joltage_par1_test() {
        assert_eq!(find_maximum_joltage_part1("987654321111111"), 98_usize);
        assert_eq!(find_maximum_joltage_part1("811111111111119"), 89_usize);
        assert_eq!(find_maximum_joltage_part1("234234234234278"), 78_usize);
        assert_eq!(find_maximum_joltage_part1("818181911112111"), 92_usize);
    }

    #[test]
    fn find_maximum_joltage_par2_test() {
        assert_eq!(find_maximum_joltage_part2("987654321111111"), 987654321111_usize);
        assert_eq!(find_maximum_joltage_part2("811111111111119"), 811111111119_usize);
        assert_eq!(find_maximum_joltage_part2("234234234234278"), 434234234278_usize);
        assert_eq!(find_maximum_joltage_part2("818181911112111"), 888911112111_usize);
    }
}
