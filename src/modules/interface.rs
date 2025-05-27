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

/// Spawn interface settings content
pub fn spawn_interface_settings(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, settings: &GameSettings) {
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

    // Add the interface settings content to the scrollable area
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
                SettingsTab::Interface,
            ))
            .with_children(|parent| {
                // UI Scale section
                let scale_section = utils::create_section(parent, asset_server, "UI Scale & Layout");
                
                // Add UI scale settings to the section
                parent.get_entity_mut(scale_section).unwrap().with_children(|parent| {
                    // UI Scale
                    parent.spawn(TextBundle::from_section(
                        "UI Scale",
                        styles::subsection_title_style(asset_server),
                    ));
                    
                    // UI scale slider would go here
                    
                    // HUD Layout
                    parent.spawn(TextBundle::from_section(
                        "HUD Layout",
                        styles::subsection_title_style(asset_server),
                    ));
                    
                    // HUD layout options would go here
                    
                    // Minimap Position
                    parent.spawn(TextBundle::from_section(
                        "Minimap Position",
                        styles::subsection_title_style(asset_server),
                    ));
                    
                    // Minimap position options would go here
                });
                
                // Information Display section
                let info_section = utils::create_section(parent, asset_server, "Information Display");
                
                // Add information display settings to the section
                parent.get_entity_mut(info_section).unwrap().with_children(|parent| {
                    // Show Tooltips
                    parent.spawn(TextBundle::from_section(
                        "Show Tooltips",
                        styles::subsection_title_style(asset_server),
                    ));
                    
                    // Show tooltips toggle would go here
                    
                    // Damage Numbers
                    parent.spawn(TextBundle::from_section(
                        "Damage Numbers",
                        styles::subsection_title_style(asset_server),
                    ));
                    
                    // Damage numbers toggle would go here
                    
                    // Health Bars
                    parent.spawn(TextBundle::from_section(
                        "Health Bars",
                        styles::subsection_title_style(asset_server),
                    ));
                    
                    // Health bars toggle would go here
                });
                
                // Chat & Social section
                let chat_section = utils::create_section(parent, asset_server, "Chat & Social");
                
                // Add chat & social settings to the section
                parent.get_entity_mut(chat_section).unwrap().with_children(|parent| {
                    // Chat Window
                    parent.spawn(TextBundle::from_section(
                        "Chat Window",
                        styles::subsection_title_style(asset_server),
                    ));
                    
                    // Chat window options would go here
                    
                    // Social Notifications
                    parent.spawn(TextBundle::from_section(
                        "Social Notifications",
                        styles::subsection_title_style(asset_server),
                    ));
                    
                    // Social notification options would go here
                });
                
                // Accessibility section
                let accessibility_section = utils::create_section(parent, asset_server, "Accessibility");
                
                // Add accessibility settings to the section
                parent.get_entity_mut(accessibility_section).unwrap().with_children(|parent| {
                    // Colorblind Mode
                    parent.spawn(TextBundle::from_section(
                        "Colorblind Mode",
                        styles::subsection_title_style(asset_server),
                    ));
                    
                    // Colorblind mode options would go here
                    
                    // Text Size
                    parent.spawn(TextBundle::from_section(
                        "Text Size",
                        styles::subsection_title_style(asset_server),
                    ));
                    
                    // Text size slider would go here
                    
                    // High Contrast Mode
                    parent.spawn(TextBundle::from_section(
                        "High Contrast Mode",
                        styles::subsection_title_style(asset_server),
                    ));
                    
                    // High contrast mode toggle would go here
                });
                
                // Reset to Defaults button
                parent.spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(40.0),
                        margin: UiRect::top(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgb(0.5, 0.1, 0.1)),
                    ..default()
                }).with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Reset to Defaults",
                        styles::button_text_style(asset_server),
                    ));
                });
            });
    });
}
