


use std::collections::HashMap;
use crate::zobrist::ZobristTableStruct;
use crate::chess_board::ChessBoard;
pub struct TranspositionTable {
    hash_map : HashMap<u64,i32>,
    zobrist_hash : ZobristTableStruct,
}


impl TranspositionTable {

    pub fn init_transpo_table() -> Self {
        let zobrist_hash = ZobristTableStruct::init_zobrist_table();
        let hash_map: HashMap<u64,i32> = HashMap::new();
        TranspositionTable { hash_map,zobrist_hash }
    }

    pub fn get_hash(&self,chess_board: &ChessBoard) -> u64 {
        ZobristTableStruct::zobrist_hash(&self.zobrist_hash, chess_board)
    }
    pub fn insert_into_table(&mut self,hash: u64, value: i32) {
        self.hash_map.insert(hash,value);
    }

    pub fn get_from_table(&self,hash: u64 ) -> Option<i32> {
        self.hash_map.get(&hash).copied()
    }



}