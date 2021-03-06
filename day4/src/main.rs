use std::collections::{HashMap, HashSet};

fn main() -> std::result::Result<(), std::io::Error> {
    let lines = std::fs::read_to_string("input.txt")?;
    let bingo_game = parse_input(lines.split("\n").collect());
    let answer1 = compute_part1(&bingo_game);
    let answer2 = compute_part2(&bingo_game);
    println!("part 1 answer is {}, part 2 answer is {}", answer1, answer2);
    Ok(())
}

#[derive(Clone, Debug)]
struct BingoGame {
    drawn_numbers: Vec<u32>,
    boards: Vec<BingoBoard>,
}

#[derive(Clone, Debug)]
struct BingoBoard {
    drawn_numbers: HashSet<u32>,

    // auxiliary struct for quick lookup
    number_to_coordinates: HashMap<u32, (usize, usize)>,
    drawn_per_column: [u8; 5],
    drawn_per_row: [u8; 5],
}

fn parse_input(lines: Vec<&str>) -> BingoGame {
    let first_line = lines[0];
    let drawn_numbers: Vec<u32> = first_line
        .split(",")
        .map(|nr| nr.parse::<u32>().unwrap())
        .collect();
    let boards: Vec<BingoBoard> = lines
        .into_iter()
        .skip(2)
        .filter(|&line| !line.trim().is_empty())
        .collect::<Vec<&str>>()
        .chunks(5)
        .map(|l| parse_board(l))
        .collect();
    BingoGame {
        drawn_numbers: drawn_numbers,
        boards: boards,
    }
}

#[test]
fn test_parse_given_example_input() {
    // GIVEN
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
     
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7";
    let lines: Vec<&str> = input.split("\n").collect();

    // WHEN
    let bingo_game = parse_input(lines);

    // THEN
    assert!(bingo_game.drawn_numbers.len() == 27);
    assert!(bingo_game.boards.len() == 3);
}

fn parse_board(lines: &[&str]) -> BingoBoard {
    let rows: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|nr| nr.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    let mut number_to_coordinates = HashMap::new();
    for (row, cols) in rows.iter().enumerate() {
        for (col, &nr) in cols.iter().enumerate() {
            number_to_coordinates.insert(nr, (row, col));
        }
    }
    BingoBoard {
        drawn_numbers: HashSet::new(),
        number_to_coordinates: number_to_coordinates,
        drawn_per_column: [0u8; 5],
        drawn_per_row: [0u8; 5],
    }
}

fn compute_part1(original_bingo_game: &BingoGame) -> u32 {
    let mut bingo_game: BingoGame = original_bingo_game.clone();
    // draw number
    for draw in original_bingo_game.drawn_numbers.iter() {
        // update boards
        for board in bingo_game.boards.iter_mut() {
            if let Some(&(row, col)) = board.number_to_coordinates.get(draw) {
                board.drawn_numbers.insert(*draw);
                board.drawn_per_row[row] |= 1 << col;
                board.drawn_per_column[col] |= 1 << row;
            }

            // continue until one board wins
            if !board.drawn_per_column.contains(&31) && !board.drawn_per_row.contains(&31) {
                continue;
            }

            // when a board wins, compute score: all non drawn numbers added up
            let sum_numbers_not_drawn: u32 = board
                .number_to_coordinates
                .keys()
                .filter(|n| !board.drawn_numbers.contains(n))
                .sum();
            return sum_numbers_not_drawn * draw;
        }
    }
    0
}

fn compute_part2(original_bingo_game: &BingoGame) -> u32 {
    let mut bingo_game: BingoGame = original_bingo_game.clone();
    // draw number
    for draw in original_bingo_game.drawn_numbers.iter() {
        // update boards
        for board in bingo_game.boards.iter_mut() {
            if let Some(&(row, col)) = board.number_to_coordinates.get(draw) {
                board.drawn_numbers.insert(*draw);
                board.drawn_per_row[row] |= 1 << col;
                board.drawn_per_column[col] |= 1 << row;
            }
        }

        // if there is only one board, it must be the last one that can win
        if bingo_game.boards.len() == 1 {
            // when a board wins, compute score: all non drawn numbers added up
            let sum_numbers_not_drawn: u32 = bingo_game.boards[0]
                .number_to_coordinates
                .keys()
                .filter(|n| !bingo_game.boards[0].drawn_numbers.contains(n))
                .sum();
            return sum_numbers_not_drawn * draw;
        }

        // otherwise filter out the winning ones until only one remains
        bingo_game.boards = bingo_game
            .boards
            .into_iter()
            .filter(|board| {
                !board.drawn_per_column.contains(&31) && !board.drawn_per_row.contains(&31)
            })
            .collect();
    }
    0
}

#[test]
fn test_parse_given_example_board() {
    // GIVEN
    let input = "22 13 17 11  0
    8  2 23  4 24
   21  9 14 16  7
    6 10  3 18  5
    1 12 20 15 19";
    let lines: Vec<&str> = input.split("\n").collect();
    assert!(lines.len() == 5);

    // WHEN
    let board = parse_board(&lines[0..5]);

    // THEN
    assert!(board.number_to_coordinates[&22] == (0, 0));
    assert!(board.number_to_coordinates[&2] == (1, 1));
    assert!(board.number_to_coordinates[&19] == (4, 4));
    assert!(board.number_to_coordinates[&1] == (4, 0));
    assert!(board.number_to_coordinates[&0] == (0, 4));

    assert!(board.drawn_numbers.is_empty());

    for n in 0..5 {
        assert!(board.drawn_per_column[n] == 0);
        assert!(board.drawn_per_row[n] == 0);
    }
}

#[test]
fn part_1_given_example() {
    // GIVEN
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7";

    // WHEN
    let bingo_game = parse_input(input.split("\n").collect());
    let r = compute_part1(&bingo_game);

    // THEN
    assert!(r == 4512)
}

#[test]
fn part_2_given_example() {
    // GIVEN
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7";

    // WHEN
    let bingo_game = parse_input(input.split("\n").collect());
    let r = compute_part2(&bingo_game);

    // THEN
    assert!(r == 1924)
}
