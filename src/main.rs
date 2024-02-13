

mod chess_board;
mod minimax;


use std::{thread,time};
use std::io;
use chess_board::{Color, PieceType,Position};

use crate::chess_board::{ChessBoard, ChessMove};

fn main () {
    let five_sec = time::Duration::from_millis(2000);
    let mut chess_board = chess_board::ChessBoard::new();
    loop {
        print_board(&chess_board);

        println!("Au tour des Blancs. Entrez votre mouvement (format attendu : 'x y x_dest y_dest') : ");
        let (from_pos, to_pos) = read_user_input();
        let chess_move = ChessMove::OrdinaryMove  { from : from_pos, to : to_pos};
        ChessBoard::update_board(&mut chess_board, chess_move);

        print_board(&chess_board); // Assurez-vous que cette fonction existe et est correctement importée
        println!("turn : {:?}",ChessBoard::get_turn(&chess_board));
        thread::sleep(five_sec);
        let computer_move =  minimax::start_minimax(&chess_board).expect("should be a move");
        println!("the Black bot choose : {:?}",computer_move);
        ChessBoard::update_board(&mut chess_board, computer_move);
    }

    
}

fn print_board(chess_board: &ChessBoard) {
    println!("  a b c d e f g h"); // En-tête pour les colonnes
    for (i, row) in ChessBoard::get_board(&chess_board).iter().enumerate().rev() {
        print!("{} ", i+1); // Affiche le numéro de rangée avant chaque ligne, en commençant par 8
        for piece_option in row {
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
                    _ => ".",
                },
                None => ".",
            };
            print!("{} ", piece_symbol);
        }
        println!(" {}",i+1); // Affiche le numéro de rangée après chaque ligne
    }
    println!("  a b c d e f g h"); // Pied de page pour les colonnes
}

fn read_user_input() -> (Position, Position) {
    let mut input = String::new();
    println!("Enter your move (e.g., e2 e4):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let trimmed_input = input.trim().to_lowercase();
    let mut chars = trimmed_input.split_whitespace();

    let from_str = chars.next().unwrap_or("");
    let to_str = chars.next().unwrap_or("");

    let from_pos = parse_chess_position(from_str);
    let to_pos = parse_chess_position(to_str);

    (from_pos, to_pos)
}

fn parse_chess_position(pos: &str) -> Position {
    let mut chars = pos.chars();
    // Convertir la lettre de la colonne en indice (a=0, b=1, c=2, etc.)
    let col = chars.next().unwrap_or('a') as usize - 'a' as usize ;
    // Convertir le chiffre de la ligne en indice (1=7, 2=6, 3=5, etc.)
    // Notez que '1'.to_digit(10).unwrap() donne 1, donc on soustrait 1 pour obtenir un indice basé sur 0 et on inverse ensuite pour correspondre à la disposition du tableau d'échecs
    let row = chars.next().unwrap_or('1').to_digit(10).unwrap() as usize -1;
    println!("{} {}",row, col );
    Position { row, col }
}