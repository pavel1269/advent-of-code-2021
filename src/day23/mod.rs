pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = least_energy_move(input);
    return result;
}

#[derive(Clone, Copy)]
struct Map {
    rooms: [Option<i64>; 8],
    hallways: [Option<i64>; 7],
}

impl Map {
    pub fn possible_moved(&self) -> &Fn(&Map) -> (Map, i64) {
        let moves_rooms: [Vec<(Box<Fn(&Map) -> bool>, Box<Fn(&Map) -> Map>, i64)>; 1] = [vec![
            (
                Box::from(|map: &Map| map.hallways[1].is_none()),
                Box::from(|map: &Map| {
                    let mut map = map.clone();
                    map.hallways[1] = map.rooms[0];
                    map.rooms[0] = None;
                    return map;
                }),
                2,
            ),
            (
                Box::from(|map: &Map| map.hallways[1].is_none() && map.hallways[0].is_none()),
                Box::from(|map: &Map| {
                    let mut map = map.clone();
                    map.hallways[0] = map.rooms[0];
                    map.rooms[0] = None;
                    return map;
                }),
                3,
            ),
            (
                Box::from(|map: &Map| map.hallways[2].is_none()),
                Box::from(|map: &Map| {
                    let mut map = map.clone();
                    map.hallways[2] = map.rooms[0];
                    map.rooms[0] = None;
                    return map;
                }),
                2,
            ),
            (
                Box::from(|map: &Map| map.hallways[2].is_none() && map.hallways[3].is_none()),
                Box::from(|map: &Map| {
                    let mut map = map.clone();
                    map.hallways[3] = map.rooms[0];
                    map.rooms[0] = None;
                    return map;
                }),
                4,
            ),
            (
                Box::from(|map: &Map| {
                    map.hallways[2].is_none()
                        && map.hallways[3].is_none()
                        && map.hallways[4].is_none()
                }),
                Box::from(|map: &Map| {
                    let mut map = map.clone();
                    map.hallways[4] = map.rooms[0];
                    map.rooms[0] = None;
                    return map;
                }),
                6,
            ),
            (
                Box::from(|map: &Map| {
                    map.hallways[2].is_none()
                        && map.hallways[3].is_none()
                        && map.hallways[4].is_none()
                        && map.hallways[5].is_none()
                }),
                Box::from(|map: &Map| {
                    let mut map = map.clone();
                    map.hallways[5] = map.rooms[0];
                    map.rooms[0] = None;
                    return map;
                }),
                8,
            ),
            (
                Box::from(|map: &Map| {
                    map.hallways[2].is_none()
                        && map.hallways[3].is_none()
                        && map.hallways[4].is_none()
                        && map.hallways[5].is_none()
                        && map.hallways[6].is_none()
                }),
                Box::from(|map: &Map| {
                    let mut map = map.clone();
                    map.hallways[6] = map.rooms[0];
                    map.rooms[0] = None;
                    return map;
                }),
                9,
            ),
        ]];

        todo!();
    }

    pub fn is_done(&self) -> bool {
        self.rooms[0].is_some()
            && self.rooms[0].unwrap() == 1
            && self.rooms[1].is_some()
            && self.rooms[1].unwrap() == 1
            && self.rooms[2].is_some()
            && self.rooms[2].unwrap() == 10
            && self.rooms[3].is_some()
            && self.rooms[3].unwrap() == 10
            && self.rooms[4].is_some()
            && self.rooms[4].unwrap() == 100
            && self.rooms[5].is_some()
            && self.rooms[5].unwrap() == 100
            && self.rooms[6].is_some()
            && self.rooms[6].unwrap() == 1000
            && self.rooms[7].is_some()
            && self.rooms[7].unwrap() == 1000
    }

    pub fn from(start: ((i64, i64), (i64, i64), (i64, i64), (i64, i64))) -> Map {
        let rooms = [
            Some(start.0 .0),
            Some(start.0 .1),
            Some(start.1 .0),
            Some(start.1 .1),
            Some(start.2 .0),
            Some(start.2 .1),
            Some(start.3 .0),
            Some(start.3 .1),
        ];
        let hallway: [Option<i64>; 7] = [None, None, None, None, None, None, None];

        return Map {
            hallways: hallway,
            rooms: rooms,
        };
    }
}

fn least_energy_move(input: &str) -> i64 {
    let start = parse_input(input);
    let map = Map::from(start);
    let mut states = vec![(map, 0)];
    while let Some((map, cost)) = states.pop() {
        if map.is_done() {
            return cost;
        }

        // if rooms[0].is_some() && rooms[0].unwrap() != 1 && rooms[1].is_none() {
        //     let amphipod = rooms[0].unwrap();
        //     if hallways[1].is_none() {
        //         {
        //             states.push(move_from_room_to_hallway(
        //                 amphipod,
        //                 &rooms,
        //                 0,
        //                 &hallways,
        //                 1,
        //                 cost + 3 * amphipod,
        //             ));
        //         }

        //         if hallways[0].is_none() {
        //             states.push(move_from_room_to_hallway(
        //                 amphipod,
        //                 &rooms,
        //                 0,
        //                 &hallways,
        //                 0,
        //                 cost + 4 * amphipod,
        //             ));
        //         }
        //     }
        // }

        todo!();

        states.sort_by(|a, b| a.1.cmp(&b.1));
    }

    panic!();
}

fn move_from_room_to_hallway(
    amphipod: i64,
    rooms: &[Option<i64>; 8],
    room: usize,
    hallways: &[Option<i64>; 7],
    hallway: usize,
    cost: i64,
) -> ([Option<i64>; 8], [Option<i64>; 7], i64) {
    let mut rooms = rooms.clone();
    rooms[room] = None;
    let mut hallways = hallways.clone();
    hallways[hallway] = Some(amphipod);
    return (rooms, hallways, cost + 3 * amphipod);
}

fn parse_input(input: &str) -> ((i64, i64), (i64, i64), (i64, i64), (i64, i64)) {
    use regex::Regex;

    lazy_static::lazy_static! {
        static ref REGEX: Regex = Regex::new(r"\#([A-D])\#([A-D])\#([A-D])\#([A-D])\#").unwrap();
    }

    let mut lines = input.lines().skip(2);
    let captures = REGEX.captures(lines.next().unwrap()).unwrap();
    let a1 = convert(captures[1].chars().nth(0).unwrap());
    let b1 = convert(captures[2].chars().nth(0).unwrap());
    let c1 = convert(captures[3].chars().nth(0).unwrap());
    let d1 = convert(captures[4].chars().nth(0).unwrap());

    let captures = REGEX.captures(lines.next().unwrap()).unwrap();
    let a2 = convert(captures[1].chars().nth(0).unwrap());
    let b2 = convert(captures[2].chars().nth(0).unwrap());
    let c2 = convert(captures[3].chars().nth(0).unwrap());
    let d2 = convert(captures[4].chars().nth(0).unwrap());

    return ((a1, a2), (b1, b2), (c1, c2), (d1, d2));
}

fn convert(input: char) -> i64 {
    let mut result = 1;
    for _ in 1..input as u8 - 'A' as u8 + 1 {
        result *= 10;
    }
    return result;
}

fn get_input() -> &'static str {
    "#############
#...........#
###D#A#A#D###
  #C#C#B#B#
  #########
"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"
    }

    #[test]
    fn parse_input_example_test() {
        let input = get_example_input();
        let result = parse_input(input);
        assert_eq!(((10, 1), (100, 1000), (10, 100), (1000, 1)), result);
    }

    #[test]
    fn least_energy_move_example_test() {
        let input = get_example_input();
        let result = least_energy_move(input);
        assert_eq!(12521, result);
    }
}
