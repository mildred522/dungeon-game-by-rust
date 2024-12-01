pub struct Monster {
    pub name: String,
    pub health: i32,
    pub attack: i32,
    pub carried_item: String,
}

impl Monster {
    pub fn new(name: &str, health: i32, attack: i32, carried_item: &str) -> Self {
        Monster {
            name: name.to_string(),
            health,
            attack,
            carried_item: carried_item.to_string(),
        }
    }
}
