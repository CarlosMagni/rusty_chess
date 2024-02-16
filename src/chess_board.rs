const INITIAL_BOARD: [[(PieceType, Color); 8]; 8] = [
    [(PieceType::Rook, Color::White), (PieceType::Knight, Color::White), (PieceType::Bishop, Color::White), (PieceType::Queen, Color::White), (PieceType::King, Color::White), (PieceType::Bishop, Color::White), (PieceType::Knight, Color::White), (PieceType::Rook, Color::White)],
    [(PieceType::Pawn, Color::White); 8],
    [(PieceType::None, Color::None); 8],
    [(PieceType::None, Color::None); 8],
    [(PieceType::None, Color::None); 8],
    [(PieceType::None, Color::None); 8],
    [(PieceType::Pawn, Color::Black); 8],
    [(PieceType::Rook, Color::Black), (PieceType::Knight, Color::Black), (PieceType::Bishop, Color::Black), (PieceType::Queen, Color::Black), (PieceType::King, Color::Black), (PieceType::Bishop, Color::Black), (PieceType::Knight, Color::Black), (PieceType::Rook, Color::Black)],
];


#[derive(Copy, Clone,PartialEq,Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    None
}
#[derive(Copy, Clone,Debug,PartialEq)]
pub enum Color {
    Black,
    White,
    None
}
#[derive(Copy, Clone,Debug,PartialEq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}
#[derive(Copy, Clone,Debug,PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    position: Position,
    pub color : Color,
    has_moved : bool,
}
#[derive(Clone,Debug,PartialEq)]
pub enum ChessMove {
    OrdinaryMove { from: Position, to: Position },
    Castle { king: Position, rook: Position },
    Promotion {from: Position, to: Position, piece_type: PieceType},
   // EnPassant {from: Position, to: Position, pawn_pos : Piece},
}
#[derive(PartialEq,Clone,Debug,Copy)]
pub enum PartyState {
    Check,
    CheckMate,
    Pat,
    None
}
#[derive(Clone,Debug)]
pub struct ChessBoard {
    pub board: Vec<Vec<Option<Piece>>>,
    black_positions : Vec<Position>,
    white_positions : Vec<Position>,
    last_move: Option<ChessMove>,
    party_state: PartyState,
    turn: Color,
}

macro_rules! is_move_possible {
    ($x:expr, $y:expr) => {
        if $x < 8 && $x >= 0 && $y < 8 && $y >= 0 {
            true
        }
        else{
            false
        }
    };
}

macro_rules! is_king_in_check {
    //is_king_in_check!(king.position,to,piece.piece_type,piece.color)
    ($king_pos:expr, $piece_pos:expr, $piece_type:expr, $piece_color:expr) => {{
        let mut check = false;
        
        match $piece_type {
            PieceType::Rook => {
                // Rook logic here
                check = ($king_pos.row == $piece_pos.row || $king_pos.col == $piece_pos.col);
            },
            PieceType::Bishop => {
                // Bishop logic here
                check = (i32::abs($king_pos.row as i32 - $piece_pos.row as i32) == i32::abs($king_pos.col as i32 - $piece_pos.col as i32));
            },
            PieceType::Queen => {
                // Queen combines Rook and Bishop logic
                check = ($king_pos.row == $piece_pos.row || $king_pos.col == $piece_pos.col) ||
                        (i32::abs($king_pos.row as i32 - $piece_pos.row as i32) == i32::abs($king_pos.col as i32 - $piece_pos.col as i32));
            },
            PieceType::Knight => {
                // Knight logic here
                check = [(2, 1), (1, 2), (-1, 2), (-2, 1), (-2, -1), (-1, -2), (1, -2), (2, -1)].iter().any(|&(dx, dy)| {
                    $king_pos.row as i32 + dx == $piece_pos.row as i32 && $king_pos.col as i32 + dy == $piece_pos.col as i32
                });
            },
            PieceType::King => {
                // King logic here (for completeness, normally king cannot put another king in check due to check rules)
                check = i32::abs($king_pos.row as i32 - $piece_pos.row as i32) <= 1 &&
                        i32::abs($king_pos.col as i32 - $piece_pos.col as i32) <= 1;
            },
            PieceType::Pawn => {
                // Pawn logic adapted from previous macro
                check = match $piece_color {
                    Color::White => {
                        ($king_pos.row as i32 == $piece_pos.row as i32 + 1) &&
                        (($king_pos.col as i32 == $piece_pos.col as i32 + 1) || ($king_pos.col as i32 == $piece_pos.col as i32 - 1))
                    },
                    Color::Black => {
                        ($king_pos.row as i32 == $piece_pos.row as i32 - 1) &&
                        (($king_pos.col as i32 == $piece_pos.col as i32 + 1) || ($king_pos.col as i32 == $piece_pos.col as i32 - 1))
                    },
                    _ => false,
                };
            },
            _ => {}
        }
        
        check
    }};
}



impl ChessBoard {
    pub fn new() -> Self {
        let mut black_positions = vec![];
        let mut white_positions = vec![];
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
                        Color::Black => black_positions.push(position),
                        Color::White => white_positions.push(position),
                        _ => (),
                    };
                }
            }
        }
        let turn = Color::White;
        ChessBoard {board,black_positions,white_positions,party_state: PartyState::None,turn,last_move : None}
    }

    pub fn update_board(&mut self, chess_move : ChessMove) -> Vec<ChessMove> {
        self.make_a_move(&chess_move);
        return self._update_party();
    }
    fn make_a_move(&mut self,chess_move : &ChessMove) {
        match chess_move {
            ChessMove::Castle { king,rook } => {
                let new_rook_col;
                let new_king_col;
                if let Some(mut king_piece) = self.board[king.row][king.col] {
                    if let Some(mut rook_piece) = self.board[rook.row][rook.col] {
                        king_piece.has_moved = true;
                        rook_piece.has_moved = true;
                        if rook.col == 7 {
                            new_rook_col = rook.col - 2;
                            new_king_col =  king.col + 2;
                        }
                        else {
                            new_rook_col = rook.col + 3;
                            new_king_col =  king.col - 2;
                            
                        }
                        self.remove_piece_position(&rook_piece);
                        rook_piece.position.col = new_rook_col;
                        self.add_piece_position(&rook_piece);
                        self.board[rook.row][new_rook_col] = Some(rook_piece);
                        self.board[rook.row][rook.col] = None;

                        self.remove_piece_position(&king_piece);
                        king_piece.position.col = new_king_col;
                        self.add_piece_position(&king_piece);
                        self.board[king.row][new_king_col] = Some(king_piece);      
                        self.board[king.row][king.col] = None;  
                }
                    else {
                        panic!("Rook not found during castle");
                    }
                }
                else {
                    panic!("King not found during castle");
                }
            },
            ChessMove::OrdinaryMove { from, to } => {
                if  let Some(mut piece) =  self.board[from.row][from.col] {
                    if  let Some(attacked_piece) =  self.board[to.row][to.col] {
                        self.remove_piece_position(&attacked_piece);
                    }
                    piece.has_moved = true;
                    self.remove_piece_position(&piece);
                    piece.position = *to;
                    self.add_piece_position(&piece);
                    self.board[to.row][to.col] = None;
                    self.board[to.row][to.col] = Some(piece);
                    self.board[from.row][from.col] = None;

                    

                }
                else{
                    panic!("piece not found during ordinary move !");
                }
    
            },
            ChessMove::Promotion { from, to, piece_type } => {
                if  let Some(mut piece) =  self.board[from.row][from.col] {
                    if  let Some(attacked_piece) =  self.board[to.row][to.col] {
                        self.remove_piece_position(&attacked_piece);
                    }
                    piece.has_moved = true;
                    self.remove_piece_position(&piece);
                    piece.position = *to;
                    piece.piece_type = *piece_type;
                    self.add_piece_position(&piece);
                    self.board[to.row][to.col] = None;
                    self.board[to.row][to.col] = Some(piece);
                    self.board[from.row][from.col] = None;

                    

                }
                else{
                    panic!("piece not found during promotion!");
                }
            }
        }
        self.last_move = Some(chess_move.clone());
        self.change_turn();
        
    }

    pub fn change_turn(&mut self) {
        if self.turn == Color::White { self.turn = Color::Black;} else { self.turn = Color::White;}
    }

    fn get_piece_from_pos(&self,p: &Position) -> Option<Piece> {
        self.board[p.row][p.col]
    }
    fn add_piece_position(&mut self, piece: &Piece) {
        match piece.color {
            Color::Black => self.black_positions.push(piece.position),
            Color::White => self.white_positions.push(piece.position),
            _ => (),
        }
    }
    fn remove_piece_position(&mut self,piece: &Piece) {
        match piece.color {
            Color::Black => {
                if let Some(index) = self.black_positions.iter().position(|&p| p == piece.position) {
                    self.black_positions.remove(index);
                }
                else {
                    panic!("cannot found piece position in remove for black");
                }
            },
            Color::White => {
                if let Some(index) = self.white_positions.iter().position(|&p| p == piece.position) {
                    self.white_positions.remove(index);
                }
                else {
                    panic!("cannot found piece position in remove for white");
                }
            },
            _ => (),
        }
    }

    pub fn get_turn(&self) -> Color {
        return self.turn
    }
    
    pub fn get_ennemy_pieces_by_color(&self,color: Color) -> Vec<Piece> {
        let mut pieces = vec![]; 
        match self.turn {
            Color::White => for pos in self.black_positions.iter() {
                pieces.push(self.board[pos.row][pos.col].unwrap());
            },
            Color::Black => for pos in self.white_positions.iter() {
                pieces.push(self.board[pos.row][pos.col].unwrap());
            },
            _ => panic!("turn error"),

        }
        pieces
    }
    pub fn get_friendly_pieces_by_color(&self,color: Color) -> Vec<Piece> {
        let mut pieces = vec![]; 
        match color {
            Color::White => for pos in self.white_positions.iter() {
                pieces.push(self.board[pos.row][pos.col].unwrap());
            },
            Color::Black => for pos in self.black_positions.iter() {
                pieces.push(self.board[pos.row][pos.col].unwrap());
            },
            _ => panic!("turn error"),

        }
        pieces
    }

    fn get_friendly_pieces(&self) -> Vec<Piece> {
        let mut pieces = vec![];
        match self.turn {
            Color::White => for pos in self.white_positions.iter() {
                pieces.push(self.board[pos.row][pos.col].unwrap());
            },
            Color::Black => for pos in self.black_positions.iter() {
                pieces.push(self.board[pos.row][pos.col].unwrap());
            },
            _ => panic!("turn error"),

        }
        pieces
    }
    fn get_ennemy_pieces(&self) -> Vec<Piece> {
        let mut pieces = vec![];
        match self.turn {
            Color::White => for pos in self.black_positions.iter() {
                pieces.push(self.board[pos.row][pos.col].unwrap());
            },
            Color::Black => for pos in self.white_positions.iter() {
                pieces.push(self.board[pos.row][pos.col].unwrap());
            },
            _ => panic!("turn error"),

        }
        pieces
    }

    fn get_castle_moves(&self,king: Piece) -> Vec<ChessMove> {
        let row :usize;
        let mut chess_moves: Vec<ChessMove> = vec![];
        let mut is_col_empty : bool = true;
        if king.has_moved == false {
            if let Some(piece) = self.board[king.position.row][0] {
                if piece.piece_type == PieceType::Rook && !piece.has_moved {
                    let start = piece.position.col  + 1 ;
                    let end = king.position.col - 1;
                    // Vérifiez chaque case entre le roi et la tour
                    for col in start..end {
                        if self.board[king.position.row][col].is_some() {
                            is_col_empty = false; // Trouvé une pièce, donc les cases ne sont pas vides
                            break;
                        }
                    }
                    if is_col_empty == true {
                            chess_moves.push(ChessMove::Castle { king: king.position, rook: piece.position });
                    }
                }
            }

            is_col_empty = true;


            if let Some(piece) = self.board[king.position.row][7] {
                if piece.piece_type == PieceType::Rook && !piece.has_moved {
                    let end = piece.position.col - 1;
                    let start = king.position.col + 1;
                
                    // Vérifiez chaque case entre le roi et la tour
                    for col in start..end {
                        if self.board[king.position.row][col].is_some() {
                            is_col_empty = false; // Trouvé une pièce, donc les cases ne sont pas vides
                            break;
                        }
                    }
                    if is_col_empty == true {
                            chess_moves.push(ChessMove::Castle { king: king.position, rook: piece.position });
                    }
                }
            }
        }

    
    
        //chess_moves
        vec![]
    
    }
    fn generate_promotion_moves(&self, from: Position, to: Position) -> Vec<ChessMove> {
        let piece_types = vec![PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight];
        piece_types.into_iter()
              .map(|piece_type| ChessMove::Promotion { from, to, piece_type })
              .collect()
    }
    

    fn get_other_moves(&self, piece: Piece, is_checking : bool) ->  Vec<ChessMove>  {
        let mut chess_moves = vec![];
        
        let directions: Vec<(i32, i32)> = match piece.piece_type {
            PieceType::Pawn => {
                let forward = if piece.color == Color::Black { -1 } else { 1 };
                let promotion_row = if piece.color == Color::Black { 0 } else { 7 };
                if is_checking == false {
        
                    let one_step = Position { row: (piece.position.row as i32 + forward) as usize, col: piece.position.col };
                    if is_move_possible!(one_step.row as i32,one_step.col as i32) &&  self.is_any_piece(&one_step).is_none() {

                        if one_step.row == promotion_row {
                            let moves = self.generate_promotion_moves( piece.position,one_step);
                            chess_moves.extend(moves); 
                        }
                        else {
                            chess_moves.push(ChessMove::OrdinaryMove { from: piece.position, to: one_step });
                        }

                        // Mouvement initial de deux cases du pion
                        if piece.has_moved == false {
                            let two_steps = Position { row: (piece.position.row as i32 + 2 * forward) as usize, col: piece.position.col };
                            if self.is_any_piece(&one_step).is_none() && self.is_any_piece(&two_steps).is_none() {
                                chess_moves.push(ChessMove::OrdinaryMove { from: piece.position, to: two_steps });
                            }
                        }


                        
                    }

                }
                for &offset in [-1, 1].iter() {
                    let capture_pos = Position { row: (piece.position.row as i32 + forward) as usize, col: (piece.position.col as i32 + offset) as usize };
                    if is_move_possible!(capture_pos.row as i32 , capture_pos.col as i32) {
                        if let Some(target_piece) = self.board[capture_pos.row][capture_pos.col] {
                            if target_piece.color != piece.color {
                                if is_checking == true {
                                    if target_piece.piece_type == PieceType::King {
                                        chess_moves.push(ChessMove::OrdinaryMove { from: piece.position, to: capture_pos });
                                    }
                                }
                                else {
                                    if  target_piece.piece_type != PieceType::King  {
                                        if capture_pos.row == promotion_row {
                                            let moves = self.generate_promotion_moves( piece.position,capture_pos);
                                            chess_moves.extend(moves); 
                                        }
                                        else {
                                            chess_moves.push(ChessMove::OrdinaryMove { from: piece.position, to: capture_pos });
                                        }     
                                    }
                                }
                            }
                        }
                        else if let Some(target_piece) = self.board[piece.position.row][((piece.position.col) as i32 +offset) as usize] { //en passant
                            if target_piece.color != piece.color {
                                match self.last_move {
                                    Some(ChessMove::OrdinaryMove { from, to }) => {
                                        match  target_piece.piece_type {
                                            PieceType::Pawn => {
                                                if target_piece.position.row == to.row &&  target_piece.position.col == to.col && i32::abs(from.row as i32 - to.row as i32) == 2 {
                                                    chess_moves.push(ChessMove::OrdinaryMove { from: piece.position, to: capture_pos });
                                                }
                                            },
                                            _ => (),
                                        }
                                    },
                                    _ => (),
                                }
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
    
                        if  let Some(result) =  self.is_any_piece(&position) {
                            
                            if result.color != piece.color { //I can eat a piece
                                    match result.piece_type {
                                        PieceType::King => { 
                                            if is_checking == true { 
                                                chess_moves.push(ChessMove::OrdinaryMove { from: piece.position , to: position })
                                            };
                                        },
                                        _ => {
                                            if is_checking == false {
                                                chess_moves.push(ChessMove::OrdinaryMove { from: piece.position , to: position })
                                            };
                                        },
                                    }
                            }
                            break;
                        }
                        else {
                            if is_checking == false {
                                chess_moves.push(ChessMove::OrdinaryMove { from: piece.position , to: position });
                            }
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
    
    fn is_any_piece(&self, position: &Position) -> Option<Piece> {
        match self.board[position.row][position.col] {
            Some(piece) => Some(piece),
            None => None,
        }
    
    }

    fn is_king_in_check(&self) -> bool {
    
        let opposite_pieces = self.get_ennemy_pieces();
    
        let king = self.get_king();


        for piece in opposite_pieces.iter() {
            let moves = self.get_other_moves(*piece,true);
            for m in moves.iter() {
                match piece.piece_type {
                    PieceType::King => (),
                    _ => {
                        match m {
                            ChessMove::OrdinaryMove { from: _, to } => {
                                if to.row == king.position.row && to.col == king.position.col {
                                    return true;
                                }
                            },
                            _ => (),
                        }

                    }
                }

            }

        }
        false
    }
    
    fn get_all_moves(&self) -> Vec<ChessMove> {
        let mut chess_moves = vec![];
        let pieces = self.get_friendly_pieces();
        for piece in pieces {
            match piece.piece_type {
                PieceType::King => {
                    let moves = self.get_other_moves(piece,false);
                    for m in moves.iter() {
                        let mut chess_board_clone = self.clone();
                        Self::make_a_move(&mut chess_board_clone, m);
                        Self::change_turn(&mut chess_board_clone);
                        if Self::is_king_in_check(&chess_board_clone) == false {
                            chess_moves.push(m.clone());
                        }
                    }
    
                    let castle_moves = self.get_castle_moves(piece);
                    for castle_move in castle_moves.iter() {
                        let mut chess_board_clone = self.clone();
                        Self::make_a_move(&mut chess_board_clone, castle_move);
                        Self::change_turn(&mut chess_board_clone);
                        if Self::is_king_in_check(&chess_board_clone) == false {
                            chess_moves.push(castle_move.clone());
                        }
                    }
                }
                _ => {
                    let moves = self.get_other_moves(piece,false);
                    chess_moves.extend(moves);
                }
            }
        }
        chess_moves
    
    }

    pub fn get_party_moves(&self) -> Vec<ChessMove>  {
        if self.is_king_in_check() {
            let safe_moves = self.find_safe_moves();
            return safe_moves
        }
        else {
            let moves = self.get_all_moves();

            return moves;
        }
    }


    fn _update_party(&mut self) -> Vec<ChessMove> {

  

        if self.is_king_in_check() {
            let safe_moves = self.find_safe_moves();
            if safe_moves.len() == 0 {
                self.change_party_state(PartyState::CheckMate);

            }
            else {
                self.change_party_state(PartyState::Check);
            }
            return safe_moves
        }
        else {
            let moves = self.get_all_moves();
            if moves.len() == 0 {
                self.change_party_state(PartyState::Pat);
            }
            return moves;

        }
    }

    fn change_party_state(&mut self,party_state: PartyState) {
        self.party_state = party_state;
    }

    fn find_safe_moves(&self) -> Vec<ChessMove> {
        let friendly_pieces = self.get_friendly_pieces();
        let mut safe_moves = vec![];
        for piece in friendly_pieces.iter() {
            let chess_moves =  self.get_other_moves(*piece,false);
            for m in chess_moves {
                match m {
                    ChessMove::OrdinaryMove { from: _, to: _ } => {

                        let mut chess_board_clone =  self.clone();
                        Self::make_a_move(&mut chess_board_clone, &m);
                        Self::change_turn(&mut chess_board_clone); // to check the right king
                        if !Self::is_king_in_check(&chess_board_clone) {
                            safe_moves.push(m);
                        }

                    }
                    _ => {
                        continue;
                    }
                }

            }
        }
        safe_moves
    }

    pub fn get_king(&self) -> Piece {
        let binding = self.get_friendly_pieces();
        let king = binding.iter().find(|piece| piece.piece_type == PieceType::King).ok_or_else(|| format!("the king must be on the board, {:?}", self.print_board()))
        .unwrap_or_else(|e| panic!("{}", e));
        *king
    }

    pub fn get_party_state(&self) -> PartyState {
        return self.party_state;
    }

    pub fn get_board(&self) -> Vec<Vec<Option<Piece>>> {
        return self.board.clone();
    }

    pub fn print_board(&self) {
        println!("  a b c d e f g h"); // En-tête pour les colonnes
        for (i, row) in self.board.iter().enumerate().rev() {
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

        // else {
        //     println!("the king is check");
        //     let friendly_pieces: Vec<Piece> =  self.board.iter()
        //     .flat_map(|col| col.iter())
        //     .flatten()
        //     .filter(|piece| piece.color == king.color)
        //     .cloned()
        //     .collect();
        //     let mut safe_moves = vec![];
            
        //     for piece in friendly_pieces.iter() {
        //         let chess_moves =  self.get_ordinary_moves(*piece,false);
        //         for m in chess_moves {
        //             println!("inside last loop");
        //             match m {
        //                 ChessMove::OrdinaryMove { from: _, to : _ } => {
                            
        //                     let mut chess_board_clone =  self.clone();
        //                     Self::make_a_move(&mut chess_board_clone, m.clone());
        //                     if !Self::is_king_in_check(&chess_board_clone, self.get_turn()) {
        //                         safe_moves.push(m);
        //                         println!("push save moves");
        //                     }
        //                 }
        //                 _ => {
        //                     continue;
        //                 }
        //             }
    
        //         }
        //     }
        //     if safe_moves.len() == 0  {
        //         party_state = PartyState::CheckMate;
        //     }
        //     (party_state,safe_moves)
        // }
    
    



}