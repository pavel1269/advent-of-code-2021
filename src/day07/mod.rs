
pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = get_fuel_required_to_align(input);
    return result;
}

fn get_fuel_required_to_align(input: &str) -> i64 {
    let mut positions = parse_input(input);
    positions.sort();
    let middle = positions[positions.len() / 2] as i64;
    let result: i64 = positions.iter().copied().map(|position| (position as i64 - middle).abs()).sum();
    return result;
}

fn parse_input(input: &str) -> Vec<i32> {
    return input.trim().split(',').map(|clock| clock.parse().unwrap()).collect();
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        return "16,1,2,0,4,2,7,1,2,14";
    }

    #[test]
    fn example_part1_correct_result() {
        let result = get_fuel_required_to_align(get_example_input());

        assert_eq!(37, result);
    }
    
    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(351901, result);
    }
}
