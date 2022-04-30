#[derive(Clone)]
pub struct Player {
    pub symbol: char,
}

impl Player {
    pub fn new(symbol: char) -> Self {
        Self { symbol }
    }
}
