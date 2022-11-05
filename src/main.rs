use std::io;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Player {
    X,
    O,
}

impl Player {
    fn display(self) -> String {
        match self {
            Player::X => "X".to_string(),
            Player::O => "O".to_string(),
        }
    }
}

fn next_player(player: Player) -> Player {
    match player {
        Player::X => Player::O,
        Player::O => Player::X,
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Occupited(Player),
    Vacant,
}

impl Tile {
    fn display(self, cell: i32) -> String {
        match self {
            Tile::Occupited(player) => player.display(),
            Tile::Vacant => cell.to_string(),
        }
    }
}

#[derive(PartialEq, Eq)]
enum GameState {
    Ok,
    Win(Player),
    Tie,
}

fn dispaly_board(board: [Tile; 9]) {

    let horizontal_line = "+---+---+---+";

    println!("{horizontal_line}");
    println!("| {} | {} | {} |", board[0].display(1), board[1].display(2), board[2].display(3));
    println!("{horizontal_line}");
    println!("| {} | {} | {} |", board[3].display(4), board[4].display(5), board[5].display(6));
    println!("{horizontal_line}");
    println!("| {} | {} | {} |", board[6].display(7), board[7].display(8), board[8].display(9));
    println!("{horizontal_line}");
}

fn get_tile_player(tile: Tile) -> Option<Player> {
    match tile {
        Tile::Occupited(player) => Some(player),
        Tile::Vacant => None,
    }
}

fn check_state(board: [Tile; 9]) -> GameState {
    let winning_combos: [[usize; 3]; 8] = [
        [0,1,2],
        [3,4,5],
        [6,7,8],
        [0,3,6],
        [1,4,7],
        [2,5,8],
        [0,4,8],
        [2,4,6],
    ];

    for winning_combo in winning_combos {
        if board[winning_combo[0]] == board[winning_combo[1]] && board[winning_combo[1]] == board[winning_combo[2]] 
        && board[winning_combo[0]] != Tile::Vacant { 
            return GameState::Win(get_tile_player(board[winning_combo[0]]).unwrap())
        }
    } 
    if board.iter().any(|v: &Tile| v == &Tile::Vacant) {
        GameState::Ok
    }
    else {
        GameState::Tie
    }
}

fn valid_move(choice: usize, board: [Tile; 9]) -> bool {
    if choice > 9 {
        false
    } else {
        match board[choice] {
            Tile::Vacant => true,
            _ => false,
        }
    }
}
fn main() {

let mut board:  [ Tile; 9] = [Tile::Vacant; 9];

let mut current_player: Player = Player::X;

let mut game_state: GameState = GameState::Ok;

println!("Welcome to Tic Tac Toe...");

while game_state == GameState::Ok {
    let mut get_input = String::new();

    dispaly_board(board);

    println!("Player {}, Select Move: ", current_player.display());

    io::stdin()
        .read_line(&mut get_input)
        .expect("Failed to read line");
    match get_input.trim().parse::<usize>() {
        Ok(_i) => (),
        Err(..) => { 
            println!("Not a valid selection");
            continue
        }
    };
    
    let choice: usize = get_input.trim().parse::<usize>().unwrap()-1;

    if valid_move(choice, board) {
        board[choice] = Tile::Occupited(current_player);
        game_state = check_state(board); 
        current_player = next_player(current_player);
    } 
    else {
        println!("Not a valid move, please choose again");
    }
}

dispaly_board(board);

match game_state {
    GameState::Win(player) => println!("Player {} Wins!", player.display()),
    GameState::Tie => print!("Its a tie!"),
    _ => println!("Error: You shouldn't be here"),
} 

}