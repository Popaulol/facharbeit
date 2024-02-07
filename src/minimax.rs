use chess::{Board, MoveGen};

pub fn minimax<F>(board: Board, depth: i32, our_move: bool, evaluation_function: F) -> i32
where
    F: Fn(Board) -> i32,
    F: Copy,
{
    if depth == 0 {
        return if our_move {
            evaluation_function(board)
        } else {
            -evaluation_function(board)
        };
    }
    let iterator = MoveGen::new_legal(&board);
    let moves = iterator.map(|m| {
        minimax(
            board.make_move_new(m),
            depth - 1,
            !our_move,
            evaluation_function,
        )
    });

    let score = if our_move { moves.max() } else { moves.min() };

    if let Some(score) = score {
        score
    } else if our_move {
        evaluation_function(board)
    } else {
        -evaluation_function(board)
    }
}
