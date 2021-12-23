pub fn get_solution_part1() -> i64 {
    // A = 0, B = 0, C = 0, D = 0
    // #############
    // #...........#
    // ###D#A#A#D###
    //   #C#C#B#B#
    //   #########

    // A = 0, B = 0, C = 0, D = 2*
    // #############
    // #.........D.#
    // ###D#A#A#.###
    //   #C#C#B#B#
    //   #########

    // A = 5 + 6*, B = 0, C = 0, D = 2
    // #############
    // #AA.......D.#
    // ###D#.#.#.###
    //   #C#C#B#B#
    //   #########

    // A = 11, B = 3*, C = 6*, D = 2
    // #############
    // #AA.....B.D.#
    // ###D#.#.#.###
    //   #C#.#C#B#
    //   #########

    // A = 11, B = 3 + 5 + 7*, C = 6, D = 2 + 3 + 8*
    // #############
    // #AA.........#
    // ###.#B#.#D###
    //   #C#B#C#D#
    //   #########

    // A = 11 + 3 + 3*, B = 15, C = 6 + 7*, D = 13
    // #############
    // #...........#
    // ###A#B#C#D###
    //   #A#B#C#D#
    //   #########
    
    return 17 + 150 + 1300 + 13000;
}

pub fn get_solution_part2() -> i64 {
    // A = 0, B = 0, C = 0, D = 0
    // #############
    // #...........#
    // ###D#A#A#D###
    //   #D#C#B#A#
    //   #D#B#A#C#
    //   #C#C#B#B#
    //   #########
    
    // A = 5 + 5, B = 0, C = 0, D = 0
    // #############
    // #A.........A#
    // ###D#.#.#D###
    //   #D#C#B#A#
    //   #D#B#A#C#
    //   #C#C#B#B#
    //   #########
    
    // A = 10 + 8, B = 5 + 5, C = 0, D = 0
    // #############
    // #AA.....B.BA#
    // ###D#.#.#D###
    //   #D#C#.#A#
    //   #D#B#.#C#
    //   #C#C#.#B#
    //   #########
    
    // A = 18, B = 10 + 4, C = 8 + 9, D = 0
    // #############
    // #AA.B...B.BA#
    // ###D#.#.#D###
    //   #D#.#.#A#
    //   #D#.#C#C#
    //   #C#.#C#B#
    //   #########
    
    // A = 18, B = 14 + 5 + 6 + 7, C = 17, D = 0
    // #############
    // #AA........A#
    // ###D#.#.#D###
    //   #D#B#.#A#
    //   #D#B#C#C#
    //   #C#B#C#B#
    //   #########

    // A = 18 + 3, B = 32 + 9, C = 17 + 7, D = 6
    // #############
    // #AA.D.....AA#
    // ###D#B#.#.###
    //   #D#B#C#.#
    //   #D#B#C#.#
    //   #C#B#C#.#
    //   #########
    
    // A = 21, B = 41, C = 24, D = 6 + 9 + 10 + 10 + 10
    // #############
    // #AA.......AA#
    // ###.#B#.#D###
    //   #.#B#C#D#
    //   #.#B#C#D#
    //   #C#B#C#D#
    //   #########
    
    // A = 21 + 5 + 5 + 9 + 9, B = 41, C = 24 + 9, D = 45
    // A = 49, B = 41, C = 33, D = 45
    // #############
    // #AA.......AA#
    // ###.#B#C#D###
    //   #.#B#C#D#
    //   #.#B#C#D#
    //   #.#B#C#D#
    //   #########
    return 49 + 410 + 3300 + 45000;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input() {
        let result = get_solution_part1();
        assert_eq!(14467, result);
    }

    #[test]
    fn part2_input() {
        let result = get_solution_part2();
        assert_eq!(48759, result);
    }
}
