//! Settings menu plugin for StrategyForge

pub mod settings;
pub use settings::*;

use bevy::prelude::*;
use strategyforge_core::menu::MenuItemPlugin;

/// Main plugin for settings menu
pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SettingsPlugin)
            .add_systems(Startup, setup_settings_menu);
    }
}

impl MenuItemPlugin for SettingsMenuPlugin {
    fn add_menu_item(&self, app: &mut App, parent: Entity) {
        app.add_systems(Startup, move |mut commands: Commands| {
            setup_settings_button(&mut commands, parent);
        });
    }
}

// Re-export for easier use
pub use settings::{
    VideoSettings, AudioSettings, GameSettings,
    Settings, SettingsState, SettingsTab
};

fn setup_settings_button(
    commands: &mut Commands,
    parent: Entity,
) {
    commands.entity(parent).with_children(|parent| {
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(50.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            Name::new("SettingsButton"),
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Settings",
                TextStyle {
                    font_size: 24.0,
                    ..default()
                },
            ));
        });
    });
}

fn setup_settings_menu(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(80.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgba(0.1, 0.1, 0.1, 0.9).into(),
            visibility: Visibility::Hidden,
            ..default()
        },
        Name::new("SettingsPanel"),
    )).with_children(|parent| {
        // Add tab navigation for settings categories
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            Name::new("SettingsTabs"),
        )).with_children(|parent| {
            // Video tab
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(33.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
                Name::new("VideoTab"),
            )).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Video",
                    TextStyle {
                        font_size: 20.0,
                        ..default()
                    },
                ));
            });

            // Audio tab
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(33.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
                Name::new("AudioTab"),
            )).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Audio",
                    TextStyle {
                        font_size: 20.0,
                        ..default()
                    },
                ));
            });

            // Game tab
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(34.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
                Name::new("GameTab"),
            )).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Game",
                    TextStyle {
                        font_size: 20.0,
                        ..default()
                    },
                ));
            });
        });

        // Settings content area
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            },
            Name::new("SettingsContent"),
        ));
    });
}
