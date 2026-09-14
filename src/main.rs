use std::fs;

const DIAL_SIZE: isize = 100;

#[derive(Debug)]
pub struct Dial {
    cursor: isize, // debería ser usize
    zero_counts: usize,
    clicks_on_zero: usize,
}

impl Dial {
    pub fn new() -> Self {
        Dial { cursor: 50, clicks_on_zero: 0, zero_counts: 0 }
    }

    pub fn move_steps(&mut self, steps: isize) {
        if steps == 0 {
            return;
        }

        // Quitamos las rotaciones completas
        self.clicks_on_zero += (steps / DIAL_SIZE).abs() as usize;

        // Pasos que quedan después de las rotaciones completas
        let steps = steps % DIAL_SIZE;

        // tremenda función la de signum
        let min_clicks_to_zero = match steps.signum() {
            1 => DIAL_SIZE - self.cursor,
            -1 => self.cursor,
            _ => return, // esto es la ostia porque sale de la función, y los clicks ya los hemos hecho arriba
            // la movida es: si el cursor está en 0 y hacemos 10 vueltas completas, por ejemplo, debería pasar
            // 10 veces por 0 y subar 1 a zero_counts?
        };

        // Si estamos en 0, necesitamos una rotación entera hasta 0, no 0 clicks
        let min_clicks_to_zero = if min_clicks_to_zero == 0 { DIAL_SIZE } else { min_clicks_to_zero };
        if steps.abs() >= min_clicks_to_zero {
            self.clicks_on_zero += 1;
        }

        // puede dar negativo
        let current = self.cursor + steps;

        // Si el resto es negativo, encuentra su congruente positivo
        // a === b (mod DIAL_SIZE), siendo a current
        self.cursor = current.rem_euclid(DIAL_SIZE);

        if self.cursor == 0 {
            self.zero_counts += 1;
        }
    }
}

fn parse(line: &str) -> isize {
    let line = line.trim();
    if line.len() < 2 {
        return 0;
    }

    let (direction, number) = line.split_at(1);

    let sign = match direction {
        "L" => -1,
        "R" => 1,
        _ => panic!("{line}????"),
    };

    number.parse::<isize>().unwrap() * sign
}

fn main() {
    let input = fs::read_to_string("./inputs/day_1.txt").unwrap();

    let mut day1 = Dial::new();
    for line in input.lines() {
        day1.move_steps(parse(line));
    }

    println!("Parte 1: {}", day1.zero_counts);
    println!("Parte 2: {}", day1.clicks_on_zero);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXERCISE_EXAMPLE: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    #[test]
    fn example() {
        let mut day1 = Dial::new();

        for line in EXERCISE_EXAMPLE.lines() {
            day1.move_steps(parse(line));
        }

        assert_eq!(day1.zero_counts, 3);
        assert_eq!(day1.clicks_on_zero, 6);
    }

    #[test]
    fn full_rotations() {
        let mut dial = Dial::new();

        dial.move_steps(1000);

        assert_eq!(dial.clicks_on_zero, 10);
        assert_eq!(dial.cursor, 50);
    }
}
