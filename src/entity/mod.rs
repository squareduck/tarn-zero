pub mod player;

#[derive(Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

pub trait Movable {
    fn set_position(&mut self, x: i32, y: i32);
    fn get_position(&self) -> Position;

    fn move_with(&mut self, delta_x: i32, delta_y: i32) {
        let position = self.get_position();
        self.set_position(position.x + delta_x, position.y + delta_y);
    }

    fn move_to(&mut self, x: i32, y: i32) {
        self.set_position(x, y);
    }

}


pub trait Visible {
    fn get_character(&self) -> char;
    fn set_character(&mut self, character: char);
}
