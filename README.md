# octui

A terminal application that displays your GitHub contribution graph, built with Ratatui.

<img width="1900" height="670" alt="image" src="https://github.com/user-attachments/assets/a571c0c5-ed50-4acc-a631-0f8cc6240591" />
<img width="1926" height="622" alt="image" src="https://github.com/user-attachments/assets/b79dd164-a3d0-4038-8dba-ca98acafb26c" />


## Features

- Displays the GitHub contribution graph with the official dark theme colour palette
- Navigate between days using vim-style keybindings
- View contribution details for any selected day
- Browse contribution history across different years
- Auto-refreshes data every 5 minutes when viewing the current year

## Requirements

- [GitHub CLI](https://cli.github.com/) (`gh`) installed and authenticated

## Installation

### Homebrew (macOS)

```sh
brew tap abosnjakovic/octui https://github.com/abosnjakovic/octui
brew install octui
```

### Cargo

```sh
cargo install octui
```

### From source

```sh
git clone https://github.com/abosnjakovic/octui
cd octui
cargo install --path .
```

## Usage

```sh
# Display your own contribution graph
octui

# Display another user's contribution graph
octui --user octocat
```

## Keybindings

| Key | Action |
|-----|--------|
| `h` `j` `k` `l` / Arrow keys | Navigate between days |
| `p` | Previous year |
| `n` | Next year |
| `?` | Toggle help |
| `q` / `Esc` | Quit |

## Colour Palette

The contribution levels use GitHub's dark theme colours:

| Level | Colour |
|-------|--------|
| None | #161B22 |
| First quartile | #0E4429 |
| Second quartile | #006D32 |
| Third quartile | #26A641 |
| Fourth quartile | #39D353 |

## License

MIT
