use bevy::prelude::*;
use bevy::input::keyboard::{KeyboardInput, KeyCode};
use bevy::input::ButtonState;
use bevy::a11y::{
    accesskit::{NodeBuilder, Role},
    AccessibilityNode,
};
use crate::{
    ui::{
        common::{
            components::{UIButton, UICheckbox, UISlider, Focusable, FocusState, FocusableType},
            utils::{
                NORMAL_BUTTON, HOVERED_BUTTON, PRESSED_BUTTON, DISABLED_BUTTON,
                TEXT_COLOR, SLIDER_HEIGHT, SLIDER_HANDLE_SIZE,
                FOCUSED_COLOR, FOCUSED_BORDER_COLOR, FOCUSED_TEXT_COLOR
            },
        },
        common::scrollable::create_scrollable_area,
        menu::{
            settings_menu::GameSettings,
            settings::{
                components::*,
                styles::*,
                utils::*,
            },
        },
    },
};

/// State for the controls settings menu
#[derive(Resource, Default)]
pub struct ControlsSettingsState {
    pub focusable_elements: Vec<Entity>,
    pub current_focus_index: Option<usize>,
}

/// System to handle keyboard navigation in the controls settings
pub fn handle_controls_settings_keyboard_navigation(
    mut state: ResMut<ControlsSettingsState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Focusable, &mut BorderColor, &mut BackgroundColor)>,
) {
    if state.focusable_elements.is_empty() {
        return;
    }

    let current_index = state.current_focus_index.unwrap_or(0);
    let max_index = state.focusable_elements.len().saturating_sub(1);
    let current_index = current_index.min(max_index);
    let mut new_index = current_index;
    let mut changed = false;

    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        new_index = (current_index + 1) % state.focusable_elements.len();
        changed = true;
    } else if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        new_index = current_index.checked_sub(1).unwrap_or(state.focusable_elements.len() - 1);
        changed = true;
    } else if keyboard_input.just_pressed(KeyCode::Tab) {
        if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight) {
            new_index = current_index.checked_sub(1).unwrap_or_else(|| state.focusable_elements.len().saturating_sub(1));
        } else {
            new_index = if state.focusable_elements.is_empty() { 0 } else { (current_index + 1) % state.focusable_elements.len() };
        }
        changed = true;
    }

    if changed && new_index != current_index {
        // Update focus state
        if let Some(prev_entity) = state.focusable_elements.get(current_index) {
            if let Ok((mut focusable, mut border_color, mut bg_color)) = query.get_mut(*prev_entity) {
                focusable.focus_state = FocusState::NotFocused;
                border_color.0 = Color::NONE;
                bg_color.0 = NORMAL_BUTTON;
            }
        }

        // Update the focused element
        if let Some(new_entity) = state.focusable_elements.get(new_index) {
            if let Ok((mut focusable, mut border_color, mut bg_color)) = query.get_mut(*new_entity) {
                focusable.focus_state = FocusState::Focused;
                border_color.0 = FOCUSED_BORDER_COLOR;
                bg_color.0 = FOCUSED_COLOR;
                state.current_focus_index = Some(new_index);
            }
        }
    }
}

/// System to update visual feedback for focused elements
pub fn update_focus_visuals(
    mut query: Query<(
        &mut Focusable,
        &mut BorderColor,
        &mut BackgroundColor,
        &mut Style,
        Option<&Interaction>,
    )>,
) {
    for (mut focusable, mut border_color, mut bg_color, mut style, interaction) in query.iter_mut() {
        match focusable.focus_state {
            FocusState::Focused => {
                border_color.0 = FOCUSED_BORDER_COLOR;
                bg_color.0 = FOCUSED_COLOR;
                style.border = UiRect::all(Val::Px(2.0));
            }
            _ => {
                if let Some(interaction) = interaction {
                    match interaction {
                        Interaction::Pressed => {
                            bg_color.0 = PRESSED_BUTTON;
                            style.border = UiRect::all(Val::Px(1.0));
                        }
                        Interaction::Hovered => {
                            bg_color.0 = HOVERED_BUTTON;
                            style.border = UiRect::all(Val::Px(1.0));
                        }
                        _ => {
                            bg_color.0 = NORMAL_BUTTON;
                            style.border = UiRect::all(Val::Px(1.0));
                        }
                    }
                } else {
                    bg_color.0 = NORMAL_BUTTON;
                    style.border = UiRect::all(Val::Px(1.0));
                }
                border_color.0 = Color::NONE;
            }
        }
    }
}

/// System to handle interaction feedback for controls settings
pub fn handle_interaction_feedback(
    mut interaction_query: Query<(
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
        &mut Style,
        &Focusable,
    ), (Changed<Interaction>, With<Focusable>)>,
) {
    for (interaction, mut bg_color, mut border_color, mut style, focusable) in interaction_query.iter_mut() {
        if focusable.focus_state == FocusState::Focused {
            continue; // Skip if already focused
        }

        match interaction {
            Interaction::Pressed => {
                bg_color.0 = PRESSED_BUTTON;
                border_color.0 = FOCUSED_BORDER_COLOR;
                style.border = UiRect::all(Val::Px(2.0));
            }
            Interaction::Hovered => {
                bg_color.0 = HOVERED_BUTTON;
                border_color.0 = FOCUSED_BORDER_COLOR.with_a(0.5);
                style.border = UiRect::all(Val::Px(1.5));
            }
            Interaction::None => {
                bg_color.0 = NORMAL_BUTTON;
                border_color.0 = Color::NONE;
                style.border = UiRect::all(Val::Px(1.0));
            }
        }
    }
}

/// Spawn controls settings content with keyboard navigation support
pub fn spawn_controls_settings(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    settings: &GameSettings,
    settings_state: &mut ResMut<ControlsSettingsState>,
) -> Entity {
    // Clear previous focusable elements
    settings_state.focusable_elements.clear();
    settings_state.current_focus_index = None;

    // Create a scrollable area for the settings content
    let scroll_content = create_scrollable_area(
        parent,
        Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        true,
    )
    .id();

    // Add the controls settings content to the scrollable area
    let content_entity = parent
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(20.0),
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            // Main container with padding
            let main_container = parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            min_height: Val::Auto,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::FlexStart,
                            justify_content: JustifyContent::FlexStart,
                            padding: UiRect::all(Val::Px(20.0)),
                            row_gap: Val::Px(20.0),
                            ..default()
                        },
                        ..default()
                    },
                    SettingsTab::Controls,
                ))
                .id();

            // Add content to the main container
            parent.entity(main_container).with_children(|parent| {
                // Keyboard section
                let keyboard_section = create_section(parent, asset_server, "Keyboard");
                
                // Add keyboard controls to the section
                parent.entity(keyboard_section).with_children(|parent| {
                    // Key bindings section
                    let key_bindings_section = create_section(parent, asset_server, "Key Bindings");
                    
                    // Add key binding rows
                    parent.entity(key_bindings_section).with_children(|parent| {
                        // Movement key bindings
                        create_key_binding_row(parent, asset_server, "Move Forward", "W", &mut settings_state.focusable_elements);
                        create_key_binding_row(parent, asset_server, "Move Backward", "S", &mut settings_state.focusable_elements);
                        create_key_binding_row(parent, asset_server, "Strafe Left", "A", &mut settings_state.focusable_elements);
                        create_key_binding_row(parent, asset_server, "Strafe Right", "D", &mut settings_state.focusable_elements);
                        create_key_binding_row(parent, asset_server, "Jump", "Space", &mut settings_state.focusable_elements);
                        create_key_binding_row(parent, asset_server, "Crouch", "Left Ctrl", &mut settings_state.focusable_elements);
                    });
                });
                
                // Mouse section
                let mouse_section = create_section(parent, asset_server, "Mouse");
                
                // Add mouse controls to the section
                parent.entity(mouse_section).with_children(|parent| {
                    // Mouse sensitivity slider
                    let sensitivity_row = create_setting_row(parent, "Mouse Sensitivity", asset_server);
                    
                    parent.entity(sensitivity_row).with_children(|parent| {
                        let slider_entity = parent
                            .spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Px(200.0),
                                        height: Val::Px(SLIDER_HEIGHT),
                                        ..default()
                                    },
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                Focusable {
                                    focus_state: FocusState::NotFocused,
                                    focusable_type: FocusableType::Slider,
                                },
                                SettingsSlider {
                                    value: 0.5,
                                    min: 0.0,
                                    max: 1.0,
                                    step: Some(0.05),
                                    slider_type: SliderType::Horizontal,
                                },
                            ))
                            .id();
                        
                        settings_state.focusable_elements.push(slider_entity);
                        
                        // Slider fill
                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(50.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                background_color: Color::rgb(0.2, 0.5, 0.8).into(),
                                ..default()
                            },
                        ));
                        
                        // Slider handle
                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(SLIDER_HANDLE_SIZE),
                                    height: Val::Px(SLIDER_HANDLE_SIZE),
                                    position_type: PositionType::Absolute,
                                    left: Val::Percent(50.0 - (SLIDER_HANDLE_SIZE / 200.0 * 50.0)),
                                    top: Val::Px((SLIDER_HEIGHT as f32 - SLIDER_HANDLE_SIZE as f32) / 2.0),
                                    ..default()
                                },
                                background_color: Color::WHITE.into(),
                                ..default()
                            },
                        ));
                    });
                    
                    // Invert Y-axis toggle
                    let invert_y_row = create_setting_row(parent, "Invert Y-Axis", asset_server);
                    
                    parent.entity(invert_y_row).with_children(|parent| {
                        let checkbox_entity = parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(24.0),
                                        height: Val::Px(24.0),
                                        border: UiRect::all(Val::Px(2.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    border_color: Color::GRAY.into(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                SettingsCheckbox {
                                    checked: false,
                                    checkbox_type: CheckboxType::Toggle,
                                },
                                Focusable {
                                    focus_state: FocusState::NotFocused,
                                    focusable_type: FocusableType::Checkbox,
                                },
                            ))
                            .id();
                        
                        settings_state.focusable_elements.push(checkbox_entity);
                        
                        // Checkmark
                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(16.0),
                                    height: Val::Px(16.0),
                                    ..default()
                                },
                                background_color: Color::GREEN.into(),
                                visibility: Visibility::Hidden,
                                ..default()
                            },
                        ));
                    });
                });
                
                // Gamepad section
                let gamepad_section = create_section(parent, asset_server, "Gamepad");
                
                // Add gamepad controls to the section
                parent.entity(gamepad_section).with_children(|parent| {
                    // Gamepad bindings section
                    let gamepad_bindings_section = create_section(parent, asset_server, "Gamepad Bindings");
                    
                    // Add gamepad binding rows
                    parent.entity(gamepad_bindings_section).with_children(|parent| {
                        create_key_binding_row(parent, asset_server, "Move", "Left Stick", &mut settings_state.focusable_elements);
                        create_key_binding_row(parent, asset_server, "Look", "Right Stick", &mut settings_state.focusable_elements);
                        create_key_binding_row(parent, asset_server, "Jump", "A", &mut settings_state.focusable_elements);
                        create_key_binding_row(parent, asset_server, "Crouch", "B", &mut settings_state.focusable_elements);
                        create_key_binding_row(parent, asset_server, "Sprint", "Left Stick Click", &mut settings_state.focusable_elements);
                    });
                });
                
                // Reset to Defaults button
                let reset_button = parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(200.0),
                                height: Val::Px(40.0),
                                margin: UiRect::top(Val::Px(20.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        },
                        UIButton::default(),
                        Focusable {
                            focus_state: FocusState::NotFocused,
                            focusable_type: FocusableType::Button,
                        },
                        Name::new("ResetToDefaultsButton"),
                    ))
                    .with_children(|button| {
                        button.spawn(TextBundle::from_section(
                            "Reset to Defaults",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                                font_size: 16.0,
                                color: TEXT_COLOR,
                            },
                        ));
                    })
                    .id();
                
                settings_state.focusable_elements.push(reset_button);
                
                // Set initial focus if no element is focused
                if settings_state.focusable_elements.len() > 0 && settings_state.current_focus_index.is_none() {
                    settings_state.current_focus_index = Some(0);
                }
            });
        })
        .id();

    content_entity
}

/// Helper function to create a key binding row
fn create_key_binding_row(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    action: &str,
    key: &str,
    focusable_elements: &mut Vec<Entity>,
) -> Entity {
    let row = parent
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(40.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    padding: UiRect::horizontal(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            // Action label
            parent.spawn(TextBundle::from_section(
                action,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 16.0,
                    color: TEXT_COLOR,
                },
            ));
            
            // Spacer
            parent.spawn(NodeBundle {
                style: Style {
                    flex_grow: 1.0,
                    ..default()
                },
                ..default()
            });
            
            // Key binding button
            let button = parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(30.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        border_color: Color::GRAY.into(),
                        ..default()
                    },
                    Focusable {
                        focus_state: FocusState::NotFocused,
                        focusable_type: FocusableType::Button,
                    },
                    Name::new(format!("{}KeyBinding", action)),
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        key,
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                            font_size: 14.0,
                            color: TEXT_COLOR,
                        },
                    ));
                })
                .id();
            
            focusable_elements.push(button);
        })
        .id();
    
    row
}
