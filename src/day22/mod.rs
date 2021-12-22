use std::collections::HashSet;

pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = count_cubes_initialization(input);
    return result;
}

fn count_cubes_initialization(input: &str) -> i64 {
    const BOUNDARY_BOTTOM: i64 = -50;
    const BOUNDARY_UPPER: i64 = 51;

    let steps = parse_input(input);
    let mut cubes: HashSet<(i64, i64, i64)> = HashSet::new();

    for (is_on, x, y, z) in steps.iter() {
        for x in x.0.max(BOUNDARY_BOTTOM)..(x.1 + 1).min(BOUNDARY_UPPER) {
            for y in y.0.max(BOUNDARY_BOTTOM)..(y.1 + 1).min(BOUNDARY_UPPER) {
                for z in z.0.max(BOUNDARY_BOTTOM)..(z.1 + 1).min(BOUNDARY_UPPER) {
                    if *is_on {
                        cubes.insert((x, y, z));
                    } else {
                        cubes.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    return cubes.len() as i64;
}

fn parse_input(input: &str) -> Vec<(bool, (i64, i64), (i64, i64), (i64, i64))> {
    use regex::Regex;

    lazy_static::lazy_static! {
        static ref REGEX: Regex = Regex::new("^(on|off) x=(-?\\d+)..(-?\\d+),y=(-?\\d+)..(-?\\d+),z=(-?\\d+)..(-?\\d+)$").unwrap();
    }

    return input
        .lines()
        .map(|line| {
            let captures = REGEX.captures(line).unwrap();
            let is_on = &captures[1] == "on";
            let x = (captures[2].parse().unwrap(), captures[3].parse().unwrap());
            let y = (captures[4].parse().unwrap(), captures[5].parse().unwrap());
            let z = (captures[6].parse().unwrap(), captures[7].parse().unwrap());
            return (is_on, x, y, z);
        })
        .collect();
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input_1() -> &'static str {
        return "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";
    }

    fn get_example_input_2() -> &'static str {
        return "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";
    }

    #[test]
    fn count_cubes_initialization_example1_test() {
        let input = get_example_input_1();
        let result = count_cubes_initialization(input);
        assert_eq!(39, result);
    }

    #[test]
    fn count_cubes_initialization_example2_test() {
        let input = get_example_input_2();
        let result = count_cubes_initialization(input);
        assert_eq!(590784, result);
    }

    #[test]
    fn part1_test() {
        let result = get_solution_part1();
        assert_eq!(588200, result);
    }
}
