use std::sync::{Arc, Mutex};
use std::{io, thread};
use std::io::{stderr};
use std::io::Write;
use std::str::FromStr;
use chess::{Board, ChessMove, Game, MoveGen};
use vampirc_uci::{parse, Serializable, UciMessage, UciOptionConfig};
use crate::evaluation_functions::piece_count::piece_count;
use crate::evaluation_functions::piece_value::piece_value;
use crate::minimax::minimax;

enum EvalFunction {
    PieceCount,
    PieceValue,
}

struct SharedState {
    game: Game,
    eval_function: EvalFunction,
    depth: i32
}

fn run_engine_thread<F>(state: Arc<Mutex<SharedState>>, evaluation_function: F) where F: Fn(Board) -> i32 + Copy + Send + Sync + 'static {
    thread::spawn(move || {
        let board = {
            let lock = state.lock().expect("Concurrency Error I guess");
            lock.game.current_position().clone()
        };
        let depth: i32 = {
            let lock = state.lock().expect("Concurrency Error I guess");
            lock.depth
        };

        let moves = MoveGen::new_legal(&board);

        let mut best_move_score = i32::MIN;
        let mut best_move: ChessMove = ChessMove::default();
        for m in moves {
            let score = minimax(board.make_move_new(m), depth, true, evaluation_function);

            if score > best_move_score {
                best_move = m;
                best_move_score = score;
            }

        };

        println!("{}", UciMessage::BestMove { best_move, ponder: None }.serialize());} );
}

pub(crate) fn uci_main() {

    let state = Arc::new(Mutex::new(SharedState {
        game: Game::new(),
        eval_function: EvalFunction::PieceCount,
        depth: 3,
    }));
    'input: loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Reading from stdin failed");
        let messages = parse(&buf);

        for message in messages {
            writeln!(stderr(), "{}", message).expect("TODO: panic message");
            match message {
                UciMessage::Uci => {
                    println!("{}", UciMessage::Id {
                        name: Some("Strawberry Chess".parse().unwrap()),
                        author: Some("Strawberry Fragemann".parse().unwrap())
                    }.serialize());
                    println!("{}", UciMessage::Option(UciOptionConfig::Combo {
                        name: "EvalFunction".to_string(),
                        default: Some("piece_value".into()),
                        var: vec!["piece_count".into(), "piece_value".into()],
                    }).serialize());
                    println!("{}", UciMessage::Option(UciOptionConfig::Spin {
                        name: "Depth".to_string(),
                        default: Some(3),
                        min: Some(0),
                        max: Some(1000),
                    }).serialize());
                    println!("{}", UciMessage::UciOk.serialize());
                }
                UciMessage::Debug(_) => {}
                UciMessage::IsReady => {
                    println!("{}", UciMessage::ReadyOk.serialize());
                }
                UciMessage::Register { .. } => {todo!() }
                UciMessage::Position { startpos, fen, moves } => {
                    let mut lock = state.lock().expect("Concurrency Error I guess");
                    if startpos {
                        lock.game = Game::new()
                    }
                    else if let Some(fen) = fen {
                        lock.game = Game::from_str(fen.as_str()).expect("Chess engine Send Invalid fen");
                    } else {
                        panic!("Neither startpos nor fen was supplied!")
                    }

                    for m in moves {
                        lock.game.make_move(m);
                    }
                }
                UciMessage::SetOption { name, value } => {

                    match name.as_str() {
                        "EvalFunction" => {
                            let mut lock = state.lock().expect("Concurrency Error I guess");
                            lock.eval_function = match value.clone().expect("This needs to have a value").as_str() {
                                "piece_count" => {EvalFunction::PieceCount},
                                "piece_value" => {EvalFunction::PieceValue},
                                _ => {
                                    panic!("Unkown Eval Function {:?}", value)
                                }
                            }
                        },
                        "Depth" => {
                            let mut lock = state.lock().expect("Concurrency Error I guess");
                            lock.depth = value.expect("This needs to have a value").parse().unwrap()
                        },
                        _ => {
                            panic!("Unkown Option {}", name)
                        }
                    }
                }
                UciMessage::UciNewGame => { }
                UciMessage::Stop => { }
                UciMessage::PonderHit => { }
                UciMessage::Quit => {break 'input}
                UciMessage::Go { time_control: _, search_control: _ } => {
                    let lock = state.lock().expect("Concurrency Error I guess");
                    match lock.eval_function {
                        EvalFunction::PieceCount => {
                            run_engine_thread(state.clone(), piece_count)
                        }
                        EvalFunction::PieceValue => {
                            run_engine_thread(state.clone(), piece_value)
                        }
                    };

                }
                UciMessage::Id { .. } => {}
                UciMessage::UciOk => {  }
                UciMessage::ReadyOk => { }
                UciMessage::BestMove { .. } => { }
                UciMessage::CopyProtection(_) => { }
                UciMessage::Registration(_) => { }
                UciMessage::Option(_) => { }
                UciMessage::Info(_) => { }
                UciMessage::Unknown(_, _) => { }
            }
        }

    }
}