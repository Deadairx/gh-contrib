# gh-contrib
display GitHub contribution grid in CLI

## Direction

This repo started as a CLI-first experiment. It now also serves as the likely source of reusable Rust GitHub contribution logic for a future native macOS widget project.

Planned long-term shape:

- extract GitHub fetching and normalization into a reusable Rust core crate
- keep terminal rendering in a CLI-specific crate or module
- add a future UniFFI-facing crate so Swift can consume the same Rust domain logic

See `ROADMAP.md` for the planned extraction path.

## Planned usage

run `gh-contrib [user]` to display the contribution grid of user.
(if user is not provided, pulls from `GH_USER` or displays help)

by default it will show █ blocks of different shades of green depending on amount
of contributions just like GitHub displays on the user's page.

### Arugments to implement
- `-y <year>` show grid of provided year
- `--no-color` use ANSI characters instead (eg █ ▓ ▒ ░)
- `-s --short` show last 8 weeks
- `-c <color>` use color pallet (options: `red`, `blue`, `green`, `orange`, `yellow`, `pink`)

