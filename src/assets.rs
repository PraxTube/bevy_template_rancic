use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_trickfilm::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "dummy_background.png")]
    pub dummy_background: Handle<Image>,

    #[asset(path = "player/player.png")]
    pub player_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 32, tile_size_y = 32, columns = 1, rows = 1))]
    pub player_layout: Handle<TextureAtlasLayout>,
    #[asset(paths("player/player.trickfilm#idle",), collection(typed))]
    pub player_animations: Vec<Handle<AnimationClip2D>>,

    // --- FONTS ---
    #[asset(path = "fonts/PressStart2P.ttf")]
    pub pixel_font: Handle<Font>,
}
