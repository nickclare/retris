use bevy::prelude::*;

pub struct SpriteDataPlugin;

impl Plugin for SpriteDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_spritesheet_assets);
    }
}

pub struct SpritesheetAtlas(Handle<TextureAtlas>);

impl SpritesheetAtlas {
    #[inline]
    pub fn handle(&self) -> Handle<TextureAtlas> {
        self.0.clone()
    }
}

#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SpriteIndex {
    Block = 0,
    Top = 1,
    Wall = 2,
    Bottom = 3,
}

impl SpriteIndex {
    #[inline]
    pub fn index(&self) -> usize {
        *self as usize
    }
}

fn load_spritesheet_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let sheet = asset_server.load("retris.png");
    let atlas = TextureAtlas::from_grid(sheet, Vec2::new(16.0, 16.0), 1, 4);
    let id = atlases.add(atlas);
    commands.insert_resource(SpritesheetAtlas(id));
}
