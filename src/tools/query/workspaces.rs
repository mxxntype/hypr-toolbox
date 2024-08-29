use hyprland::data::{Workspace, Workspaces};
use hyprland::shared::{Address, HyprData};

/// Get the list of workspaces the compositor holds.
///
/// By default, Hyprland doesn't return workspaces that have no
/// open windows, which might be inconvenient for some usecases.
/// This function inserts "dummy" workspaces so that the user
/// always gets back all workspaces 1-10.
/// Pass `skip_missing = false` to disable this behaviour.
///
/// # Errors
///
/// This function will propagate any errors that occur while
/// querying the compositor through the [`hyprland`] crate.
///
/// # Examples
///
/// ```rust
/// // This doctest will fail if Hyprland is not running!
///
/// use hyprland::data::{Workspace, Workspaces};
/// use hyprland::shared::{Address, HyprData};
/// use hypr_toolbox::tools::query::workspaces;
///
/// # fn main() -> hyprland::Result<()> {
/// let workspaces: Vec<Workspace> = workspaces::get(false)?;
///
/// assert!(!workspaces.is_empty());
/// assert_eq!(workspaces.len(), 10);
/// # Ok(())
/// # }
/// ```
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
