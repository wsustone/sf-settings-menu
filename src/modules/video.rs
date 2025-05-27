use bevy::{
    prelude::*,
    window::{WindowMode, PrimaryWindow, WindowResolution, Window},
    input::keyboard::KeyCode,
    input::mouse::MouseButton,
    a11y::accesskit::Role,
    log::info,
};

use crate::{
    ui::{
        common::{
            components::{Focusable, FocusState, FocusableType},
            scrollable::create_scrollable_area,
            utils::{
                NORMAL_BUTTON, HOVERED_BUTTON, PRESSED_BUTTON, DISABLED_BUTTON,
                TEXT_COLOR, FOCUSED_COLOR, FOCUSED_BORDER_COLOR, FOCUSED_TEXT_COLOR,
                SLIDER_HEIGHT, SLIDER_HANDLE_SIZE
            },
        },
        menu::settings_menu::GameSettings,
    },
};

// Import the specific style functions we need
use super::{
    components::{
        SettingsTab, SettingsCheckbox, CheckboxType,
        SettingsSlider, SliderType, Tooltip
    },
    styles::*,
    utils::create_section,
};

use super::styles::{subsection_title_style, regular_text_style};

use sf_ui_common::{
    components::{Focusable, FocusState, FocusableType, UiSlider, UiCheckbox},
    systems::{focus_navigation_system, focus_visual_system}
};

/// Simplified state since focus is now handled by sf-ui-common
#[derive(Resource, Default)]
pub struct VideoSettingsState {
    pub display_mode_options: Vec<WindowMode>,
    pub selected_display_mode: Option<WindowMode>,
    pub selected_resolution: Option<(u32, u32)>,
}

impl Default for VideoSettingsState {
    fn default() -> Self {
        Self {
            display_mode_options: vec![
                WindowMode::Windowed,
                WindowMode::BorderlessFullscreen,
                WindowMode::Fullscreen,
                WindowMode::SizedFullscreen,
            ],
            selected_display_mode: None,
            selected_resolution: None,
        }
    }
}

/// System to handle keyboard navigation in the video settings
pub fn handle_video_settings_keyboard_navigation(
    mut state: ResMut<VideoSettingsState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        Entity,
        &mut Focusable,
        Option<&mut UiSlider>,
        Option<&UiCheckbox>
    )>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    // Handle keyboard input for the currently focused element
    for (entity, mut focusable, slider, checkbox) in &mut query {
        if focusable.state != FocusState::Focused {
            continue;
        }

        // Handle keyboard input for sliders
        if let Some(mut slider) = slider {
            let step = slider.step.unwrap_or_else(|| (slider.max - slider.min) / 20.0);
            
            if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
                slider.value = (slider.value - step).max(slider.min);
            } else if keyboard_input.just_pressed(KeyCode::ArrowRight) {
                slider.value = (slider.value + step).min(slider.max);
            }
        }
        
        // Handle keyboard input for checkboxes
        if keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::Enter) {
            if let Some(checkbox) = checkbox {
                // Toggle checkbox state (actual toggle happens in interaction system)
                focusable.state = FocusState::Active;
            }
        }
    }
}

/// System to handle interaction feedback for video settings
pub fn handle_video_interaction_feedback(
    mut interaction_query: Query<(
        &Interaction,
        &mut Focusable,
        &mut BackgroundColor,
    )>,
) {
    // Implementation using common focus system
    for (interaction, mut focusable, mut background_color) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                // Update visuals for pressed state
            }
            Interaction::Hovered => {
                // Update visuals for hovered state
            }
            Interaction::None => {
                // Update visuals for normal state
            }
        }
    }
}

/// Helper function to update slider visuals
fn update_slider_visuals(
    query: &mut Query<(
        Entity,
        &mut Focusable,
        &mut Style,
        &mut BorderColor,
        &mut BackgroundColor,
        Option<&mut SettingsSlider>
    )>,
    slider: &SettingsSlider,
) {
    // Update slider handle position
    if let Some(handle_entity) = slider.handle_entity {
        if let Ok((_, _, mut style, _, _, _)) = query.get_mut(handle_entity) {
            let percent = (slider.value - slider.min) / (slider.max - slider.min);
            style.position = UiRect::left(Val::Percent(percent * 100.0));
        }
    }

    // Update slider background fill
    if let Some(background_entity) = slider.background_entity {
        if let Ok((_, _, mut style, _, _, _)) = query.get_mut(background_entity) {
            let percent = (slider.value - slider.min) / (slider.max - slider.min);
            style.width = Val::Percent(percent * 100.0);
        }
    }

    // Update slider value text
    if let Some(text_entity) = slider.value_text_entity {
        if let Ok((_, _, _, _, _, _)) = query.get_mut(text_entity) {
            // Text updates are handled by a separate system
        }
    }
}

/// Apply slider setting changes to the game
fn apply_slider_setting(slider: &SettingsSlider) {
    match slider.slider_type {
        SliderType::Brightness => {
            // TODO: Apply brightness change to the game
            info!("Brightness set to: {:.2}", slider.value);
        }
        SliderType::Contrast => {
            // TODO: Apply contrast change to the game
            info!("Contrast set to: {:.2}", slider.value);
        }
        SliderType::Gamma => {
            // TODO: Apply gamma correction
            info!("Gamma set to: {:.2}", slider.value);
        }
        SliderType::FpsLimit => {
            // TODO: Apply FPS limit
            info!("FPS limit set to: {}", slider.value);
        }
        _ => {
            warn!("Unexpected slider type in video settings: {:?}", slider.slider_type);
        }
    }
}

/// Spawn video settings content
pub fn spawn_video_settings(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    settings: &GameSettings,
) -> Entity {
    // Create a new state for this instance
    let mut state = VideoSettingsState::default();
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
    
    // Clear previous state
    state.selected_display_mode = None;
    state.selected_resolution = None;

    // Create video settings entity
    let video_settings_entity = parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ..default()
        },
        Name::new("VideoSettings"),
    )).id();
    
    // Add children to the video settings entity using the refactored functions
    if let Some(mut entity_commands) = parent.world_scope(|world| world.get_entity_mut(video_settings_entity)) {
        entity_commands.with_children(|parent| {
            spawn_main_container(parent, asset_server, settings, &mut state);
        });
    }

    video_settings_entity
}

/// Spawn main container for video settings
fn spawn_main_container(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    settings: &GameSettings,
    state: &mut VideoSettingsState,
) {
    // Main container with padding
    let main_container = parent.spawn((
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
        Name::new("VideoSettingsContainer"),
    )).id();
    
    // Add content to main container
    parent.entity(main_container).with_children(|parent| {
        // Main content container
        let content_container = parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                ..default()
            },
            Name::new("VideoContentContainer"),
        )).id();
        
        // Add description section
        spawn_description_section(parent, asset_server, content_container);
        
        // Add display mode section
        spawn_display_mode_section(parent, asset_server, settings, state, content_container);
        
        // Add brightness slider section
        spawn_brightness_slider_section(parent, asset_server, settings, state, content_container);
        
        // Add graphics settings section
        spawn_graphics_section(parent, asset_server, settings, state, content_container);
        
        // Add resolution section (placeholder for now)
        // spawn_resolution_section(parent, asset_server, settings, state, content_container);
        
        // Add advanced settings section (placeholder for now)
        // spawn_advanced_settings_section(parent, asset_server, settings, state, content_container);
    });
}

/// Spawn description section
fn spawn_description_section(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    content_container: Entity,
) {
    parent.entity(content_container).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Adjust your display settings to match your preferences.",
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
    });
}

/// Spawn display mode section
fn spawn_display_mode_section(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    settings: &GameSettings,
    state: &mut VideoSettingsState,
    content_container: Entity,
) {
    parent.entity(content_container).with_children(|parent| {
        // Display Mode header
        parent.spawn(TextBundle::from_section(
            "Display Mode",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 18.0,
                color: TEXT_COLOR,
            },
        ));
        
        // Add display mode buttons container
        let display_modes_container = parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            },
            Name::new("DisplayModesContainer"),
        )).id();
        
        // Add display mode buttons
        spawn_display_mode_buttons(parent, asset_server, settings, state, display_modes_container);
    });
}

/// Spawn display mode buttons
fn spawn_display_mode_buttons(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    settings: &GameSettings,
    state: &mut VideoSettingsState,
    container: Entity,
) {
    parent.entity(container).with_children(|parent| {
        let modes = [
            ("Fullscreen", WindowMode::Fullscreen),
            ("Windowed", WindowMode::Windowed),
            ("Borderless", WindowMode::BorderlessFullscreen),
        ];

        for (i, (label, mode)) in modes.iter().enumerate() {
            let is_selected = settings.window_mode == *mode;
            let button = spawn_display_mode_button(
                parent,
                asset_server,
                *label,
                is_selected,
                i,
                state,
            );
            
            // Set initial focus if this is the selected mode
            if is_selected {
                // state.focused_element = Some(button);
                // state.current_focus_index = i;
            }
        }
    });
}

/// Spawn a single display mode button
fn spawn_display_mode_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    label: &str,
    is_selected: bool,
    index: usize,
    state: &mut VideoSettingsState,
) -> Entity {
    let button = parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(120.0),
                height: Val::Px(40.0),
                margin: UiRect::right(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            border_color: if is_selected { BUTTON_BORDER_COLOR } else { Color::TRANSPARENT },
            background_color: if is_selected { BUTTON_COLOR } else { BUTTON_COLOR.with_a(0.5) }.into(),
            ..default()
        },
        UIButton::default()
            .with_colors(
                BUTTON_COLOR.with_a(0.5),
                BUTTON_COLOR.with_a(0.7),
                BUTTON_COLOR,
                BUTTON_BORDER_COLOR,
            ),
        Name::new(format!("{}Button", label)),
        Interaction::None,
        Focusable {
            node_type: FocusableType::Button,
            focus_state: FocusState::NotFocused,
        },
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            label,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-SemiBold.ttf"),
                font_size: 14.0,
                color: if is_selected { Color::WHITE } else { TEXT_COLOR },
            },
        ));
    }).id();
    
    // Add to state management
    // state.buttons.push(button);
    button
}

/// Spawn brightness slider section
fn spawn_brightness_slider_section(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    settings: &GameSettings,
    state: &mut VideoSettingsState,
    content_container: Entity,
) {
    parent.entity(content_container).with_children(|parent| {
        // Add some vertical spacing
        parent.spawn(NodeBundle {
            style: Style {
                height: Val::Px(20.0),
                ..default()
            },
            ..default()
        });
        
        // Brightness section
        parent.spawn(TextBundle::from_section(
            "Brightness",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 18.0,
                color: TEXT_COLOR,
            },
        ));
        
        // Brightness slider container
        let brightness_container = parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::horizontal(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            },
            Name::new("BrightnessContainer"),
        )).id();
        
        // Add brightness slider and value text
        parent.entity(brightness_container).with_children(|parent| {
            // Brightness slider
            let slider = parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(70.0),
                        height: Val::Px(6.0),
                        margin: UiRect::horizontal(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                },
                Name::new("BrightnessSlider"),
            )).id();
            
            // Brightness value text
            let value_text = parent.spawn(TextBundle::from_section(
                format!("{:.0}%", settings.brightness * 100.0),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 14.0,
                    color: TEXT_COLOR,
                },
            )).id();
            
            // Add slider to state management
            // state.sliders.push(slider);
        });
    });
}

/// Spawn graphics settings section
fn spawn_graphics_section(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    settings: &GameSettings,
    state: &mut VideoSettingsState,
    content_container: Entity,
) {
    parent.entity(content_container).with_children(|parent| {
        let graphics_section = parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(15.0)),
                        margin: UiRect::vertical(Val::Px(10.0)),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: Color::rgba(0.1, 0.1, 0.1, 0.5).into(),
                    ..default()
                },
                Name::new("GraphicsSection"),
            )).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Graphics Settings",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.0,
                        color: TEXT_COLOR,
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
                
                // Add graphics quality preset dropdown
                let quality_container = parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            margin: UiRect::vertical(Val::Px(10.0)),
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("QualityPresetContainer"),
                )).id();
                
                // Add quality preset label
                parent.entity(quality_container).with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Quality Preset",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-SemiBold.ttf"),
                            font_size: 16.0,
                            color: TEXT_COLOR,
                        },
                    ));
                });
                
                // Add quality preset dropdown
                let quality_dropdown = parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(40.0),
                            margin: UiRect::top(Val::Px(5.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        border_color: Color::GRAY.into(),
                        background_color: Color::DARK_GRAY.into(),
                        ..default()
                    },
                    Name::new("QualityDropdown"),
                    Interaction::None,
                    Focusable {
                        node_type: FocusableType::Button,
                        focus_state: FocusState::NotFocused,
                    },
                )).with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Custom", // Default to Custom if no preset matches
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                            font_size: 16.0,
                            color: TEXT_COLOR,
                        },
                    ));
                }).id();
                
                // Add dropdown to state management
                // state.dropdowns.push(quality_dropdown);
                
            // Advanced section
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(15.0)),
                        margin: UiRect::vertical(Val::Px(10.0)),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: Color::rgba(0.1, 0.1, 0.1, 0.5).into(),
                    ..default()
                },
                Name::new("AdvancedSection"),
            )).with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Advanced Settings",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: TEXT_COLOR,
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
                    
                    // FPS Limit toggle and input
                    let fps_container = parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceBetween,
                                margin: UiRect::vertical(Val::Px(10.0)),
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("FPSLimitContainer"),
                    )).id();
                    
                // FPS Limit label
                parent.entity(fps_container).with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "FPS Limit",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-SemiBold.ttf"),
                            font_size: 16.0,
                            color: TEXT_COLOR,
                        },
                    ));
                });
                
                // FPS Limit toggle and input field
                parent.entity(fps_container).with_children(|parent| {
                    let fps_toggle = parent.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(80.0),
                                height: Val::Px(30.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: if settings.fps_limit.is_some() {
                                PRESSED_BUTTON.into()
                            } else {
                                NORMAL_BUTTON.into()
                            },
                            ..default()
                        },
                        UIButton::default()
                            .with_colors(
                                NORMAL_BUTTON,
                                HOVERED_BUTTON,
                                PRESSED_BUTTON,
                                DISABLED_BUTTON,
                            ),
                        Interaction::None,
                        Focusable {
                            node_type: FocusableType::Button,
                            focus_state: FocusState::NotFocused,
                        },
                        Name::new("FPSLimitToggle"),
                    )).with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            if settings.fps_limit.is_some() { "On" } else { "Off" },
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                                font_size: 16.0,
                                color: TEXT_COLOR,
                            },
                        ));
                    }).id();
                    
                    // Add to state management
                    // state.buttons.push(fps_toggle);
                });
            });
            
            // VSync toggle
            let vsync_row = create_setting_row(parent, "VSync", asset_server);
            parent.entity(vsync_row).with_children(|parent| {
                let vsync_checkbox = parent.spawn((
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
                        background_color: if settings.vsync {
                            PRESSED_BUTTON.into()
                        } else {
                            NORMAL_BUTTON.into()
                        },
                        ..default()
                    },
                    UICheckbox {
                        checked: settings.vsync,
                        checkbox_type: CheckboxType::VSync,
                    },
                    Interaction::None,
                    Focusable {
                        focusable_type: FocusableType::Checkbox,
                        state: FocusState::Unfocused,
                    },
                    Name::new("VSyncCheckbox"),
                )).with_children(|parent| {
                    if settings.vsync {
                        parent.spawn(TextBundle::from_section(
                            "✓",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 16.0,
                                color: TEXT_COLOR,
                            },
                        ));
                    }
                }).id();
                
                // Add to state management
                // state.checkboxes.push(vsync_checkbox);
            });
            
            // FPS Limit
            let fps_row = create_setting_row(parent, "FPS Limit", asset_server);
            parent.entity(fps_row).with_children(|parent| {
                // FPS limit slider container
                let fps_slider = parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(300.0),
                            height: Val::Px(40.0),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("FPSSliderContainer"),
                )).id();
                
                // Slider track
                let track = parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(6.0),
                            margin: UiRect::right(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: Color::DARK_GRAY.into(),
                        ..default()
                    },
                    Name::new("FPSSliderTrack"),
                )).id();
                
                // Slider handle
                let handle = parent.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(16.0),
                            height: Val::Px(16.0),
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                    UIButton::default()
                        .with_colors(
                            Color::WHITE,
                            Color::rgb(0.9, 0.9, 0.9),
                            Color::rgb(0.8, 0.8, 0.8),
                            Color::rgb(0.5, 0.5, 0.5),
                        ),
                    Name::new("FPSSliderHandle"),
                    Interaction::None,
                )).id();
                
                // Slider value display
                let value_text = parent.spawn((
                    TextBundle::from_section(
                        format!("{} FPS", settings.fps_limit.unwrap_or(60)),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                            font_size: 14.0,
                            color: TEXT_COLOR,
                        },
                    ),
                    Name::new("FPSValueText"),
                )).id();
                
                // Create the slider component
                let slider = SettingsSlider {
                    min: 30.0,
                    max: 360.0,
                    value: settings.fps_limit.unwrap_or(60) as f32,
                    step: 1.0,
                    slider_type: SliderType::FPSLimit,
                    dragging: false,
                    track_entity: Some(track),
                    handle_entity: Some(handle),
                    value_text_entity: Some(value_text),
                };
                
                // Add the slider to state management
                // state.sliders.push(parent.spawn((slider,)).id());
                
                // Add children to container
                parent.entity(fps_slider).push_children(&[track, handle, value_text]);
            });
            
            // Additional graphics settings
            let additional_settings = [
                ("Ambient Occlusion", settings.ambient_occlusion, "On", "Off"),
                ("Depth of Field", settings.depth_of_field, "On", "Off"),
                ("Motion Blur", settings.motion_blur, "On", "Off"),
                ("Bloom", settings.bloom, "On", "Off"),
                ("Lens Flare", settings.lens_flare, "On", "Off"),
            ];
            
            for (label, value, on_text, off_text) in additional_settings.iter() {
                let row = create_setting_row(parent, label, asset_server);
                parent.entity(row).with_children(|parent| {
                    let checkbox = parent.spawn((
                        SettingsCheckbox {
                            checked: *value,
                            checkbox_type: match *label {
                                "Ambient Occlusion" => CheckboxType::AmbientOcclusion,
                                "Depth of Field" => CheckboxType::DepthOfField,
                                "Motion Blur" => CheckboxType::MotionBlur,
                                "Bloom" => CheckboxType::Bloom,
                                "Lens Flare" => CheckboxType::LensFlare,
                                _ => CheckboxType::VSync, // Default fallback
                            },
                        },
                        Interaction::None,
                        Focusable {
                            focusable_type: FocusableType::Checkbox,
                            state: FocusState::Unfocused,
                        },
                        Name::new(format!("{}Checkbox", label.replace(" ", ""))),
                    )).with_children(|parent| {
                        if *value {
                            parent.spawn(TextBundle::from_section(
                                *on_text,
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                                    font_size: 16.0,
                                    color: TEXT_COLOR,
                                },
                            ));
                        } else {
                            parent.spawn(TextBundle::from_section(
                                *off_text,
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                                    font_size: 16.0,
                                    color: TEXT_COLOR,
                                },
                            ));
                        }
                    }).id();
                    
                    // Add to state management
                    // state.checkboxes.push(checkbox);
                });
            }
        });
    });
}

// Add a helper function to create setting rows
fn create_setting_row(parent: &mut ChildBuilder, label: &str, asset_server: &Res<AssetServer>) -> Entity {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::horizontal(Val::Px(10.0)),
                ..default()
            },
            ..default()
        },
        Name::new(format!("{}Row", label.replace(" ", ""))),
    )).with_children(|parent| {
        // Setting label
        parent.spawn(TextBundle::from_section(
            label,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-SemiBold.ttf"),
                font_size: 16.0,
                color: TEXT_COLOR,
            },
        ));
    }).id()
}

pub struct VideoPlugin;

impl Plugin for VideoPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<VideoSettingsState>()
            .add_systems(Update, (
                handle_video_settings_keyboard_navigation
                    .after(focus_navigation_system),
                handle_video_interaction_feedback
                    .after(focus_visual_system),
            ).run_if(in_state(SettingsTab::Video)));
    }
}
