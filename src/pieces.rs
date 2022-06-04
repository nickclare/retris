use crate::{
    game_field::{BLOCK_SIZE, DEFAULT_SIZE},
    spritedata::{self, SpriteIndex, SpritesheetAtlas},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Block;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

/// A `block` that makes up part of a current `piece`
#[derive(Component)]
pub struct PieceBlock;

impl PieceType {
    fn initial_blocks(&self) -> impl IntoIterator<Item = (u8, u8)> {
        match *self {
            PieceType::O => vec![(1, 0), (2, 0), (1, 1), (2, 1)],
            _ => todo!("piece not implemented"),
        }
    }

    fn color(&self) -> Color {
        match *self {
            PieceType::O => Color::BLUE,
            _ => Color::BLACK,
        }
    }
}

#[derive(Component, Debug)]
pub struct Piece {
    typ: PieceType,
}

pub struct BlockGraphic(Handle<Image>);

/// Position on the game field
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position(pub u8, pub u8);

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, Self::load_block_sprite)
            .add_system(Self::update_piece_translation.label("translation_calc"))
            .add_startup_system(Self::test_startup);
    }
}

pub const SPAWN_POINT: Position = Position(4, 20);

impl PiecesPlugin {
    pub fn load_block_sprite(mut commands: Commands, assets: Res<AssetServer>) {
        let handle = assets.load("block.png");
        commands.insert_resource(BlockGraphic(handle));
    }

    pub fn test_startup(mut commands: Commands, graphics: Res<SpritesheetAtlas>) {
        Self::spawn_piece(&mut commands, &*graphics, PieceType::O);
    }

    fn spawn_piece(commands: &mut Commands, graphics: &SpritesheetAtlas, typ: PieceType) -> Entity {
        // spawn the piece root entity
        let mut blocks = Vec::new();
        for (i, b) in typ.initial_blocks().into_iter().enumerate() {
            blocks.push(
                commands
                    .spawn()
                    .insert(Name::new(format!("Block {i}")))
                    .insert_bundle(SpriteSheetBundle {
                        texture_atlas: graphics.handle(),
                        sprite: TextureAtlasSprite {
                            color: typ.color(),
                            index: SpriteIndex::Block.index(),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert_bundle(TransformBundle::from_transform(
                        Transform::from_translation(Vec3::new(
                            (b.0 as f32) * BLOCK_SIZE,
                            (b.1 as f32) * BLOCK_SIZE,
                            0.0,
                        )),
                    ))
                    .insert(Position(SPAWN_POINT.0 + b.0, SPAWN_POINT.1 + b.1))
                    .insert(PieceBlock)
                    .id(),
            )
        }

        commands
            .spawn()
            .insert(Piece { typ })
            .insert(SPAWN_POINT)
            .insert(Name::new(format!("{:?}-Piece", typ)))
            .insert_bundle(TransformBundle::default())
            .push_children(&blocks)
            .id()
    }

    pub fn update_piece_translation(
        mut pieces: Query<(Entity, &Piece, &Position, &mut Transform), Without<PieceBlock>>,
    ) {
        for (_e, _piece, pos, mut transform) in pieces.iter_mut() {
            let x = (pos.0 as f32 - 5.0) * BLOCK_SIZE;
            let y = (pos.1 as f32 - 11.0) * BLOCK_SIZE;
            transform.translation.x = x;
            transform.translation.y = y;
        }
    }
}
