use crate::board::{self, Board};
use crate::player::Player;
use crate::state::{Game, Turn};

pub struct Control {
    board: Board,
    players: (Player, Player),
    gs: Game,
    ts: Turn,
    amount_of_turns: u8,
    turns_left: u8,
}

impl Control {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            players: (
                Player::new("Player One".to_string(), 'x'),
                Player::new("Player Two".to_string(), 'o'),
            ),
            gs: Game::Initial,
            ts: Turn::PlayerOne,
            amount_of_turns: board::num_board_squares(),
            turns_left: board::num_board_squares(),
        }
    }

    pub fn start_loop(&mut self) {
        println!("{}", self.gs);

        loop {
            self.gs = self.next_state();

            match self.gs {
                Game::PlayerOneWin(_) | Game::PlayerTwoWin(_) | Game::Tie => {
                    println!("{}", self.gs);
                    println!("{}", self.board);
                    break;
                }
                _ => {
                    println!("{}", self.gs);
                    self.ts = self.take_turn();
                    self.turns_left -= 1;
                }
            }
        }
    }

    fn next_state(&self) -> Game {
        if self.board.check_win_for(self.players.0.symbol) {
            Game::PlayerOneWin(self.amount_of_turns - self.turns_left)
        } else if self.board.check_win_for(self.players.1.symbol) {
            Game::PlayerTwoWin(self.amount_of_turns - self.turns_left)
        } else if self.turns_left == 0 {
            Game::Tie
        } else {
            Game::Playing
        }
    }

    fn take_turn(&mut self) -> Turn {
        self.print_turn_start_message();

        let p = match &self.ts {
            Turn::PlayerOne => &self.players.0,
            Turn::PlayerTwo => &self.players.1,
        };

        loop {
            let (x, y) = self.get_input();

            let succeeded = self.board.mark_pos((x, y), p.symbol);
            if succeeded {
                break;
            }

            println!("That position is owned by the other player, try another!");
        }

        self.ts.next()
    }

    fn print_turn_start_message(&self) {
        let p = match &self.ts {
            Turn::PlayerOne => &self.players.0,
            Turn::PlayerTwo => &self.players.1,
        };

        println!(
            "Turn # {}: {}'s turn! (Symbol: {})",
            self.amount_of_turns - self.turns_left + 1,
            p.name,
            p.symbol
        );

        println!("{}", self.board);
    }

    fn get_input(&self) -> (usize, usize) {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read input!");
        let num = buffer.trim().parse::<usize>().unwrap();
        (
            num / board::DEFAULT_BOARD_SIZE_LEN,
            num % board::DEFAULT_BOARD_SIZE_LEN,
        )
    }
}
