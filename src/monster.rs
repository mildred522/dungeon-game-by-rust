pub struct Monster {
    pub name: String,
    pub health: i32,
    pub attack: i32,
}

impl Monster {
    pub fn new(name: &str, health: i32, attack: i32) -> Self {
        Monster {
            name: name.to_string(),
            health,
            attack,
        }
    }
}
