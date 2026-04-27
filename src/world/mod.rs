pub mod terrain;

use raylib::{color::Color, ffi::Rectangle, math::Vector2, prelude::{RaylibDraw, RaylibDrawHandle}, texture::Texture2D};

use crate::{game::context::Context, player::{Player, PlayerConfig}, tileset::{LevelData}, world::terrain::Terrain};

pub struct World {
    pub player: Player,
    pub terrains: Vec<Terrain>,
    terrain_texture: Texture2D
}

impl World {
    pub fn new(ctx: &mut Context) -> Self {
        let level_json_str = std::fs::read_to_string("assets/levels/level_01.json")
            .expect("level data");
        let level_data: LevelData = serde_json::from_str(&level_json_str).unwrap();

        if level_data.layers.len() == 0 {
            panic!("this level must have map data layout")
        }
        let tileset = &level_data.tilesets[0];
        // load terrain texture
        let sprite_source = &tileset.source;
        let terrain_texture = ctx.rl.load_texture(&ctx.thread, sprite_source).expect("Terrain sprite source");
        
        let terrain_size = Vector2 { x: level_data.tilewidth as f32, y: level_data.tileheight as f32 };
        // load map data
        let map_data = &level_data.layers[0].data;
        let mut terrains = Vec::default();
        let tileset_columns = &tileset.columns;
        for i in 0..level_data.height {
            for j in 0..level_data.width {
                let tile_id = map_data[(i * level_data.width + j) as usize];
                if tile_id != 0 {
                    let tile_index = tile_id - tileset.firstgrid as usize;
                    let x_pos = j * level_data.tilewidth;
                    let y_pos = i * level_data.tileheight;
                    let collidable = &level_data.tilesets[0].tiles[&map_data[(i * level_data.width + j) as usize]].solid;
                    
                    let texture_rect = Rectangle {
                        x: (tile_index as u32%tileset_columns * level_data.tilewidth) as f32,
                        y: (tile_index as u32/tileset_columns* level_data.tileheight) as f32,
                        width: terrain_size.x,
                        height: terrain_size.y
                    };
                    let new_terrain = Terrain::new(Vector2 { x: x_pos as f32, y: y_pos as f32 }, terrain_size, texture_rect, *collidable);
                    terrains.push(new_terrain);
                }
            }
        }

        let player = Player::load(
            PlayerConfig {
                start_position: Vector2::new(200., 200.),
                speed: 0.0
            },
            ctx,
        );

        Self { player, terrains, terrain_texture }
    }

    pub fn update(&mut self, dt: f32) {
        self.player.update(&dt);

        for terrain in &self.terrains {
            if let Some(collider) = &terrain.collider {
                if collider.does_intersect(terrain.position, &self.player.hitbox, self.player.position) {
                    self.player.resolve_collision(collider);
                }
            }
        }
    }

    pub fn render(&mut self, d: &mut RaylibDrawHandle) {
        for terrain in &self.terrains {
            draw_terrain(d, &self.terrain_texture, terrain);
        }

        self.player.render(d);

    }
}

fn draw_terrain(d: &mut RaylibDrawHandle, atlas: &Texture2D, terrain: &Terrain) {
    d.draw_texture_pro(
        atlas, 
        terrain.texture_rect, 
        Rectangle {
            x: terrain.position.x,
            y: terrain.position.y,
            width: terrain.size.x,
            height: terrain.size.y
        }, 
        Vector2::zero(), 
        0.0, 
        Color::WHITE);
    
    #[cfg(feature = "debug")]
    {
        if let Some(aabb) = terrain.collider.as_ref() {
            d.draw_rectangle_lines_ex(aabb.world_rect(terrain.position), 1.0, Color::CYAN);
            d.draw_circle(terrain.position.x as i32, terrain.position.y as i32, 2.0, Color::GREEN);
        }
        
    }
}