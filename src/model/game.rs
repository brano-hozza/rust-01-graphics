#[derive(Copy, Clone)]
pub enum BoardPiece {
    None,
    Red,
    Black,
}


pub fn make_empty_board() -> [[BoardPiece; 5]; 5] {
    [[BoardPiece::None; 5]; 5]
}

pub struct Game {
    pub board: [[BoardPiece; 5]; 5],
}

impl Game {
    pub fn jumble_board(&mut self) {
        self.board[0][0] = BoardPiece::Red;
        self.board[1][1] = BoardPiece::Black;
    }

    pub fn print_board(&self) {
        for row in self.board.iter() {
            for cell in row.iter() {
                match cell {
                    BoardPiece::None => print!("-"),
                    BoardPiece::Red => print!("R"),
                    BoardPiece::Black => print!("B"),
                }
            }
            println!();
        }
    }

    pub fn place_piece(&mut self, row: usize, col: usize, piece: BoardPiece) {
        self.board[row][col] = piece;
    }
}