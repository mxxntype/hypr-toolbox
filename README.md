<!-- cargo-rdme start -->

## `hypr-toolbox`

An abomination of various scripts and actions for Hyprland, packaged as a single Rust CLI tool.
Run the binary with `--help` to figure out how to use the different tools.

## `query` - Query the compositor for information, once or subscribe-style.

```console
Usage: hypr-toolbox query [OPTIONS] <COMMAND>
Commands:
  active-window     Get the currently focused window
  active-workspace  Get the ID of the currently focused workspace
  keyboard-layout   Get the current keyboard layout name
  workspaces        Get the list of workspaces the compositor holds
```

## `profile` - Detect user actions and adapt to them on-the-fly.

The configuration for this tool lives in `${XDG_CONFIG_HOME}/hypr_toolbox/profile.json`
and looks like this:

```json
{
    "output": "eDP-1",
    "default_scale": 1.5,
    "workspace_scale_map": {
        "7": 2.0,
        "10": 1.0
    },
    "xwayland_scaling_workspaces": [7]
}
```

Explanation: by default on every workspaces, use a monitor scale of 1.5.
If the user goes to workspace 7, set the scale to 2.0 and reset to 1.5 once
the user goes to any other workspace. Same for workspace 10, except set the
scale to 1.0 when there.
Also, while on workspace 7, disable the `xwayland:force_zero_scaling` option.
(Useful to force some games to run at a lower resolution)

<!-- cargo-rdme end -->
