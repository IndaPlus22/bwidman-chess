use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 */

#[derive(Copy, Clone)]
 pub struct Piece {
    color: Color,
    piece_type: PieceType,
}

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    active_color: Color,
    board: [Option<Piece>; 8*8],
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        use Color::*;
        use PieceType::*;

        let w_pawn = Some(Piece { color: White, piece_type: Pawn });
        let w_rook = Some(Piece { color: White, piece_type: Rook });
        let w_knight = Some(Piece { color: White, piece_type: Knight });
        let w_bishop = Some(Piece { color: White, piece_type: Bishop });
        let w_queen = Some(Piece { color: White, piece_type: Queen });
        let w_king = Some(Piece { color: White, piece_type: King });

        let b_pawn = Some(Piece { color: Black, piece_type: Pawn });
        let b_rook = Some(Piece { color: Black, piece_type: Rook });
        let b_knight = Some(Piece { color: Black, piece_type: Knight });
        let b_bishop = Some(Piece { color: Black, piece_type: Bishop });
        let b_queen = Some(Piece { color: Black, piece_type: Queen });
        let b_king = Some(Piece { color: Black, piece_type: King });

        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            active_color: White,
            board: [
                b_rook, b_knight, b_bishop, b_queen, b_king, b_bishop, b_knight, b_rook,
                b_pawn, b_pawn,   b_pawn,   b_pawn,  b_pawn, b_pawn,   b_pawn,   b_pawn,
                None,   None,     None,     None,    None,   None,     None,     None,
                None,   None,     None,     None,    None,   None,     None,     None,
                None,   None,     None,     None,    None,   None,     None,     None,
                None,   None,     None,     None,    None,   None,     None,     None,
                w_pawn, w_pawn,   w_pawn,   w_pawn,  w_pawn, w_pawn,   w_pawn,   w_pawn,
                w_rook, w_knight, w_bishop, w_queen, w_king, w_bishop, w_knight, w_rook,
            ],
        }
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, from: String, to: String) -> Option<GameState> {
        let possible_moves: Vec<String> = match self.get_possible_moves(&from) {
            Some(i) => i,
            None => {
                println!("There is no piece on that slot!");
                return None;
            },
        };

        let old_index = an_to_index(&from);
        let new_index = an_to_index(&to);
        
        // Player is trying to move an enemy piece
        if self.board[old_index].unwrap().color != self.active_color {
            println!("Cannot move enemy piece!");
            return None;
        }

        // If the suggested move isn't in the list of possible ones, it's illegal
        if !possible_moves.contains(&to) {
            println!("That move is illegal!");
            return None;
        }

        let mut new_state: GameState = GameState::InProgress;

        // We know that the move is legal, the other potential piece must be the other color
        if self.board[new_index].is_some() && 
        self.board[new_index].unwrap().piece_type == PieceType::King {
            new_state = GameState::GameOver;
        }
        
        // Move piece
        self.board[new_index] = self.board[old_index];
        self.board[old_index] = None;

        // Check for GameState::Check on own king if you make the move
        // Check on every enemy piece to see if they can attack own king, if so the move is illegal
        // However, if gamestate is already in check player is free to move (no checkmate implementation)
        if self.state != GameState::Check {
            for i in 0..64 as usize {
                if self.board[i].is_some() && self.board[i].unwrap().color != self.active_color {
                    let enemy_possible_moves = self.get_possible_moves(&index_to_an(i)).unwrap(); // We know there is a piece, safe to unwrap
                    
                    for possible_move in enemy_possible_moves {
                        let possible_index = an_to_index(&possible_move);
                        if self.board[possible_index].is_some() && self.board[possible_index].unwrap().piece_type == PieceType::King {
                            // Move back piece
                            self.board[old_index] = self.board[new_index];
                            self.board[new_index] = None;
                            println!("That move will result in check!");
                            return None;
                        }
                    }
                }
            }
        }

        // Check for GameState::Check on enemy
        let new_possible_moves = self.get_possible_moves(&to).unwrap(); // We know we just moved a piece there, safe to unwrap
        // Loop through all new possible moves and check if any of them contains a king, it's the enemy since it's legal
        for new_move in new_possible_moves {
            let new_move_index = an_to_index(&new_move);
            if self.board[new_move_index].is_some() &&
            self.board[new_move_index].unwrap().piece_type == PieceType::King {
                new_state = GameState::Check;
            }
        }
        
        // Switch turn
        self.active_color = if self.active_color == Color::White { Color::Black } else { Color::White };

        println!("{:?}", self);
        self.state = new_state;
        Some(new_state)
    }

    /// Set the piece type that a peasant becomes following a promotion.
    pub fn set_promotion(&mut self, piece: String, new_type: PieceType) {
        let index = an_to_index(&piece);
        // Update piece to new type
        self.board[index] = Some(Piece { color: self.board[index].unwrap().color, piece_type: new_type });
        println!("{:?}", self);
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Don't forget to the rules for check.
    ///
    /// (optional) Don't forget to include en passent and castling.
    /// None if there is no piece.
    pub fn get_possible_moves(&self, position: &String) -> Option<Vec<String>> {
        let index: usize = an_to_index(position);
        let piece = match self.board[index] {
            Some(i) => i,
            None => return None,
        };
        
        let rook_moves = self.rook_moves(index);
        let bishop_moves = self.bishop_moves(index);
        
        // List all theoretically possible moves
        use PieceType::*;
        let mut option_moves: Vec<Option<usize>> = match piece.piece_type {
            Pawn => self.pawn_moves(index),
            Rook => rook_moves,
            Knight => vec![ 
                self.rel_pos(index, -1, -2), self.rel_pos(index, 1, -2),// #1#1#
                self.rel_pos(index, -2, -1), self.rel_pos(index, 2, -1),// 1###1
                                                                        // ##X##    (Knight moves)
                self.rel_pos(index, -2, 1), self.rel_pos(index, 2, 1),  // 1###1
                self.rel_pos(index, -1, 2), self.rel_pos(index, 1, 2),  // #1#1#
            ],
            Bishop => bishop_moves,
            Queen => [rook_moves, bishop_moves].concat(),
            King => vec![
                self.rel_pos(index, -1, -1), self.rel_pos(index, 0, -1), self.rel_pos(index, 1, -1),
                self.rel_pos(index, -1, 0),                               self.rel_pos(index, 1, 0),
                self.rel_pos(index, -1, 1),  self.rel_pos(index, 0, 1), self.rel_pos(index, 1, 1),
            ],
        };

        // Remove all None
        option_moves.retain(|x| x.is_some());

        // New Vec of positions with algebraic notation (Strings)
        let moves = option_moves.iter()
            .map(|x| index_to_an(x.unwrap()))
            .collect();

        Some(moves)
    }

    /// Returns relative index position to pos with difference in x (dx) and difference in y (dy).
    /// Returns None if relative position is outside the board or there is a piece of the same color there.
    fn rel_pos(&self, pos: usize, dx: i32, dy: i32) -> Option<usize> {
        let rel_pos = pos as i32 + dy * 8 + dx;
    
        let different_row: bool = rel_pos / 8 != (pos as i32 + dy * 8) / 8;
        let outside_bounds: bool = rel_pos < 0 || rel_pos > 63;
        let same_color: bool = !outside_bounds && self.board[rel_pos as usize].is_some() &&
            self.board[rel_pos as usize].unwrap().color == self.board[pos].unwrap().color;

        if outside_bounds || different_row || same_color {
            return None;
        }

        Some(rel_pos as usize)
    }
    
    /// Marches with a leap size, adds position to Vec and stops when it hits a piece
    fn march(&self, start: usize, dx: i32, dy: i32, moves: &mut Vec<Option<usize>>) {        
        for i in 1..7 {
            match self.rel_pos(start, dx * i, dy * i) {
                Some(i) => {
                    moves.push(Some(i));
                    if self.board[i].is_some() { break; } // So that rooks and bishops cannot pass through enemy pieces
                },
                None => break,
            }
        }
    }

    /// List all possible moves for a rook
    fn rook_moves(&self, start: usize) -> Vec<Option<usize>> {
        let mut moves: Vec<Option<usize>> = Vec::new();        
        self.march(start, 0, -1, &mut moves); // Up
        self.march(start, 0, 1, &mut moves); // Down
        self.march(start, -1, 0, &mut moves); // Left
        self.march(start, 1, 0, &mut moves); // Right

        moves
    }

    /// List all possible moves for a bishop
    fn bishop_moves(&self, start: usize) -> Vec<Option<usize>> {
        let mut moves: Vec<Option<usize>> = Vec::new();
        self.march(start, -1, -1, &mut moves); // Up left
        self.march(start, 1, -1, &mut moves); // Up right
        self.march(start, -1, 1, &mut moves); // Down left
        self.march(start, 1, 1, &mut moves); // Down right

        moves
    }

    /// List all possible moves for a pawn
    fn pawn_moves(&self, start: usize) -> Vec<Option<usize>> {
        let mut moves: Vec<Option<usize>> = Vec::new();

        let pawn_color = self.board[start].unwrap().color;
        let direction: i32 = if pawn_color == Color::White { -1 } else { 1 };

        // One step forward
        moves.push(self.rel_pos(start, 0, direction));

        // Start position for black or white pawns, two steps forward
        if start / 8 == 1 && direction == 1 ||
        start / 8 == 6 && direction == -1 {
            moves.push(self.rel_pos(start, 0, 2 * direction));
        }

        let left_attack = self.rel_pos(start, -1, direction);
        let right_attack = self.rel_pos(start, 1, direction);

        // There is a piece diagonally to the left and it's an enemy
        if left_attack.is_some() && 
        self.board[left_attack.unwrap()].is_some() &&
        self.board[left_attack.unwrap()].unwrap().color != pawn_color {
            moves.push(left_attack);
        }

        // There is a piece diagonally to the right and it's an enemy
        if right_attack.is_some() && 
        self.board[right_attack.unwrap()].is_some() &&
        self.board[right_attack.unwrap()].unwrap().color != pawn_color {
            moves.push(right_attack);
        }

        moves
    }
}

/// Implement print routine for Game.
///
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */
        let mut s: String = String::from("|:--------------------------------:|\n");

        for piece in self.board.iter() {
            if s.len() % 37 == 0 {
                s.push_str("|  ");
            }

            if piece.is_none() {
                s.push_str("*   ");
            } else {
                match piece.unwrap().color {
                    Color::Black => s.push_str("b"),
                    Color::White => s.push_str("w"),
                }

                match piece.unwrap().piece_type {
                    PieceType::Pawn => s.push_str("P  "),
                    PieceType::Rook => s.push_str("R  "),
                    PieceType::Knight => s.push_str("Kn "),
                    PieceType::Bishop => s.push_str("B  "),
                    PieceType::Queen => s.push_str("Q  "),
                    PieceType::King => s.push_str("K  "),
                }
            }

            if s.len() % 37 == 35 {
                s.push_str("|\n");
            }
        }

        s.push_str("|:--------------------------------:|");
        
        write!(f, "{}", s)
    }
}

/// Returns index in game board based on position input using algebraic notation, AN (Ex. B4)
fn an_to_index(position: &String) -> usize {
    let lowercase = position.to_lowercase();
    let mut chars = lowercase.chars();
    let column: usize = match chars.next().unwrap() {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => todo!(),
    };
    let row: usize = chars.next().unwrap().to_digit(10).unwrap() as usize - 1;
    
    row * 8 + column
}

/// Returns position on game board in algebraic notation (AN) based on index
fn index_to_an(position: usize) -> String {
    let column = match position % 8 {
        0 => "A",
        1 => "B",
        2 => "C",
        3 => "D",
        4 => "E",
        5 => "F",
        6 => "G",
        7 => "H",
        _ => todo!(),
    };

    let row = format!("{}", position / 8 + 1);

    format!("{}{}", column, row)
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }

    #[test]
    fn an_index_conversion() {
        assert_eq!(index_to_an(32), String::from("A5"));
        assert_eq!(an_to_index(&String::from("A5")), 32);
    }

    #[test]
    fn movesets() {
        // To see game play out in console: cargo test -- --nocapture
        let mut game = Game::new();
        println!("{:?}", game);
        
        // Pawn
        assert_eq!(game.get_possible_moves(&String::from("B7")), Some(vec![String::from("B6"), String::from("B5")]));

        assert_eq!(game.make_move(String::from("A7"), String::from("A5")), Some(GameState::InProgress)); // White
        assert_eq!(game.make_move(String::from("A5"), String::from("A4")), None); // White cannot move again
        assert_eq!(game.make_move(String::from("B2"), String::from("B4")), Some(GameState::InProgress)); // Black

        // Rook
        assert_eq!(game.make_move(String::from("A8"), String::from("A5")), None); // Cannot move into white pawn
        println!("{:?}", game.get_possible_moves(&String::from("A8")));
        assert_eq!(game.make_move(String::from("A8"), String::from("A6")), Some(GameState::InProgress));
        
        // Knight
        assert_eq!(game.make_move(String::from("B1"), String::from("C3")), Some(GameState::InProgress));
        assert_eq!(game.make_move(String::from("A5"), String::from("B4")), Some(GameState::InProgress)); // White pawn kills black pawn

        // Bishop
        assert_eq!(game.make_move(String::from("C1"), String::from("A3")), Some(GameState::InProgress));
        assert_eq!(game.make_move(String::from("B4"), String::from("B3")), Some(GameState::InProgress)); // White pawn forward

        // Queen
        assert_eq!(game.make_move(String::from("D1"), String::from("C1")), Some(GameState::InProgress));
        assert_eq!(game.make_move(String::from("B3"), String::from("B2")), Some(GameState::InProgress)); // White pawn forward
        
        // King
        assert_eq!(game.make_move(String::from("E1"), String::from("D1")), Some(GameState::InProgress));
        assert_eq!(game.make_move(String::from("B2"), String::from("B1")), Some(GameState::InProgress)); // White pawn forward

        // Promote pawn
        game.set_promotion(String::from("B1"), PieceType::Queen);

        assert_eq!(game.make_move(String::from("C1"), String::from("E3")), None); // Black queen cannot move through pawn
        assert_eq!(game.make_move(String::from("C1"), String::from("B2")), None); // Queen, cannot escape since it causes check
        assert_eq!(game.make_move(String::from("F2"), String::from("F3")), Some(GameState::InProgress)); // Idly black pawn forward
        assert_eq!(game.make_move(String::from("B1"), String::from("C1")), Some(GameState::Check)); // Kill black queen
        assert_eq!(game.make_move(String::from("F3"), String::from("F4")), Some(GameState::InProgress)); // Idly black pawn forward
        assert_eq!(game.make_move(String::from("C1"), String::from("D1")), Some(GameState::GameOver)); // Kill black king
        // Game over
    }
}