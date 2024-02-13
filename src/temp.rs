

const BOARD_SIZE: usize = 8;


const INITIAL_BOARD: [[(PieceType, Color); 8]; 8] = [
    [(PieceType::None, Color::None), (PieceType::None, Color::Black), (PieceType::None, Color::Black), (PieceType::None, Color::Black), (PieceType::King, Color::Black), (PieceType::None, Color::None), (PieceType::None, Color::None), (PieceType::None, Color::None)],
    [(PieceType::None, Color::None); 8],
    [(PieceType::None, Color::None); 8],
    [(PieceType::None, Color::None); 8],
    [(PieceType::None, Color::None); 8],
    [(PieceType::None, Color::None); 8],
    [(PieceType::None, Color::None); 8],
    [(PieceType::Rook, Color::White), (PieceType::Knight, Color::White), (PieceType::Bishop, Color::White), (PieceType::Queen, Color::White), (PieceType::King, Color::White), (PieceType::Bishop, Color::White), (PieceType::Knight, Color::White), (PieceType::Rook, Color::White)],
];
#[derive(Copy, Clone,PartialEq,Debug)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    None
}
#[derive(Copy, Clone,Debug,PartialEq)]
enum Color {
    Black,
    White,
    None
}
#[derive(Copy, Clone,Debug,PartialEq)]
struct Position {
    row: usize,
    col: usize,
}
#[derive(Copy, Clone,Debug)]
struct Piece {
    piece_type: PieceType,
    position: Position,
    color : Color,
    has_moved : bool,
}
#[derive(Clone,Debug,PartialEq)]
enum ChessMove {
    OrdinaryMove { from: Position, to: Position },
    Castle { king: Position, rook: Position },
    // Vous pouvez ajouter d'autres types de mouvements si nécessaire
}
#[derive(PartialEq)]
enum PartyState {
    Check,
    CheckMate,
    Pat,
    None
}
#[derive(Clone,Debug)]
pub struct ChessBoard {
    board: Vec<Vec<Option<Piece>>>,
    black_pieces : Vec<PieceType>,
    white_pieces : Vec<PieceType>,
    turn: Color,
}
#[derive(Clone,Copy)]
struct MinimaxInfo {
    depth : usize,
    max_depth : usize,
    color : Color,
    alpha : i32,
    beta : i32,
}




impl ChessBoard {
    pub fn new() -> Self {
        let mut black_pieces = vec![];
        let mut white_pieces = vec![];
        let mut board = vec![vec![None; 8]; 8];
        for (i, row) in INITIAL_BOARD.iter().enumerate() {
            for (j,&(piece_type, color)) in row.iter().enumerate() {
                if piece_type != PieceType::None {
                    let position = Position {
                        row: i,
                        col: j,
                    };
                    board[i][j] = Some(Piece {
                        piece_type,
                        position,
                        color,
                        has_moved: false,
                    });
                    match color {
                        Color::Black => black_pieces.push(piece_type),
                        Color::White => white_pieces.push(piece_type),
                        _ => (),
                    };
                }
            }
        }
        let turn = Color::White;
        ChessBoard {board,turn,black_pieces,white_pieces}
    }
}

macro_rules! is_move_possible {
    ($x:expr, $y:expr) => {
        if $x <= 7 && $x >= 0 && $y <= 7 && $y >= 0 {
            true
        }
        else{
            false
        }
    };
}


fn is_any_piece(chess_board: &ChessBoard, position: &Position) -> Option<Piece> {
    match chess_board.board[position.row][position.col] {
        Some(piece) => Some(piece),
        None => None,
    }

}


fn get_ordinary_moves(chess_board: &ChessBoard, piece: Piece, is_checking : bool) ->  Vec<ChessMove>  {
    let mut chess_moves = vec![];
    
    let directions: Vec<(i32, i32)> = match piece.piece_type {
        PieceType::Pawn => {

            let forward = if piece.color == Color::Black { 1 } else { -1 };

            let one_step = Position { row: (piece.position.row as i32 + forward) as usize, col: piece.position.col };
            if is_move_possible!(one_step.row,one_step.col) && is_any_piece(chess_board, &one_step).is_none() {
                chess_moves.push(ChessMove::OrdinaryMove { from: piece.position, to: one_step });

                // Mouvement initial de deux cases du pion
                if piece.has_moved == false {
                    let two_steps = Position { row: (piece.position.row as i32 + 2 * forward) as usize, col: piece.position.col };
                    if is_any_piece(chess_board, &two_steps).is_none() == false {
                        chess_moves.push(ChessMove::OrdinaryMove { from: piece.position, to: two_steps });
                    }
                }
            }

            for &offset in [-1, 1].iter() {
                let capture_pos = Position { row: (piece.position.row as i32 + forward) as usize, col: (piece.position.col as i32 + offset) as usize };
                if is_move_possible!(capture_pos.row, capture_pos.col) {
                    if let Some(target_piece) = chess_board.board[capture_pos.row][capture_pos.col] {
                        if target_piece.color != piece.color && target_piece.piece_type != PieceType::King && is_checking == true {
                            chess_moves.push(ChessMove::OrdinaryMove { from: piece.position, to: capture_pos });
                        }
                    }
                }
            }
            vec![]
        },
        PieceType::Rook => vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
        PieceType::Bishop => vec![(1, 1), (1, -1), (-1, -1), (-1, 1)],
        PieceType::Knight => vec![(2, 1), (2, -1), (-2, 1), (-2, -1), (1, 2), (1, -2), (-1, 2), (-1, -2)],
        PieceType::Queen => vec![(0, 1), (0, -1), (1, 0), (-1, 0),(1, 1), (1, -1), (-1, -1), (-1, 1)],
        PieceType::King => vec![  (1, 0),(0, 1),(-1, 1),(-1, 0), (-1, -1),(0, -1),(1, -1)],
        _ => vec![],
    };
    if piece.piece_type != PieceType::Pawn {
        let is_single_move_piece = piece.piece_type == PieceType::Knight || piece.piece_type == PieceType::King;
        for (dx, dy) in directions.iter() {
            let mut x = piece.position.row as i32;
            let mut y = piece.position.col as i32;
            
            loop {
                if is_move_possible!(x+dx,y+dy) {
                    let row = (x+dx) as usize;
                    let col = (y+dy) as usize;
                    let position = Position {
                        row,
                        col
                    };

                    if  let Some(result) = is_any_piece(&chess_board, &position) {
                        if result.color != piece.color { //I can eat a piece
                                match result.piece_type {
                                    PieceType::King => { 
                                        if is_checking == true { 
                                            chess_moves.push(ChessMove::OrdinaryMove { from: piece.position , to: position })
                                         };
                                    },
                                    _ => chess_moves.push(ChessMove::OrdinaryMove { from: piece.position , to: position }),
                                }
                        }
                        break;
                    }
                    else {
                        chess_moves.push(ChessMove::OrdinaryMove { from: piece.position , to: position });
                    }
                    x += dx;
                    y += dy;

                    if is_single_move_piece {
                        break;
                    }
                }
                else{
                    break;
                }

            }
        }
    }
    
    chess_moves

}

fn get_castle_moves(chess_board: &ChessBoard,king: Piece) -> Vec<ChessMove> {
    let row :usize;
    let mut chess_moves: Vec<ChessMove> = vec![];
    let mut is_col_empty : bool = true;
    if king.color == Color::Black { row = 0; }
    else{ row = 7; }

    if !is_king_in_check(chess_board,chess_board.turn) {
        if let Some(piece) = chess_board.board[row][0] {
            let start = piece.position.col + 1;
            let end = king.position.col;
        
            // Vérifiez chaque case entre le roi et la tour
            for col in start..end {
                if chess_board.board[king.position.row][col].is_some() {
                    is_col_empty = false; // Trouvé une pièce, donc les cases ne sont pas vides
                    break;
                }
            }
            if is_col_empty == true {
                if piece.piece_type == PieceType::Rook && !piece.has_moved {
                    chess_moves.push(ChessMove::Castle { king: king.position, rook: piece.position });

                }
            }
        }

        is_col_empty = true;


        if let Some(piece) = chess_board.board[row][6] {
            let end = piece.position.col - 1;
            let start = king.position.col;
        
            // Vérifiez chaque case entre le roi et la tour
            for col in start..end {
                if chess_board.board[king.position.row][col].is_some() {
                    is_col_empty = false; // Trouvé une pièce, donc les cases ne sont pas vides
                    break;
                }
            }
            if is_col_empty == true {
                if piece.piece_type == PieceType::Rook && !piece.has_moved {
                    chess_moves.push(ChessMove::Castle { king: king.position, rook: piece.position });

                }
            }
        }
    }
    else {
        return chess_moves;
    }


    chess_moves


}
fn make_a_move(chess_board: &mut ChessBoard,chess_move : ChessMove) {

   // if moves.contains(&position) {
    match chess_move {
        ChessMove::Castle { king,rook } => {
            let new_rook_col;
            let new_king_col;
            if let Some(mut king_piece) = chess_board.board[king.row][king.col] {
                if let Some(mut rook_piece) = chess_board.board[rook.row][rook.col] {
                    king_piece.has_moved = true;
                    rook_piece.has_moved = true;
                    if rook.col == 6 {
                        new_rook_col = rook.col - 2;
                        new_king_col =  king.col + 2;
                    }
                    else {
                        new_rook_col = rook.col + 3;
                        new_king_col =  king.col - 2;
                        
                    }
                    rook_piece.position.col = new_rook_col;
                    chess_board.board[rook.row][new_rook_col] = Some(rook_piece);
                    chess_board.board[rook.row][rook.col] = None;
                    king_piece.position.col = new_king_col;
                    chess_board.board[king.row][new_king_col] = Some(king_piece);      
                    chess_board.board[king.row][king.col] = None;  
            }
                else {
                    panic!("Rook not found");
                }
            }
            else {
                panic!("King not found");
            }
        },
        ChessMove::OrdinaryMove { from, to } => {
            if  let Some(mut piece) =  chess_board.board[from.row][from.col] {
                if  let Some(piece) =  chess_board.board[to.row][to.col] {
                    match piece.color {
                        Color::Black => {
                            if let Some(index) = chess_board.black_pieces.iter().position(|&p| p == piece.piece_type) {
                                chess_board.black_pieces.remove(index);
                            }
                        },
                        Color::White => {
                            if let Some(index) = chess_board.white_pieces.iter().position(|&p| p == piece.piece_type) {
                                chess_board.white_pieces.remove(index);
                            }
                        },
                        _ => (),
                    }
                }
                piece.has_moved = true;
                piece.position = to;
                chess_board.board[piece.position.row][piece.position.col] = Some(piece);
                chess_board.board[from.row][from.col] = None;
            }
            else{
                panic!("piece not found !");
            }

        }
    }
    if chess_board.turn == Color::White { chess_board.turn = Color::Black;} else { chess_board.turn = Color::White;}

}

fn is_king_in_check(chess_board: &ChessBoard,color : Color) -> bool {
    let mut is_check : bool = false;
    let opposite_color:Color;

    if chess_board.turn == Color::Black { opposite_color = Color::White;}
    else { opposite_color = Color::Black; }

    let opposite_pieces: Vec<Piece> = chess_board.board.iter()
    .flat_map(|col| col.iter())
    .flatten()
    .filter(|piece| piece.color == opposite_color)
    .cloned()
    .collect();

    let king = chess_board.board.iter()
    .flat_map(|col| col.iter())
    .flatten()
    .find(|piece| piece.piece_type == PieceType::King && piece.color == color)
    .expect("the king must be on the board");
    for piece in opposite_pieces.iter() {
        let chess_moves = get_ordinary_moves(&chess_board,*piece,true);
        for m in chess_moves.iter() {
            match m {
                ChessMove::OrdinaryMove { from: _, to } => {
                    if to.row == king.position.row && to.col == king.position.col  {
                        is_check = true;
                        break;
                    }
                }
                ChessMove::Castle { king : _ , rook }  => {
                    if rook.row == king.position.row && rook.col == king.position.col  {
                        is_check = true;
                        break;
                    }                  
                    
                }
            }

        }
    }
    is_check
}

fn get_party_state(chess_board: &ChessBoard, king: Piece) -> (PartyState,Vec<ChessMove>) {

    //let chess_moves : Vec<ChessMove>;
    
    let mut party_state = PartyState::None;
    let is_check : bool = is_king_in_check(&chess_board,chess_board.turn);
    if is_check == false {
        party_state = PartyState::None;
        return (party_state,vec![]);
    }
    else {
        println!("the king is check");
        let friendly_pieces: Vec<Piece> = chess_board.board.iter()
        .flat_map(|col| col.iter())
        .flatten()
        .filter(|piece| piece.color == king.color)
        .cloned()
        .collect();
        let mut safe_moves = vec![];
        
        for piece in friendly_pieces.iter() {
            let chess_moves = get_ordinary_moves(&chess_board,*piece,false);
            for m in chess_moves {
                println!("inside last loop");
                match m {
                    ChessMove::OrdinaryMove { from: _, to : _ } => {
                        
                        let mut chess_board_clone = chess_board.clone();
                        make_a_move(&mut chess_board_clone, m.clone());
                        if !is_king_in_check(&chess_board_clone,chess_board.turn) {
                            safe_moves.push(m);
                            println!("push save moves");
                        }
                    }
                    _ => {
                        continue;
                    }
                }

            }
        }
        if safe_moves.len() == 0  {
            party_state = PartyState::CheckMate;
        }
        (party_state,safe_moves)
    }



    
}

fn get_all_moves(chess_board: &ChessBoard) -> Vec<ChessMove> {
    let mut chess_moves = vec![];

    let king = chess_board.board.iter()
    .flat_map(|col| col.iter())
    .flatten()
    .find(|piece| piece.piece_type == PieceType::King && piece.color == chess_board.turn)
    .expect("the king must be on the board");
    let (party_state,possible_moves) = get_party_state(chess_board,*king);

    if party_state == PartyState::Check {
        return possible_moves;
    }
    else if party_state == PartyState::CheckMate {
        println!("[get_all_moves] checkMate");
        return vec![];
    }
    let pieces: Vec<Piece> = chess_board.board.iter()
    .flat_map(|col| col.iter())
    .flatten()
    .filter(|piece| piece.color == chess_board.turn)
    .cloned()
    .collect();

    for piece in pieces {
        match piece.piece_type {
            PieceType::King => {
                let moves = get_ordinary_moves(chess_board, piece,false);
                for m in moves.iter() {
                    let mut chess_board_clone = chess_board.clone();
                    make_a_move(&mut chess_board_clone, m.clone());
                    if is_king_in_check(&chess_board_clone,chess_board.turn) == false {
                        chess_moves.push(m.clone());
                    }
                }

                let castle_moves = get_castle_moves(chess_board, piece);
                for castle_move in castle_moves.iter() {
                    let mut chess_board_clone = chess_board.clone();
                    make_a_move(&mut chess_board_clone, castle_move.clone());
                    if is_king_in_check(&chess_board_clone,chess_board.turn) == false {
                        chess_moves.push(castle_move.clone());
                    }
                }
            }
            _ => {
                let moves = get_ordinary_moves(&chess_board,piece,false);
                chess_moves.extend(moves);
            }
        }
    }
    chess_moves

}


fn start_minimax(chess_board: &ChessBoard) -> Option<ChessMove>{
    let (tx, rx) = mpsc::channel();
    let mut threads = vec![];
    let all_moves = get_all_moves(chess_board);
    println!("all moves : {:?}",all_moves);
    let chunk_size = all_moves.len(); 
    let minimax_info = MinimaxInfo {
        max_depth : 2,
        color : chess_board.turn,
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
    if chess_board.turn == minimax_info.color {
        
        best_score = i32::MIN;
        for chess_move in chess_moves.iter() {
            
            let mut chess_board_clone = chess_board.clone();
            make_a_move(&mut chess_board_clone,chess_move.clone());
            let new_all_moves = get_all_moves(&chess_board_clone);
            
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
            make_a_move(&mut chess_board_clone,chess_move.clone());
            let new_all_moves = get_all_moves(&chess_board_clone);
            let (score , _ ) = minimax(chess_board_clone,new_all_moves,minimax_info);
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
    let king = chess_board.board.iter()
        .flat_map(|col| col.iter())
        .flatten()
        .find(|piece| piece.piece_type == PieceType::King && piece.color == chess_board.turn)
        .expect("the king must be on the board");
    let mut party_state = get_party_state(&chess_board, *king).0;
    if get_all_moves(&chess_board).len() == 0 {
        if party_state != PartyState::CheckMate {
            party_state = PartyState::Pat;
        }
    }
    match party_state {
        PartyState::CheckMate => {
            println!("found check mate");
            if minimax_info.color == chess_board.turn {
                i32::MIN
            } else {

                i32::MAX
            }
        },
        PartyState::Pat => {
            if minimax_info.color == chess_board.turn {
                -10
            } else {
                10
            }
        },
        _ => {
            let mut white_points : i32 = 0;
            let mut black_points : i32 = 0;
            for piece in chess_board.white_pieces {
                match piece {
                    PieceType::Rook => white_points+=5,
                    PieceType::Bishop => white_points+=3,
                    PieceType::Knight => white_points+=3,
                    PieceType::Queen => white_points+=9,
                    PieceType::Pawn =>  white_points+= 1,
                    _ => (),
                }
            }
            for piece in chess_board.black_pieces {
                match piece {
                    PieceType::Rook => black_points+=5,
                    PieceType::Bishop => black_points+=3,
                    PieceType::Knight => black_points+=3,
                    PieceType::Queen => black_points+=9,
                    PieceType::Pawn =>  black_points+= 1,
                    _ => (),
                }
            }
            if minimax_info.color == Color::White {
                white_points - black_points
            } else {
                black_points - white_points
            }
        }
    }
}

fn print_board(chess_board: &ChessBoard) {
    for col in &chess_board.board {
        for piece_option in col {
            let piece_symbol = match piece_option {
                Some(piece) => match (piece.piece_type, piece.color) {
                    (PieceType::Pawn, Color::White) => "P",
                    (PieceType::Pawn, Color::Black) => "p",
                    (PieceType::Knight, Color::White) => "N",
                    (PieceType::Knight, Color::Black) => "n",
                    (PieceType::Bishop, Color::White) => "B",
                    (PieceType::Bishop, Color::Black) => "b",
                    (PieceType::Rook, Color::White) => "R",
                    (PieceType::Rook, Color::Black) => "r",
                    (PieceType::Queen, Color::White) => "Q",
                    (PieceType::Queen, Color::Black) => "q",
                    (PieceType::King, Color::White) => "K",
                    (PieceType::King, Color::Black) => "k",
                    _ => " ",
                },
                None => ".",
            };
            print!("{} ", piece_symbol);
        }
        println!();
    }
}

use std::sync::mpsc;
use std::{thread,time};
fn main (){
    let mut chess_board = ChessBoard::new();
    let five_sec = time::Duration::from_millis(5000);
    loop {

        let computer_move = start_minimax(&chess_board).expect("should be a move");
        println!("the White bot choose : {:?}",computer_move);
        make_a_move(&mut chess_board, computer_move);
        print_board(&chess_board);
        
        thread::sleep(five_sec);
        let computer_move = start_minimax(&chess_board).expect("should be a move");
        println!("the Black bot choose : {:?}",computer_move);
        make_a_move(&mut chess_board, computer_move);

        print_board(&chess_board);
        thread::sleep(five_sec);

        let all_moves_len = get_all_moves(&chess_board).len();
        if  all_moves_len == 0 {
            break;
        }
    }

}