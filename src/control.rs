use crate::board;
use crate::player;
use crate::state;

pub struct Control {
    board: board::Board,
    players: (player::Player, player::Player),
    gs: state::Game,
    ts: state::Turn,
    amount_of_turns: u8,
    turns_left: u8,
}

impl Control {
    pub fn new() -> Self {
        Self {
            board: board::Board::new(),
            players: (
                player::Player::new("Player One".to_string(), 'x'),
                player::Player::new("Player Two".to_string(), 'o'),
            ),
            gs: state::Game::Initial,
            ts: state::Turn::PlayerOne,
            amount_of_turns: board::num_board_squares(),
            turns_left: board::num_board_squares(),
        }
    }

    pub fn start_loop(&mut self) {
        self.gs.print_state();

        loop {
            self.gs = self.next_state();

            match self.gs {
                state::Game::PlayerOneWin(_) | state::Game::PlayerTwoWin(_) => {
                    self.gs.print_state();
                    self.board.print();
                    break;
                }
                _ => {
                    self.gs.print_state();
                    self.ts = self.take_turn();
                    self.turns_left -= 1;
                }
            }
        }
    }

    fn next_state(&self) -> state::Game {
        if self.board.check_win_for(self.players.0.symbol) {
            state::Game::PlayerOneWin(self.amount_of_turns - self.turns_left)
        } else if self.board.check_win_for(self.players.1.symbol) {
            state::Game::PlayerTwoWin(self.amount_of_turns - self.turns_left)
        } else if self.turns_left == 0 {
            state::Game::Tie
        } else {
            state::Game::Playing
        }
    }

    fn take_turn(&mut self) -> state::Turn {
        self.print_turn_start_message();

        let p = match &self.ts {
            state::Turn::PlayerOne => &self.players.0,
            state::Turn::PlayerTwo => &self.players.1,
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
            state::Turn::PlayerOne => &self.players.0,
            state::Turn::PlayerTwo => &self.players.1,
        };

        println!(
            "Turn # {}: {}'s turn! (Symbol: {})",
            self.amount_of_turns - self.turns_left + 1,
            p.name,
            p.symbol
        );

        self.board.print();
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
