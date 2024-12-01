pub struct Player {
    pub x: usize,
    pub y: usize,
    pub health: i32,
    pub attack: i32,
    pub items: Vec<String>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            x: 1,
            y: 1,
            health: 20,
            attack: 5,
            items: vec![],
        }
    }
}
