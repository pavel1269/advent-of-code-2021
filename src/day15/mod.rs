use std::collections::{hash_map::Entry, HashMap};

pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = cheapest_path(input);
    return result;
}

fn cheapest_path(input: &str) -> i64 {
    let (width, height, map) = parse_input(input);
    let width_bound = width - 1;
    let height_bound = height - 1;
    const STEP_COST: i64 = 1;
    let mut path = vec![((0, 0), 0, (width_bound + height_bound) as i64 * STEP_COST); 1];
    let mut visited = HashMap::new();

    while let Some(((x, y), cost, distance)) = path.pop() {
        match visited.entry((x, y)) {
            Entry::Vacant(entry) => {
                entry.insert(cost);
            }
            Entry::Occupied(mut visited_cost) => {
                if *visited_cost.get() <= cost {
                    continue;
                } else {
                    *visited_cost.get_mut() = cost;
                }
            },
        };

        // println!("[{},{}] cost: {}, distance: {}", x, y, cost, distance);
        if x == width - 1 && y == height - 1 {
            return cost;
        }
        if x + 1 < width {
            path.push(((x + 1, y), cost + map[y][x + 1], distance - STEP_COST));
        }
        if y + 1 < height {
            path.push(((x, y + 1), cost + map[y + 1][x], distance - STEP_COST));
        }
        if x > 0 {
            path.push(((x - 1, y), cost + map[y][x - 1], distance + STEP_COST));
        }
        if y > 0 {
            path.push(((x, y - 1), cost + map[y - 1][x], distance + STEP_COST));
        }

        path.sort_unstable_by_key(|(_, cost, distance)| *cost + *distance);
        path.reverse();
    }

    panic!();
}

fn parse_input(input: &str) -> (usize, usize, Vec<Vec<i64>>) {
    let mut width = None;
    let mut map = Vec::new();
    for line in input.lines() {
        let row = line
            .as_bytes()
            .into_iter()
            .map(|c| (c - '0' as u8) as i64)
            .collect::<Vec<_>>();
        width = Some(row.len());
        map.push(row);
    }
    let width = width.unwrap();
    let height = map.len();

    return (width, height, map);
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_1_input() -> &'static str {
        return "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    }

    fn get_example_2_input() -> &'static str {
        return "19999
19111
11191";
    }

    #[test]
    fn example1_part1_correct_result() {
        let result = cheapest_path(get_example_1_input());

        assert_eq!(40, result);
    }

    #[test]
    fn example2_part1_correct_result() {
        let result = cheapest_path(get_example_2_input());

        assert_eq!(8, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(462, result);
    }
}
