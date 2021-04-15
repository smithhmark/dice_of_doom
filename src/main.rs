mod board;

fn main() {
    println!("Welcome to Dice of Doom, the Rust edition!");
    let sz = 2;
    let board = board::new_board(sz);
    print_board(&board, sz);
}

fn print_board(hs: &board::Board, sz: usize) {
    print!("{}", board::board_as_string(hs, sz));
}
