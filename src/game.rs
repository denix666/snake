pub struct Game {
    pub score: i32,
    pub hi_score: i32,
}

impl Game {
    pub async fn new()  -> Self {
        Self {
            score: 0,
            hi_score: 0,
        }
    }
}
