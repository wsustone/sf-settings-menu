use bevy::prelude::*;
use strategyforge_core::menu::MenuItemPlugin;

pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_settings_menu);
    }
}

impl MenuItemPlugin for SettingsMenuPlugin {
    fn add_menu_item(&self, app: &mut App, parent: Entity) {
        app.add_systems(Startup, move |mut commands: Commands| {
            setup_settings_button(&mut commands, parent);
        });
    }
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
    // Settings panel implementation would go here
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(80.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::rgba(0.1, 0.1, 0.1, 0.9).into(),
            visibility: Visibility::Hidden,
            ..default()
        },
        Name::new("SettingsPanel"),
    ));
}
