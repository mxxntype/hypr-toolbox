use hyprland::data::{Workspace, Workspaces};
use hyprland::shared::{Address, HyprData};

/// Get the list of workspaces the compositor holds.
///
/// # Errors
///
/// This function will propagate any errors that occur while
/// querying the compositor through the [`hyprland`] crate.
pub fn get(skip_missing: bool) -> hyprland::Result<Vec<Workspace>> {
    let mut workspaces: Vec<Workspace> =
        Workspaces::get().map(|workspaces| workspaces.into_iter().collect())?;

    for id in 1..=10 {
        if !skip_missing && !workspaces.iter().any(|ws| ws.id == id) {
            workspaces.push(Workspace {
                id,
                name: "Placeholder workspace".into(),
                monitor: "Placeholder monitor".into(),
                windows: 0,
                fullscreen: false,
                last_window: Address::new(String::new()),
                last_window_title: String::new(),
                monitor_id: 0,
            });
        }
    }

    workspaces.sort_unstable_by_key(|workspace| workspace.id);

    Ok(workspaces)
}
