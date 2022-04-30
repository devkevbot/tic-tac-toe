use std::fmt::{self, Display, Formatter};

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

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut output_str = String::new();
        for (i, row) in self.squares.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                match j {
                    MAX_COL_INDEX => match self.squares[i][j] {
                        '_' => output_str
                            .push_str(format!("| {} |\n", i * DEFAULT_BOARD_SIZE_LEN + j).as_str()),
                        _ => output_str.push_str(format!("| {} |\n", self.squares[i][j]).as_str()),
                    },
                    _ => match self.squares[i][j] {
                        '_' => output_str
                            .push_str(format!("| {} ", i * DEFAULT_BOARD_SIZE_LEN + j).as_str()),
                        _ => output_str.push_str(format!("| {} ", self.squares[i][j]).as_str()),
                    },
                }
            }
        }
        write!(f, "{}", output_str)
    }
}
