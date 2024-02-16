






use crate::chess_board::{ChessBoard, ChessMove, Color,};
use crate::evaluation::eval_function;
use crate::transposition_table::*;
use std::sync::mpsc;
use std::thread;






#[derive(Clone,Debug,Copy)]
pub struct Minimax {
    pub depth : usize,
    pub max_depth : usize,
    pub color : Color,
    pub alpha : i32,
    pub beta : i32,
}

impl Minimax {


    pub fn start_minimax(chess_board: &ChessBoard) -> Option<ChessMove>{

    
        let (tx, rx) = mpsc::channel();
        let mut threads = vec![];
        let all_moves = ChessBoard::get_party_moves(chess_board);
        println!("all moves : {:?} turn {:?} ",all_moves,ChessBoard::get_turn(&chess_board));
        let chunk_size = all_moves.len()/4; 
        let minimax = Minimax {
            max_depth : 5,
            color : ChessBoard::get_turn(chess_board),
            depth : 0,
            alpha : i32::MIN,
            beta : i32::MAX,
        };
        for chunk in all_moves.chunks(chunk_size) { 
            
            let chess_board_clone = chess_board.clone();
            let moves_chunk = chunk.to_vec();
            let tx_clone = tx.clone();

            let thread = thread::spawn(move || {
                println!("[thread] first move : {:?}",moves_chunk[0]);
                let mut transposition_table:TranspositionTable = TranspositionTable::init_transpo_table();
                let (score,m ) = Self::minimax_v2(minimax.clone(),chess_board_clone.clone(), moves_chunk,&mut transposition_table);
                tx_clone.send((score, m)).unwrap();
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

    pub fn start_minimax_mono(chess_board: &ChessBoard) -> Option<ChessMove> {
        let all_moves = ChessBoard::get_party_moves(chess_board);
        println!("all moves : {:?} turn {:?} ",all_moves,ChessBoard::get_turn(&chess_board));
        let minimax = Minimax {
            max_depth : 5,
            color : ChessBoard::get_turn(chess_board),
            depth : 0,
            alpha : i32::MIN,
            beta : i32::MAX,
        };
        let mut transposition_table:TranspositionTable = TranspositionTable::init_transpo_table();
        let (score,m ) = Self::minimax_v2(minimax.clone(),chess_board.clone(), all_moves,&mut transposition_table);
        println!("found,{:?}",score);
        m
    }



    fn minimax_v2(mut self, chess_board: ChessBoard,chess_moves: Vec<ChessMove>,transposition_table: &mut TranspositionTable) -> (i32,Option<ChessMove>) {
        let mut best_score;
        let mut best_move = None;
        if self.depth == self.max_depth || chess_moves.len() == 0  {

            let hash = TranspositionTable::get_hash(&transposition_table, &chess_board);

            if let Some(score) = TranspositionTable::get_from_table(&transposition_table, hash){
                return (score,None);
            }
            else{
                let score = eval_function(&self,chess_board);

                TranspositionTable::insert_into_table(transposition_table, hash,score);
                return (score,None);
            }
            
        }

        self.depth += 1;
        if ChessBoard::get_turn(&chess_board) == self.color {
            
            best_score = i32::MIN;
            for chess_move in chess_moves.iter() {
                
                let mut chess_board_clone = chess_board.clone();
                let new_all_moves = ChessBoard::update_board(&mut chess_board_clone,chess_move.clone());
                
                let (score , _ ) = self.minimax_v2(chess_board_clone,new_all_moves,transposition_table);
                if score > best_score {
                    best_score = score;
                    best_move = Some(chess_move.clone());
                }
                if best_score >= self.alpha  {
                    self.alpha = best_score;

                    
                }
                if self.alpha >= self.beta {
                    break;
                }
            }
        }
        else {
            best_score = i32::MAX;
            for chess_move in chess_moves.iter() {
                let mut chess_board_clone = chess_board.clone();
                let new_all_moves = ChessBoard::update_board(&mut chess_board_clone,chess_move.clone());
                let (score, _) =  self.minimax_v2(chess_board_clone,new_all_moves,transposition_table);
                if score < best_score {
                    best_score = score;
                    best_move = Some(chess_move.clone());
                }
                if self.beta > best_score {
                    self.beta = best_score;
                }
                if self.alpha >= self.beta {
                    break;
                }
            }
        }
        (best_score,best_move)
    }


    fn minimax(mut self, chess_board: ChessBoard,chess_moves: Vec<ChessMove>) -> (i32,Option<ChessMove>) {
        let mut best_score;
        let mut best_move = None;

        if self.depth == self.max_depth || chess_moves.len() == 0  {
            //let hash = zobrist_hash(&chess_board);
            //match transposition_table.get(&hash) {
                //Some(value) => return (*value,None) ,
                //None => {
                    let score = eval_function(&self,chess_board);
                    return (score,None);
                //},
            //}
            
        }

        self.depth += 1;
        if ChessBoard::get_turn(&chess_board) == self.color {
            
            best_score = i32::MIN;
            for chess_move in chess_moves.iter() {
                
                let mut chess_board_clone = chess_board.clone();
                let new_all_moves = ChessBoard::update_board(&mut chess_board_clone,chess_move.clone());
                
                let (score , _ ) = self.minimax(chess_board_clone,new_all_moves);
                if score > best_score {
                    best_score = score;
                    best_move = Some(chess_move.clone());
                }
                if best_score >= self.alpha  {
                    self.alpha = best_score;

                    
                }
                if self.alpha >= self.beta {
                    break;
                }
            }
        }
        else {
            best_score = i32::MAX;
            for chess_move in chess_moves.iter() {
                let mut chess_board_clone = chess_board.clone();
                let new_all_moves = ChessBoard::update_board(&mut chess_board_clone,chess_move.clone());
                let (score, _) =  self.minimax(chess_board_clone,new_all_moves);
                if score < best_score {
                    best_score = score;
                    best_move = Some(chess_move.clone());
                }
                if self.beta > best_score {
                    self.beta = best_score;
                }
                if self.alpha >= self.beta {
                    break;
                }
            }
        }
        (best_score,best_move)
    }

    
}









    