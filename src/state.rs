use std::fmt::{self, Display, Formatter};

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

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Initial => write!(f, "Starting game!"),
            Self::Playing => write!(f, "Playing!"),
            Self::PlayerOneWin(turns_taken) => {
                write!(f, "Player One won in {turns_taken} turns!")
            }
            Self::PlayerTwoWin(turns_taken) => {
                write!(f, "Player Two won in {turns_taken} turns!")
            }
            Self::Tie => write!(f, "Game has ended in a tie!"),
        }
    }
}
