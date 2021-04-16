use super::board;

#[derive(Debug)]
pub struct GameState {
    pub board: board::Board,
    pub player: usize,
    pub captured_dice: usize,
    pub can_pass: bool,
    pub moves: Vec<Box<GameState>>,
}

pub fn create_root(board: board::Board) -> Box<GameState> {
    Box::new(GameState {
        board,
        player: 0,
        captured_dice: 0,
        can_pass: false,
        moves: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_basic_root() {
        let sz = 2;
        let board1 = board::new_board(sz);
        let board2 = board::new_board(sz);
        assert_eq!(board::as_string(&board1, sz), board::as_string(&board2, sz));
        let root = create_root(board2);
        assert_eq!(
            board::as_string(&board1, sz),
            board::as_string(&root.board, sz)
        );
        assert_eq!(0, root.player);
        assert_eq!(0, root.captured_dice);
        assert_eq!(false, root.can_pass);
        assert_eq!(0, root.moves.len());
    }
}
