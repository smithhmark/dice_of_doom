use rand::Rng;

use super::hex;

pub type Board = Vec<hex::Hex>;

pub fn new_board(size: usize) -> Board {
    let hex_count = size * size;
    let mut hs = Vec::with_capacity(hex_count);
    for _i in 0..hex_count {
        hs.push(hex::Hex { owner: 0, dice: 0 });
    }
    hs
}

pub fn random_board(size: usize, player_cnt: usize, max_dice: usize) -> Board {
    let mut rng = rand::thread_rng();
    let hex_count = size * size;
    let mut hs = Vec::with_capacity(hex_count);
    for _i in 0..hex_count {
        let owner = rng.gen_range(0..player_cnt);
        let dice = rng.gen_range(1..max_dice);
        hs.push(hex::Hex { owner, dice });
    }
    hs
}

pub fn as_string(hs: &Board, sz: usize) -> String {
    let mut rep = String::new();

    for row in 0..sz {
        let off_set = sz - row;
        for _row_skew in 0..off_set {
            rep.push_str("   ");
        }
        for col in 0..(sz - 1) {
            let idx = sz * row + col;
            rep.push_str(&format!("{} ", hex::as_string(&hs[idx])));
        }
        let idx = sz * row + sz - 1;
        rep.push_str(&format!("{}\n", hex::as_string(&hs[idx])));
    }
    rep
}

pub fn neighbor_indices(pos: usize, sz: usize) -> Vec<usize> {
    let pos = pos as isize;
    let sz = sz as isize;
    let highest = sz * sz;
    let up: isize = pos - sz;
    let down: isize = pos + sz;
    let mut ns: Vec<isize> = Vec::with_capacity(6);
    let column = pos % sz;
    let left = 0;
    let right = sz - 1;
    ns.push(up);
    ns.push(down);
    if column != right {
        ns.push(pos + 1);
        ns.push(down + 1);
    }
    if column != left {
        ns.push(up - 1);
        ns.push(pos - 1);
    }
    let mut output: Vec<usize> = Vec::with_capacity(6);
    for i in ns.iter().filter(|&x| *x >= 0 && *x < highest) {
        output.push(*i as usize);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_empty_board() {
        let sz: usize = 2;
        let board = new_board(sz);
        assert_eq!(board.capacity(), board.len());
        assert_eq!(sz * sz, board.len());
        for h in &board {
            assert_eq!(h.owner, 0);
            assert_eq!(h.dice, 0);
        }
    }

    #[test]
    fn test_board_as_string() {
        let sz: usize = 2;
        let board = new_board(sz);
        let rep = as_string(&board, sz);
        let exp = "      0-0 0-0\n   0-0 0-0\n".to_string();
        assert_eq!(rep, exp);
    }

    #[test]
    fn test_neighbor_indices() {
        let sz = 2;
        let pos = 2;
        let expected: Vec<usize> = vec![0, 3];
        assert_eq!(expected, neighbor_indices(pos, sz));
    }
}
