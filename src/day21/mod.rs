use std::collections::HashMap;

const DICE: (i64, i64) = (1, 100);
const PAWN_POSITION: (i64, i64) = (1, 10);

pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = calculate_winner(input, 1000);
    return result;
}

fn calculate_winner(input: &str, target_score: i64) -> i64 {
    let (first, second) = parse_input(input);

    let mut dice = Dice::prepare();
    let mut position = [first, second];
    let mut score = [0, 0];
    let mut current_player = 0;
    let mut rolls = 0;
    while score[0] < target_score && score[1] < target_score {
        let moves = dice.get_next();
        position[current_player] += moves;
        while position[current_player] > PAWN_POSITION.1 {
            position[current_player] -= PAWN_POSITION.1;
        }
        score[current_player] += position[current_player];

        // println!("Player {} rolls {} and moves to space {} for a total score of {}", current_player + 1, moves, position[current_player], score[current_player]);

        current_player = if current_player == 0 { 1 } else { 0 };
        rolls += 1;
    }

    let loser = score.iter().min().copied().unwrap();
    let result = loser * rolls * 3;
    return result;
}

struct Dice {
    current: i64,
    cache: HashMap<i64, i64>,
}

impl Dice {
    pub fn get_next(&mut self) -> i64 {
        let result = self.cache[&self.current];
        self.current += 3;
        if self.current > DICE.1 {
            self.current -= DICE.1;
        }
        return result;
    }

    pub fn prepare() -> Dice {
        let mut cache = HashMap::new();

        for index in DICE.0..DICE.1 + 1 {
            *cache.entry(index).or_default() += index;
            let mut index2 = index - 1;
            if index2 < DICE.0 {
                index2 += DICE.1;
            }
            *cache.entry(index2).or_default() += index;

            let mut index2 = index2 - 1;
            if index2 < DICE.0 {
                index2 += DICE.1;
            }
            *cache.entry(index2).or_default() += index;
        }

        return Dice {
            current: DICE.0,
            cache: cache,
        };
    }
}

fn parse_input(input: &str) -> (i64, i64) {
    let (first, second) = input.split_once('\n').unwrap();
    let skip = "Player 1 starting position: ".len();
    let first = first.chars().nth(skip).unwrap() as u8 - '0' as u8;
    let second = second.chars().nth(skip).unwrap() as u8 - '0' as u8;

    return (first as i64, second as i64);
}

fn get_input() -> &'static str {
    return "Player 1 starting position: 4
Player 2 starting position: 9";
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        return "Player 1 starting position: 4
Player 2 starting position: 8";
    }

    #[test]
    fn calculate_winner_example_test() {
        let input = get_example_input();
        let result = calculate_winner(input, 1000);
        assert_eq!(739785, result);
    }

    #[test]
    fn part1_test() {
        let result = get_solution_part1();
        assert_eq!(903630, result);
    }
}
