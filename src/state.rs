pub enum Turn {
    PlayerOne,
    PlayerTwo,
}

impl Turn {
    pub fn next(&self) -> Self {
        match self {
            Self::PlayerOne => Self::PlayerTwo,
            Self::PlayerTwo => Self::PlayerOne,
        }
    }
}

pub enum Game {
    Initial,
    Playing,
    PlayerOneWin(u8),
    PlayerTwoWin(u8),
    Tie,
}

impl Game {
    pub fn print_state(&self) {
        match self {
            Self::Initial => println!("Starting game!"),
            Self::Playing => (),
            Self::PlayerOneWin(turns_taken) => {
                println!("Player One won in {turns_taken} turns!")
            }
            Self::PlayerTwoWin(turns_taken) => {
                println!("Player Two won in {turns_taken} turns!")
            }
            Self::Tie => println!("Game has ended in a tie!"),
        }
    }
}
