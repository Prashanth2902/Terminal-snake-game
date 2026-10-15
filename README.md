# Terminal-snake-game

A classic snake game for the terminal, written in Rust using [crossterm](https://crates.io/crates/crossterm).

## Run

```bash
cargo run --release
```

## Controls

| Key | Action |
| --- | --- |
| Arrow keys / WASD | Move |
| P / Space | Pause |
| R | Restart after game over |
| Q / Esc | Quit |

The snake speeds up as your score grows. Hitting a wall or yourself ends the game.
