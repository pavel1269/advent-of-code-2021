
pub fn get_solution_part1() -> i64 {
    let diagnostic = get_input();
    let result = calculate_power_consumption(diagnostic);
    return result;
}

fn calculate_power_consumption(diagnostic: Vec<&'static str>) -> i64 {
    let mut common_bits = vec![(0, 0); diagnostic[0].len()];

    for diag_line in diagnostic {
        for (index, bit) in diag_line.chars().enumerate() {
            let is_one = match bit {
                '1' => true,
                _ => false,
            };

            if is_one {
                common_bits[index].1 += 1;
            }
            else {
                common_bits[index].0 += 1;
            }
        }
    }

    let mut epsilon = 0;
    let mut gamma = 0;
    for (zeroes, ones) in common_bits {
        epsilon <<= 1;
        gamma <<= 1;
        if ones > zeroes {
            epsilon += 1;
        } else {
            gamma += 1;
        }
    }

    return gamma * epsilon;
}

fn get_input() -> Vec<&'static str> {
    let file_contents = include_str!("./input.txt").lines().collect();
    return file_contents;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> Vec<&'static str> {
        return vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ];
    }

    #[test]
    fn example_part1_correct_result() {
        let result = calculate_power_consumption(get_example_input());

        assert_eq!(198, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(3320834, result);
    }
}
