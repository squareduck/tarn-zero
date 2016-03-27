extern crate tcod;

use self::tcod::console::{Root, Console, FontLayout, FontType, BackgroundFlag, Offscreen, blit};
use self::tcod::colors::{self, Color};
use self::tcod::input::Key;
use self::tcod::input::KEY_PRESSED;
use self::tcod::input::KeyCode::{Up, Down, Left, Right, Escape, Enter};

use world::{World, TerrainType, WORLD_SIZE};
use entity::{Movable, Visible};
use entity::player::{Player};

const SCREEN_WIDTH: i32 = 81;
const SCREEN_HEIGHT: i32 = 51;
const LIMIT_FPS: i32 = 20;

pub struct Game {
    root: Root,
    world: World,
    map_buffer: Offscreen
}

impl Game {
    pub fn new() -> Game {
        let root = Root::initializer()
            .font("./fonts/dejavu16x16_gs_tc.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("Tarn Zero")
            .init();

        let mut world = World::new();
        world.generate_tiles();

        const WORLD_OFFSET: i32 = SCREEN_HEIGHT / 2;
        // Size of the world including void around boundaries
        const WORLD_BOUNDARY: i32 = WORLD_SIZE as i32 + WORLD_OFFSET * 2;
        // Size of the void  boundaries

        let mut map_buffer = Offscreen::new(WORLD_BOUNDARY, WORLD_BOUNDARY);
        for x in 0..WORLD_BOUNDARY {
            for y in 0..WORLD_BOUNDARY {
                let tile = if x < WORLD_OFFSET
                    || x >= WORLD_BOUNDARY - WORLD_OFFSET
                    || y < WORLD_OFFSET
                    || y >= WORLD_BOUNDARY - WORLD_OFFSET
                {
                    Color { r: 20, g: 20, b: 20 }
                } else {
                    match world.get_tile((x - WORLD_OFFSET) as usize, (y - WORLD_OFFSET) as usize).terrain_type {
                        TerrainType::Void => Color { r: 20, g: 20, b: 20 },
                        TerrainType::Deep => Color { r: 65, g: 105, b: 225 },
                        TerrainType::Shallow => Color { r: 135, g: 206, b: 235 },
                        TerrainType::Shore => Color { r: 210, g: 180, b: 140 },
                        TerrainType::Basic => Color { r: 160, g: 82, b: 45 },
                        TerrainType::High => Color { r: 188, g: 143, b: 143 },
                        TerrainType::Block => Color { r: 192, g: 192, b: 192 },
                    }
                };

                map_buffer.set_char_background(x, y, tile, BackgroundFlag::Set)
            }
        }

        Game {
            root: root,
            world: world,
            map_buffer: map_buffer
        }
    }

    pub fn start(&mut self) {
        tcod::system::set_fps(LIMIT_FPS);

        // let mut player_x = SCREEN_WIDTH / 2;
        // let mut player_y = SCREEN_HEIGHT / 2;
        let root = &mut self.root;

        blit(&mut self.map_buffer, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT), root, (0, 0), 1.0, 1.0);

        while !root.window_closed() {
            root.set_default_foreground(colors::WHITE);
            {
                let player = self.world.player;
                let x = player.get_position().x;
                let y = player.get_position().y;
                blit(&mut self.map_buffer, (x, y), (x + SCREEN_WIDTH, y + SCREEN_HEIGHT), root, (0, 0), 1.0, 1.0);
                root.put_char(SCREEN_HEIGHT/2, SCREEN_HEIGHT/2, player.get_character(), BackgroundFlag::None);
                //root.put_char(player.get_position().x, player.get_position().y, player.get_character(), BackgroundFlag::None);
                root.flush();
                //root.put_char(player.get_position().x, player.get_position().y, ' ', BackgroundFlag::None);

                // handle keys and exit game if needed
                let exit = Game::handle_keys(root, &mut self.world);
                if exit {
                    break
                }
            }
        }
    }

    fn handle_keys(root: &mut Root, world: &mut World) -> bool {
        let keypress = root.check_for_keypress(KEY_PRESSED);

        match keypress {
            // Alt+Enter: toggle fullscreen
            Some(Key { code: Enter, alt: true, .. }) => {
                let fullscreen = !root.is_fullscreen();
                root.set_fullscreen(fullscreen);
            }
            Some(Key { code: Escape, .. }) => {
                return true  // exit game
            }
            // movement keys
            Some(Key { code: Up, .. }) =>  world.move_player_with(0, -1),
            Some(Key { code: Down, .. }) => world.move_player_with(0, 1),
            Some(Key { code: Left, .. }) => world.move_player_with(-1, 0),
            Some(Key { code: Right, .. }) => world.move_player_with(1, 0),
            _ => {}
        }
        return false;
    }

}
