use crate::board::{self, Board};
use crate::player::Player;
use control::Control;
use state::{Game, PlayerSelector};

pub fn start_game() {
    let mut control = Control::new();

    control.print_game_state();

    loop {
        control.next_state();

        if control.game_is_over() {
            control.print_game_state();
            control.print_board();
            break;
        }

        control.print_game_state();
        control.process_player_input();
        control.end_turn();
    }
}

mod control {
    use super::*;

    pub struct Control {
        board: Board,
        players: [Player; 2],
        game_state: Game,
        player_selector: PlayerSelector,
        turns_left: u8,
    }

    impl Control {
        pub fn new() -> Self {
            Self {
                board: Board::new(),
                players: [Player::new('x'), Player::new('o')],
                game_state: Game::Initial,
                player_selector: PlayerSelector::new(2),
                turns_left: board::num_board_squares(),
            }
        }

        pub fn next_state(&mut self) {
            for player in &self.players {
                if self.board.check_win_for(player.symbol) {
                    self.game_state = Game::Win(player.symbol);
                    return;
                }
            }

            if self.turns_left == 0 {
                self.game_state = Game::Tie;
            } else {
                self.game_state = Game::Playing(self.turns_left);
            }
        }

        pub fn process_player_input(&mut self) {
            let active_player = self.next_active_player();
            self.print_turn_start_message(&active_player);

            loop {
                self.print_board();

                let (x, y) = input::read_stdin();

                if self.validate_player_move((x, y), &active_player) {
                    break;
                }

                self.print_spot_taken_message();

                self.print_board();
            }
        }

        fn print_turn_start_message(&self, active_player: &Player) {
            println!("{}'s turn!", active_player.symbol);
        }

        fn next_active_player(&mut self) -> Player {
            self.players[self.player_selector.next()].clone()
        }

        fn validate_player_move(&mut self, pos: (usize, usize), player: &Player) -> bool {
            self.board.mark_position_with(pos, player.symbol)
        }

        fn print_spot_taken_message(&self) {
            println!("That spot is already taken, try another.");
        }

        pub fn end_turn(&mut self) {
            self.turns_left -= 1;
        }

        pub fn print_game_state(&self) {
            println!("{}", self.game_state);
        }

        pub fn print_board(&self) {
            println!("{}", self.board);
        }

        pub fn game_is_over(&self) -> bool {
            match self.game_state {
                Game::Win(_) | Game::Tie => true,
                _ => false,
            }
        }
    }
}

mod state {
    use rand::Rng;
    use std::fmt::{self, Display, Formatter};

    pub struct PlayerSelector {
        current_index: usize,
        max_players: usize,
    }

    impl PlayerSelector {
        pub fn new(max_players: usize) -> Self {
            let mut rng = rand::thread_rng();

            Self {
                current_index: rng.gen_range(0usize, max_players),
                max_players,
            }
        }

        pub fn next(&mut self) -> usize {
            self.current_index = (self.current_index + 1).rem_euclid(self.max_players);
            self.current_index
        }
    }

    #[derive(Debug)]
    pub enum Game {
        Initial,

        /// Represents an in-progress state with the amount of turns remaining.
        Playing(u8),

        /// Represents a win condition with the symbol of winning player.
        Win(char),

        Tie,
    }

    impl Display for Game {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Self::Initial => write!(f, "Starting game!"),
                Self::Playing(turns_left) => {
                    write!(f, "Starting turn: ({} turns left)", turns_left)
                }
                Self::Win(winner) => write!(f, "{} has won!", winner),
                Self::Tie => write!(f, "Game has ended in a tie!"),
            }
        }
    }
}

mod input {
    use super::*;

    type Position2D = (usize, usize);

    pub fn read_stdin() -> Position2D {
        let mut buffer = String::new();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read input!");

        let num = buffer.trim().parse::<usize>().unwrap();

        let x_pos = num / board::DEFAULT_BOARD_SIZE_LEN;
        let y_pos = num % board::DEFAULT_BOARD_SIZE_LEN;
        (x_pos, y_pos)
    }
}
