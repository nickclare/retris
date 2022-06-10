use bevy::prelude::*;

use crate::spritedata::{SpriteIndex, SpritesheetAtlas};

#[derive(Component)]
pub struct GameField {}

#[derive(Component)]
pub struct Wall;

pub struct GameFieldPlugin;

impl Plugin for GameFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::spawn_field);
    }
}

pub const DEFAULT_SIZE: (f32, f32) = (10.0, 20.0);
pub const BLOCK_SIZE: f32 = 16.0;

impl GameFieldPlugin {
    #[inline]
    fn calc_wall_offset(size: (f32, f32)) -> Vec3 {
        Vec3::X * (0.5 * size.0)
    }

    fn spawn_field(mut commands: Commands, graphics: Res<SpritesheetAtlas>) {
        let left = Self::spawn_wall(&mut commands, -1.0, Name::new("Left"), &graphics);
        let right = Self::spawn_wall(&mut commands, 1.0, Name::new("Right"), &graphics);

        commands
            .spawn()
            .insert(GameField {})
            .insert_bundle(TransformBundle::default())
            .insert(Name::new("Field"))
            .add_child(left)
            .add_child(right);
    }
    /// Helper to spawn a wall, which consists of 3 sprite entities, for the top, bottom, and main body
    fn spawn_wall(
        commands: &mut Commands,
        xcoeff: f32,
        name: Name,
        graphics: &Res<SpritesheetAtlas>,
    ) -> Entity {
        let y_offset: f32 = 0.5 * (DEFAULT_SIZE.1 as f32 - 2.0);
        let top = commands
            .spawn()
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: graphics.handle(),
                sprite: TextureAtlasSprite {
                    index: SpriteIndex::Top.index(),
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, y_offset, 0.0)),
                ..Default::default()
            })
            .id();
        let bottom = commands
            .spawn()
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: graphics.handle(),
                sprite: TextureAtlasSprite {
                    index: SpriteIndex::Bottom.index(),
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, -y_offset, 0.0)),
                ..Default::default()
            })
            .id();
        let body = commands
            .spawn()
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: graphics.handle(),
                sprite: TextureAtlasSprite {
                    index: SpriteIndex::Wall.index(),
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..Default::default()
                },
                transform: Transform::from_scale(Vec3::new(1.0, DEFAULT_SIZE.1 as f32 - 2.0, 1.0)),
                ..Default::default()
            })
            .id();

        commands
            .spawn()
            .insert_bundle(TransformBundle::from_transform(
                Transform::from_translation(xcoeff * Self::calc_wall_offset(DEFAULT_SIZE)),
            ))
            .insert(Wall)
            .add_child(top)
            .add_child(body)
            .add_child(bottom)
            .insert(name)
            .id()
    }
}
