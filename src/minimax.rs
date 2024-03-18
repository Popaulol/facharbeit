use chess::{Board, BoardStatus, Color, Game, MoveGen};

pub fn minimax_ab<F>(board: Board, depth: i32, evaluation_function: F) -> f32
where
    F: Fn(Board) -> f32,
    F: Copy,
{
    minimax_alpha_beta(
        board,
        depth,
        f32::NEG_INFINITY,
        f32::INFINITY,
        |board: Board| {
            if board.side_to_move() == Color::White {
                evaluation_function(board)
            } else {
                -evaluation_function(board)
            }
        },
    )
}

pub fn minimax_td<F>(board: Board, depth: i32, evaluation_function: F) -> f32
where
    F: Fn(Board) -> f32,
    F: Copy,
{
    minimax_traditional(board, depth, |board: Board| {
        if board.side_to_move() == Color::White {
            evaluation_function(board)
        } else {
            -evaluation_function(board)
        }
    })
}

pub fn minimax_traditional<F>(board: Board, depth: i32, evaluation_function: F) -> f32
where
    F: Fn(Board) -> f32,
    F: Copy,
{
    if depth == 0 {
        return evaluation_function(board);
    }

    match board.status() {
        BoardStatus::Ongoing => {
            let iterator = MoveGen::new_legal(&board);
            let moves = iterator.map(|m| {
                minimax_traditional(board.make_move_new(m), depth - 1, evaluation_function)
            });

            let score = if board.side_to_move() == Color::White {
                moves.fold(f32::NAN, f32::max)
            } else {
                moves.fold(f32::NAN, f32::min)
            };

            if score.is_nan() {
                evaluation_function(board)
            } else {
                score
            }
        }
        BoardStatus::Stalemate => 0.0,
        BoardStatus::Checkmate => {
            if board.side_to_move() == Color::White {
                f32::INFINITY
            } else {
                f32::NEG_INFINITY
            }
        }
    }
}

pub fn minimax_alpha_beta<F>(
    board: Board,
    depth: i32,
    mut alpha: f32,
    mut beta: f32,
    evaluation_function: F,
) -> f32
where
    F: Fn(Board) -> f32,
    F: Copy,
{
    if depth <= 0 {
        return evaluation_function(board);
    }

    match board.status() {
        BoardStatus::Ongoing => {
            if board.side_to_move() == Color::White {
                let mut score = f32::NEG_INFINITY;
                let iterator = MoveGen::new_legal(&board);
                for r#move in iterator {
                    score = score.max(minimax_alpha_beta(
                        board.make_move_new(r#move),
                        depth - 1,
                        alpha,
                        beta,
                        evaluation_function,
                    ));
                    if score > beta {
                        break;
                    }
                    alpha = alpha.max(score);
                }
                score
            } else {
                let mut score = f32::INFINITY;
                let iterator = MoveGen::new_legal(&board);
                for r#move in iterator {
                    score = score.min(minimax_alpha_beta(
                        board.make_move_new(r#move),
                        depth - 1,
                        alpha,
                        beta,
                        evaluation_function,
                    ));
                    if score < alpha {
                        break;
                    }
                    beta = beta.min(score);
                }
                score
            }
        }
        BoardStatus::Stalemate => 0.0,
        BoardStatus::Checkmate => {
            if board.side_to_move() == Color::White {
                f32::INFINITY
            } else {
                f32::NEG_INFINITY
            }
        }
    }
}
/*
function negamax(node, depth, α, β, color) is
    if depth = 0 or node is a terminal node then
        return color × the heuristic value of node

    childNodes := generateMoves(node)
    childNodes := orderMoves(childNodes)
    value := −∞
    foreach child in childNodes do
        value := max(value, −negamax(child, depth − 1, −β, −α, −color))
        α := max(α, value)
        if α ≥ β then
            break (* cut-off *)
    return value
 */
const DRAW_VALUE: f32 = f32::MIN;

pub fn negamax<F>(game: Game, depth: i32, evaluation_function: F) -> f32
where
    F: Fn(Board) -> f32,
    F: Copy,
{
    negamax_internal(
        game,
        depth,
        f32::NEG_INFINITY,
        f32::INFINITY,
        evaluation_function,
    )
}

fn negamax_internal<F>(
    game: Game,
    depth: i32,
    mut alpha: f32,
    beta: f32,
    evaluation_function: F,
) -> f32
where
    F: Fn(Board) -> f32,
    F: Copy,
{
    if game.can_declare_draw() {
        return DRAW_VALUE;
    }
    match game.current_position().status() {
        BoardStatus::Ongoing => {
            if depth <= 0 {
                return evaluation_function(game.current_position());
            }

            let mut value = f32::NEG_INFINITY;
            let iterator = MoveGen::new_legal(&game.current_position());
            for r#move in iterator {
                let mut new_game = game.clone();
                new_game.make_move(r#move);
                value = value.max(-negamax_internal(
                    new_game,
                    depth - 1,
                    -beta,
                    -alpha,
                    evaluation_function,
                ));
                alpha = alpha.max(value);
                if alpha >= beta {
                    break;
                }
            }

            value
        }
        BoardStatus::Stalemate => DRAW_VALUE,
        BoardStatus::Checkmate => {
            if game.side_to_move() == Color::White {
                f32::INFINITY
            } else {
                f32::NEG_INFINITY
            }
        }
    }
}
