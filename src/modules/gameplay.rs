use bevy::prelude::*;
use crate::{
    ui::{
        common::*,
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

/// Spawn gameplay settings content
pub fn spawn_gameplay_settings(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, settings: &GameSettings) {
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

    // Add the gameplay settings content to the scrollable area
    parent
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
        parent
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
                SettingsTab::Gameplay,
            ))
            .with_children(|parent| {
                // Game Rules section
                let rules_section = utils::create_section(parent, asset_server, "Game Rules");
                
                // Add game rules settings to the section
                parent.entity(rules_section).with_children(|parent| {
                    // Difficulty
                    parent.spawn(TextBundle::from_section(
                        "Difficulty",
                        styles::subsection_title_style(asset_server),
                    ));
                    
                    // Difficulty options would go here
                    
                    // Game Speed
                    parent.spawn(TextBundle::from_section(
                        "Game Speed",
                        styles::subsection_title_style(asset_server),
                    ));
                    
                    // Game speed slider would go here
                });
                
                // User Interface section
                let ui_section = utils::create_section(parent, asset_server, "User Interface");
                
                // Add UI settings to the section
                parent.entity(ui_section).with_children(|parent| {
                    // Show Tutorials
                    parent.spawn(TextBundle::from_section(
                        "Show Tutorials",
                        styles::subsection_title_style(asset_server),
                    ));
                    
                    // Show tutorials toggle would go here
                    
                    // Tooltip Delay
                    parent.spawn(TextBundle::from_section(
                        "Tooltip Delay",
                        styles::subsection_title_style(asset_server),
                    ));
                    
                    // Tooltip delay slider would go here
                });
                
                // Notifications section
                let notifications_section = utils::create_section(parent, asset_server, "Notifications");
                
                // Add notification settings to the section
                parent.entity(notifications_section).with_children(|parent| {
                    // Enable Notifications
                    parent.spawn(TextBundle::from_section(
                        "Enable Notifications",
                        styles::subsection_title_style(asset_server),
                    ));
                    
                    // Enable notifications toggle would go here
                    
                    // Notification Types
                    parent.spawn(TextBundle::from_section(
                        "Notification Types",
                        styles::subsection_title_style(asset_server),
                    ));
                    
                    // Notification type checkboxes would go here
                });
                
                // Save & Load section
                let save_section = utils::create_section(parent, asset_server, "Save & Load");
                
                // Add save/load settings to the section
                parent.entity(save_section).with_children(|parent| {
                    // Auto-save
                    parent.spawn(TextBundle::from_section(
                        "Auto-save",
                        styles::subsection_title_style(asset_server),
                    ));
                    
                    // Auto-save toggle and interval would go here
                    
                    // Cloud Saves
                    parent.spawn(TextBundle::from_section(
                        "Cloud Saves",
                        styles::subsection_title_style(asset_server),
                    ));
                    
                    // Cloud saves toggle would go here
                });
            });
    });
}
