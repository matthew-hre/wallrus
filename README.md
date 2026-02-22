# Wallrus

A GNOME application for generating abstract wallpapers using GPU shaders.

Browse curated color palettes, pick a shader pattern, tweak the parameters, and
export a crisp wallpaper at up to 4K — or set it as your desktop background in
one click.

## Features

- **5 shader presets** — Bars, Circle, Plasma, Waves, and Terrain, each with
  dedicated parameters (angle, scale, time scrub, center position)
- **1,600+ bundled palette images** across 10 categories (cold, dark, fall,
  gradient, light, pastel, retro, sunset, warm, winter)
- **Blend control** — go from hard flag-like stripes to fully smooth gradients
- **Effects** — swirl distortion, film grain noise, and ordered Bayer dithering
  for a retro look
- **Export** — PNG or JPEG at 1080p, 1440p, or 4K (default auto-detected from
  your display)
- **Set as wallpaper** — writes to `~/.local/share/backgrounds/` and applies
  via `gsettings` for both light and dark mode
- **Custom palettes** — drop 400×400 px palette images into
  `~/.local/share/wallrus/palettes/<category>/` and they appear automatically
- **Keyboard shortcuts** — Ctrl+E (export PNG), Ctrl+Shift+E (export JPEG),
  Ctrl+Shift+W (set as wallpaper)

## Requirements

- GTK 4 (≥ 4.10)
- libadwaita (≥ 1.4)
- OpenGL 3.3+ capable GPU
- Rust 1.70+

System packages (Fedora):

```
sudo dnf install gtk4-devel libadwaita-devel
```

System packages (Ubuntu/Debian):

```
sudo apt install libgtk-4-dev libadwaita-1-dev
```

## Building

```
cargo build --release
```

The binary is at `target/release/wallrus`.

## Installing

The included install script builds a release binary and copies everything to
`~/.local` (binary, desktop file, icon, metainfo, and bundled palettes):

```
./install.sh
```

To install to a different prefix:

```
PREFIX=/usr/local ./install.sh
```

You may need to log out and back in for the application icon to appear in your
launcher.

## Custom palettes

Palette images are 400×400 px PNGs with four horizontal color bands (100 px
each, top to bottom). Wallrus samples the center pixel of each band to extract
the four colors.

Place them in subdirectories under `~/.local/share/wallrus/palettes/`:

```
~/.local/share/wallrus/palettes/
├── mytheme/
│   ├── ocean.png
│   └── forest.png
└── another-category/
    └── fire.png
```

Subdirectory names become selectable categories in the UI (capitalized
automatically). Restart Wallrus to pick up new palettes.

## License

GPL-3.0-or-later. See [LICENSE](LICENSE) for details.
