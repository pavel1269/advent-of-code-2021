
pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = get_number_of_overlaps(input);
    return result;
}

fn get_number_of_overlaps(input: &str) -> i64 {
    let (max_x, max_y, vents) = parse_input(input);
    let mut map = vec![vec![0; max_x]; max_y];

    for (mut x1, mut y1, mut x2, mut y2) in vents {
        if y1 == y2 {
            if x1 > x2 {
                let tmp = x1;
                x1 = x2;
                x2 = tmp;
            }

            for index_x in x1..x2+1 {
                map[y1][index_x] += 1;
            }
        }
        else if x1 == x2 {
            if y1 > y2 {
                let tmp = y1;
                y1 = y2;
                y2 = tmp;
            }

            for index_y in y1..y2+1 {
                map[index_y][x1] += 1;
            }
        }
    }

    let mut sum = 0;
    for row in map {
        for point in row {
            if point > 1 {
                sum += 1;
            }
        }
    }

    return sum;
}

fn parse_input(input: &str) -> (usize, usize, Vec<(usize, usize, usize, usize)>) {
    use regex::Regex;
    lazy_static::lazy_static!
    {
        static ref REGEX: Regex = Regex::new("^(\\d+),(\\d+) -> (\\d+),(\\d+)$").unwrap();
    }

    let mut max_x = 0;
    let mut max_y = 0;
    let mut vents = Vec::new();
    for line in input.lines() {
        let captures = REGEX.captures(line).unwrap();
        let x1 = captures[1].parse().unwrap();
        let y1 = captures[2].parse().unwrap();
        let x2 = captures[3].parse().unwrap();
        let y2 = captures[4].parse().unwrap();

        if x1 > max_x {
            max_x = x1;
        }
        if x2 > max_x {
            max_x = x2;
        }
        if y1 > max_y {
            max_y = y1;
        }
        if y2 > max_y {
            max_y = y2;
        }

        vents.push((x1, y1, x2, y2));
    }

    return (max_x + 1, max_y + 1, vents);
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        return "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";
    }

    #[test]
    fn example_part1_correct_result() {
        let result = get_number_of_overlaps(get_example_input());

        assert_eq!(5, result);
    }
    

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(6283, result);
    }
}
