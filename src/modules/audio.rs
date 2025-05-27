use bevy::{
    prelude::*,
    input::keyboard::KeyCode,
    a11y::accesskit::Role,
    ui::{Style, BackgroundColor, BorderColor, Interaction},
};

use crate::{
    ui::{
        common::{
            components::{Focusable, FocusState, FocusableType, UIButton, UICheckbox, UISlider},
            utils::{
                NORMAL_BUTTON, HOVERED_BUTTON, PRESSED_BUTTON, DISABLED_BUTTON,
                TEXT_COLOR, FOCUSED_COLOR, FOCUSED_BORDER_COLOR, FOCUSED_TEXT_COLOR,
                SLIDER_HEIGHT, SLIDER_HANDLE_SIZE,
            },
        },
        menu::settings_menu::GameSettings,
    },
};

// Import the specific style functions we need
use super::{
    components::{SettingsCheckbox, CheckboxType, SliderType, Tooltip},
    styles::*,
    utils::{create_section, create_setting_row},
};

/// Audio settings state to track UI state and navigation
#[derive(Resource, Default)]
pub struct AudioSettingsState {
    pub focused_element: Option<Entity>,
    pub volume_sliders: Vec<Entity>,
    pub checkboxes: Vec<Entity>,
    pub buttons: Vec<Entity>,
    pub test_sound_playing: bool,
    pub current_focus_index: usize,
    pub input_device: String,
    pub output_device: String,
}

// ============================================
// System: handle_audio_slider_changes
// Handles volume slider interactions and updates audio system
// ============================================
pub fn handle_audio_slider_changes(
    mut slider_query: Query<(&mut SettingsSlider, &Interaction), Changed<Interaction>>,
    audio: Res<Audio>,
) {
    for (slider, interaction) in slider_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match slider.slider_type {
                SliderType::MasterVolume => {
                    // Apply master volume change to audio system
                    audio.set_volume(slider.value / 100.0);
                    info!("Master volume set to: {:.0}%", slider.value);
                }
                SliderType::MusicVolume => {
                    // Apply music volume change to audio system
                    // Note: This requires a custom audio system that handles separate music tracks
                    info!("Music volume set to: {:.0}%", slider.value);
                }
                SliderType::SfxVolume => {
                    // Apply SFX volume change to audio system
                    info!("SFX volume set to: {:.0}%", slider.value);
                }
                SliderType::VoiceVolume => {
                    // Apply voice volume change to audio system
                    info!("Voice volume set to: {:.0}%", slider.value);
                }
                SliderType::AmbientVolume => {
                    // Apply ambient volume change to audio system
                    info!("Ambient volume set to: {:.0}%", slider.value);
                }
                _ => {
                    warn!("Unexpected slider type in audio settings: {:?}", slider.slider_type);
                }
            }
        }
    }
}

// ============================================
// System: handle_audio_settings_keyboard_navigation
// Manages keyboard navigation between audio settings controls
// ============================================
pub fn handle_audio_settings_keyboard_navigation(
    mut state: ResMut<AudioSettingsState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Focusable, &mut Style, &mut BorderColor, &mut BackgroundColor)>,
) {
    // Get all focusable elements in order
    let all_focusable: Vec<Entity> = state.volume_sliders
        .iter()
        .chain(state.checkboxes.iter())
        .chain(state.buttons.iter())
        .cloned()
        .collect();

    if all_focusable.is_empty() {
        return;
    }

    // Handle navigation
    let current_index = state.current_focus_index.min(all_focusable.len().saturating_sub(1));
    let mut new_index = current_index;
    let mut changed = false;

    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        new_index = (current_index + 1) % all_focusable.len();
        changed = true;
    } else if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        new_index = current_index.checked_sub(1).unwrap_or(all_focusable.len() - 1);
        changed = true;
    } else if keyboard_input.just_pressed(KeyCode::Tab) {
        if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight) {
            new_index = current_index.checked_sub(1).unwrap_or(all_focusable.len() - 1);
        } else {
            new_index = (current_index + 1) % all_focusable.len();
        }
        changed = true;
    }

    if changed && new_index != current_index {
        // Update focus state
        if let Some(prev_entity) = all_focusable.get(current_index) {
            if let Ok((mut focusable, mut style, mut border_color, mut bg_color)) = query.get_mut(*prev_entity) {
                focusable.focus_state = FocusState::NotFocused;
                border_color.0 = Color::NONE;
                bg_color.0 = NORMAL_BUTTON;
            }
        }

        if let Some(new_entity) = all_focusable.get(new_index) {
            if let Ok((mut focusable, mut style, mut border_color, mut bg_color)) = query.get_mut(*new_entity) {
                focusable.focus_state = FocusState::Focused;
                border_color.0 = FOCUSED_BORDER_COLOR;
                bg_color.0 = FOCUSED_COLOR;
                state.focused_element = Some(*new_entity);
                state.current_focus_index = new_index;
            }
        }
    }

    // Handle keyboard input for focused element
    if let Some(focused_entity) = state.focused_element {
        if let Ok((focusable, _, _, _)) = query.get(focused_entity) {
            match focusable.node_type {
                FocusableType::Slider => {
                    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::ArrowRight) {
                        // Handle slider value changes with arrow keys
                        // This would be implemented in the slider system
                    }
                }
                FocusableType::Checkbox => {
                    if keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::Enter) {
                        // Toggle checkbox
                        // This would be implemented in the checkbox system
                    }
                }
                FocusableType::Button => {
                    if keyboard_input.just_pressed(KeyCode::Enter) {
                        // Activate button
                        // This would be implemented in the button system
                    }
                }
                _ => {}
            }
        }
    }
    // End of handle_audio_settings_keyboard_navigation function
}

// ============================================
// System: handle_interaction_feedback
// Provides visual feedback for interactive elements on hover/click
// ============================================
pub fn handle_interaction_feedback(
    mut interaction_query: Query<(
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
        &mut Style,
        &Focusable,
    ), (Changed<Interaction>, With<Focusable>)>,
) {
    // Start of handle_interaction_feedback function
    for (interaction, mut bg_color, mut border_color, mut style, focusable) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                bg_color.0 = PRESSED_BUTTON;
                border_color.0 = PRESSED_BUTTON;
                style.border = UiRect::all(Val::Px(1.0));
            }
            Interaction::Hovered => {
                if focusable.focus_state != FocusState::Focused {
                    bg_color.0 = HOVERED_BUTTON;
                    border_color.0 = HOVERED_BUTTON;
                    style.border = UiRect::all(Val::Px(1.0));
                }
            }
            Interaction::None => {
                if focusable.focus_state != FocusState::Focused {
                    bg_color.0 = NORMAL_BUTTON;
                    border_color.0 = Color::NONE;
                    style.border = UiRect::all(Val::Px(0.0));
                }
            }
        }
    }
}

// ============================================
// System: update_focus_visuals
// Updates visual styling for focused UI elements
// ============================================
pub fn update_focus_visuals(
    mut query: Query<(
        &mut Focusable,
        &mut BorderColor,
        &mut BackgroundColor,
        &mut Style,
        Option<&Interaction>,
    )>,
) {
    for (focusable, mut border_color, mut bg_color, mut style, interaction) in query.iter_mut() {
        // Skip if the element is being interacted with (handled by handle_interaction_feedback)
        if let Some(interaction) = interaction {
            if *interaction != Interaction::None {
                continue;
            }
        }

        match focusable.focus_state {
            FocusState::Focused => {
                border_color.0 = FOCUSED_BORDER_COLOR;
                bg_color.0 = FOCUSED_COLOR;
                style.border = UiRect::all(Val::Px(2.0));
            }
            FocusState::Hovered => {
                border_color.0 = HOVERED_BUTTON;
                bg_color.0 = HOVERED_BUTTON;
                style.border = UiRect::all(Val::Px(1.0));
            }
            FocusState::Pressed => {
                border_color.0 = PRESSED_BUTTON;
                bg_color.0 = PRESSED_BUTTON;
                style.border = UiRect::all(Val::Px(1.0));
            }
            FocusState::NotFocused => {
                border_color.0 = Color::NONE;
                bg_color.0 = NORMAL_BUTTON;
                style.border = UiRect::all(Val::Px(0.0));
            }
        }
    }
}

// ============================================
// Function: create_volume_control
// Creates a complete volume control with label, tooltip, and slider
// ============================================
fn create_volume_control(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    label: &str,
    tooltip: &str,
    slider_type: SliderType,
    initial_value: f32,
    settings_state: &mut ResMut<AudioSettingsState>,
) {
    // Create a row for the volume control
    let row = create_setting_row(parent, label, asset_server);
    parent.entity(row).with_children(|parent| {
        // Add a tooltip
        parent.spawn(TextBundle::from_section(
            tooltip,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Italic.ttf"),
                font_size: 12.0,
                color: TEXT_COLOR.with_a(0.7),
            },
        ));
        
        // Create the volume slider
        let slider = create_volume_slider(
            parent,
            asset_server,
            initial_value,
            slider_type,
            settings_state,
        );
        settings_state.volume_sliders.push(slider);
    });
}

// ============================================
// Function: create_volume_slider
// Creates a volume slider with keyboard navigation and visual feedback
// Returns: Entity - The created slider entity
// ============================================
fn create_volume_slider(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    initial_value: f32,
    slider_type: SliderType,
    settings_state: &mut ResMut<AudioSettingsState>,
) -> Entity {
    // Create a container for the slider and value display
    let container = parent.spawn((
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            border_color: Color::NONE.into(),
            ..default()
        },
        Focusable {
            focus_state: FocusState::NotFocused,
            node_type: FocusableType::Slider,
        },
    )).id();
    
    // Store the slider entity for keyboard navigation
    settings_state.volume_sliders.push(container);
    
    // Create the slider using the common UISlider component
    let slider = parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(70.0),
                height: Val::Px(SLIDER_HEIGHT),
                margin: UiRect::right(Val::Px(10.0)),
                ..default()
            },
            ..default()
        },
        UISlider {
            min: 0.0,
            max: 100.0,
            value: initial_value,
            step: Some(1.0),
            show_value: false,
        },
        SettingsSlider {
            min: 0.0,
            max: 100.0,
            value: initial_value,
            slider_type: slider_type.clone(),
            dragging: false,
            handle_entity: None,
            background_entity: None,
            value_text_entity: None,
        },
    )).id();
    
    // Add the slider to the container
    parent.add_child(container, slider);
    
    // Add value display
    let value_display = parent.spawn((
        TextBundle::from_section(
            format!("{:.0}%", initial_value),
            TextStyle {
                font: asset_server.load("fonts/FiraSans-SemiBold.ttf"),
                font_size: 14.0,
                color: TEXT_COLOR,
            },
        ),
        Label,
    )).id();
    
    // Add value display to container
    parent.add_child(container, value_display);
    
    // Add keyboard navigation hints for screen readers
    parent.commands().entity(container).insert(
        AccessibilityNode::from(Role::Slider)
            .with_name("Volume Slider")
            .with_value(initial_value.to_string())
            .with_min_value(0.0)
            .with_max_value(100.0)
            .with_step(1.0)
    );
    
    container
}

// ============================================
// Function: spawn_audio_settings
// Main function to spawn the audio settings UI
// ============================================
pub fn spawn_audio_settings(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    settings: &GameSettings,
    settings_state: &mut ResMut<AudioSettingsState>,
) {
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

    // Add the audio settings content to the scrollable area
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    padding: UiRect::all(20.0),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| { // 1. Start of scroll content children
            // Main container with padding
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        min_height: Val::Auto,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
                        justify_content: JustifyContent::FlexStart,
                        padding: UiRect::all(Val::Px(20.0)),
                        row_gap: Val::Px(25.0),
                        ..default()
                    },
                    ..default()
                },
                SettingsTab::Audio,
            ))
            .with_children(|parent| { // 2. Start of main container children
                // Add a description
                parent.spawn(TextBundle::from_section(
                    "Adjust audio settings to your preference. Changes are applied immediately.",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                        font_size: 14.0,
                        color: TEXT_COLOR.with_a(0.8),
                    },
                ));
                
                // Add some vertical spacing
                parent.spawn(NodeBundle {
                    style: Style {
                        height: Val::Px(15.0),
                        ..default()
                    },
                    ..default()
                });
                
                // Audio Device Section
                let device_section = create_section(parent, asset_server, "Audio Devices");
                parent.entity(device_section).with_children(|parent| {
                    // Input Device Selection
                    let input_row = create_setting_row(parent, "Input Device", asset_server);
                    parent.entity(input_row).with_children(|parent| {
                        // In a real implementation, you would query the system for available input devices
                        let available_inputs = ["Default Input", "Microphone", "Line In"];
                        parent.spawn((
                            ButtonBundle {
                                style: Style {
                                    padding: UiRect::all(Val::Px(8.0)),
                                    border: UiRect::all(Val::Px(1.0)),
                                    ..default()
                                },
                                border_color: Color::GRAY.into(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            Focusable {
                                focus_state: FocusState::NotFocused,
                                node_type: FocusableType::Dropdown,
                            },
                        )).with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                available_inputs[0],
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                                    font_size: 14.0,
                                    color: TEXT_COLOR,
                                },
                            ));
                        });
                    });

                    // Output Device Selection
                    let output_row = create_setting_row(parent, "Output Device", asset_server);
                    parent.entity(output_row).with_children(|parent| {
                        // In a real implementation, you would query the system for available output devices
                        let available_outputs = ["Default Output", "Headphones", "Speakers"];
                        parent.spawn((
                            ButtonBundle {
                                style: Style {
                                    padding: UiRect::all(Val::Px(8.0)),
                                    border: UiRect::all(Val::Px(1.0)),
                                    ..default()
                                },
                                border_color: Color::GRAY.into(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            Focusable {
                                focus_state: FocusState::NotFocused,
                                node_type: FocusableType::Dropdown,
                            },
                        )).with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                available_outputs[0],
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                                    font_size: 14.0,
                                    color: TEXT_COLOR,
                                },
                            ));
                        });
                    });
                });

                // Volume section
                let volume_section = create_section(parent, asset_server, "Volume Controls");
                
                // Add volume controls to the section
                parent.entity(volume_section).with_children(|parent| {
                    // Master Volume
                    create_volume_control(
                        parent,
                        asset_server,
                        "Master Volume",
                        "Adjusts the overall volume level",
                        SliderType::MasterVolume,
                        settings.master_volume * 100.0,
                        settings_state,
                    );

                    // Music Volume
                    create_volume_control(
                        parent,
                        asset_server,
                        "Music Volume",
                        "Adjusts the volume of background music",
                        SliderType::MusicVolume,
                        settings.music_volume * 100.0,
                        settings_state,
                    );

                    // SFX Volume
                    create_volume_control(
                        parent,
                        asset_server,
                        "Sound Effects",
                        "Adjusts the volume of sound effects",
                        SliderType::SfxVolume,
                        settings.sfx_volume * 100.0,
                        settings_state,
                    );

                    // Voice Volume
                    create_volume_control(
                        parent,
                        asset_server,
                        "Voice Volume",
                        "Adjusts the volume of character voices",
                        SliderType::VoiceVolume,
                        settings.voice_volume * 100.0,
                        settings_state,
                    );

                    // Ambient Volume
                    create_volume_control(
                        parent,
                        asset_server,
                        "Ambient Volume",
                        "Adjusts the volume of ambient sounds",
                        SliderType::AmbientVolume,
                        settings.ambient_volume * 100.0,
                        settings_state,
                    );
                });

                // Audio Settings Section
                let settings_section = create_section(parent, asset_server, "Audio Settings");
                parent.entity(settings_section).with_children(|parent| {
                    // Mute When Inactive
                    let mute_row = create_setting_row(parent, "Mute When Inactive", asset_server);
                    parent.entity(mute_row).with_children(|parent| {
                        parent.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(40.0),
                                    height: Val::Px(20.0),
                                    border: UiRect::all(Val::Px(1.0)),
                                    ..default()
                                },
                                background_color: if settings.show_audio_indicators {
                                    PRESSED_BUTTON.into()
                                } else {
                                    NORMAL_BUTTON.into()
                                },
                                border_color: Color::GRAY.into(),
                                ..default()
                            },
                            SettingsCheckbox {
                                checked: settings.show_audio_indicators,
                                checkbox_type: CheckboxType::ShowAudioIndicators,
                            },
                            Focusable {
                                focus_state: FocusState::NotFocused,
                                node_type: FocusableType::Checkbox,
                            },
                        ));
                    });

                    // Test Sound Button
                    let test_row = create_setting_row(parent, "Test Sound", asset_server);
                    parent.entity(test_row).with_children(|parent| {
                        let button = parent.spawn((
                            ButtonBundle {
                                style: Style {
                                    padding: UiRect::all(Val::Px(8.0)),
                                    border: UiRect::all(Val::Px(1.0)),
                                    ..default()
                                },
                                background_color: NORMAL_BUTTON.into(),
                                border_color: Color::GRAY.into(),
                                ..default()
                            },
                            Focusable {
                                focus_state: FocusState::NotFocused,
                                node_type: FocusableType::Button,
                            },
                        )).with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Play Test Sound",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-SemiBold.ttf"),
                                    font_size: 14.0,
                                    color: TEXT_COLOR,
                                },
                            ));
                        }).id();
                        settings_state.buttons.push(button);
                    });
                    
                    // Add some vertical spacing
                    parent.spawn(NodeBundle {
                        style: Style {
                            height: Val::Px(15.0),
                            ..default()
                        },
                        ..default()
                    });
                });
                
                // Add a footer with reset to defaults button
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Auto,
                            padding: UiRect::all(20.0),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    // Reset to Defaults button
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                padding: UiRect::all(Val::Px(10.0)),
                                margin: UiRect::left(Val::Px(10.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        },
                        UIButton {
                            normal_color: NORMAL_BUTTON,
                            hover_color: HOVERED_BUTTON,
                            press_color: PRESSED_BUTTON,
                            disabled_color: DISABLED_BUTTON,
                            is_toggle: false,
                            is_toggled: false,
                        },
                    )).with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Reset to Defaults",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-SemiBold.ttf"),
                                font_size: BUTTON_TEXT_SIZE,
                                color: TEXT_COLOR,
                            },
                        ));
                    });
                });
            });
        });
}
