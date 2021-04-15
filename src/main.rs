#[derive(Debug)]
struct Hex {
    owner: usize,
    dice: usize,
}

fn new_board(size: usize) -> Vec<Hex> {
    let hex_count = size * size;
    let mut hs = Vec::with_capacity(hex_count);
    for _i in 0..hex_count {
        hs.push( Hex { owner: 0, dice: 0 } );
    }
    hs
}

fn print_board(hs: Vec<Hex>, sz: usize) {
    for row in 0..sz {
        let off_set = sz - row;
        for _row_skew in 0..off_set {
            print!("   ");
        }
        for col in 0..sz {
            let idx = sz * row + col;
            print!("{} ", format!("{}-{}", hs[idx].owner, hs[idx].dice));
        }
        print!("\n");
    }
}

fn main() {
    println!("Welcome to Dice of Doom, the Rust edition!");
    let sz = 2;
    let board = new_board(sz);
    print_board(board, sz);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_empty_board() {
        let sz: usize = 2;
        let board = new_board(sz);
        assert_eq!(board.capacity(), board.len());
        assert_eq!(sz*sz, board.len());
        for h in &board {
            assert_eq!(h.owner, 0);
            assert_eq!(h.dice, 0);
        }
    }
}