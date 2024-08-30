use hyprland::{data::Devices, shared::HyprData};
use std::str::pattern::Pattern;

/// Get the current keyboard layout name.
///
/// Hyprland treats a lot of other devices as keyboards, for example:
///
/// ```not-rust
/// ╭───┬────────────┬────────────────────────────────┬────────╮
/// │ # │  address   │              name              │ layout │
/// ├───┼────────────┼────────────────────────────────┼────────┤
/// │ 0 │ 0x38b02830 │ power-button                   │ us,ru  │
/// │ 1 │ 0x38bf4390 │ video-bus                      │ us,ru  │
/// │ 2 │ 0x38c48c70 │ power-button-1                 │ us,ru  │
/// │ 3 │ 0x38cb1a10 │ company--usb-device-           │ us,ru  │
/// │ 4 │ 0x38d18820 │ company--usb-device--keyboard  │ us,ru  │
/// │ 5 │ 0x38d9a740 │ asue140d:00-04f3:31b9-keyboard │ us,ru  │
/// │ 6 │ 0x38de9c60 │ intel-hid-events               │ us,ru  │
/// │ 7 │ 0x38e50180 │ intel-hid-5-button-array       │ us,ru  │
/// │ 8 │ 0x38eb7330 │ asus-wmi-hotkeys               │ us,ru  │
/// │ 9 │ 0x38f1e570 │ at-translated-set-2-keyboard   │ us,ru  │
/// ╰───┴────────────┴────────────────────────────────┴────────╯
/// ```
///
/// Out of all of these, only the last one is the one you most likely
/// actually want, so that's why we need a pattern.
///
/// # Errors
///
/// This function will propagate any errors that occur while
/// querying the compositor through the [`hyprland`] crate.
///
/// This function may return `Ok(None)` if none of the keyboards'
/// names returned by the compositor match the provided pattern.
#[allow(clippy::needless_pass_by_value)]
pub fn get<P>(keyboard_name_pattern: P) -> hyprland::Result<Option<String>>
where
    P: Pattern + Clone,
{
    let mut keyboards = Devices::get()?.keyboards;

    keyboards.retain(|keyboard| keyboard.name.contains(keyboard_name_pattern.clone()));

    match keyboards.first() {
        None => Ok(None),
        Some(keyboard) => Ok(Some(
            keyboard
                .active_keymap
                .split_whitespace()
                .next()
                .map(ToString::to_string)
                .unwrap_or_default(),
        )),
    }
}
