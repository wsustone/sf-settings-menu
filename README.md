# Settings Menu Plugin for StrategyForge

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Bevy](https://img.shields.io/badge/Bevy-0.14-blue)](https://bevyengine.org)

A modular settings menu plugin for the StrategyForge RTS game, implemented as a Bevy plugin.

## Features

- 🛠️ Plugin-based architecture
- 🎨 Pre-styled UI components
- ⚙️ Expandable settings system
- 🏗️ Built on StrategyForge Core UI components

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
sf-settings-menu = { path = "../sf-settings-menu" }
```

## Usage

```rust
use bevy::prelude::*;
use sf_settings_menu::SettingsMenuPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SettingsMenuPlugin)
        .run();
}
```

## Plugin Architecture

The menu system uses StrategyForge's plugin interface:

```rust
impl MenuItemPlugin for SettingsMenuPlugin {
    fn add_menu_item(&self, app: &mut App, parent: Entity) {
        // Adds settings button to parent menu
    }
}
```

## Customizing Settings

To add new settings:
1. Create new UI components
2. Implement settings logic
3. Register with the settings panel

## Contributing

Pull requests welcome! Please follow:
- StrategyForge's coding standards
- Bevy's ECS patterns
- Keep UI consistent with the game's style

## License

MIT - See [LICENSE](LICENSE)
