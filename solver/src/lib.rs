use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Board {
    bitboard: u64,
    pub placed_bricks: Vec<u64>,
}

impl Board {
    fn new() -> Board {
        Board {
            bitboard: 0b00000011_00000011_00000001_00000001_00000001_00000001_00011111_11111111u64,
            placed_bricks: Vec::with_capacity(8),
        }
    }
    pub fn for_date(day: u8, month: u8) -> Result<Board, String> {
        let mut empty_board = Board::new();
        match month {
            1..=6 => empty_board.set_index(month - 1),
            7..=12 => empty_board.set_index(month + 1),
            _ => return Err(format!("Invalid month {month}. Valid months: 1-12")),
        }
        match day {
            1..=7 => empty_board.set_index(day + 15),
            8..=14 => empty_board.set_index(day + 16),
            15..=21 => empty_board.set_index(day + 17),
            22..=28 => empty_board.set_index(day + 18),
            29..=31 => empty_board.set_index(day + 19),
            _ => return Err(format!("Invalid day {day}. Valid days: 1-31")),
        }
        Ok(empty_board)
    }

    fn set_index(&mut self, index: u8) {
        self.bitboard |= 1u64 << 63 >> index;
    }

    #[allow(dead_code)] // Only used in tests
    fn is_free(&self, index: u8) -> bool {
        !self.is_occupied(index)
    }
    #[allow(dead_code)] // Only used in tests
    fn is_occupied(&self, index: u8) -> bool {
        (1_u64 << 63 >> index & self.bitboard) > 0
    }
    fn valid_placements<'a>(&'a self, brick: &'a Brick) -> ValidPlacementIterator<'a> {
        ValidPlacementIterator::new(self, brick)
    }
}

pub fn solve(initial_board: Board, bricks: &[Brick]) -> impl Iterator<Item = SolvedBoard> {
    SolveIterator::new(initial_board, bricks)
}

pub fn hints(board: Board, bricks: &[Brick]) -> Vec<Hint> {
    let mut brick_in_solution: HashMap<u64, usize> = HashMap::new();
    for solution in solve(board, bricks) {
        for brick in solution.placed_bricks {
            *brick_in_solution.entry(brick).or_default() += 1;
        }
    }
    let mut hints: Vec<Hint> = brick_in_solution
        .iter()
        .map(|(brick, solutions)| Hint {
            brick: *brick,
            solutions: *solutions,
        })
        .collect();
    hints.sort_unstable_by(|hint1, hint2| hint2.solutions.cmp(&hint1.solutions));
    hints
}

pub struct Hint {
    pub brick: u64,
    pub solutions: usize,
}

struct ValidPlacementIterator<'a> {
    index: usize,
    brick_index: usize,
    board: &'a Board,
    brick: &'a Brick,
}

impl ValidPlacementIterator<'_> {
    fn new<'a>(board: &'a Board, brick: &'a Brick) -> ValidPlacementIterator<'a> {
        ValidPlacementIterator {
            index: 0,
            brick_index: 0,
            board,
            brick,
        }
    }
}

impl Iterator for ValidPlacementIterator<'_> {
    type Item = Board;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        while self.brick_index < self.brick.brick_variants.len() {
            let brick_variant = self.brick.brick_variants.get(self.brick_index)?;
            while self.index <= 42_usize {
                let indexed_brick_pattern = brick_variant.bit_pattern >> self.index;
                self.index += 1;
                if (self.board.bitboard & indexed_brick_pattern) == 0 {
                    let mut placed_bricks = self.board.placed_bricks.clone();
                    placed_bricks.push(indexed_brick_pattern);
                    return Some(Board {
                        bitboard: self.board.bitboard | indexed_brick_pattern,
                        placed_bricks,
                    });
                }
            }
            self.index = 0;
            self.brick_index += 1;
        }
        None
    }
}

pub struct SolvedBoard {
    pub placed_bricks: Vec<u64>,
    pub test_count: u32,
}

struct SolveIterator<'a> {
    stack: Vec<(Board, &'a [Brick])>,
    test_count: u32,
}

impl<'a> SolveIterator<'a> {
    fn new(board: Board, bricks: &'a [Brick]) -> Self {
        let mut initial_stack = Vec::with_capacity(256);
        initial_stack.push((board, bricks));
        SolveIterator {
            stack: initial_stack,
            test_count: 0,
        }
    }
}

impl<'a> Iterator for SolveIterator<'a> {
    type Item = SolvedBoard;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((current_board, bricks)) = self.stack.pop() {
            self.test_count += 1;
            if bricks.is_empty() {
                return Some(SolvedBoard {
                    placed_bricks: current_board.placed_bricks,
                    test_count: self.test_count,
                });
            }

            if let Some((brick, remaining)) = bricks.split_first() {
                let valid_placements = current_board.valid_placements(brick);
                for valid_placement in valid_placements {
                    self.stack.push((valid_placement, remaining));
                }
            }
        }

        None
    }
}

#[derive(Clone)]
struct BrickVariant {
    bit_pattern: u64,
}

impl BrickVariant {
    fn new(bit_pattern: u64) -> Self {
        BrickVariant { bit_pattern }
    }
}

#[derive(Clone)]
pub struct Brick {
    brick_variants: Box<[BrickVariant]>,
}

impl Brick {
    fn new(brick_variants: Box<[BrickVariant]>) -> Brick {
        Brick { brick_variants }
    }
    pub fn all_bricks() -> Box<[Brick]> {
        Box::new([
            Brick::new(Box::new([
                BrickVariant::new(0b01100000_01000000_11000000 << (5 * 8)),
                BrickVariant::new(0b11000000_01000000_01100000 << (5 * 8)),
                BrickVariant::new(0b10000000_11100000_00100000 << (5 * 8)),
                BrickVariant::new(0b00100000_11100000_10000000 << (5 * 8)),
            ])),
            Brick::new(Box::new([
                BrickVariant::new(0b00010000_11110000 << (6 * 8)),
                BrickVariant::new(0b10000000_11110000 << (6 * 8)),
                BrickVariant::new(0b11110000_00010000 << (6 * 8)),
                BrickVariant::new(0b11110000_10000000 << (6 * 8)),
                BrickVariant::new(0b10000000_10000000_10000000_11000000 << (4 * 8)),
                BrickVariant::new(0b01000000_01000000_01000000_11000000 << (4 * 8)),
                BrickVariant::new(0b11000000_10000000_10000000_10000000 << (4 * 8)),
                BrickVariant::new(0b11000000_01000000_01000000_01000000 << (4 * 8)),
            ])),
            Brick::new(Box::new([
                BrickVariant::new(0b11100000_10000000_10000000 << (5 * 8)),
                BrickVariant::new(0b11100000_00100000_00100000 << (5 * 8)),
                BrickVariant::new(0b00100000_00100000_11100000 << (5 * 8)),
                BrickVariant::new(0b10000000_10000000_11100000 << (5 * 8)),
            ])),
            Brick::new(Box::new([
                BrickVariant::new(0b11100000_11100000 << (6 * 8)),
                BrickVariant::new(0b11000000_11000000_11000000 << (5 * 8)),
            ])),
            Brick::new(Box::new([
                BrickVariant::new(0b11100000_10100000 << (6 * 8)),
                BrickVariant::new(0b10100000_11100000 << (6 * 8)),
                BrickVariant::new(0b11000000_10000000_11000000 << (5 * 8)),
                BrickVariant::new(0b11000000_01000000_11000000 << (5 * 8)),
            ])),
            Brick::new(Box::new([
                BrickVariant::new(0b11100000_11000000 << (6 * 8)),
                BrickVariant::new(0b11000000_11100000 << (6 * 8)),
                BrickVariant::new(0b11100000_01100000 << (6 * 8)),
                BrickVariant::new(0b01100000_11100000 << (6 * 8)),
                BrickVariant::new(0b11000000_11000000_10000000 << (5 * 8)),
                BrickVariant::new(0b11000000_11000000_01000000 << (5 * 8)),
                BrickVariant::new(0b10000000_11000000_11000000 << (5 * 8)),
                BrickVariant::new(0b01000000_11000000_11000000 << (5 * 8)),
            ])),
            Brick::new(Box::new([
                BrickVariant::new(0b11110000_01000000 << (6 * 8)),
                BrickVariant::new(0b11110000_00100000 << (6 * 8)),
                BrickVariant::new(0b01000000_11110000 << (6 * 8)),
                BrickVariant::new(0b00100000_11110000 << (6 * 8)),
                BrickVariant::new(0b10000000_11000000_10000000_10000000 << (4 * 8)),
                BrickVariant::new(0b10000000_10000000_11000000_10000000 << (4 * 8)),
                BrickVariant::new(0b01000000_11000000_01000000_01000000 << (4 * 8)),
                BrickVariant::new(0b01000000_01000000_11000000_01000000 << (4 * 8)),
            ])),
            Brick::new(Box::new([
                BrickVariant::new(0b11100000_00110000 << (6 * 8)),
                BrickVariant::new(0b01110000_11000000 << (6 * 8)),
                BrickVariant::new(0b11000000_01110000 << (6 * 8)),
                BrickVariant::new(0b00110000_11100000 << (6 * 8)),
                BrickVariant::new(0b10000000_10000000_11000000_01000000 << (4 * 8)),
                BrickVariant::new(0b01000000_11000000_10000000_10000000 << (4 * 8)),
                BrickVariant::new(0b10000000_11000000_01000000_01000000 << (4 * 8)),
                BrickVariant::new(0b01000000_01000000_11000000_10000000 << (4 * 8)),
            ])),
        ])
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn initial_empty_board() {
        let empty_board = Board::new();
        let empty_free_indexes = [
            0, 1, 2, 3, 4, 5, 8, 9, 10, 11, 12, 13, 16, 17, 18, 19, 20, 21, 22, 24, 25, 26, 27, 28,
            29, 30, 32, 33, 34, 35, 36, 37, 38, 40, 41, 42, 43, 44, 45, 46, 48, 49, 50,
        ];
        let empty_occupied_indexes = [
            6, 7, 14, 15, 15, 23, 31, 47, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
        ];
        println!(
            "TEST. Board value: {}, {:b}, {:b}",
            empty_board.bitboard,
            empty_board.bitboard,
            1u64 << 63
        );
        println!("{:b}", empty_board.bitboard);
        println!("{:b}", 1u64 << 63);

        for idx in empty_free_indexes {
            println!("Checking idx {idx}");
            assert!(empty_board.is_free(idx));
            assert!(!empty_board.is_occupied(idx));
        }
        for idx in empty_occupied_indexes {
            println!("Checking idx {idx}");
            assert!(empty_board.is_occupied(idx));
            assert!(!empty_board.is_free(idx));
        }

        assert!(empty_board.is_free(0));
    }

    #[test]
    fn place_all_brick_variants_on_empty_board() {
        let empty_board = Board::new();
        let mut placement_counter = 0;
        for brick in Brick::all_bricks() {
            placement_counter += empty_board
                .valid_placements(&brick)
                .collect::<Vec<_>>()
                .len()
        }
        assert_eq!(placement_counter, 961);
    }

    #[test]
    fn solve_jan_1() {
        let board = Board::for_date(1, 1).unwrap(); // January 1st.
        let solutions = solve(board, &Brick::all_bricks()).collect::<Vec<_>>();
        assert_eq!(solutions.len(), 64);
        assert!(
            solutions.last().unwrap().test_count <= 4_704_245,
            "Regression, used {} tests",
            solutions.last().unwrap().test_count
        );
    }

    #[test]
    fn solve_dec_31() {
        let board = Board::for_date(31, 12).unwrap(); // December 31st.
        let solutions = solve(board, &Brick::all_bricks()).collect::<Vec<_>>();
        assert_eq!(solutions.len(), 77);
        assert!(
            solutions.last().unwrap().test_count <= 4_790_901,
            "Regression, used {} tests",
            solutions.last().unwrap().test_count
        );
    }

    #[test]
    fn solve_sep_22() {
        let board = Board::for_date(22, 9).unwrap(); // December 31st.
        let solutions = solve(board, &Brick::all_bricks()).collect::<Vec<_>>();
        assert_eq!(solutions.len(), 29);
        assert!(
            solutions.last().unwrap().test_count <= 1_983_044,
            "Regression, used {} tests",
            solutions.last().unwrap().test_count
        );
    }

    #[test]
    fn hints_july_29() {
        let board = Board::for_date(29, 7).unwrap(); // July 29th.
        let hints = hints(board, &Brick::all_bricks());
        // There are 155 possible valid hint bricks
        assert_eq!(hints.len(), 155);
        // The "best" hint has 12 possible solutions
        assert_eq!(hints.first().unwrap().solutions, 12);
        // The "worst" hint has only one possible solution
        assert_eq!(hints.last().unwrap().solutions, 1);
    }
}
