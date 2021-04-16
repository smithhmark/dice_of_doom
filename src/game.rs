use super::board;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Game {
    pub num_players: usize,
    pub board_sz: usize,
}

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

/// add_passing turns the gave over to the next player.
#[allow(dead_code)]
fn add_passing(node: &mut GameState) {
    if node.can_pass {
        println!("allowed to pass");
    } else {
        println!("NOT allowed to pass");
    }
}

/// reinforce gives dice to a player. it spreads them out from the upper left
fn reinforce(board: &board::Board, player: usize, dice: usize) -> board::Board {
    let mut new_b = board::new_board(board.len());
    let mut remaining = dice;
    for (i, j) in new_b.iter_mut().zip(board.iter()) {
        i.owner = j.owner;
        if remaining > 0 && i.owner == player {
            i.dice = j.dice + 1;
            remaining = remaining - 1;
        } else {
            i.dice = j.dice;
        }
    }
    new_b
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

    #[test]
    fn test_reinforce() {
        let sz = 2;
        let board1 = board::new_board(sz);
        let starting_state = "      0-0 0-0\n   0-0 0-0\n";
        assert_eq!(board::as_string(&board1, sz), starting_state);
        let board2 = reinforce(&board1, 1, 4);
        assert_eq!(board::as_string(&board2, sz), starting_state);
        let board2 = reinforce(&board1, 0, 4);
        assert_eq!(board::as_string(&board2, sz), "      0-1 0-1\n   0-1 0-1\n");
    }
}
