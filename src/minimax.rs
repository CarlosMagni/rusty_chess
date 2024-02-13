



use crate::chess_board::{ChessBoard, ChessMove, Color, PartyState, PieceType};
use std::sync::mpsc;
use std::thread;

#[derive(Clone,Debug,Copy)]
struct MinimaxInfo {
    depth : usize,
    max_depth : usize,
    color : Color,
    alpha : i32,
    beta : i32,
}



pub fn start_minimax(chess_board: &ChessBoard) -> Option<ChessMove>{
    let (tx, rx) = mpsc::channel();
    let mut threads = vec![];
    let all_moves = ChessBoard::get_party_moves(chess_board);
    println!("all moves : {:?} turn {:?} ",all_moves,ChessBoard::get_turn(&chess_board));
    let chunk_size = all_moves.len()/4; 
    let minimax_info = MinimaxInfo {
        max_depth : 4,
        color : ChessBoard::get_turn(chess_board),
        depth : 0,
        alpha : i32::MIN,
        beta : i32::MAX,
    };
    for chunk in all_moves.chunks(chunk_size) { 
        
        let chess_board_clone = chess_board.clone();
        let moves_chunk = chunk.to_vec();
        let tx_clone = tx.clone();
        let minimax_info_clone = minimax_info.clone();
        let thread = thread::spawn(move || {
            println!("[thread] first move : {:?}",moves_chunk[0]);
            let (score, minimax_move) = minimax(chess_board_clone, moves_chunk, minimax_info_clone);
            tx_clone.send((score, minimax_move)).unwrap();
        });
        threads.push(thread);
            
    }

    drop(tx);

    let best_move = rx.into_iter()
        .max_by_key(|&(score, _)|  {println!("found,{:?}",score);score})
        .map(|(_, best_move)| best_move)
        .expect("Aucun coup valide trouvé");
    best_move
}

fn minimax(chess_board: ChessBoard,chess_moves: Vec<ChessMove>, mut minimax_info: MinimaxInfo) -> (i32,Option<ChessMove>) {
    let mut best_score: i32 = 0;
    let mut best_move = None;
    if minimax_info.depth == minimax_info.max_depth || chess_moves.len() == 0  {
        let score = eval_function(chess_board, minimax_info);
        return (score,None);
    }
    minimax_info.depth += 1;
    if ChessBoard::get_turn(&chess_board) == minimax_info.color {
        
        best_score = i32::MIN;
        for chess_move in chess_moves.iter() {
            
            let mut chess_board_clone = chess_board.clone();
            let new_all_moves = ChessBoard::update_board(&mut chess_board_clone,chess_move.clone());
            
            let (score , _ ) = minimax(chess_board_clone,new_all_moves,minimax_info);
            if score > best_score {
                best_score = score;
                best_move = Some(chess_move.clone());
            }
            if minimax_info.alpha >= best_score {
                minimax_info.alpha = best_score
            }
            if minimax_info.alpha >= minimax_info.beta {
                break;
            }
        }
    }
    else {
        best_score = i32::MAX;
        for chess_move in chess_moves.iter() {
            let mut chess_board_clone = chess_board.clone();
            let new_all_moves = ChessBoard::update_board(&mut chess_board_clone,chess_move.clone());
            let (score , _ ) =  minimax(chess_board_clone,new_all_moves,minimax_info);
            if score < best_score {
                best_score = score;
                best_move = Some(chess_move.clone());
            }
            if minimax_info.beta > best_score {
                minimax_info.beta = best_score;
            }
            if minimax_info.alpha >= minimax_info.beta {
                break;
            }
        }
    }
    (best_score,best_move)
}


fn eval_function(chess_board: ChessBoard, minimax_info: MinimaxInfo) -> i32 {
    // Vérifier l'état de la partie
    let party_state =  ChessBoard::get_party_state(&chess_board);
    match party_state {
        PartyState::CheckMate => {
            println!("found check mate");
            if minimax_info.color == ChessBoard::get_turn(&chess_board) {
                i32::MIN
            } else {

                i32::MAX
            }
        },
        PartyState::Pat => {
            10

        },
        _ => {
            let mut friend_points : i32 = 0;
            let mut ennemy_points : i32 = 0;
            for piece in ChessBoard::get_ennemy_pieces(&chess_board) {
                match piece.piece_type {
                    PieceType::Rook => ennemy_points+=5,
                    PieceType::Bishop => ennemy_points+=3,
                    PieceType::Knight => ennemy_points+=3,
                    PieceType::Queen => ennemy_points+=9,
                    PieceType::Pawn =>  ennemy_points+= 1,
                    _ => (),
                }
            }
            for piece in ChessBoard::get_friendly_pieces(&chess_board) {
                match piece.piece_type {
                    PieceType::Rook => friend_points+=5,
                    PieceType::Bishop => friend_points+=3,
                    PieceType::Knight => friend_points+=3,
                    PieceType::Queen => friend_points+=9,
                    PieceType::Pawn =>  friend_points+= 1,
                    _ => (),
                }
            }
            return friend_points - ennemy_points
        }
    }
}
    