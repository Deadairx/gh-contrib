# gh-contrib
display GitHub contribution grid in CLI

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


