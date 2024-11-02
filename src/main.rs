use std::io;

const PLAYER_X:char = 'X';
const PLAYER_O:char = 'O';
const BOARD_SIZE:usize =3;

type Board = [[char; BOARD_SIZE]; BOARD_SIZE];

fn print_board(board: &Board) {
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            print!("{} ", board[i as usize][j as usize]);
        }
        println!();
    }
}

fn initialize_board()->Board {
    return [['-';BOARD_SIZE]; BOARD_SIZE];
}
fn check_winner(board: &Board, winner: &char)->bool {
    for i in 0..BOARD_SIZE {
        if board[i][0] == *winner && board[i][1] == *winner && board[i][2] == *winner {
            return true;
        }
        if board[0][i] == *winner && board[1][i] == *winner && board[2][i] == *winner {
            return true;
        }
    }
    if board[0][0] == *winner && board[1][1] == *winner && board[2][2] == *winner {
        return true;
    }
    if board[0][2] == *winner && board[1][1] == *winner && board[2][0] == *winner {
        return true;
    }
    return false;
}
fn check_draw(board: &Board)->bool {
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            if board[i][j] == '-' {
                return false;
            }
        }
    }
    return true;
}

fn play_game(){
    let mut board = initialize_board();
    let mut player = PLAYER_X;
    loop {
        println!("Current board:");
        print_board(&board);

        println!("Enter row and column");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut parts = input.trim().split_whitespace();
        let row:usize = parts.next().unwrap().parse().unwrap();
        let col:usize = parts.next().unwrap().parse().unwrap();
        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            println!("Invalid input. Try again");
            continue;
        }
        if board[row][col] != '-' {
            println!("Cell already taken. Try again");
            continue;
        }
        board[row][col] = player;

        if check_winner(&board, &player){
            println!("Player {} wins", player);
            break;
        }
        if check_draw(&board){
            println!("Draw");
            break;
        }
        player = if player==PLAYER_X{
        PLAYER_O
        }else{
            PLAYER_X
        };
    }

}

fn main() {
    play_game();
}
