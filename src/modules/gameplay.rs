use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonInput;
use bevy::ui::{Interaction, BackgroundColor};
use sf_ui_common::components::{Focusable, FocusState};
use sf_ui_common::colors::{
    button::NORMAL as NORMAL_BUTTON,
    text::NORMAL as TEXT_COLOR,
};

/// State for managing gameplay settings UI
#[derive(Resource, Default)]
pub struct GameplaySettingsState {
    pub focused_element: Option<Entity>,
}

#[derive(Default)]
pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameplaySettingsState>()
            .add_systems(Update, (
                handle_gameplay_settings_keyboard_navigation,
                update_focus_visuals,
                handle_interaction_feedback
            ));
    }
}

/// Handle keyboard navigation for gameplay settings
fn handle_gameplay_settings_keyboard_navigation(
    state: Res<GameplaySettingsState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    focus_query: Query<(&Focusable, &mut BackgroundColor), With<Focusable>>,
) {
    // Handle keyboard navigation for gameplay settings
    // This is a simplified version - you might want to add more specific navigation logic
    if keyboard_input.just_pressed(KeyCode::Tab) {
        // Handle tab navigation between focusable elements
        // This is a placeholder - implement actual navigation logic here
    }
}

/// Update focus visuals for gameplay settings
fn update_focus_visuals(
    mut query: Query<(&Focusable, &mut BackgroundColor), Changed<Focusable>>,
) {
    use sf_ui_common::colors::{
        button::{NORMAL, HOVERED, PRESSED},
        focus::HIGHLIGHT,
    };

    for (focusable, mut background_color) in &mut query {
        *background_color = match focusable.state {
            FocusState::Focused => HIGHLIGHT.into(),
            FocusState::Active => PRESSED.into(),
            FocusState::NotFocused => NORMAL.into(),
        };
    }
}

/// Handle interaction feedback for gameplay settings
fn handle_interaction_feedback(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    use sf_ui_common::colors::button::{NORMAL, HOVERED, PRESSED};

    for (interaction, mut color) in &mut interaction_query {
        *color = match interaction {
            Interaction::Pressed => PRESSED.into(),
            Interaction::Hovered => HOVERED.into(),
            Interaction::None => NORMAL.into(),
        };
    }
}

pub fn spawn_gameplay_settings(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent.spawn(TextBundle::from_section(
        "Gameplay Settings", 
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 24.0,
            color: TEXT_COLOR,
        })
    );

    parent.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        background_color: NORMAL_BUTTON.into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section("Test", 
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: TEXT_COLOR,
            })
        );
    });
}