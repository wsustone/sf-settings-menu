//! Settings menu modules

pub mod audio;
pub mod controls;
pub mod gameplay;
pub mod interface;
pub mod video;

use bevy::prelude::*;
use sf_ui_common::components::*;
use sf_ui_common::types::{SettingsTab, SliderType, CheckboxType, WindowMode};
use sf_ui_common::accessibility::{AccessibilityNode, Role};

// Re-exports
pub use audio::AudioPlugin;
pub use controls::ControlsPlugin;
pub use gameplay::GameplayPlugin;
pub use interface::InterfacePlugin;
pub use video::VideoPlugin;
