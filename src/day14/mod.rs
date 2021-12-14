use std::collections::HashMap;

pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let polymer = process_polymers(input, 10);
    let result = calculate_part1(polymer);
    return result;
}

fn calculate_part1(polymer: String) -> i64 {
    let mut counts: HashMap<char, i64> = HashMap::new();

    for char in polymer.chars() {
        if counts.contains_key(&char) {
            *counts.get_mut(&char).unwrap() += 1;
        } else {
            counts.insert(char, 1);
        }
    }

    let mut counts = counts.iter().map(|(_, count)| count).copied().collect::<Vec<_>>();
    counts.sort();
    return counts.last().unwrap() - counts.first().unwrap();
}

fn process_polymers(input: &str, iterations: usize) -> String {
    let (mut template, rules) = parse_input(input);

    for _ in 0..iterations {
        let mut new_template = Vec::new();
        for (first, second) in template.iter().copied() {
            if let Some((_, _, new_char)) = rules
                .iter()
                .copied()
                .find(|(rule_first, rule_second, _)| first == *rule_first && second == *rule_second)
            {
                new_template.push((first, new_char));
                new_template.push((new_char, second));
            } else {
                new_template.push((first, second));
            }
        }

        template = new_template;
    }

    let mut result = template.iter().copied().map(|(c, _)| c).collect::<String>();
    result.push(template.last().unwrap().1);

    return result;
}

fn parse_input(input: &str) -> (Vec<(char, char)>, Vec<(char, char, char)>) {
    let mut input_iter = input.lines();
    let template = input_iter.next().unwrap();

    let mut previous_char = None;
    let mut template_doubles = Vec::new();
    for start_char in template.chars() {
        if let Some(previous_char) = previous_char {
            template_doubles.push((previous_char, start_char));
        }

        previous_char = Some(start_char);
    }

    let mut rules = Vec::new();
    for line in input_iter {
        if line.len() <= 1 {
            continue;
        }

        let (input, output) = line.split_once(" -> ").unwrap();
        let output = output.chars().next().unwrap();
        let mut input = input.chars();
        let first = input.next().unwrap();
        let second = input.next().unwrap();
        rules.push((first, second, output))
    }

    return (template_doubles, rules);
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        return "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    }

    #[test]
    fn example_parsed_correctly() {
        let (start, rules) = parse_input(get_example_input());

        assert_eq!(3, start.len());
        assert_eq!(16, rules.len());
    }

    macro_rules! example_process_tests {
        ($name: ident, $iterations: expr, $expect_result: expr) => {
            #[test]
            fn $name() {
                let result = process_polymers(get_example_input(), $iterations);

                assert_eq!($expect_result, result.len());
            }
        };
    }

    example_process_tests!(example_part1_process_1_iteration_correct_length, 1, 7);
    example_process_tests!(example_part1_process_2_iteration_correct_length, 2, 13);
    example_process_tests!(example_part1_process_3_iteration_correct_length, 3, 25);
    example_process_tests!(example_part1_process_4_iteration_correct_length, 4, 49);
    example_process_tests!(example_part1_process_5_iteration_correct_length, 5, 97);
    example_process_tests!(example_part1_process_10_iteration_correct_length, 10, 3073);

    #[test]
    fn example_part1_correct_result() {
        let polymer = process_polymers(get_example_input(), 10);
        let result = calculate_part1(polymer);

        assert_eq!(1588, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(2590, result);
    }
}
