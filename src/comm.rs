use std::{io, str::FromStr, time::Instant};

use serde::{Deserialize, Serialize};

use crate::{
    net::FeedForwardNetwork,
    pos::{Action, Position},
    search,
};

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
    let mut total_moves = 0;

    loop {
        buffer.clear();
        stdin.read_line(&mut buffer)?;
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
                total_moves = 0;
            }
            In::Go => {
                if let Some(nn) = &mut net {
                    let (best, action_list) = search::find_best_move(nn, &pos);
                    pos = pos.apply_move(best.0, best.1, best.2, true).unwrap();
                    send(&Out::Move { action_list })?;
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
                    let (mut best, _) = search::find_best_move(nn, &pos);
                    while let Some(new_pos) = pos.apply_move(best.0, best.1, best.2, true) {
                        if total_moves <= 500 {
                            pos = new_pos;
                            best = search::find_best_move(nn, &pos).0;
                            total_moves += 1;
                        } else {
                            break;
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
}
