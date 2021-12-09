pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = calculate_risk_level(input);
    return result;
}

fn calculate_risk_level(input: &str) -> i64 {
    let map = parse_input(input);
    let mut risk_level = 0;
    let map_width = map[0].len();
    for (row_index, row) in map.iter().enumerate() {
        for (column_index, height) in row.iter().copied().enumerate() {
            if is_lowest(&map, map_width, row_index, column_index) {
                risk_level += height as i64 + 1;
            }
        }
    }

    return risk_level;
}

fn is_lowest(map: &Vec<Vec<u8>>, map_width: usize, row_index: usize, column_index: usize) -> bool {
    let height = map[row_index][column_index];
    let mut is_lowest = true;

    if row_index > 0 {
        let height_up = map[row_index - 1][column_index];
        is_lowest &= height < height_up;
    }
    if is_lowest && column_index > 0 {
        let height_left = map[row_index][column_index - 1];
        is_lowest &= height < height_left;
    }
    if is_lowest && row_index < map.len() - 1 {
        let height_down = map[row_index + 1][column_index];
        is_lowest &= height < height_down;
    }
    if is_lowest && column_index < map_width - 1 {
        let height_right = map[row_index][column_index + 1];
        is_lowest &= height < height_right;
    }

    return is_lowest;
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    return input
        .trim()
        .lines()
        .map(|line| line.as_bytes().iter().map(|char| char - '0' as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        return "2199943210
3987894921
9856789892
8767896789
9899965678";
    }

    #[test]
    fn example_part1_correct_result() {
        let result = calculate_risk_level(get_example_input());

        assert_eq!(15, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(607, result);
    }
}
