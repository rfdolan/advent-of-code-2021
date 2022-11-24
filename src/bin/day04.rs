use std::vec::Vec;

const BOARD_SIZE: u32 = 5;

fn main() {
    let vec = inp::parse_file("day04.txt");
    part1(&vec);
    part2(&vec);
}
struct Tile {
    value: u32,
    has_been_called: bool
}

struct Board {
    size: u32,
    tiles: Vec<Tile>,
    has_won: bool
}

fn _print_board(board: &Board) {
    let mut row = 0;
    for tile in &board.tiles {
        if row >= board.size {
            println!();
            row = 0;
        }
        print!("{}:{} ", tile.value, if tile.has_been_called {"X"} else {"O"});
        row +=1;
    }
    println!("\n");
}

fn _print_boards(in_vec: &Vec<Board>) {
    for board in in_vec {
        _print_board(board);
    }
}

// vec<vec<(int, called?)>>
fn parse_boards(in_vec: &[String]) -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();
    let mut i = 0;
    while i <= in_vec.len() as i32 / (BOARD_SIZE as i32 +1) {
        let mut b: Board = Board { size: BOARD_SIZE, tiles: Vec::new(), has_won: false};
        let mut line_num = 0;
        while line_num < BOARD_SIZE {
            let pos = (i as u32 * (BOARD_SIZE + 1)) + line_num;
            let line = &in_vec[pos as usize];
            let num_list: Vec<i32> = line.split(" ").collect::<Vec<&str>>().iter()
                .filter(|x| !x.is_empty())
                .map(|x| x.parse().expect(&format!("Error parsing {} as int.",x)))
                .collect::<Vec<i32>>();
            for square in num_list {
                let tile = Tile {value: square as u32, has_been_called: false};
                b.tiles.push(tile);
            }
            line_num += 1;
        }
        boards.push(b);
        i+=1;
    }
    boards
}

// pass a search function a slice containing a single board, return true if board just won. (or maybe just answer)
fn has_bingo(board: &Board) -> bool {
    let mut valid_cols = vec![true, true, true, true, true];
    let mut valid_rows = vec![true, true, true, true, true];
    let mut diagonal_down_valid = false;
    let mut diagonal_up_valid = false;
    let mut row = 0;
    let mut col = 0;
    while col < board.size {
        while row < board.size {
            let tile = &board.tiles[((col * board.size) + row) as usize];
            if !tile.has_been_called {
                valid_cols[col as usize] = false;
                valid_rows[row as usize] = false;
                if col == row {
                    diagonal_down_valid = false;
                }
                if (board.size - col) == row {
                    diagonal_up_valid = false;
                }
            }
            row += 1;
        }
        row = 0;
        col += 1;
    }
    return valid_cols.contains(&true) || valid_rows.contains(&true) ||
        diagonal_down_valid || diagonal_up_valid; 
}

fn compute_score(board: &Board) -> u32 {
    let mut total = 0;
    for tile in board.tiles.iter() {
        if !tile.has_been_called  {
            total += tile.value;
        }
    }
    total
}

// Solution for part 1
fn part1(vec: &Vec<String>) {
    let numbers = vec[0].split(",").map(|x| x.parse()
        .expect(&format!("Error parsing {} as int.",x))).collect::<Vec<u32>>();
    let mut boards = parse_boards(&vec[2..]);
    let mut found = false;
    for number in numbers {
        if found {
            break;
        }
        for board in boards.iter_mut() {
            for mut tile in board.tiles.iter_mut() {
                if tile.value == number {
                    tile.has_been_called = true;
                }
            }
            // check for bingo, break and print solution if so
            if has_bingo(board) {
                let total = compute_score(board);
                println!("Winning board:");
                _print_board(board);
                println!("Part 1: {} * {} = {}\n", total, number, total * number);
                found = true;
            }
        }
    }
}

// Solution for part 2
fn part2(vec: &Vec<String>) {
    let numbers = vec[0].split(",").map(|x| x.parse()
        .expect(&format!("Error parsing {} as int.",x))).collect::<Vec<u32>>();
    let mut boards = parse_boards(&vec[2..]);
    let mut num_winning_boards = 0;
    let mut num = 0;
    let mut last_board_to_win = 0;
    for number in numbers {
        if num_winning_boards >= boards.len() {
            break;
        }
        num = number;
        let mut board_index = 0;
        for board in boards.iter_mut() {
            for mut tile in board.tiles.iter_mut() {
                if tile.value == number {
                    tile.has_been_called = true;
                }
            }
            // check for bingo, break and print solution if so
            if !board.has_won && has_bingo(board) {
                num_winning_boards +=1;
                last_board_to_win = board_index;
                board.has_won = true;
            }
            board_index += 1;
        }
    }
    let last_to_win = &boards[last_board_to_win as usize];
    // Find losing board
    let total = compute_score(&last_to_win);
    println!("Losing board:");
    _print_board(&last_to_win);
    println!("Part 2: {} * {} = {}", total, num, total * num);
}