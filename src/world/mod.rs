extern crate noise;
extern crate rand;

use self::noise::{Brownian2, open_simplex2, Seed};
use self::rand::Rng;

use entity::{Position, Movable};
use entity::player::{Player};

pub const WORLD_SIZE: usize = 500;

#[derive(Copy, Clone)]
pub enum TerrainType {
    Void,
    Deep,
    Shallow,
    Shore,
    Basic,
    High,
    Block
}

#[derive(Copy, Clone)]
struct Tile {
    pub terrain_type: TerrainType
}

impl Tile {
    fn new(terrain_type: TerrainType) -> Tile {
        Tile {terrain_type: terrain_type}
    }
}

pub struct World {
    map: [Tile; WORLD_SIZE * WORLD_SIZE],
    pub player: Player
}

impl World {
    pub fn new() -> World {
        World {
            map: [Tile {terrain_type: TerrainType::Basic}; WORLD_SIZE * WORLD_SIZE],
            player: Player::new()
        }
    }

    pub fn move_player_with(&mut self, delta_x: i32, delta_y: i32) {
        let player = &mut self.player;
        let position = player.get_position();
        let new_position = Position {x: position.x + delta_x, y: position.y + delta_y};
        if new_position.x >= 0 && new_position.x < (WORLD_SIZE as i32)
            && new_position.y >= 0 && new_position.y < (WORLD_SIZE as i32)
        {
            player.move_with(delta_x, delta_y)
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        self.map[y * WORLD_SIZE + x] = tile;
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Tile {
        self.map[y * WORLD_SIZE + x]
    }

    pub fn generate_tiles(&mut self) {
        const NOISE_ZOOM: f64 = 0.16;
        let mut rng = rand::thread_rng();
        let seed = &Seed::new(rng.gen::<u32>());

        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
                let value = Brownian2::new(open_simplex2, 8).wavelength(1024.0).apply(seed, &[x as f64 / NOISE_ZOOM, y as f64 / NOISE_ZOOM]);
                let tile = if value < -0.5 {
                    Tile::new(TerrainType::Deep)
                } else if value < -0.45 {
                    Tile::new(TerrainType::Shallow)
                } else if value > 0.5 {
                    Tile::new(TerrainType::Block)
                } else if value > 0.4 {
                    Tile::new(TerrainType::High)
                } else if value > -0.3 {
                    Tile::new(TerrainType::Basic)
                } else {
                    Tile::new(TerrainType::Shore)
                };

                self.set_tile(x, y, tile);
            }
        }

    }
}
