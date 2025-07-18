use std::fmt;

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

    pub fn solve(self) -> impl Iterator<Item = SolvedBoard> {
        SolveIterator::new(self, Brick::all_bricks())
    }

    fn set_index(&mut self, index: u8) {
        self.bitboard = self.bitboard | 1u64 << 63 >> index;
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
        ValidPlacementIterator::create(self, brick)
    }
}

struct ValidPlacementIterator<'a> {
    index: usize,
    brick_index: usize,
    board: &'a Board,
    brick: &'a Brick,
}

impl ValidPlacementIterator<'_> {
    fn create<'a>(board: &'a Board, brick: &'a Brick) -> ValidPlacementIterator<'a> {
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
        while self.brick_index < self.brick.bit_patterns.len() {
            let brick_pattern = self.brick.bit_patterns.get(self.brick_index)?;
            while self.index <= 42_usize {
                let indexed_brick_pattern = brick_pattern >> self.index;
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

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: [u8; 51] = [0; 51];
        for (brick_number, brick) in self.placed_bricks.iter().enumerate() {
            for i in 0..=50 {
                if 1 << 63 >> i & brick > 0 {
                    result[i] = brick_number as u8 + 1;
                }
            }
        }
        for lines in result.chunks(8) {
            for b in lines {
                if *b > 0_u8 {
                    write!(f, "{}", b)?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)?;
        Ok(())
    }
}

pub struct SolvedBoard {
    pub placed_bricks: Vec<u64>,
    pub test_count: u32,
}

struct SolveIterator {
    stack: Vec<(Board, Vec<Brick>)>,
    test_count: u32,
}

impl SolveIterator {
    fn new(board: Board, remaining_bricks: Vec<Brick>) -> Self {
        SolveIterator {
            stack: vec![(board, remaining_bricks)],
            test_count: 0,
        }
    }
}

impl Iterator for SolveIterator {
    type Item = SolvedBoard;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((current_board, mut bricks)) = self.stack.pop() {
            self.test_count += 1;
            if bricks.is_empty() {
                return Some(SolvedBoard {
                    placed_bricks: current_board.placed_bricks,
                    test_count: self.test_count,
                });
            }

            if let Some(brick) = bricks.pop() {
                let valid_placements = current_board.valid_placements(&brick);
                for valid_placement in valid_placements {
                    self.stack.push((valid_placement, bricks.clone()));
                }
            }
        }

        None
    }
}

#[derive(Clone)]
struct Brick {
    bit_patterns: Vec<u64>,
}

impl Brick {
    fn create(bit_patterns: Vec<u64>) -> Brick {
        Brick { bit_patterns }
    }
    fn all_bricks() -> Vec<Brick> {
        vec![
            Brick::create(vec![
                0b01100000_01000000_11000000 << (5 * 8),
                0b11000000_01000000_01100000 << (5 * 8),
                0b10000000_11100000_00100000 << (5 * 8),
                0b00100000_11100000_10000000 << (5 * 8),
            ]),
            Brick::create(vec![
                0b00010000_11110000 << (6 * 8),
                0b10000000_11110000 << (6 * 8),
                0b11110000_00010000 << (6 * 8),
                0b11110000_10000000 << (6 * 8),
                0b10000000_10000000_10000000_11000000 << (4 * 8),
                0b01000000_01000000_01000000_11000000 << (4 * 8),
                0b11000000_10000000_10000000_10000000 << (4 * 8),
                0b11000000_01000000_01000000_01000000 << (4 * 8),
            ]),
            Brick::create(vec![
                0b11100000_10000000_10000000 << (5 * 8),
                0b11100000_00100000_00100000 << (5 * 8),
                0b00100000_00100000_11100000 << (5 * 8),
                0b10000000_10000000_11100000 << (5 * 8),
            ]),
            Brick::create(vec![
                0b11100000_11100000 << (6 * 8),
                0b11000000_11000000_11000000 << (5 * 8),
            ]),
            Brick::create(vec![
                0b11100000_10100000 << (6 * 8),
                0b10100000_11100000 << (6 * 8),
                0b11000000_10000000_11000000 << (5 * 8),
                0b11000000_01000000_11000000 << (5 * 8),
            ]),
            Brick::create(vec![
                0b11100000_11000000 << (6 * 8),
                0b11000000_11100000 << (6 * 8),
                0b11100000_01100000 << (6 * 8),
                0b01100000_11100000 << (6 * 8),
                0b11000000_11000000_10000000 << (5 * 8),
                0b11000000_11000000_01000000 << (5 * 8),
                0b10000000_11000000_11000000 << (5 * 8),
                0b01000000_11000000_11000000 << (5 * 8),
            ]),
            Brick::create(vec![
                0b11110000_01000000 << (6 * 8),
                0b11110000_00100000 << (6 * 8),
                0b01000000_11110000 << (6 * 8),
                0b00100000_11110000 << (6 * 8),
                0b10000000_11000000_10000000_10000000 << (4 * 8),
                0b10000000_10000000_11000000_10000000 << (4 * 8),
                0b01000000_11000000_01000000_01000000 << (4 * 8),
                0b01000000_01000000_11000000_01000000 << (4 * 8),
            ]),
            Brick::create(vec![
                0b11100000_00110000 << (6 * 8),
                0b01110000_11000000 << (6 * 8),
                0b11000000_01110000 << (6 * 8),
                0b00110000_11100000 << (6 * 8),
                0b10000000_10000000_11000000_01000000 << (4 * 8),
                0b01000000_11000000_10000000_10000000 << (4 * 8),
                0b10000000_11000000_01000000_01000000 << (4 * 8),
                0b01000000_01000000_11000000_10000000 << (4 * 8),
            ]),
        ]
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
            println!("Checking idx {}", idx);
            assert!(empty_board.is_free(idx));
            assert!(!empty_board.is_occupied(idx));
        }
        for idx in empty_occupied_indexes {
            println!("Checking idx {}", idx);
            assert!(empty_board.is_occupied(idx));
            assert!(!empty_board.is_free(idx));
        }

        assert_eq!(empty_board.is_free(0), true);
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
        let solutions = board.solve().collect::<Vec<_>>();
        assert_eq!(solutions.len(), 64);
    }

    #[test]
    fn solve_dec_31() {
        let board = Board::for_date(31, 12).unwrap(); // December 31st.
        let solutions = board.solve().collect::<Vec<_>>();
        assert_eq!(solutions.len(), 77);
    }
}
