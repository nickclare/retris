use crate::spritedata::{SpriteIndex, SpritesheetAtlas};
use bevy::{prelude::*, sprite::Anchor, utils::HashMap};
use serde::Deserialize;

#[derive(Component)]
pub struct Block;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum PieceType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

pub type Blocks = [(f32, f32); 4];

#[derive(Debug, Clone, Deserialize)]
pub struct PieceData {
    pub layouts: [Blocks; 4],
    pub color: Color,
    #[serde(default)]
    pub spawn_offset: (f32, f32),
}

type PiecesData = HashMap<PieceType, PieceData>;

/// A `block` that makes up part of a current `piece`
#[derive(Component)]
pub struct PieceBlock;

impl PieceType {}

#[derive(Component, Debug)]
pub struct Piece {
    typ: PieceType,
    rotation: usize,
}

pub struct BlockGraphic(Handle<Image>);

/// Position on the game field
#[derive(Component, Debug, Clone, Copy)]
pub struct Position(pub f32, pub f32);

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::update_piece_translation.label("translation_calc"))
            .add_startup_system_to_stage(StartupStage::PreStartup, Self::load_piece_data)
            .add_startup_system(Self::test_startup)
            .add_system(Self::handle_rotation)
            .add_event::<RotationEvent>();
    }
}

pub const SPAWN_POINT: Position = Position(5.0, 20.0);
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

pub struct RotationEvent {
    pub(crate) direction: Direction,
}

impl PiecesPlugin {
    pub fn test_startup(
        mut commands: Commands,
        graphics: Res<SpritesheetAtlas>,
        pieces: Res<PiecesData>,
    ) {
        Self::spawn_piece(&mut commands, &*graphics, PieceType::L, 0, pieces);
    }

    /// System to load the piece data
    fn load_piece_data(mut commands: Commands) {
        let pieces: PiecesData = ron::from_str(include_str!("../data/piece_data.ron")).unwrap();
        commands.insert_resource(pieces);
    }

    fn spawn_piece(
        commands: &mut Commands,
        graphics: &SpritesheetAtlas,
        typ: PieceType,
        rotation: usize,
        pieces: Res<PiecesData>,
    ) -> Entity {
        // spawn the piece root entity
        assert!(rotation < 4);
        let mut blocks = Vec::new();
        let piece_data = pieces.get(&typ).unwrap();
        let offset = &piece_data.spawn_offset;
        for (_i, b) in piece_data.layouts[rotation].into_iter().enumerate() {
            blocks.push(Self::spawn_block(
                commands,
                graphics,
                b,
                *offset,
                SPAWN_POINT,
                piece_data.color,
            ))
        }

        commands
            .spawn()
            .insert(Piece { typ, rotation })
            .insert(SPAWN_POINT)
            .insert(Name::new(format!("{:?}-Piece", typ)))
            .insert_bundle(TransformBundle::default())
            .push_children(&blocks)
            .id()
    }

    fn spawn_block(
        commands: &mut Commands,
        graphics: &SpritesheetAtlas,
        position: (f32, f32),
        offset: (f32, f32),
        piece_position: Position,
        color: Color,
    ) -> Entity {
        commands
            .spawn()
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: graphics.handle(),
                sprite: TextureAtlasSprite {
                    color,
                    index: SpriteIndex::Block.index(),
                    custom_size: Some(Vec2::splat(1.0)),
                    anchor: Anchor::Center,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert_bundle(TransformBundle::from_transform(
                Transform::from_translation(Vec3::new(
                    position.0 + offset.0,
                    position.1 + offset.1,
                    0.0,
                )),
            ))
            .insert(Position(
                piece_position.0 + position.0 + offset.0,
                piece_position.1 + position.1 + offset.1,
            ))
            .insert(PieceBlock)
            .id()
    }

    pub fn update_piece_translation(
        mut pieces: Query<
            (Entity, &Piece, &Position, &mut Transform),
            (Changed<Position>, Without<PieceBlock>),
        >,
    ) {
        for (_e, _piece, pos, mut transform) in pieces.iter_mut() {
            let x = pos.0 as f32 - 5.0;
            let y = pos.1 as f32 - 11.0;
            transform.translation.x = x;
            transform.translation.y = y;
        }
    }

    pub fn handle_rotation(
        mut input: EventReader<RotationEvent>,
        mut pieces: Query<(Entity, &mut Piece, &Position, &Children), Without<PieceBlock>>,
        blocks: Query<(Entity, &PieceBlock)>,
        mut commands: Commands,
        pieces_data: Res<PiecesData>,
        graphics: Res<SpritesheetAtlas>,
    ) {
        for ev in input.iter() {
            let (ent, mut piece, pos, children) = pieces.get_single_mut().unwrap();
            let new_rot = match ev.direction {
                Direction::Clockwise if piece.rotation >= 3 => 0,
                Direction::Clockwise => piece.rotation + 1,
                Direction::CounterClockwise if piece.rotation == 0 => 3,
                Direction::CounterClockwise => piece.rotation - 1,
            };
            piece.rotation = new_rot;
            for ch in children.iter() {
                if blocks.get(*ch).is_ok() {
                    // child is a block
                    commands.entity(ent).remove_children(&[*ch]);
                    commands.entity(*ch).despawn();
                }
            }
            // spawn four new blocks, in the new rotation
            let piece_data = pieces_data.get(&piece.typ).unwrap();
            let mut blocks = Vec::new();
            for b in piece_data.layouts[new_rot] {
                blocks.push(Self::spawn_block(
                    &mut commands,
                    &*graphics,
                    b,
                    piece_data.spawn_offset,
                    *pos,
                    piece_data.color,
                ));
            }
            commands.entity(ent).push_children(&blocks);
        }
    }
}
