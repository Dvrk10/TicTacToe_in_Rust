use std::io;
use std::process::exit;

fn display_board(grid: [[&str; 3]; 3]) {
    for i in 0..3 {
        for j in 0..3 {
            print!("| {} |", grid[i][j]);
        }
        println!();
        println!("---------------")
    }
}

fn win_checker(board: [[&str; 3]; 3]) -> &str{
    for i in 0..3 {
        if board[0][i] == board[1][i] && board[1][i] == board[2][i] {
            return board[0][i];
        }
        if board[i][0] == board[i][1] && board[i][1] == board[i][2] {
            return board[i][0];
        }
    }
    if board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        return board[0][2];
    }
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return board[0][0];
    }
    return "";
}

fn show_the_winner(board: [[&str; 3]; 3]) {
    let winner = win_checker(board);
    if winner == "X" {
        println!("X wins!!!");
        exit(0);
    } else if winner == "O" {
        println!("O wins!!!");
        exit(0);
    }
}

fn main() {
    let game_on = true;

    let mut player_turn = "O";

    let mut board = [["1", "2", "3"], ["4", "5", "6"], ["7", "8", "9"]];

    display_board(board);

    while game_on {
        if player_turn == "O" {
            println!("Choose where to put O: ");

            let mut player_input_o = String::new();
            io::stdin()
                .read_line(&mut player_input_o)
                .expect("Failed to read your input");

            let player_input_o: usize = match player_input_o.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("input must be a number!");
                    continue;
                }
            };
            if player_input_o < 1 || player_input_o > 9 {
                println!("Out of range");
                continue;
            }

            let row_o: usize = (player_input_o - 1) / 3;
            let col_o: usize = (player_input_o - 1) % 3;

            if board[row_o][col_o] != "O" && board[row_o][col_o] != "X" {
                board[row_o][col_o] = "O";
                display_board(board);
                show_the_winner(board);
                player_turn = "X";
            } else {
                println!("You can't put an O there!");
                player_turn = "O";
                continue;
            }
        }

        if player_turn == "X" {
            println!("Choose where to put X: ");

            let mut player_input_x = String::new();
            io::stdin()
                .read_line(&mut player_input_x)
                .expect("Failed to read your input");

            let player_input_x: usize = match player_input_x.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            if player_input_x < 1 || player_input_x > 9 {
                println!("Out of range");
                continue;
            }

            let row_x: usize = (player_input_x - 1) / 3;
            let col_x: usize = (player_input_x - 1) % 3;

            println!("{}", board[row_x][col_x]);

            if board[row_x][col_x] != "O" && board[row_x][col_x] != "X" {
                board[row_x][col_x] = "X";
                display_board(board);
                show_the_winner(board);
                player_turn = "O"
            } else {
                println!("You can't put an X there!");
                player_turn = "X";
                continue;
            }
        }
    }
}
