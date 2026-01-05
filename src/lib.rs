pub mod control;
pub mod kinematics;
pub mod plugin;
pub mod spawn;
pub mod uav;
pub mod urdf_asset_loader;
pub mod uuv;

pub use plugin::*;
pub use spawn::*;

/// Returns whether BEVY_ASSET_ROOT environment variable is set
pub fn has_bevy_asset_root() -> bool {
    std::env::var("BEVY_ASSET_ROOT").is_ok()
}

/// Converts Bevy asset path to filesystem path (for collider generation)
///
/// When BEVY_ASSET_ROOT is set, adds `assets/` prefix if not present.
/// This is required because rapier3d_urdf expects actual filesystem paths.
pub fn to_filesystem_path(mesh_dir: &str) -> String {
    if has_bevy_asset_root() && !mesh_dir.starts_with("assets/") {
        format!("assets/{}", mesh_dir)
    } else {
        mesh_dir.to_string()
    }
}

/// Converts filesystem path to Bevy asset path (for visual mesh loading)
///
/// When BEVY_ASSET_ROOT is not set, removes `assets/` prefix.
/// This is required because Bevy's asset_server looks in `assets/` directory by default.
pub fn to_asset_path(mesh_dir: &str) -> String {
    if has_bevy_asset_root() {
        mesh_dir.to_string()
    } else {
        mesh_dir.replace("assets/", "")
    }
}
