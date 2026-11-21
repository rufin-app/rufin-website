# Themes

Rufin relies on GTK, so if you have a custom GTK4 theme on your system, Rufin will pick on it.

If you wish to have a theme just for rufin, go to `Preferences -> Theme` and click on the folder icon in the top-right corner. This will open up the rufin themes directory, that you can add your theme into.

On Linux this directory is at `~/.config/rufin`.

## Making custom themes
To create a custom theme, first create a file in themes folder in `json` format.

Then, you can add the default dark theme json into it and play with the values.
```json
{
  "name": "Dark",
  "mode": "dark",
  "colors": {
    "window-bg-color": "#222226",
    "window-fg-color": "#ffffff",
    "view-bg-color": "#1d1d20",
    "view-fg-color": "#ffffff",
    "headerbar-bg-color": "#2e2e32",
    "headerbar-fg-color": "#ffffff",
    "headerbar-border-color": "#ffffff",
    "headerbar-backdrop-color": "#28282c",
    "headerbar-shade-color": "rgb(0 0 6 / 36%)",
    "headerbar-darker-shade-color": "rgb(0 0 12 / 90%)",
    "sidebar-bg-color": "#2e2e32",
    "sidebar-fg-color": "#ffffff",
    "sidebar-backdrop-color": "#28282c",
    "sidebar-shade-color": "rgb(0 0 6 / 25%)",
    "sidebar-border-color": "rgb(0 0 6 / 36%)",
    "secondary-sidebar-bg-color": "#28282c",
    "secondary-sidebar-fg-color": "#ffffff",
    "secondary-sidebar-backdrop-color": "#252529",
    "secondary-sidebar-shade-color": "rgb(0 0 6 / 25%)",
    "secondary-sidebar-border-color": "rgb(0 0 6 / 36%)",
    "card-bg-color": "rgb(255 255 255 / 8%)",
    "card-fg-color": "#ffffff",
    "card-shade-color": "rgb(0 0 6 / 36%)",
    "dialog-bg-color": "#36363a",
    "dialog-fg-color": "#ffffff",
    "popover-bg-color": "#36363a",
    "popover-fg-color": "#ffffff",
    "popover-shade-color": "rgb(0 0 6 / 25%)",
    "thumbnail-bg-color": "#39393d",
    "thumbnail-fg-color": "#ffffff",
    "shade-color": "rgb(0 0 6 / 25%)",
    "scrollbar-outline-color": "rgb(0 0 12 / 95%)",
    "active-toggle-bg-color": "rgb(255 255 255 / 20%)",
    "active-toggle-fg-color": "#ffffff",
    "overview-bg-color": "#28282c",
    "overview-fg-color": "#ffffff",
    "standalone-color-oklab": "max(l, 0.85) a b"
  }
}
```

Note that if your theme looks nice, a PR can be made to later add it into the main Rufin themes.
