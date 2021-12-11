pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = calculate_flashes_count(input, 100);
    return result;
}

#[derive(Clone)]
struct Map {
    pub map: Vec<Vec<u8>>,
}

impl Map {
    pub fn iter_surrounding(&self, row: usize, column: usize) -> Vec<(usize, usize)> {
        let size = self.map.len();
        let mut result = Vec::new();
        for row_offset in -1..2 {
            if row_offset == -1 && row == 0 {
                continue;
            }
            if row_offset == 1 && row == size - 1 {
                continue;
            }
            for column_offset in -1..2 {
                if column_offset == -1 && column == 0 {
                    continue;
                }
                if column_offset == 1 && column == size - 1 {
                    continue;
                }

                result.push(((row as i32 + row_offset) as usize, (column as i32 + column_offset) as usize));
            }
        }

        return result;
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for row in self.map.iter() {
            for energy in row.iter().copied() {
                print!("{}", energy);
            }
            println!();
        }
    }
}

fn calculate_flashes_count(input: &str, iterations: usize) -> i64 {
    use std::collections::HashSet;

    let mut map = parse_input(input);
    let mut flashes = 0;

    for _iter in 0..iterations {
        let mut map_next = map.clone();
        let mut to_flash: Vec<(usize, usize)> = Vec::new();
        for (row_index, row) in map_next.map.iter_mut().enumerate() {
            for (column_index, energy) in row.iter_mut().enumerate() {
                *energy += 1;
                if *energy > 9 {
                    to_flash.push((row_index, column_index));
                }
            }
        }

        // println!("ro flash: {}", to_flash.len());
        let mut flashed: HashSet<(usize, usize)> = HashSet::new();
        while to_flash.len() > 0 {
            let to_flash_next = to_flash.pop().unwrap();
            if flashed.contains(&to_flash_next) {
                continue;
            }

            flashes += 1;
            for (row_index, column_index) in map_next.iter_surrounding(to_flash_next.0, to_flash_next.1) {
                let energy = &mut map_next.map[row_index][column_index];
                *energy += 1;
                if *energy > 9 {
                    to_flash.push((row_index, column_index));
                }
            }

            flashed.insert(to_flash_next);
        }

        for flashed in flashed {
            map_next.map[flashed.0][flashed.1] = 0;
        }

        map = map_next;

        // println!("iter: {}", _iter);
        // map.print();
        // println!();
    }

    return flashes;
}

fn parse_input(input: &str) -> Map {
    return Map {
        map: input
            .lines()
            .map(|line| {
                line.trim()
                    .as_bytes()
                    .iter()
                    .map(|char| char - '0' as u8)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    };
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        return "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    }

    #[test]
    fn example_part1_10_steps_correct_result() {
        let result = calculate_flashes_count(get_example_input(), 10);

        assert_eq!(204, result);
    }

    #[test]
    fn example_part1_100_steps_correct_result() {
        let result = calculate_flashes_count(get_example_input(), 100);

        assert_eq!(1656, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(1723, result);
    }
}
