pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = identify_corrupted_lines(input);
    return result;
}

fn identify_corrupted_lines(input: &str) -> i64 {
    #[derive(PartialEq)]
    enum BracketType {
        Round,  // ()
        Square, // []
        Joined, // {}
        Pointy, // <>
    }

    let mut squares: Vec<BracketType> = Vec::new();
    let mut sum = 0;
    for input_line in input.trim().lines() {
        for char in input_line.chars() {
            match char {
                '(' => squares.push(BracketType::Round),
                '[' => squares.push(BracketType::Square),
                '{' => squares.push(BracketType::Joined),
                '<' => squares.push(BracketType::Pointy),
                ')' | ']' | '}' | '>' => {
                    let expected = squares.pop().unwrap();
                    match char {
                        ')' => {
                            if expected != BracketType::Round {
                                sum += 3;
                                break;
                            }
                        },
                        ']' => {
                            if expected != BracketType::Square {
                                sum += 57;
                                break;
                            }
                        },
                        '}' => {
                            if expected != BracketType::Joined {
                                sum += 1197;
                                break;
                            }
                        },
                        '>' => {
                            if expected != BracketType::Pointy {
                                sum += 25137;
                                break;
                            }
                        },
                        _ => panic!("Unexpected char '{}'", char),
                    }
                },
                _ => panic!("Unexpected char '{}'", char),
            }
        }
    }

    return sum;
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        return "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    }

    #[test]
    fn example_part1_correct_result() {
        let result = identify_corrupted_lines(get_example_input());

        assert_eq!(26397, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(442131, result);
    }
}
