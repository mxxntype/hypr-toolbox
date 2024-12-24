use hyprland::data::Workspace;
use hyprland::shared::HyprDataActive;

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn get() -> hyprland::Result<Workspace> {
    Workspace::get_active()
}
