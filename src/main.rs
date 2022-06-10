#![allow(clippy::type_complexity)]
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::WorldInspectorPlugin;
use game_field::BLOCK_SIZE;
use gravity::GravityPlugin;
use input::InputPlugin;
use spritedata::SpriteDataPlugin;

pub mod game_field;
pub mod gravity;
pub mod input;
pub mod pieces;
pub mod spritedata;

pub const WIDTH: f32 = 1600.0;
pub const HEIGHT: f32 = 900.0;
pub const ASPECT_RATIO: f32 = WIDTH / HEIGHT;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: String::from("Retris"),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::GRAY))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(spawn_camera)
        .add_plugin(SpriteDataPlugin)
        .add_plugin(pieces::PiecesPlugin)
        .add_plugin(game_field::GameFieldPlugin)
        .add_plugin(GravityPlugin)
        .add_plugin(InputPlugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    let proj = &mut camera.orthographic_projection;
    proj.left = -WIDTH / BLOCK_SIZE / 2.0;
    proj.right = WIDTH / BLOCK_SIZE / 2.0;
    proj.top = HEIGHT / BLOCK_SIZE / 2.0;
    proj.bottom = -HEIGHT / BLOCK_SIZE / 2.0;
    proj.scaling_mode = ScalingMode::None;
    commands.spawn_bundle(camera);
}
