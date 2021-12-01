
pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = count_increments(&input);
    return result;
}

fn count_increments(measurements: &[i32]) -> i64 {
    let mut last_number = measurements[0];
    let mut increments = 0;
    for &entry1 in measurements.iter() {
        if entry1 > last_number {
            increments += 1;
        }
        last_number = entry1;
    }

    return increments;
}

fn get_input() -> Vec<i32> {
    let file_contents = include_str!("./input.txt").split_whitespace();
    let entries = file_contents.map(|entry| entry.trim().parse().expect("Failed to convert number to integer")).collect();

    return entries;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &[i32] = &[
        199,
        200,
        208,
        210,
        200,
        207,
        240,
        269,
        260,
        263,
    ];

    #[test]
    fn example_part1_correct_result() {
        let result = count_increments(EXAMPLE_INPUT);

        assert_eq!(7, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(1681, result);
    }
}
