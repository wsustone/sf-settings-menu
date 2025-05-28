//! Settings menu implementation for StrategyForge

pub mod modules;
pub mod settings;

// Re-exports - only export the spawn functions to avoid conflicts
pub use modules::audio::spawn_audio_settings;
pub use modules::controls::spawn_controls_settings;
pub use modules::gameplay::spawn_gameplay_settings;
pub use modules::interface::spawn_interface_settings;
pub use modules::video::spawn_video_settings;

use bevy::prelude::*;
use bevy_reflect::Reflect;
use sf_plugin_template::{MenuItemPlugin, MenuItem, MenuContent};
use sf_ui_common::colors;
use sf_ui_common::components::{Focusable, FocusState, FocusableType};

// Re-export settings types
pub use settings::{
    AudioSettings,
    VideoSettings,
    ControlsSettings,
    GameplaySettings,
    InterfaceSettings,
    Settings,
    SettingsState,
    SettingsTab,
};

/// Main plugin for settings menu
#[derive(Default, Clone)]
pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        // Register types for reflection
        app.register_type::<Settings>()
            .register_type::<AudioSettings>()
            .register_type::<VideoSettings>()
            .register_type::<ControlsSettings>()
            .register_type::<GameplaySettings>()
            .register_type::<InterfaceSettings>();
            
        // Add settings menu systems
        app.add_systems(Startup, setup_settings_menu);
    }
}

impl MenuItemPlugin for SettingsMenuPlugin {
    fn menu_name(&self) -> &'static str {
        "Settings"
    }
    
    fn add_menu_item(&self, world: &mut World, parent: Entity) {
        // Add menu item button to the menu
        let mut entity = world.entity_mut(parent);
        entity.with_children(|parent| {
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    background_color: colors::button::NORMAL.into(),
                    ..default()
                },
                MenuItem {
                    plugin_name: self.menu_name().to_string(),
                    selected: false,
                }
            )).with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        self.menu_name(),
                        TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        }
                    )
                );
            });
        });
    }
    
    fn on_selected(&self, world: &mut World, content_entity: Entity) {
        // Display settings content when this menu item is selected
        // Get the asset server before we borrow world
        let asset_server = world.resource::<AssetServer>().clone();
        
        let mut entity = world.entity_mut(content_entity);
        entity.despawn_descendants();
        
        // Create settings panel with tabbed interface
        entity.with_children(|parent| {
            // Settings container
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    ..default()
                },
            )).with_children(|parent| {
                // Title
                parent.spawn(
                    TextBundle::from_section(
                        "Game Settings",
                        TextStyle {
                            font_size: 32.0,
                            color: Color::WHITE,
                            ..default()
                        }
                    ).with_style(Style {
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    })
                );
                
                // Tabs for different settings categories
                parent.spawn(
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(50.0),
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        background_color: Color::srgb(0.2, 0.2, 0.2).into(),
                        ..default()
                    }
                ).with_children(|parent| {
                    // Video tab
                    parent.spawn(
                        ButtonBundle {
                            style: Style {
                                width: Val::Percent(25.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: colors::button::PRESSED.into(),
                            ..default()
                        }
                    ).with_children(|parent| {
                        parent.spawn(
                            TextBundle::from_section(
                                "Video",
                                TextStyle {
                                    font_size: 18.0,
                                    color: Color::WHITE,
                                    ..default()
                                }
                            )
                        );
                    });
                    
                    // Audio tab
                    parent.spawn(
                        ButtonBundle {
                            style: Style {
                                width: Val::Percent(25.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: colors::button::NORMAL.into(),
                            ..default()
                        }
                    ).with_children(|parent| {
                        parent.spawn(
                            TextBundle::from_section(
                                "Audio",
                                TextStyle {
                                    font_size: 18.0,
                                    color: Color::WHITE,
                                    ..default()
                                }
                            )
                        );
                    });
                    
                    // Controls tab
                    parent.spawn(
                        ButtonBundle {
                            style: Style {
                                width: Val::Percent(25.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: colors::button::NORMAL.into(),
                            ..default()
                        }
                    ).with_children(|parent| {
                        parent.spawn(
                            TextBundle::from_section(
                                "Controls",
                                TextStyle {
                                    font_size: 18.0,
                                    color: Color::WHITE,
                                    ..default()
                                }
                            )
                        );
                    });
                    
                    // Other tab
                    parent.spawn(
                        ButtonBundle {
                            style: Style {
                                width: Val::Percent(25.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: colors::button::NORMAL.into(),
                            ..default()
                        }
                    ).with_children(|parent| {
                        parent.spawn(
                            TextBundle::from_section(
                                "Other",
                                TextStyle {
                                    font_size: 18.0,
                                    color: Color::WHITE,
                                    ..default()
                                }
                            )
                        );
                    });
                });
                
                // Content area for the selected tab
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(20.0)),
                            ..default()
                        },
                        background_color: colors::button::NORMAL.into(),
                        ..default()
                    },
                )).with_children(|parent| {
                    // Add video settings content by default using our custom spawn function
                    spawn_video_settings_for_world_builder(parent, &asset_server);
                });
            });
        });
    }
    
    fn clone_box(&self) -> Box<dyn MenuItemPlugin> {
        Box::new(self.clone())
    }
}

/// Helper function to spawn video settings in a WorldChildBuilder context
/// This adapter function converts between different builder types
fn spawn_video_settings_for_world_builder(parent: &mut WorldChildBuilder, asset_server: &AssetServer) {
    // Create a custom UI directly to avoid type conversion issues
    // Implement the video settings UI inline here
    parent.spawn(
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }
    ).with_children(|parent| {
        // Section title
        parent.spawn(
            TextBundle::from_section(
                "Video Settings",
                TextStyle {
                    font_size: 24.0,
                    color: colors::WHITE,
                    ..default()
                }
            ).with_style(Style {
                margin: UiRect::vertical(Val::Px(20.0)),
                ..default()
            })
        );
        
        // Add settings options directly
        // Resolution setting
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Px(40.0),
                margin: UiRect::bottom(Val::Px(10.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            // Label
            parent.spawn(TextBundle::from_section(
                "Resolution",
                TextStyle {
                    font_size: 18.0,
                    color: colors::WHITE,
                    ..default()
                }
            ));
            
            // Dropdown button
            parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(180.0),
                    height: Val::Px(30.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: colors::button::NORMAL.into(),
                ..default()
            }).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "1920x1080",
                    TextStyle {
                        font_size: 16.0,
                        color: colors::WHITE,
                        ..default()
                    }
                ));
            });
        });
    });
}

// Video settings UI is now directly implemented in spawn_video_settings_for_world_builder

/// Helper function to spawn a settings row with a label and content
fn spawn_setting_row<F>(parent: &mut ChildBuilder, label: &str, content_builder: F) 
where
    F: FnOnce(&mut ChildBuilder),
{
    parent.spawn(
        NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Px(60.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                margin: UiRect::vertical(Val::Px(5.0)),
                padding: UiRect::horizontal(Val::Px(20.0)),
                ..default()
            },
            background_color: Color::rgb(0.2, 0.2, 0.2).into(),
            ..default()
        }
    ).with_children(|parent| {
        // Setting label
        parent.spawn(
            TextBundle::from_section(
                label,
                TextStyle {
                    font_size: 18.0,
                    color: Color::WHITE,
                    ..default()
                }
            )
        );
        
        // Setting control - built by the provided function
        content_builder(parent);
    });
}

#[derive(Component)]
struct SettingsButtonMarker;

fn spawn_settings_button_for_marked_entities(
    mut commands: Commands,
    query: Query<Entity, With<SettingsButtonMarker>>,
) {
    for entity in &query {
        commands.entity(entity).with_children(|parent| {
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
                Focusable {
                    state: FocusState::NotFocused,
                    focus_type: FocusableType::Button,
                },
                Name::new("SettingsButton"),
            )).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Settings",
                    TextStyle {
                        font_size: 24.0,
                        ..default()
                    }
                ));
            });
        });
        
        // Remove the marker after processing
        commands.entity(entity).remove::<SettingsButtonMarker>();
    }
}

// Make Settings implement Clone
#[derive(Clone)]
pub struct SettingsWrapper(pub settings::Settings);

// Also make Settings implement the new MenuItemPlugin trait
impl MenuItemPlugin for SettingsWrapper {
    fn menu_name(&self) -> &'static str {
        "Settings"
    }
    
    fn add_menu_item(&self, world: &mut World, parent: Entity) {
        let mut entity = world.entity_mut(parent);
        entity.insert(SettingsButtonMarker);
    }
    
    fn on_selected(&self, world: &mut World, content_entity: Entity) {
        // Use the same implementation as SettingsMenuPlugin
        let settings_plugin = SettingsMenuPlugin::default();
        settings_plugin.on_selected(world, content_entity);
    }
    
    fn clone_box(&self) -> Box<dyn MenuItemPlugin> {
        Box::new(self.clone())
    }
}

#[derive(Default)]
pub struct GameSettings {
    pub audio: AudioSettings,
    pub video: VideoSettings,
    pub controls: ControlsSettings,
    pub gameplay: GameplaySettings,
    pub interface: InterfaceSettings,
}

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
            Focusable {
                state: FocusState::NotFocused,
                focus_type: FocusableType::Button,
            },
            Name::new("SettingsButton"),
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Settings",
                TextStyle {
                    font_size: 24.0,
                    ..default()
                }
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

            // Controls tab
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(33.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
                Name::new("ControlsTab"),
            )).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Controls",
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
                    height: Val::Percent(90.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            },
            Name::new("SettingsContent"),
        ));
    });
}

fn settings_menu_system() {
    // Implementation placeholder
}

fn video_settings_system() {
    // Implementation placeholder
}

fn audio_settings_system() {
    // Implementation placeholder
}

fn controls_settings_system() {
    // Implementation placeholder
}
