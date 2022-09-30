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

#[derive(Copy, Clone)]
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
    pieceType: PieceType,
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

        let w_pawn = Some(Piece { color: White, pieceType: Pawn });
        let w_rook = Some(Piece { color: White, pieceType: Rook });
        let w_knight = Some(Piece { color: White, pieceType: Knight });
        let w_bishop = Some(Piece { color: White, pieceType: Bishop });
        let w_queen = Some(Piece { color: White, pieceType: Queen });
        let w_king = Some(Piece { color: White, pieceType: King });

        let b_pawn = Some(Piece { color: Black, pieceType: Pawn });
        let b_rook = Some(Piece { color: Black, pieceType: Rook });
        let b_knight = Some(Piece { color: Black, pieceType: Knight });
        let b_bishop = Some(Piece { color: Black, pieceType: Bishop });
        let b_queen = Some(Piece { color: Black, pieceType: Queen });
        let b_king = Some(Piece { color: Black, pieceType: King });

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
        let possible_moves: Vec<String> = match self.get_possible_moves(from) {
            Some(i) => i,
            None => return None,
        };

        if !possible_moves.contains(&to) {
            // Move is illegal
            None
        }

        let old_index = AN_to_index(from);
        let new_index = AN_to_index(to);

        let mut state: GameState = GameState::InProgress;

        // We know that the move is legal, the other potential piece must be the other color
        if self.board[new_index].is_some() && 
        self.board[new_index].unwrap().pieceType == PieceType::King {
            state = GameState::GameOver;
        }

        // Todo: check for GameState::Check

        self.board[new_index] = self.board[old_index];
        self.board[old_index] = None;

        Some(state)
    }

    /// Set the piece type that a peasant becomes following a promotion.
    pub fn set_promotion(&mut self, piece: String) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Don't forget to the rules for check.
    ///
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self, position: String) -> Option<Vec<String>> {
        use PieceType::*;
        let index: usize = AN_to_index(position);
        let piece = match self.board[index] {
            Some(i) => i,
            None => return None,
        };

        // 7 steps up, right, down and left
        let rook_moves = self.rook_moves(index);

        // 7 diagonal moves in every direction
        let bishop_moves = self.bishop_moves(index);

        // List all theoretically possible moves
        let mut moves: Vec<usize> = match piece.pieceType {
            Pawn => if piece.color == Color::White {
                vec![rel_pos(index, 0, -1), rel_pos(index, 0, -2)]
            } else {
                vec![rel_pos(index, 0, 1), rel_pos(index, 0, 2)]
            }
            .retain(|x| x.is_some())
            .map(|x| x.unwrap()),
            Rook => rook_moves,
            Knight => vec![rel_pos(index, -1, -2), rel_pos(index, 1, -2), // #1#1#
                           rel_pos(index, -2, -1), rel_pos(index, 2, -1), // 1###1
                                                                          // ##X##    (Knight moves)
                           rel_pos(index, -2, 1), rel_pos(index, 2, 1),   // 1###1
                           rel_pos(index, -1, 2), rel_pos(index, 1, 2),   // #1#1#
                        ]
                        .retain(|x| x.is_some())
                        .map(|x| x.unwrap()),
            Bishop => bishop_moves,
            Queen => vec![rook_moves, bishop_moves].into_iter().flatten().collect(),
            King => vec![rel_pos(index, -1, -1), rel_pos(index, 0, -1), rel_pos(index, 1, -1),
                         rel_pos(index, -1, 0),                         rel_pos(index, 1, 0),
                         rel_pos(index, -1, 1),  rel_pos(index, 0, 1),  rel_pos(index, 1, 1),
                        ]
                        .retain(|x| x.is_some())
                        .map(|x| x.unwrap()),
        };

        // // Check all moves for if a potential ally piece resides
        // for i in 0..moves.len() {
        //     if self.board[moves[i]].is_some() &&
        //     self.board[moves[i]].unwrap().color == self.board[moves[index]].unwrap().color {
        //         moves.remove(i);
        //     }
        // }

        Some(moves)
    }

    /// Returns relative index position to pos with difference in x (dx) and difference in y (dy)
    fn rel_pos(pos: usize, dx: i32, dy: i32) -> Option<usize> {
        let rel_pos = pos as i32 + dy * 8 + dx;
    
        let different_row: bool = rel_pos / 8 != (pos as i32 + dy * 8) / 8;
        let same_color: bool = self.board[rel_pos as usize].unwrap().color == self.board[pos].unwrap().color;

        if rel_pos < 0 || rel_pos > 63 || different_row || same_color {
            None
        }

        Some(rel_pos as usize)
    }
    
    /// Marches with a leap size, adds position to Vec and stops when it hits a piece
    fn march(start: usize, leap: i32, moves: &mut Vec<usize>) -> Vec<usize> {        
        for i in 0..7 {
            let index: usize = start + leap * i;
            if self.board[index].is_some() {
                if self.board[index].unwrap().color != self.board[start].unwrap().color { // Other piece has same color
                    moves.push(index);
                }
                break;
            }
            moves.push(index);
        }
    }

    fn rook_moves(start: usize) -> Vec<usize> {
        let mut moves: Vec<usize> = Vec::new();        
        march(-8, moves); // Up
        march(8, moves); // Down
        march(-1, moves); // Left
        march(1, moves); // Right

        moves
    }

    fn bishop_moves(start: usize) -> Vec<usize> {
        let mut moves: Vec<usize> = Vec::new();
        march(-9, moves); // Up left
        march(-7, moves); // Up right
        march(7, moves); // Down left
        march(9, moves); // Down right

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
        let mut s: String = String::from("|:------------------------:|\n");

        for piece in self.board.iter() {
            if piece.is_none() {
                s.push_str("*  ");
            } else {
                match piece.unwrap().pieceType {
                    PieceType::Pawn => s.push_str("P  "),
                    PieceType::Rook => s.push_str("R  "),
                    PieceType::Knight => s.push_str("Kn "),
                    PieceType::Bishop => s.push_str("B  "),
                    PieceType::Queen => s.push_str("Q  "),
                    PieceType::King => s.push_str("K  "),
                }
            }

            if s.len() % 29 == 0 {
                s.push_str("|  ");
            } else if s.len() % 29 == 27 {
                s.push_str("|\n");
            }
        }

        s.push_str("|:------------------------:|");
        
        write!(f, "{}", s)
    }
}

/// Returns index in game board based on position input using algebraic notation, AN (Ex. B4)
fn AN_to_index(position: String) -> usize {
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
    let row: usize = chars.next().unwrap() as usize - 1;
    
    row * 8 + column
}

/// Returns position on game board in algebraic notation based on index
fn index_to_AN(position: usize) -> String {
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
    fn AN_index_conversion() {
        assert_eq!(AN_to_index(index_to_AN(32)), 32);
        assert_eq!(index_to_AN(AN_to_index(String::from("A5"))), String::from("A5"));
    }
}