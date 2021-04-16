mod board;
mod game;

fn main() {
    println!("Welcome to Dice of Doom, the Rust edition!");
    let sz = 2;
    //let board = board::new_board(sz);
    let board = board::random_board(sz, 2, 4);
    print_board(&board, sz);
}

fn print_board(hs: &board::Board, sz: usize) {
    print!("{}", board::as_string(hs, sz));
}
