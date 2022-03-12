use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;
use bevy_ecs_ldtk::LdtkAsset;
use bevy_kira_audio::AudioSource;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Loading,
    MainMenu,
    InGame,
    GameOver,
}

#[derive(AssetCollection, Clone)]
pub struct ImageAssets {
    #[asset(path = "oracle_1bit_assets/8x8.png")]
    pub sprite_sheet: Handle<Image>,

    #[asset(path = "projectiles/bork_3.png")]
    pub bork: Handle<Image>,

    #[asset(path = "projectiles/energy_star.png")]
    pub energy_star: Handle<Image>,

    #[asset(path = "icons/heart.png")]
    pub heart: Handle<Image>,

    #[asset(path = "icons/empty_heart_container.png")]
    pub empty_heart: Handle<Image>,

    #[asset(path = "icons/potion.png")]
    pub potion: Handle<Image>,

    #[asset(path = "icons/coin.png")]
    pub coin: Handle<Image>,

    #[asset(path = "text/tile-0.png")]
    pub text0: Handle<Image>,

    #[asset(path = "text/tile-1.png")]
    pub text1: Handle<Image>,

    #[asset(path = "text/tile-2.png")]
    pub text2: Handle<Image>,

    #[asset(path = "text/tile-3.png")]
    pub text3: Handle<Image>,

    #[asset(path = "text/tile-4.png")]
    pub text4: Handle<Image>,

    #[asset(path = "text/tile-5.png")]
    pub text5: Handle<Image>,

    #[asset(path = "text/tile-6.png")]
    pub text6: Handle<Image>,

    #[asset(path = "text/tile-7.png")]
    pub text7: Handle<Image>,

    #[asset(path = "text/tile-8.png")]
    pub text8: Handle<Image>,

    #[asset(path = "text/tile-9.png")]
    pub text9: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/background_music.wav")]
    pub background_music: Handle<AudioSource>,

    #[asset(path = "audio/coin_sfx.wav")]
    pub coin: Handle<AudioSource>,

    #[asset(path = "audio/hit_sfx.wav")]
    pub hit: Handle<AudioSource>,

    #[asset(path = "audio/block_sfx.wav")]
    pub block: Handle<AudioSource>,

    #[asset(path = "audio/bork_sfx.wav")]
    pub bork: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "ConsolaMono.ttf")]
    pub font: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct LevelAssets {
    #[asset(path = "levels.ldtk")]
    pub levels: Handle<LdtkAsset>,
}
