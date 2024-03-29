use std::{io, str::FromStr, time::Instant};

use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::{
    net::FeedForwardNetwork,
    pos::{Action, Position},
    search::Search,
};

// lazy_static! {
//     pub static ref POSITION_HISTORY: Mutex<Vec<String>> = Mutex::new(Vec::new());
// }

#[derive(Deserialize)]
#[serde(tag = "type")]
enum In {
    Load {
        input_nodes: Vec<i64>,
        output_nodes: Vec<i64>,
        node_evals: Vec<(i64, f64, f64, Vec<(i64, f64)>)>,
    },
    Pos {
        tpn: String,
    },
    Peek,
    PlayGame,
    Ready,
    Go,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum Out {
    Move { action_list: Vec<Action> },
    Pos { tpn: String },
    GameResult { score: i64 },
    Ok,
    Ko,
}

fn send(msg: &Out) -> io::Result<()> {
    println!("{}", serde_json::to_string(msg)?);
    Ok(())
}

pub fn start() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    let mut pos: Position = Position::default();
    let mut net: Option<FeedForwardNetwork> = None;
    let mut search = Search::new();

    loop {
        buffer.clear();
        let lenght = stdin.read_line(&mut buffer)?;
        if lenght == 0 {
            break;
        }

        let msg_in: In = serde_json::from_str(&buffer).expect("");

        match msg_in {
            In::Load {
                input_nodes,
                output_nodes,
                node_evals,
            } => {
                net = Some(FeedForwardNetwork::new(
                    input_nodes,
                    output_nodes,
                    node_evals,
                ));
            }
            In::Pos { tpn } => {
                // TODO: Clean error handling
                pos = Position::from_str(&tpn).unwrap();
            }
            In::Go => {
                if let Some(nn) = &mut net {
                    let start = Instant::now();

                    match search.run(nn, &pos) {
                        Some((best, action_list)) => {
                            pos = pos
                                .apply_move(pos.current_piece, best.0, best.1, best.2, true);
                            let end = Instant::now();
                            eprintln!("Thinking time: {}", (end - start).as_millis());
                            send(&Out::Move { action_list })?;
                        },
                        None => send(&Out::GameResult { score: pos.score })?
                    }
                }
            }
            In::Peek => {
                println!(
                    "{}",
                    serde_json::to_string(&Out::Pos {
                        tpn: pos.to_string()
                    })?
                )
            }
            In::PlayGame => {
                if let Some(nn) = &mut net {
                    for _ in 0..1000 {
                        match search.run(nn, &pos) {
                            Some((mv, _)) => {
                                pos = pos.apply_move(pos.current_piece, mv.0, mv.1, mv.2, true);
                                // POSITION_HISTORY.lock().unwrap().push(pos.to_string());
                            },
                            None => break
                        }
                    }
                    send(&Out::GameResult { score: pos.score })?;
                    pos = Position::default();
                };
            }
            In::Ready => match net {
                Some(_) => send(&Out::Ok)?,
                None => send(&Out::Ko)?,
            },
        }
    }

    Ok(())
}
