pub const fn num_board_squares() -> u8 {
    num_board_squares_usize() as u8
}

const fn num_board_squares_usize() -> usize {
    DEFAULT_BOARD_SIZE_LEN * DEFAULT_BOARD_SIZE_LEN
}

pub const DEFAULT_BOARD_SIZE_LEN: usize = 3;
const MAX_COL_INDEX: usize = DEFAULT_BOARD_SIZE_LEN - 1;

#[derive(Debug)]
pub struct Board {
    squares: [[char; DEFAULT_BOARD_SIZE_LEN]; DEFAULT_BOARD_SIZE_LEN],
}

impl Board {
    pub fn new() -> Self {
        Self {
            squares: [['_'; DEFAULT_BOARD_SIZE_LEN]; DEFAULT_BOARD_SIZE_LEN],
        }
    }

    pub fn print(&self) {
        for (i, row) in self.squares.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                match j {
                    MAX_COL_INDEX => match self.squares[i][j] {
                        '_' => println!("| {} |", i * DEFAULT_BOARD_SIZE_LEN + j),
                        _ => println!("| {} |", self.squares[i][j]),
                    },
                    _ => match self.squares[i][j] {
                        '_' => print!("| {} ", i * DEFAULT_BOARD_SIZE_LEN + j),
                        _ => print!("| {} ", self.squares[i][j]),
                    },
                }
            }
        }
        println!()
    }

    pub fn mark_pos(&mut self, pos: (usize, usize), symbol: char) -> bool {
        return match self.squares[pos.0][pos.1] {
            '_' => {
                self.squares[pos.0][pos.1] = symbol;
                true
            }
            _ => false,
        };
    }

    pub fn check_win_for(&self, symbol: char) -> bool {
        let check = [symbol; DEFAULT_BOARD_SIZE_LEN];

        for (i, row) in self.squares.iter().enumerate() {
            if check == *row {
                println!("ROW win!");
                return true;
            }

            let mut col: [char; DEFAULT_BOARD_SIZE_LEN] = Default::default();
            for n in 0..DEFAULT_BOARD_SIZE_LEN {
                col[n] = self.squares[n][i]
            }
            if check == col {
                println!("COL win!");
                return true;
            }
        }

        let mut ltr_diagonal: [char; DEFAULT_BOARD_SIZE_LEN] = Default::default();
        for n in 0..DEFAULT_BOARD_SIZE_LEN {
            ltr_diagonal[n] = self.squares[n][n];
        }
        if check == ltr_diagonal {
            println!("LTR diag win!");
            return true;
        }

        let mut rtl_diagonal: [char; DEFAULT_BOARD_SIZE_LEN] = Default::default();
        for n in 0..DEFAULT_BOARD_SIZE_LEN {
            rtl_diagonal[n] = self.squares[n][DEFAULT_BOARD_SIZE_LEN - n - 1];
        }
        if check == rtl_diagonal {
            println!("RTL diag win!");
            return true;
        }

        false
    }
}
