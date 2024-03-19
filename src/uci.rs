use std::io::stderr;
use std::io::Write;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::{io, thread, time};

use chess::{Board, ChessMove, Color, Game, MoveGen};
use vampirc_uci::{parse, Serializable, UciInfoAttribute, UciMessage, UciOptionConfig};

use crate::evaluation_functions::attacks::{attacks, attacks_diff};
use crate::evaluation_functions::combined::combined;
use crate::evaluation_functions::piece_count::piece_count;
use crate::evaluation_functions::piece_tables::piece_tables;
use crate::evaluation_functions::piece_value::piece_value;
use crate::message;
use crate::minimax::{minimax_ab, negamax};

#[derive(Debug)]
enum EvalFunction {
    PieceCount,
    PieceValue,
    Attacks,
    PieceTables,
    AttacksDiff,
    Combined,
}

#[derive(Debug)]
struct SharedState {
    game: Game,
    eval_function: EvalFunction,
    depth: i32,
    use_negamax: bool,
    first_possible_moves: bool,
}

fn run_engine_thread<F>(state: Arc<Mutex<SharedState>>, evaluation_function: F)
where
    F: Fn(Board) -> f32 + Copy + Send + Sync + 'static,
{
    thread::spawn(move || {
        sleep(time::Duration::from_secs_f64(0.1));
        let game = {
            let lock = state.lock().expect("Concurrency Error I guess");
            lock.game.clone()
        };

        let board = game.current_position();

        let depth: i32 = {
            let lock = state.lock().expect("Concurrency Error I guess");
            lock.depth
        };

        let use_negamax: bool = {
            let lock = state.lock().expect("Concurrency Error I guess");
            lock.use_negamax
        };

        let first_possible_moves: bool = {
            let lock = state.lock().expect("Concurrency Error I guess");
            lock.first_possible_moves
        };

        if first_possible_moves {
            let mut moves = MoveGen::new_legal(&board);
            println!(
                "{}",
                UciMessage::BestMove {
                    best_move: moves.next().unwrap(),
                    ponder: None,
                }
                .serialize()
            );
        } else if !use_negamax {
            //message("Nega", "false");
            let mut moves = MoveGen::new_legal(&board);

            let mut best_move_score = f32::NEG_INFINITY;
            let mut best_move: ChessMove = moves.next().expect(
                "If there aren't any moves, something has gone quite Wrong with the driver",
            );
            for m in moves {
                let score = minimax_ab(board.make_move_new(m), depth - 1, evaluation_function);
                let score = if board.side_to_move() == Color::White {
                    score
                } else {
                    -score
                };
                if score > best_move_score {
                    best_move = m;
                    best_move_score = score;
                }
            }
            println!(
                "{}",
                UciMessage::Info(vec![UciInfoAttribute::Score {
                    cp: Some(best_move_score as i32),
                    mate: None,
                    lower_bound: None,
                    upper_bound: None,
                }])
            );
            if best_move == ChessMove::default() {
                message(
                    format!(
                        "{:?}\n{:?}\n{:?}",
                        state.lock().unwrap(),
                        best_move,
                        MoveGen::new_legal(&board).len()
                    )
                    .as_str(),
                );
            }
            println!(
                "{}",
                UciMessage::BestMove {
                    best_move,
                    ponder: None,
                }
                .serialize()
            );
        } else {
            //message("Nega", "true");
            let mut moves = MoveGen::new_legal(&board);

            let mut best_move_score = f32::NEG_INFINITY;
            let mut best_move: ChessMove = moves.next().expect(
                "If there aren't any moves, something has gone quite Wrong with the driver",
            );
            //message("Nega", "true");
            for m in moves {
                //message("Nega true", format!("{:?}", m).as_str());
                let mut new_game = game.clone();
                new_game.make_move(m);
                let score = negamax(new_game, depth - 1, evaluation_function);
                let score = if board.side_to_move() == Color::White {
                    score
                } else {
                    -score
                };
                if score > best_move_score {
                    best_move = m;
                    best_move_score = score;
                }
            }
            //message("Nega", "true");
            println!(
                "{}",
                UciMessage::Info(vec![UciInfoAttribute::Score {
                    cp: Some(best_move_score as i32),
                    mate: None,
                    lower_bound: None,
                    upper_bound: None,
                }])
            );
            if best_move == ChessMove::default() {
                message(
                    format!(
                        "{:?}\n{:?}\n{:?}",
                        state.lock().unwrap(),
                        best_move,
                        MoveGen::new_legal(&board).len()
                    )
                    .as_str(),
                );
            }
            println!(
                "{}",
                UciMessage::BestMove {
                    best_move,
                    ponder: None,
                }
                .serialize()
            );
        }
    });
}

pub(crate) fn uci_main() {
    let state = Arc::new(Mutex::new(SharedState {
        game: Game::new(),
        eval_function: EvalFunction::PieceCount,
        depth: 3,
        use_negamax: false,
        first_possible_moves: false,
    }));
    'input: loop {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Reading from stdin failed");
        let messages = parse(&buf);
        //message("Message Loop", format!("{:?} {:?}", messages, state.lock().unwrap()).as_str());
        for message in messages {
            writeln!(stderr(), "{}", message).expect("TODO: panic message");
            match message {
                UciMessage::Uci => {
                    println!(
                        "{}",
                        UciMessage::Id {
                            name: Some("Strawberry Chess".parse().unwrap()),
                            author: Some("Strawberry Fragemann".parse().unwrap()),
                        }
                        .serialize()
                    );
                    println!(
                        "{}",
                        UciMessage::Option(UciOptionConfig::Combo {
                            name: "EvalFunction".to_string(),
                            default: Some("piece_value".into()),
                            var: vec![
                                "piece_count".into(),
                                "piece_value".into(),
                                "attacks".into(),
                                "attacks_diff".into(),
                                "piece_tables".into(),
                                "combined".into(),
                            ],
                        })
                        .serialize()
                    );
                    println!(
                        "{}",
                        UciMessage::Option(UciOptionConfig::Spin {
                            name: "Depth".to_string(),
                            default: Some(3),
                            min: Some(0),
                            max: Some(1000),
                        })
                        .serialize()
                    );
                    println!(
                        "{}",
                        UciMessage::Option(UciOptionConfig::Check {
                            name: "use_negamax".to_string(),
                            default: Some(false),
                        })
                        .serialize()
                    );
                    println!(
                        "{}",
                        UciMessage::Option(UciOptionConfig::Check {
                            name: "first_possible_moves".to_string(),
                            default: Some(false),
                        })
                        .serialize()
                    );
                    println!("{}", UciMessage::UciOk.serialize());
                }
                UciMessage::Debug(_) => {}
                UciMessage::IsReady => {
                    println!("{}", UciMessage::ReadyOk.serialize());
                }
                UciMessage::Register { .. } => {
                    todo!()
                }
                UciMessage::Position {
                    startpos,
                    fen,
                    moves,
                } => {
                    let mut lock = state.lock().expect("Concurrency Error I guess");
                    if startpos {
                        lock.game = Game::new()
                    } else if let Some(fen) = fen {
                        lock.game =
                            Game::from_str(fen.as_str()).expect("Chess engine Send Invalid fen");
                    } else {
                        panic!("Neither startpos nor fen was supplied!")
                    }

                    for m in moves {
                        lock.game.make_move(m);
                    }
                }
                UciMessage::SetOption { name, value } => match name.as_str() {
                    "EvalFunction" => {
                        let mut lock = state.lock().expect("Concurrency Error I guess");
                        lock.eval_function =
                            match value.clone().expect("This needs to have a value").as_str() {
                                "piece_count" => EvalFunction::PieceCount,
                                "piece_value" => EvalFunction::PieceValue,
                                "attacks" => EvalFunction::Attacks,
                                "attacks_diff" => EvalFunction::AttacksDiff,
                                "piece_tables" => EvalFunction::PieceTables,
                                "combined" => EvalFunction::Combined,
                                _ => {
                                    panic!("Unkown Eval Function {:?}", value)
                                }
                            }
                    }
                    "Depth" => {
                        let mut lock = state.lock().expect("Concurrency Error I guess");
                        lock.depth = value.expect("This needs to have a value").parse().unwrap()
                    }
                    "use_negamax" => {
                        let mut lock = state.lock().expect("Concurrency Error I guess");
                        lock.use_negamax = match value.expect("This needs to have a value").as_str()
                        {
                            "true" => true,
                            "false" => false,
                            _ => panic!("invalid value"),
                        };
                        // crate::message(file!(), lock.use_negamax.to_string().as_str());
                    }
                    "first_possible_moves" => {
                        let mut lock = state.lock().expect("Concurrency Error I guess");
                        lock.first_possible_moves =
                            match value.expect("This needs to have a value").as_str() {
                                "true" => true,
                                "false" => false,
                                _ => panic!("invalid value"),
                            };
                    }
                    _ => {
                        panic!("Unkown Option {}", name)
                    }
                },
                UciMessage::UciNewGame => {}
                UciMessage::Stop => {}
                UciMessage::PonderHit => {}
                UciMessage::Quit => break 'input,
                UciMessage::Go {
                    time_control: _,
                    search_control: _,
                } => {
                    let lock = state.lock().expect("Concurrency Error I guess");
                    match lock.eval_function {
                        EvalFunction::PieceCount => run_engine_thread(state.clone(), piece_count),
                        EvalFunction::PieceValue => run_engine_thread(state.clone(), piece_value),
                        EvalFunction::Attacks => run_engine_thread(state.clone(), attacks),
                        EvalFunction::AttacksDiff => run_engine_thread(state.clone(), attacks_diff),
                        EvalFunction::PieceTables => run_engine_thread(state.clone(), piece_tables),
                        EvalFunction::Combined => run_engine_thread(state.clone(), combined),
                    };
                }
                UciMessage::Id { .. } => {}
                UciMessage::UciOk => {}
                UciMessage::ReadyOk => {}
                UciMessage::BestMove { .. } => {}
                UciMessage::CopyProtection(_) => {}
                UciMessage::Registration(_) => {}
                UciMessage::Option(_) => {}
                UciMessage::Info(_) => {}
                UciMessage::Unknown(_, _) => {}
            }
        }
    }
}
