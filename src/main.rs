fn main() {
    let mut board = Board::new();
    println!("Starting board");
    board.print();

    let mut turns_left = 9;

    let p1 = Player::new("Player One".to_string(), 'x');
    let p2 = Player::new("Player Two".to_string(), 'o');

    loop {
        if board.check_win_for(p1.symbol) {
            println!("{} won!", p1.name);
            break;
        }

        if board.check_win_for(p2.symbol) {
            println!("{} won!", p2.name);
            break;
        }

        if turns_left == 0 {
            println!("Tie!");
            break;
        }

        if turns_left % 2 == 1 {
            println!("{}'s turn! (Symbol: {})", p1.name, p1.symbol);
            println!("Turn # {}", 9 - turns_left + 1);
            board.print();
            loop {
                let (x, y) = get_input();
                let succeeded = board.mark_pos((x, y), p1.symbol);
                if !succeeded {
                    println!("Position is taken, try another!");
                } else {
                    break;
                }
            }
        } else {
            println!("{}'s turn! (Symbol: {})", p2.name, p2.symbol);
            println!("Turn # {}", 9 - turns_left + 1);
            board.print();
            loop {
                let (x, y) = get_input();
                let succeeded = board.mark_pos((x, y), p2.symbol);
                if !succeeded {
                    println!("Position is taken, try another!");
                } else {
                    break;
                }
            }
        }

        turns_left -= 1;
    }
}

struct Player {
    name: String,
    symbol: char,
}

impl Player {
    fn new(name: String, symbol: char) -> Self {
        Self { name, symbol }
    }
}

#[derive(Debug)]
struct Board {
    squares: [[char; 3]; 3],
}

impl Board {
    fn new() -> Self {
        Self {
            squares: [['_'; 3]; 3],
        }
    }

    fn print(&self) {
        for (i, row) in self.squares.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                match j {
                    0 | 1 => match self.squares[i][j] {
                        '_' => print!("{} | ", i * 3 + j),
                        _ => print!("{} | ", self.squares[i][j]),
                    },
                    _ => match self.squares[i][j] {
                        '_' => print!("{}", i * 3 + j),
                        _ => print!("{}", self.squares[i][j]),
                    },
                }
            }
            println!()
        }
        println!()
    }

    fn mark_pos(&mut self, pos: (usize, usize), symbol: char) -> bool {
        return match self.squares[pos.0][pos.1] {
            '_' => {
                self.squares[pos.0][pos.1] = symbol;
                true
            }
            _ => false,
        };
    }

    fn check_win_for(&self, symbol: char) -> bool {
        let check = [symbol; 3];
        if check == self.squares[0] || check == self.squares[1] || check == self.squares[2] {
            println!("Horizontal win!");
            return true;
        }
        if check == [self.squares[0][0], self.squares[1][0], self.squares[2][0]] {
            println!("Vertical win!");
            return true;
        }
        if check == [self.squares[0][1], self.squares[1][1], self.squares[2][1]] {
            println!("Vertical win!");
            return true;
        }
        if check == [self.squares[0][2], self.squares[1][2], self.squares[2][2]] {
            println!("Vertical win!");
            return true;
        }
        if check == [self.squares[0][0], self.squares[1][1], self.squares[2][2]] {
            println!("LTR diagonal win!");
            return true;
        }
        if check == [self.squares[0][2], self.squares[1][1], self.squares[2][0]] {
            println!("RTL diagonal win!");
            return true;
        } else {
            return false;
        }
    }
}

fn get_input() -> (usize, usize) {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read input!");
    let num = buffer.trim().parse::<usize>().unwrap();
    (num / 3, num % 3)
}
