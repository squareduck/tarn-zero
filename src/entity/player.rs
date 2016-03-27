use entity::{Position, Movable, Visible};

#[derive(Copy, Clone)]
pub struct Player {
    position: Position,
    character: char
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: Position {x: 0, y: 0},
            character: '@'
        }
    }
}

impl Movable for Player {
    fn get_position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.position = Position {x: x, y: y};
    }
}

impl Visible for Player {
    fn get_character(&self) -> char {
        self.character
    }
    fn set_character(&mut self, character: char) {
        self.character = character;
    }
}
