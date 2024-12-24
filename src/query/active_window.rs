use hyprland::data::Client;
use hyprland::shared::HyprDataActiveOptional;

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn get() -> hyprland::Result<Option<Client>> {
    Client::get_active()
}
