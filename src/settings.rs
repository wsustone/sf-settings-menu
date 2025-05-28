use bevy::prelude::*;
use sf_plugin_template::MenuItemPlugin;

// Module declarations
#[path = "modules/video.rs"]
pub mod video;
#[path = "modules/audio.rs"]
pub mod audio;
#[path = "modules/gameplay.rs"]
pub mod gameplay;
#[path = "modules/controls.rs"]
pub mod controls;
#[path = "modules/interface.rs"]
pub mod interface;
// Re-exports
pub use video::*;
pub use audio::*;
pub use gameplay::*;
pub use controls::*;
pub use interface::*;

// Re-export plugin types
pub use audio::AudioPlugin;
pub use controls::ControlsPlugin;
pub use gameplay::GameplayPlugin;
pub use interface::InterfacePlugin;
pub use video::VideoPlugin;

// Main settings components
#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct VideoSettings {
    pub display_mode: DisplayMode,
    pub resolution: (u32, u32),
    pub graphics_quality: GraphicsQuality,
    pub vsync: bool,
    pub fps_limit: Option<u32>,
    pub ui_scale: f32,
}

#[derive(Reflect, Default, Clone)]
pub enum DisplayMode {
    #[default]
    Windowed,
    Fullscreen,
    Borderless,
}

#[derive(Reflect, Default, Clone)]
pub enum GraphicsQuality {
    Low,
    Medium,
    High,
    #[default]
    Ultra,
}

/// Audio settings component
#[derive(Component, Reflect, Default, Clone)]
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
#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct GameplaySettings {
    pub difficulty: Difficulty,
    pub show_tutorials: bool,
    pub subtitles: bool,
}

#[derive(Reflect, Default, Clone)]
pub enum Difficulty {
    Easy,
    #[default]
    Normal,
    Hard,
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct ControlsSettings {
    pub mouse_sensitivity: f32,
    pub invert_y: bool,
    pub keybinds: KeybindSettings,
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct InterfaceSettings {
    pub ui_scale: f32,
    pub colorblind_mode: u8,
}

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct KeybindSettings {
    pub camera_pan_up: KeyCode,
    pub camera_pan_down: KeyCode,
    pub camera_pan_left: KeyCode,
    pub camera_pan_right: KeyCode,
}

impl Default for KeybindSettings {
    fn default() -> Self {
        Self {
            camera_pan_up: KeyCode::KeyW,
            camera_pan_down: KeyCode::KeyS,
            camera_pan_left: KeyCode::KeyA,
            camera_pan_right: KeyCode::KeyD,
        }
    }
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct Settings {
    pub video: VideoSettings,
    pub audio: AudioSettings,
    pub gameplay: GameplaySettings,
    pub controls: ControlsSettings,
    pub interface: InterfaceSettings,
}

#[derive(Resource, Default)]
pub struct SettingsState {
    pub current_tab: SettingsTab,
    pub is_visible: bool,
}

#[derive(Default, Reflect, Clone)]
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
            .register_type::<ControlsSettings>()
            .register_type::<InterfaceSettings>()
            .register_type::<KeybindSettings>()
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
