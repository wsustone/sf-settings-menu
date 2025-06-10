use crate::SettingsMenuPlugin;
use crate::SfPluginHandle;

// Export the plugin creation function for dynamic loading
// Using a unique name to avoid symbol conflicts
#[no_mangle]
pub extern "C" fn create_settings_plugin() -> *mut SfPluginHandle {
    // Create a new plugin handle with our settings menu plugin
    let plugin = Box::new(SettingsMenuPlugin::default());
    // Create a plugin handle with the plugin cast as a GamePlugin
    let handle = Box::new(SfPluginHandle::new(plugin));
    
    // Convert to raw pointer and return
    Box::into_raw(handle)
}
