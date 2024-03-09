use tabled::builder::Builder;
use tabled::settings::Style;
use std::io;

fn main() {
    const TOTAL_ROWS: usize = 3;
    const TOTAL_COLUMNS: usize = 3;
    clearscreen();
    let mut board = create_board(TOTAL_ROWS, TOTAL_COLUMNS);
    let human_char = ask_player_char();
    let human_move = ask_player_move(board.clone(), human_char);
    fill_box(&mut board, human_move[0], human_move[1], human_char);
    print_board(board.clone());
}

///////////////////////////////////////////////////////////////////////////////
// board functions

fn ask_player_char() -> char {
    println!("[*] First/second (X/O)?:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let character: char = input.trim().chars().next().expect("No input provided.");
    return character.to_ascii_uppercase();
}

fn ask_player_move(board: Vec<Vec<char>>, human_char: char) -> [usize; 2] {
    loop {
        println!("[+] Your move {} -> (1-9)?: ", human_char);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let player_move: usize = input.trim().parse().expect("Please enter a number");
        let player_move_array = move_num_to_array(player_move, board.len());
        if board[player_move_array[0]][player_move_array[1]] != ' ' {
            println!("[!] Invalid: {} already filled", player_move);
        } else {
            return player_move_array;
        }
    }
}

fn move_array_to_num(move_arr: [usize; 2], board_rows: usize) -> usize {
    return move_arr[0] * board_rows + move_arr[1] + 1;
}

fn move_num_to_array(num: usize, board_rows: usize) -> [usize; 2] {
    let i: usize = (num - 1) / board_rows;
    let j: usize = (num - 1) % board_rows;
    return [i, j];
}

////////////////////////////////////////////////////////////////////////////
// game state functions

fn check_winner(board: Vec<Vec<char>>) -> char {
    if is_win(board.clone(), 'X') {
        return 'X';
    } else if is_win(board.clone(), 'O') {
        return 'O';
    }

    let mut filled_count = 0;
    let x_length = board.len();
    let y_length = board[0].len();

    for i in 0..x_length {
        for j in 0..y_length {
            if board[i][j] != ' ' {
                filled_count += 1;
            }
        }
    }

    if filled_count == x_length * y_length {
        return 'D';
    }

    return ' ';
}

fn is_win(board: Vec<Vec<char>>, player_char: char) -> bool {
    let x_length = board.len();
    for i in 0..x_length {
        // check rows
        if board[i][0] == player_char && board[i][1] == player_char && board[i][2] == player_char {
            return true;
        }
        // check columns
        if board[0][i] == player_char && board[1][i] == player_char && board[2][i] == player_char {
            return true;
        }
    }
        // check diagonals
        if board[0][0] == player_char && board[1][1] == player_char && board[2][2] == player_char || board[0][2] == player_char && board[1][1] == player_char && board[2][0] == player_char {
            return true;
        }
    return false;
}

//////////////////////////////////////////////////////////////////////////
// board functions

fn clearscreen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn fill_box(board: &mut Vec<Vec<char>>, x: usize, y: usize, player_char: char) {
    if let Some(row) = board.get_mut(x) {
        if let Some(element) = row.get_mut(y) {
            *element = player_char;
        }
    }
}

fn print_board(board: Vec<Vec<char>>) {
    let x_length = board.len();
    let y_length = board[0].len();

    let mut builder = Builder::default();
    for i in 0..x_length {
        let mut row: Vec<char> = Vec::new();
        for j in 0..y_length {
            if board[i][j] == ' ' {
                let box_num = (i * x_length + j + 1);
                let box_num_char = (b'0' + box_num as u8) as char;
                row.push(box_num_char);
            } else {
                row.push(board[i][j]);
            }
        }
        builder.push_record(row);
    }
    let table = builder.build().with(Style::modern()).to_string();
    println!("{}", table);
}

fn create_board(total_rows: usize, total_colums: usize) -> Vec<Vec<char>> {
    let mut array: Vec<Vec<char>> = Vec::new();
    for _ in 0..total_rows {
        let row: Vec<char> = vec![' '; total_colums];
        array.push(row);
    }
    return array;
}


