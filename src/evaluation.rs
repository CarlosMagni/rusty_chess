


macro_rules! flip {
    ($sq : expr) => {
        sq^56
    };
}

const BOARD_START: usize = 0;
const BOARD_END: usize = 7;

#[rustfmt::skip]
const MG_PAWN_TABLE: [[i32; 8]; 8] = [
  [  0,   0,   0,   0,   0,   0,  0,   0],
  [ 98, 134,  61,  95,  68, 126, 34, -11],
  [ -6,   7,  26,  31,  65,  56, 25, -20],
  [-14,  13,   6,  21,  23,  12, 17, -23],
  [-27,  -2,  -5,  12,  17,   6, 10, -25],
  [-26,  -4,  -4, -10,   3,   3, 33, -12],
  [-35,  -1, -20, -23, -15,  24, 38, -22],
  [  0,   0,   0,   0,   0,   0,  0,   0]
];

#[rustfmt::skip]
const EG_PAWN_TABLE: [[i32; 8]; 8] = [
  [  0,   0,   0,   0,   0,   0,   0,   0],
  [178, 173, 158, 134, 147, 132, 165, 187],
  [ 94, 100,  85,  67,  56,  53,  82,  84],
  [ 32,  24,  13,   5,  -2,   4,  17,  17],
  [ 13,   9,  -3,  -7,  -7,  -8,   3,  -1],
  [  4,   7,  -6,   1,   0,  -5,  -1,  -8],
  [ 13,   8,   8,  10,  13,   0,   2,  -7],
  [  0,   0,   0,   0,   0,   0,   0,   0],
];

#[rustfmt::skip]
const MG_KNIGHT_TABLE: [[i32; 8]; 8] = [
  [-167, -89, -34, -49,  61, -97, -15, -107],
  [ -73, -41,  72,  36,  23,  62,   7,  -17],
  [ -47,  60,  37,  65,  84, 129,  73,   44],
  [  -9,  17,  19,  53,  37,  69,  18,   22],
  [ -13,   4,  16,  13,  28,  19,  21,   -8],
  [ -23,  -9,  12,  10,  19,  17,  25,  -16],
  [ -29, -53, -12,  -3,  -1,  18, -14,  -19],
  [-105, -21, -58, -33, -17, -28, -19,  -23],
];

#[rustfmt::skip]
const EG_KNIGHT_TABLE: [[i32; 8]; 8] = [
  [-58, -38, -13, -28, -31, -27, -63, -99],
  [-25,  -8, -25,  -2,  -9, -25, -24, -52],
  [-24, -20,  10,   9,  -1,  -9, -19, -41],
  [-17,   3,  22,  22,  22,  11,   8, -18],
  [-18,  -6,  16,  25,  16,  17,   4, -18],
  [-23,  -3,  -1,  15,  10,  -3, -20, -22],
  [-42, -20, -10,  -5,  -2, -20, -23, -44],
  [-29, -51, -23, -15, -22, -18, -50, -64],
];

#[rustfmt::skip]
const MG_BISHOP_TABLE: [[i32; 8]; 8] = [
  [-29,   4, -82, -37, -25, -42,   7,  -8],
  [-26,  16, -18, -13,  30,  59,  18, -47],
  [-16,  37,  43,  40,  35,  50,  37,  -2],
  [ -4,   5,  19,  50,  37,  37,   7,  -2],
  [ -6,  13,  13,  26,  34,  12,  10,   4],
  [  0,  15,  15,  15,  14,  27,  18,  10],
  [  4,  15,  16,   0,   7,  21,  33,   1],
  [-33,  -3, -14, -21, -13, -12, -39, -21],
];

#[rustfmt::skip]
const EG_BISHOP_TABLE: [[i32; 8]; 8] = [
  [-14, -21, -11,  -8, -7,  -9, -17, -24],
  [ -8,  -4,   7, -12, -3, -13,  -4, -14],
  [  2,  -8,   0,  -1, -2,   6,   0,   4],
  [ -3,   9,  12,   9, 14,  10,   3,   2],
  [ -6,   3,  13,  19,  7,  10,  -3,  -9],
  [-12,  -3,   8,  10, 13,   3,  -7, -15],
  [-14, -18,  -7,  -1,  4,  -9, -15, -27],
  [-23,  -9, -23,  -5, -9, -16,  -5, -17],
];

#[rustfmt::skip]
const MG_ROOK_TABLE: [[i32; 8]; 8] = [
  [ 32,  42,  32,  51, 63,  9,  31,  43],
  [ 27,  32,  58,  62, 80, 67,  26,  44],
  [ -5,  19,  26,  36, 17, 45,  61,  16],
  [-24, -11,   7,  26, 24, 35,  -8, -20],
  [-36, -26, -12,  -1,  9, -7,   6, -23],
  [-45, -25, -16, -17,  3,  0,  -5, -33],
  [-44, -16, -20,  -9, -1, 11,  -6, -71],
  [-19, -13,   1,  17, 16,  7, -37, -26],
];

#[rustfmt::skip]
const EG_ROOK_TABLE: [[i32; 8]; 8] = [
  [13, 10, 18, 15, 12,  12,   8,   5],
  [11, 13, 13, 11, -3,   3,   8,   3],
  [ 7,  7,  7,  5,  4,  -3,  -5,  -3],
  [ 4,  3, 13,  1,  2,   1,  -1,   2],
  [ 3,  5,  8,  4, -5,  -6,  -8, -11],
  [-4,  0, -5, -1, -7, -12,  -8, -16],
  [-6, -6,  0,  2, -9,  -9, -11,  -3],
  [-9,  2,  3, -1, -5, -13,   4, -20],
];

#[rustfmt::skip]
const MG_QUEEN_TABLE: [[i32; 8]; 8] = [
  [-28,   0,  29,  12,  59,  44,  43,  45],
  [-24, -39,  -5,   1, -16,  57,  28,  54],
  [-13, -17,   7,   8,  29,  56,  47,  57],
  [-27, -27, -16, -16,  -1,  17,  -2,   1],
  [ -9, -26,  -9, -10,  -2,  -4,   3,  -3],
  [-14,   2, -11,  -2,  -5,   2,  14,   5],
  [-35,  -8,  11,   2,   8,  15,  -3,   1],
  [ -1, -18,  -9,  10, -15, -25, -31, -50],
];

#[rustfmt::skip]
const EG_QUEEN_TABLE: [[i32; 8]; 8] = [
  [ -9,  22,  22,  27,  27,  19,  10,  20],
  [-17,  20,  32,  41,  58,  25,  30,   0],
  [-20,   6,   9,  49,  47,  35,  19,   9],
  [  3,  22,  24,  45,  57,  40,  57,  36],
  [-18,  28,  19,  47,  31,  34,  39,  23],
  [-16, -27,  15,   6,   9,  17,  10,   5],
  [-22, -23, -30, -16, -16, -23, -36, -32],
  [-33, -28, -22, -43,  -5, -32, -20, -41],
];

#[rustfmt::skip]
const MG_KING_TABLE: [[i32; 8]; 8] = [
  [-65,  23,  16, -15, -56, -34,   2,  13],
  [ 29,  -1, -20,  -7,  -8,  -4, -38, -29],
  [ -9,  24,   2, -16, -20,   6,  22, -22],
  [-17, -20, -12, -27, -30, -25, -14, -36],
  [-49,  -1, -27, -39, -46, -44, -33, -51],
  [-14, -14, -22, -46, -44, -30, -15, -27],
  [  1,   7,  -8, -64, -43, -16,   9,   8],
  [-15,  36,  12, -54,   8, -28,  24,  14],
];

#[rustfmt::skip]
const EG_KING_TABLE: [[i32; 8]; 8] = [
  [-74, -35, -18, -18, -11,  15,   4, -17],
  [-12,  17,  14,  17,  17,  38,  23,  11],
  [ 10,  17,  23,  15,  20,  45,  44,  13],
  [ -8,  22,  24,  27,  26,  33,  26,   3],
  [-18,  -4,  21,  24,  27,  23,   9, -11],
  [-19,  -3,  11,  21,  23,  16,   7,  -9],
  [-27, -11,   4,  13,  14,   4,  -5, -17],
  [-53, -34, -21, -11, -28, -14, -24, -43]
];


fn mg_table(piece: PieceType) -> &'static [[i32; 8]; 8] {
    match piece {
        PieceType::Pawn => &MG_PAWN_TABLE,
        PieceType::Bishop => &MG_BISHOP_TABLE,
        PieceType::Knight => &MG_KNIGHT_TABLE,
        PieceType::Rook => &MG_ROOK_TABLE,
        PieceType::King => &MG_KING_TABLE,
        PieceType::Queen => &MG_QUEEN_TABLE,
        PieceType::None => todo!(),
    }
}

fn eg_table(piece: PieceType) -> &'static [[i32; 8]; 8] {
    match piece {
        PieceType::Pawn => &EG_PAWN_TABLE,
        PieceType::Bishop => &EG_BISHOP_TABLE,
        PieceType::Knight => &EG_KNIGHT_TABLE,
        PieceType::Rook => &EG_ROOK_TABLE,
        PieceType::King => &EG_KING_TABLE,
        PieceType::Queen => &EG_QUEEN_TABLE,
        PieceType::None => todo!(),
    }
}
const GAMEPHASE_INC: [i32; 12] = [
    0,
    0,
    1,
    1,
    1,
    1,
    2,
    2,
    4,
    4,
    0,
    0
];


use crate::chess_board::{ChessBoard, Color, PartyState, PieceType};
use crate::minimax::Minimax;

fn mg_piece_val(piece: PieceType)  -> i32 {
    match piece {
        PieceType::Pawn => 82,
        PieceType::Knight => 337,
        PieceType::Bishop => 365,
        PieceType::Rook => 477,
        PieceType::Queen => 1025,
        PieceType::King => 0,
        _ => 0,
    }
}

fn eg_piece_val(piece: PieceType)  -> i32 {
    match piece {
        PieceType::Pawn => 94,
        PieceType::Knight => 281,
        PieceType::Bishop => 297,
        PieceType::Rook => 512,
        PieceType::Queen => 936,
        PieceType::King => 0,
        _ => 0,
    }
}

fn game_phase_val(piece: PieceType) -> i32 {
    match piece {
        PieceType::Pawn => 0,
        PieceType::Knight => 1,
        PieceType::Bishop => 1,
        PieceType::Rook => 2,
        PieceType::Queen => 4,
        PieceType::King => 0,
        _ => 0,
    }
}
pub fn eval_function(minimax: &Minimax, chess_board: ChessBoard) -> i32 {
    let mut white_mg = 0;
    let mut black_mg = 0;
    let mut white_eg = 0;
    let mut black_eg = 0;
    let mut game_phase = 0;

    for row in BOARD_START..BOARD_END {
        for col in BOARD_START..BOARD_END {
            if let Some(piece) = chess_board.board[row][col] {
                game_phase += game_phase_val(piece.piece_type);
                match piece.color {
                    Color::White => { 
                        white_mg += mg_table(piece.piece_type)[row][col] + mg_piece_val(piece.piece_type);
                        white_eg += eg_table(piece.piece_type)[row][col] + eg_piece_val(piece.piece_type);
                    }
                    Color::Black => {
                        black_mg += mg_table(piece.piece_type)[7 - row][col] + mg_piece_val(piece.piece_type);
                        black_eg += eg_table(piece.piece_type)[7 - row][col] + eg_piece_val(piece.piece_type);
                    },
                    Color::None => panic!("color cannot be none"),
                }
            }
        }
    }

    let mg_score;
    let eg_score;
    if minimax.color == Color::White {
        mg_score = white_mg - black_mg;
        eg_score = white_eg - black_eg;
    } else {
        mg_score = black_mg - white_mg;
        eg_score = black_eg - white_eg;
    }

    let mut mg_phase = game_phase;

    /* in case of early promotion */
    if mg_phase > 24 {
        mg_phase = 24;
    }
    let eg_phase = 24 - mg_phase;
    (mg_score * mg_phase + eg_score * eg_phase) / 24
}



pub fn eval_function_old(minimax: &Minimax, chess_board: ChessBoard) -> i32 {
    let mut evaluation : i32 = 0;
    let party_state =  ChessBoard::get_party_state(&chess_board);

    match party_state {
        PartyState::CheckMate => {
            if minimax.color == ChessBoard::get_turn(&chess_board) {
                evaluation = i32::MIN;
            } else {

                evaluation = i32::MAX;
            }
        },
        PartyState::Pat => {
            evaluation = 10;

        },
        _ => {
            let mut friend_points : i32 = 0;
            let mut ennemy_points : i32 = 0;
            for piece in ChessBoard::get_ennemy_pieces_by_color(&chess_board,minimax.color) {
                match piece.piece_type {
                    PieceType::Rook => ennemy_points+=500,
                    PieceType::Bishop => ennemy_points+=330,
                    PieceType::Knight => ennemy_points+=320,
                    PieceType::Queen => ennemy_points+=900,
                    PieceType::Pawn =>  ennemy_points+=100,
                    _ => (),
                }
            }
            for piece in ChessBoard::get_friendly_pieces_by_color(&chess_board,minimax.color) {
                match piece.piece_type {
                    PieceType::Rook => friend_points+=500,
                    PieceType::Bishop => friend_points+=330,
                    PieceType::Knight => friend_points+=320,
                    PieceType::Queen => friend_points+=900,
                    PieceType::Pawn =>  friend_points+= 100,
                    _ => (),
                }
            }
            evaluation =  friend_points - ennemy_points;
        }
    }

   // transposition_table.insert(hash,evaluation);
    evaluation
    
}