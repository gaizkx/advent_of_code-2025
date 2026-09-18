use std::vec;

use fancy_regex::Regex;

const EXP_PART1: &str =r"^(\d+)(\1)$";

trait DigitCount {
    /// Cuenta cuántos dígitos tiene el número en cuestión.
    /// No devuelve nunca 0
    fn digits(&self) -> u32;
}

impl DigitCount for usize  {
    fn digits(&self) -> u32 {
        if *self == 0 {
            1
        } else {
            self.ilog10() + 1
        }
    }
}

fn find_invalid_numbers_part_1((start, end): (usize, usize), regex: &Regex) -> Vec<usize> {
    // O(n)
    (start..=end)
        .filter(|n| regex.is_match(&n.to_string()).unwrap_or(false))
        .collect()
}

fn find_invalid_numbers_part_1_optimized((start, end): (usize, usize)) -> Vec<usize> {
    // Siempre tenemos que arrancar en positivo, aunque en el
    // ejercicio es irrelevante
    let start = if start == 0 { 1 } else { start };

    // El inicio no puede ser mayor que el final
    if start > end {
        return vec![];
    }

    let start_digit_count = start.digits();
    let end_digit_count = end.digits();

    if start_digit_count & 0b1 == 1 {
        if start_digit_count == end_digit_count {
            // Si ambas longitudes son impares y encima las mismas,
            // no habrá ningún número que cumpla los criterios entre ellos.
            return vec![];
        }

        // El inicio será el primer número siguiente de dígitos pares
        let start = 10_usize.pow(start_digit_count);
        return find_invalid_numbers_part_1_optimized((start, end));
    }

    if end_digit_count & 0b1 == 1 {
        // El final nuevo será el último número de digitos pares anterior
        let end = 10_usize.pow(end_digit_count - 1) - 1; // 100 -> 99
        return find_invalid_numbers_part_1_optimized((start, end));
    }

    let half_digit_count = start_digit_count >> 1; // start_digit_count / 2;
    let step = 10_usize.pow(half_digit_count);

    let mut numbers = vec![];
    let mut num = start;

    while num <= end {
        // Separamos el número en dos partes para comprobar el criterio: que ambas
        // partes sean iguales
        let first_half = num / step;
        let second_half = num - first_half * step;

        if second_half == first_half {
            numbers.push(num);
        } else if second_half < first_half {
            // Si la segunda mitad es menor, podemos saltar directamente al número
            // que buscamos, entonces a la segunda mitad le sumamos la diferencia con
            // la primera mitad.
            // No meto el número al array por si, por lo que sea, este "nuevo" número
            // es mayor que el final
            num += first_half - second_half;
            continue;
        }

        // Aumentamos lo suficiente hasta el próximo número que cumpla los criterios
        num += step - second_half + first_half;
    }

    numbers
}

/// Siguiendo un poco la idea de la parte 1 optimizada
fn find_invalid_numbers_part_2(start: usize, end: usize, part_size: u32) -> Vec<usize> {
    if start.digits() % part_size != 0 {
        let start = 10_usize.pow(start.digits());
        return find_invalid_numbers_part_2(start, end, part_size);
    }

    // Al menos se tiene que repetir dos veces el número, así que con
    // números cuyo dígito es el mismo que el part_size, falla
    if start.digits() == part_size {
        let start = 10_usize.pow(part_size);
        return find_invalid_numbers_part_2(start, end, part_size);
    }

    // Antes de quitar un dígito a la cota final, mejor comprobar
    // que la cota sea correcta.
    if start > end {
        return vec![];
    }

    if end.digits() % part_size != 0 {
        let end = 10_usize.pow(end.digits() - 2);
        return find_invalid_numbers_part_2(start, end, part_size);
    }

    let mut num = start;
    let mut candidates = vec![];

    while num <= end {
        let digit_count = num.digits();
        let step = 10_usize.pow(digit_count - part_size);

        let first_part = num / step;

        let candidate =
            first_part * step
            + (0..(digit_count/part_size - 1))
                .map(|p| first_part * 10_usize.pow(part_size * p))
                .sum::<usize>();

        if candidate >= start && candidate <= end {
            candidates.push(candidate);
        }

        num = (first_part + 1) * step;
    }

    candidates
}

fn parse_range(range: &str) -> (usize, usize) {
    let parsed: Vec<&str> = range.split('-').collect();

    if parsed.len() != 2 {
        panic!("Rango no válido: {range}");
    }

    let first: usize = parsed[0].parse().unwrap();
    let second: usize = parsed[1].parse().unwrap();

    (first, second)
}

fn run_day2_part_1(line: &str) -> usize {
    let regex = Regex::new(EXP_PART1).unwrap();

    line.split(',')
        .map(|r| parse_range(r))
        .map(|r| find_invalid_numbers_part_1(r, &regex))
        .flatten()
        .sum()
}

fn run_day2_part_1_optimized(line: &str) -> usize {
    line.split(',')
        .map(|r| parse_range(r))
        .map(|r| find_invalid_numbers_part_1_optimized(r))
        .flatten()
        .sum()
}

fn run_day2_part_2(line: &str) -> usize
{
     line.split(',')
        .map(|r| parse_range(r))
        .map(|(start, end)| {
            let digits = end.digits();

            let mut result = (1..digits)
                .filter(|s| digits % s == 0)
                .map(|s| find_invalid_numbers_part_2(start, end, s))
                .flatten()
                .collect::<Vec<usize>>();

            // Un número cuyos dígitos sean todos iguales, será válido
            // para todos los divisores de la cantidad de dígitos
            result.sort();
            result.dedup();

            result
        })
        .flatten()
        .sum()
}

fn main() {
    let my_exercise_input = "197-407,262128-339499,557930-573266,25-57,92856246-93001520,2-12,1919108745-1919268183,48414903-48538379,38342224-38444598,483824-534754,1056-1771,4603696-4688732,75712519-75792205,20124-44038,714164-782292,4429019-4570680,9648251-9913729,6812551522-6812585188,58-134,881574-897488,648613-673853,5261723647-5261785283,60035-128980,9944818-10047126,857821365-857927915,206885-246173,1922-9652,424942-446151,408-1000";

    let result = run_day2_part_1(my_exercise_input);
    println!("Parte 1: {result}");

    let result_optimized = run_day2_part_1_optimized(my_exercise_input);
    println!("Parte 1, optimizado: {result_optimized}");

    let result = run_day2_part_2(my_exercise_input);
    println!("Parte 2, optimizado: {result}");
}

#[cfg(test)]
mod tests {
    use fancy_regex::Regex;

    use super::*;

    const EXERCISE_EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    // const EXERCISE_EXAMPLE: &str = "197-407,262128-339499,557930-573266,25-57,92856246-93001520,2-12,1919108745-1919268183,48414903-48538379,38342224-38444598,483824-534754,1056-1771,4603696-4688732,75712519-75792205,20124-44038,714164-782292,4429019-4570680,9648251-9913729,6812551522-6812585188,58-134,881574-897488,648613-673853,5261723647-5261785283,60035-128980,9944818-10047126,857821365-857927915,206885-246173,1922-9652,424942-446151,408-1000";

    #[test]
    fn regex_test() {
        let regex = Regex::new(EXP_PART1).unwrap();

        assert!(regex.is_match("11").unwrap_or(false));
        assert!(regex.is_match("5252").unwrap_or(false));
        assert!(!regex.is_match("112211").unwrap_or(false));
        assert!(!regex.is_match("100").unwrap_or(false));
        assert!(!regex.is_match("112").unwrap_or(false));
    }

    #[test]
    fn exercise_example_test() {
        assert_eq!(run_day2_part_1(EXERCISE_EXAMPLE), 1227775554);
        assert_eq!(run_day2_part_2(EXERCISE_EXAMPLE), 4174379265);
    }

    #[test]
    fn digit_count_test() {
        // Para probar a ver si así puedo obtener la cantidad de dígitos
        assert_eq!((1 as usize).ilog10() + 1, 1);
        assert_eq!((3 as usize).ilog10() + 1, 1);
        assert_eq!((12 as usize).ilog10() + 1, 2);
        assert_eq!((131 as usize).ilog10() + 1, 3);
    }

    #[test]
    fn find_invalid_numbers_part_2_test() {
        assert_eq!(find_invalid_numbers_part_2(998, 1012, 1), [999]);
        assert_eq!(find_invalid_numbers_part_2(998, 1012, 2), [1010]);
    }
}
