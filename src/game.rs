use std::io::{self};
use crate::player;
use crate::monster;

pub struct Game {
    player: player::Player,
    monsters: Vec<monster::Monster>,
    map: Vec<Vec<char>>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player: player::Player::new(),
            monsters: vec![monster::Monster::new("Goblin", 10, 3)],
            map: vec![
                vec!['#', '#', '#', '#', '#'],
                vec!['#', 'P', '.', '.', '#'],
                vec!['#', '.', 'M', '.', '#'],
                vec!['#', '.', '.', '.', '#'],
                vec!['#', '#', '#', '#', '#'],
            ],
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to the Dungeon Crawler!");
        self.print_map();

        loop {
            self.handle_player_move();
            if !self.is_player_alive() {
                println!("You have been defeated. Game Over!");
                break;
            }
            self.print_map();
        }
    }

    fn handle_player_move(&mut self) {
        println!("Enter your move (w/a/s/d):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // 清除当前玩家位置
        self.map[self.player.y][self.player.x] = '.';

        // 根据输入更新玩家位置
        match input {
            "w" => {
                if self.player.y > 0 {
                    self.player.y -= 1;
                } else {
                    println!("Cannot move up, you're at the top boundary.");
                }
            }
            "a" => {
                if self.player.x > 0 {
                    self.player.x -= 1;
                } else {
                    println!("Cannot move left, you're at the left boundary.");
                }
            }
            "s" => {
                if self.player.y < self.map.len() - 1 {
                    self.player.y += 1;
                } else {
                    println!("Cannot move down, you're at the bottom boundary.");
                }
            }
            "d" => {
                if self.player.x < self.map[0].len() - 1 {
                    self.player.x += 1;
                } else {
                    println!("Cannot move right, you're at the right boundary.");
                }
            }
            _ => {
                println!("Invalid move.");
                return;
            }
        }

        // 更新地图上的玩家位置
        if self.map[self.player.y][self.player.x] == 'M' {
            self.check_encounter();
        }
        self.map[self.player.y][self.player.x] = 'P';

        // 提示玩家当前坐标
        println!("Your current position is ({}, {}).", self.player.x, self.player.y);
    }
    

    fn is_valid_move(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 || x >= self.map[0].len() as isize || y >= self.map.len() as isize {
            return false;
        }
        self.map[y as usize][x as usize] != '#'
    }

    fn check_encounter(&mut self) {
        if  self.map[self.player.y][self.player.x] == 'M' {
                println!("You encountered a monster!");

                // 调用独立的 `battle` 函数，传入 `self.player` 和 `self.monsters[0]` 的可变引用
                if let Some(monster) = self.monsters.get_mut(0) {
                    let (player_alive, monster_alive) = Game::battle(&mut self.player, monster);

                    if !monster_alive {
                        self.map[self.player.y][self.player.x] = '.';
                        // 如果怪物被击败，可以从 `self.monsters` 中移除它
                        self.monsters.remove(0);
                    }

                    if !player_alive {
                        println!("Game Over!");
                        // 你可以在这里添加更多的游戏结束逻辑
                    }
                }
            
        }
    }

    

    fn battle(
        player: &mut player::Player,
        monster: &mut monster::Monster,
    ) -> (bool, bool) {
        let mut player_health = player.health;
        let mut monster_health = monster.health;

        while player_health > 0 && monster_health > 0 {
            println!("Player attacks monster for {} damage.", player.attack);
            monster_health -= player.attack;

            if monster_health <= 0 {
                println!("You have defeated the monster!");
                return (true, false);
            }

            println!("Monster attacks player for {} damage.", monster.attack);
            player_health -= monster.attack;

            if player_health <= 0 {
                println!("You have been defeated!");
                return (false, true);
            }
        }

        (player_health > 0, monster_health > 0)
    }

    fn is_player_alive(&self) -> bool {
        self.player.health > 0
    }

    fn print_map(&self) {
        for row in &self.map {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
        println!();
    }
}