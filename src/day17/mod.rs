pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = calculate_highest_trajectory(input);
    return result;
}

#[derive(Debug)]
struct TargetArea {
    start_y: i64,
}

impl TargetArea {
    pub fn get_start_y(&self) -> i64 {
        self.start_y
    }

    pub fn from(input: &str) -> TargetArea {
        use regex::Regex;

        lazy_static::lazy_static! {
            static ref REGEX: Regex = Regex::new("^target area: x=(-?\\d+)..(-?\\d+), y=(-?\\d+)..(-?\\d+)$").unwrap();
        }
        let captures = REGEX.captures(input).unwrap();
        let start_y = captures[3].parse().unwrap();

        return TargetArea {
            start_y: start_y,
        };
    }
}

fn calculate_highest_trajectory(input: &str) -> i64 {
    let target_area = TargetArea::from(input);
    let y = target_area.get_start_y();
    let height = cumulative_sum(-y - 1);
    return height;
}

fn cumulative_sum(speed: i64) -> i64 {
    if (speed % 2) == 0 {
        let q = speed / 2;
        return speed * q + q;
    } else {
        let q = (speed + 1) / 2;
        return speed * q;
    }
}

fn get_input() -> &'static str {
    return "target area: x=144..178, y=-100..-76";
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        return "target area: x=20..30, y=-10..-5";
    }

    macro_rules! cumulative_sum_test {
        ($name: ident, $input: expr, $expected: expr) => {
            #[test]
            fn $name() {
                let result = cumulative_sum($input);
                assert_eq!($expected, result);
            }
        };
    }

    cumulative_sum_test!(cumulative_sum_test1, 1, 1);
    cumulative_sum_test!(cumulative_sum_test5, 5, 15);
    cumulative_sum_test!(cumulative_sum_test6, 6, 21);
    cumulative_sum_test!(cumulative_sum_test10, 10, 55);
    cumulative_sum_test!(cumulative_sum_test11, 11, 66);

    #[test]
    fn example_part1_correct_result() {
        let result = calculate_highest_trajectory(get_example_input());

        assert_eq!(45, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(4950, result);
    }
}
