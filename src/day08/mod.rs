// Number - Segments
// 0 - 6
// 1 - 2 *
// 2 - 5
// 3 - 5
// 4 - 4 *
// 5 - 5
// 6 - 6
// 7 - 3 *
// 8 - 7 *
// 9 - 6
// 5 Segments -> 2/3/5
// 6 Segments -> 0/6/9

pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = count_unique_output_digits(input);
    return result;
}

fn count_unique_output_digits(input: &str) -> i64 {
    let input = parse_input(input);

    let unique_output_signals = input
        .iter()
        .map(|(_, outputs)| {
            outputs
                .iter()
                .map(|signals| match signals.len() {
                    2 | 3 | 4 | 7 => 1,
                    _ => 0,
                })
                .reduce(|a, b| a + b)
                .unwrap()
        })
        .reduce(|a, b| a + b)
        .unwrap();

    return unique_output_signals as i64;
}

fn parse_input(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    use regex::Regex;
    lazy_static::lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^([a-g]{2,7}) ([a-g]{2,7}) ([a-g]{2,7}) ([a-g]{2,7}) ([a-g]{2,7}) ([a-g]{2,7}) ([a-g]{2,7}) ([a-g]{2,7}) ([a-g]{2,7}) ([a-g]{2,7}) \| ([a-g]{2,7}) ([a-g]{2,7}) ([a-g]{2,7}) ([a-g]{2,7})$").unwrap();
    }

    let result = input
        .trim()
        .lines()
        .map(|input_line| {
            let captures = REGEX.captures(input_line).unwrap();
            let patterns = vec![
                captures[1].to_string(),
                captures[2].to_string(),
                captures[3].to_string(),
                captures[4].to_string(),
                captures[5].to_string(),
                captures[6].to_string(),
                captures[7].to_string(),
                captures[8].to_string(),
                captures[9].to_string(),
                captures[10].to_string(),
            ];
            let outputs = vec![
                captures[11].to_string(),
                captures[12].to_string(),
                captures[13].to_string(),
                captures[14].to_string(),
            ];
            return (patterns, outputs);
        })
        .collect();

    return result;
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        return "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    }
    
    #[test]
    fn example_part1_correct_result() {
        let result = count_unique_output_digits(get_example_input());

        assert_eq!(26, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(342, result);
    }
}
