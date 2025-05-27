use bevy::prelude::*;
use strategyforge_core::menu::MenuItemPlugin;

// Module declarations
#[path = "modules/video.rs"]
pub mod video;
#[path = "modules/audio.rs"]
pub mod audio;
pub mod gameplay;
pub mod controls;
pub mod interface;
pub mod utils;

// Re-exports
pub use video::*;
pub use audio::*;
pub use gameplay::*;
pub use controls::*;
pub use interface::*;
pub use utils::*;

// Main settings components
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct VideoSettings {
    pub display_mode: DisplayMode,
    pub resolution: (u32, u32),
    pub graphics_quality: GraphicsQuality,
    pub vsync: bool,
    pub fps_limit: Option<u32>,
    pub ui_scale: f32,
}

#[derive(Reflect, Default)]
pub enum DisplayMode {
    #[default]
    Windowed,
    Fullscreen,
    Borderless,
}

#[derive(Reflect, Default)]
pub enum GraphicsQuality {
    Low,
    Medium,
    High,
    #[default]
    Ultra,
}

/// Audio settings component
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub voice_volume: f32,
    pub ambient_volume: f32,
    pub mute_when_inactive: bool,
}

/// Game settings component
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct GameplaySettings {
    pub difficulty: Difficulty,
    pub show_tutorials: bool,
    pub subtitles: bool,
}

#[derive(Reflect, Default)]
pub enum Difficulty {
    Easy,
    #[default]
    Normal,
    Hard,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ControlSettings {
    // TODO: Define controls settings
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct InterfaceSettings {
    // TODO: Define interface settings
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Settings {
    pub video: VideoSettings,
    pub audio: AudioSettings,
    pub gameplay: GameplaySettings,
    pub controls: ControlSettings,
    pub interface: InterfaceSettings,
}

#[derive(Resource, Default)]
pub struct SettingsState {
    pub current_tab: SettingsTab,
    pub is_visible: bool,
}

#[derive(Default, Reflect)]
pub enum SettingsTab {
    #[default]
    Video,
    Audio,
    Gameplay,
    Controls,
    Interface,
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SettingsState>()
            .register_type::<Settings>()
            .register_type::<SettingsTab>()
            .register_type::<VideoSettings>()
            .register_type::<AudioSettings>()
            .register_type::<GameplaySettings>()
            .register_type::<ControlSettings>()
            .register_type::<InterfaceSettings>()
            .add_plugins((
                VideoPlugin,
                AudioPlugin,
                GameplayPlugin,
                ControlsPlugin,
                InterfacePlugin,
            ))
            .add_systems(Startup, load_settings);
    }
}

fn load_settings() {
    // TODO: Load from config file
}
