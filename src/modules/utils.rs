use bevy::prelude::*;
use crate::ui::common::*;

/// Creates a section separator line
pub fn create_separator() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Px(2.0),
            margin: UiRect::vertical(Val::Px(15.0)),
            ..default()
        },
        background_color: Color::rgba(1.0, 1.0, 1.0, 0.1).into(),
        ..default()
    }
}

/// Creates a settings section with a title
pub fn create_section<'a>(
    parent: &'a mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    title: &str,
) -> Entity {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    width: Val::Percent(100.0),
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            // Section title
            parent.spawn(TextBundle::from_section(
                title,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 22.0,
                    color: Color::WHITE,
                },
            ));
            
            // Section content container
            parent.spawn((NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    width: Val::Percent(100.0),
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            },));
        })
        .id()
}

/// Creates a settings row with a label and control
pub fn create_setting_row<'a>(
    parent: &'a mut ChildBuilder,
    label: &str,
    asset_server: &Res<AssetServer>,
) -> Entity {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    margin: UiRect::vertical(Val::Px(5.0)),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            // Setting label
            parent.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 18.0,
                    color: Color::WHITE,
                },
            ));
            
            // Control container (to be filled by the caller)
            parent.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        margin: UiRect::left(Val::Px(20.0)),
                        ..default()
                    },
                    ..default()
                },
            ));
        })
        .id()
}

/// Creates a button with the given text and style
pub fn create_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    text: &str,
    button_style: Style,
) -> Entity {
    parent
        .spawn((
            ButtonBundle {
                style: button_style,
                background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.35)),
                ..default()
            },
            UIButton::default(),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                    font_size: 18.0,
                    color: Color::WHITE,
                },
            ));
        })
        .id()
}
