use super::board;

#[allow(dead_code)]
#[derive(Debug)]
pub struct GameSetup {
    pub players: usize,
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
    new_turn(board, 0)
}

fn new_turn(board: board::Board, player: usize) -> Box<GameState> {
    Box::new(GameState {
        board,
        player,
        captured_dice: 0,
        can_pass: false,
        moves: vec![],
    })
}

/// determine the next play id
fn next_player(current: usize, number: usize) -> usize {
    (current + 1) % number
}

/// add_passing turns the gave over to the next player.
#[allow(dead_code)]
fn add_passing(node: &mut GameState, setup: &GameSetup) {
    if node.can_pass {
        println!("allowed to pass");
        println!("  find next player -- required player count");
        let next_player = next_player(node.player, setup.players);
        println!("  determine board after reinforcements");
        let new_board = reinforce(node.board, node.player, node.captured_dice);
        println!("  create link to GameState with above");
        node.moves.push(new_turn(new_board, next_player));
    } else {
        println!("NOT allowed to pass");
        println!("  do nothing");
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

    #[test]
    fn test_next_player() {
        let number_of_players = 3;
        assert_eq!(next_player(0, number_of_players), 1);
        assert_eq!(next_player(1, number_of_players), 2);
        assert_eq!(next_player(2, number_of_players), 0);
    }
}
