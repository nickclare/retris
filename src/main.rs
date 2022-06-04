use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use gravity::GravityPlugin;
use spritedata::SpriteDataPlugin;

pub mod game_field;
pub mod gravity;
pub mod pieces;
pub mod spritedata;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1600.0,
            height: 900.0,
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
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
