# Advent-of-Code 2020

Using the Advent-of-Code 2020 to polish up on my Rust skills

## Repo Layout

```
.
└── day{N}
    ├── input
    │   └── input.txt  # contains the input for the day's challenge
    └── src
        └── main.rs    # Entrypoint for day{N}'s challenge where 1 <= N <= 25
```

## Usage

### With [`nix`](https://nixos.org/)

From the root of this repository

```bash
# Replace `$NDAY` below with the day's challenge you'd like to run
NDAY=$NDAY cat "./day${NDAY}/input/input.txt" | nix run ".#day${NDAY}"
```

### Without [`nix`](https://nixos.org/)

> [!NOTE]
> To run the binary corresponding to each day in this project without `nix`, you'll need to have [`cargo`](https://github.com/rust-lang/cargo) installed via your preferred package manager and available in your `$PATH`

From inside the `day{N}` directory.

```bash
cat './input/input.txt' | cargo run
```
