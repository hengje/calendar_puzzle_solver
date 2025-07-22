use ansi_term::Color::Fixed;
use ansi_term::{ANSIGenericString, Color, Style};
use chrono::Datelike;
use clap::Parser;
use solver::{Board, Brick, SolvedBoard, solve};
use std::time::Instant;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=31))]
    /// Day of month to solve for (1-31). If not specified, the current day of month is used.
    day: Option<u8>,
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=12))]
    /// Month to solve for (1-12). If not specified, the current month is used.
    month: Option<u8>,
    #[arg(short = 'H', long = "hint")]
    /// Just give a hint without solving the whole board. Default number of bricks to give as hint is 1.
    hint: Option<Option<u8>>,
}

fn main() {
    let current_date = chrono::Local::now();
    let cli = Cli::parse();
    let month = cli.month.unwrap_or_else(|| current_date.month() as u8);
    let day = cli.day.unwrap_or_else(|| current_date.day() as u8);

    let start = Instant::now();
    println!("Solving for day {day} and month {month}");
    let board = Board::for_date(day, month);
    match cli.hint {
        None => {
            for (i, solved_board) in solve(board.unwrap(), &Brick::all_bricks()).enumerate() {
                println!(
                    "Solution {} (time used:{:?}, test count: {}):",
                    i + 1,
                    start.elapsed(),
                    solved_board.test_count
                );
                print_board(&solved_board);
            }
        }
        Some(number_of_hints) => {
            let number_of_hints = number_of_hints.unwrap_or(1).min(8);
            let all_bricks = &Brick::all_bricks();
            let mut solved_boards = solve(board.unwrap(), all_bricks);
            let solved_board = solved_boards.next();
            match solved_board {
                None => {
                    println!("No solutions found!")
                }
                Some(mut solved_board) => {
                    solved_board
                        .placed_bricks
                        .truncate(number_of_hints as usize);
                    print_board(&solved_board);
                }
            }
        }
    }
}

fn print_board(board: &SolvedBoard) {
    let mut result: [u8; 51] = [0; 51];
    for (brick_number, brick) in board.placed_bricks.iter().enumerate() {
        for (i, result) in result.iter_mut().enumerate() {
            if 1 << 63 >> i & brick > 0 {
                *result = brick_number as u8 + 1;
            }
        }
    }
    println!("╔══════╗");
    for (y, line) in result.chunks(8).enumerate() {
        print!("║");
        for (x, b) in line.iter().enumerate() {
            if (y < 2 && x < 6) || (y > 1 && x < 7) {
                print!("{}", brick_dot(*b));
            }
        }
        match y {
            1 => println!("╚╗"),
            6 => println!("╔═══╝"),
            _ => println!("║"),
        };
    }
    println!("╚═══╝");
}

fn brick_dot<'a>(brick_number: u8) -> ANSIGenericString<'a, str> {
    match brick_number {
        0 => Style::new().bold().paint("O"),
        brick_number => Color::Black.on(Fixed(brick_number)).paint("■"),
    }
}
