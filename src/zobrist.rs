

use rand::Rng;

use std::collections::HashMap;


use crate::chess_board::{ChessBoard, ChessMove, Color, PartyState, PieceType};

pub struct ZobristTableStruct {
    table : Vec<Vec<u64>>,
    black_turn : u64,
}





impl ZobristTableStruct {
    pub fn init_zobrist_table() -> Self {
            let mut rng = rand::thread_rng();
            let mut table= vec![vec![0; 12]; 64];
            for i in 0..63 {
                for j in 0..11 {
                    table[i][j] = rng.gen::<u64>();
                }
            }
            let black_turn = rng.gen::<u64>();
            ZobristTableStruct { table, black_turn}
    }

    pub fn zobrist_hash(&self, chess_board: &ChessBoard) -> u64 {
        let mut hash = 0;
    
        if chess_board.get_turn() == Color::Black {
            hash = hash ^ self.black_turn;
        }
        for row in 0..7 {
            for col in 0..7 {
                if let Some(piece) = chess_board.board[row][col] {
                    let index = row * 8 + col;
                    let color_offset = match piece.color {
                        Color::White => 0,
                        Color::Black => 1,
                        Color::None => panic!("piece cannot have no color"),
                    };
    
                    let id = match piece.piece_type {
                        PieceType::Rook => 0,
                        PieceType::Bishop => 1,
                        PieceType::Knight => 2,
                        PieceType::Queen => 3,
                        PieceType::Pawn =>  4,
                        PieceType::King => 5,
                        PieceType::None => panic!("hash cannot have no piece"),
                    } + color_offset;
                    hash = hash ^ self.table[index][id];
                }
    
            }
        } 
        hash
    }



}