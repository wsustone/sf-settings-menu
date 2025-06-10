use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    // Add other relevant rerun-if-changed lines if specific files impact the build output.

    // Get the output directory from Cargo (where build script outputs go)
    let out_dir = env::var("OUT_DIR").unwrap();

    // Determine the profile (debug or release)
    let profile = env::var("PROFILE").unwrap(); // "debug" or "release"

    // Construct the name of the DLL
    let crate_name = env::var("CARGO_PKG_NAME").unwrap(); // sf-settings-menu
    // Convert crate name to library file name convention (e.g., sf_settings_menu.dll)
    let dll_name = format!("{}.dll", crate_name.replace("-", "_"));

    // Define the destination directory for plugins within the workspace target directory
    // e.g., StrategyForge-Workspace/target/<profile>/plugins/
    let workspace_root = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent() // Up to StrategyForge-Workspace
        .unwrap()
        .to_path_buf();

    let target_plugins_dir = workspace_root
        .join("target")
        .join(&profile) // Append profile (debug/release)
        .join("plugins");

    // Create the target plugins directory if it doesn't exist
    if !target_plugins_dir.exists() {
        fs::create_dir_all(&target_plugins_dir).expect("Failed to create plugins directory");
    }

    // Calculate the paths where the DLL might be located
    // We'll check multiple potential locations to ensure we find it
    let out_dir_path = Path::new(&out_dir);
    
    // Option 1: Direct in target/<profile>
    let path1 = out_dir_path
        .ancestors()
        .nth(3) // Up to target/<profile>
        .unwrap()
        .join(&dll_name);
    
    // Option 2: In target/<profile>/deps/
    let path2 = out_dir_path
        .ancestors()
        .nth(3) // Up to target/<profile>
        .unwrap()
        .join("deps") // target/<profile>/deps/
        .join(&dll_name);
        
    // Option 3: In the current crate's target/<profile>
    let path3 = workspace_root
        .join(&crate_name)
        .join("target")
        .join(&profile)
        .join(&dll_name);
    
    // Use the first path that exists
    let source_dll_path = if path1.exists() {
        path1
    } else if path2.exists() {
        path2
    } else if path3.exists() {
        path3
    } else {
        // Try one last path - sometimes it might be directly in the deps directory
        out_dir_path
            .ancestors()
            .nth(2) // Up to target/<profile>/build
            .unwrap()
            .parent() // Up to target/<profile>
            .unwrap()
            .join(&dll_name)
    };
    
    println!("cargo:warning=Looking for DLL in: {:?}", source_dll_path);

    let destination_path = target_plugins_dir.join(&dll_name);

    // Copy the DLL to the target plugins directory
    if source_dll_path.exists() {
        fs::copy(&source_dll_path, &destination_path)
            .expect(&format!("Failed to copy DLL from {:?} to {:?}", source_dll_path, destination_path));
        println!("cargo:warning=Copied {} to {:?}", dll_name, target_plugins_dir);
    } else {
        println!("cargo:warning=DLL not found at {:?}. Plugin will not be copied.", source_dll_path);
        println!("cargo:warning=Ensure that crate-type in Cargo.toml is [\"cdylib\"].");
        println!("cargo:warning=Crate name: {}, Expected DLL name: {}", crate_name, dll_name);
        println!("cargo:warning=Profile: {}, OUT_DIR: {}", profile, out_dir);
    }
    
    // Specify the export function name(s) for the linker.
    // For sf-settings-menu, the dynamic loader expects 'create_settings_plugin'.
    println!("cargo:rustc-cdylib-link-arg=/EXPORT:create_settings_plugin");
    // If 'create_plugin' is also a valid generic entry point for this plugin:
    // println!("cargo:rustc-cdylib-link-arg=/EXPORT:create_plugin"); 
}
